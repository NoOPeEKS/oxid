use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
    vec,
};

use serde_json::json;

use crate::{
    get_client_capabilities,
    types::{ServerCapabilities, TextDocumentItem},
};
use crate::{
    next_id,
    types::{InitializeParams, Notification, Request, Response, ResponseError},
};

pub fn process_lsp_message(
    body: &[u8],
    rtx: &std::sync::mpsc::Sender<InboundMessage>,
) -> anyhow::Result<()> {
    let body = String::from_utf8_lossy(body);
    let res: serde_json::Value = serde_json::from_str(&body)?;

    // If it has error param, it's just a response error type.
    if let Some(error) = res.get("error") {
        let code = error["code"].as_i64().unwrap(); // Should never fail.
        let message = error["message"].as_str().unwrap();
        let data = error.get("data").cloned();

        let rsp_error = ResponseError {
            code,
            message: message.to_string(),
            data,
        };
        rtx.send(InboundMessage::Error(rsp_error))?;
        return Ok(());
    }

    // If it's got an id, its just a normal response.
    if let Some(id) = res.get("id") {
        let id = id.as_i64().unwrap();
        let result = res["result"].clone();
        let response = Response {
            id,
            result: Some(result),
            error: None,
        };
        rtx.send(InboundMessage::Response(response))?;
    } else {
        // If it hasn't got an id and has method, its a notification
        let method = res["method"].as_str().unwrap().to_string();
        let params = res["params"].clone();

        let notif = Notification {
            method,
            params: Some(params),
        };
        rtx.send(InboundMessage::Notification(notif))?
    }

    Ok(())
}

/// Take a program's Stdin and send a serialized LSP request.
pub fn lsp_send_request(stdin: &mut BufWriter<ChildStdin>, req: &Request) -> anyhow::Result<()> {
    let req = json!({
        "jsonrpc": "2.0",
        "id": req.id,
        "method": req.method,
        "params": req.params,
    });

    let body = serde_json::to_string(&req)?;
    let req = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
    stdin.write_all(req.as_bytes())?;
    stdin.flush()?;

    Ok(())
}

/// Take a program's Stdin and send a serialized LSP notification.
pub fn lsp_send_notification(
    stdin: &mut BufWriter<ChildStdin>,
    notification: &Notification,
) -> anyhow::Result<()> {
    let req = json!({
        "jsonrpc": "2.0",
        "method": notification.method,
        "params": notification.params,
    });

    let body = serde_json::to_string(&req)?;
    let req = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
    stdin.write_all(req.as_bytes())?;
    stdin.flush()?;

    Ok(())
}

pub fn start_lsp() -> anyhow::Result<LspClient> {
    let mut lsp = Command::new("rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let stdin = lsp.stdin.take().unwrap();
    let stdout = lsp.stdout.take().unwrap();

    let (request_tx, request_rx) = std::sync::mpsc::channel::<OutboundMessage>();
    let (response_tx, response_rx) = std::sync::mpsc::channel::<InboundMessage>();

    // Sends requests from the client to the LSP Server's stdin
    std::thread::spawn(move || {
        let mut stdin = BufWriter::new(stdin);
        while let Ok(message) = request_rx.recv() {
            match message {
                OutboundMessage::Request(req) => {
                    let _ = lsp_send_request(&mut stdin, &req);
                }
                OutboundMessage::Notification(not) => {
                    let _ = lsp_send_notification(&mut stdin, &not);
                }
            }
        }
    });

    // Recieves responses from the LSP Server's stdout
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);

        loop {
            let mut line = String::new();
            let read = match reader.read_line(&mut line) {
                Ok(n) => n,
                Err(_) => {
                    continue;
                }
            };

            if read == 0 {
                // 0 means EOF, we're finished.
                break;
            }

            if line.starts_with("Content-Length: ") {
                let len = match line
                    .trim_start_matches("Content-Length: ")
                    .trim()
                    .parse::<usize>()
                {
                    Ok(len) => len,
                    Err(_) => {
                        // Invalid content length, just continue.
                        continue;
                    }
                };

                let mut empty_line = String::new();
                if reader.read_line(&mut empty_line).is_err() {
                    continue;
                }

                let mut response_body = vec![0; len];
                if reader.read_exact(&mut response_body).is_err() {
                    continue;
                }

                match process_lsp_message(&response_body, &response_tx.clone()) {
                    Ok(_) => (),
                    Err(_) => {
                        continue;
                    }
                }
            } else {
                // Invalid message, just continue to next iter
                continue;
            }
        }
    });

    let lsp_client = LspClient {
        request_tx: request_tx.clone(),
        response_rx,
        server_capabilities: ServerCapabilities::default(),
        initialized: false,
    };

    Ok(lsp_client)
}

/// LspClient struct containing everything necessary to communicate with
/// an LSP Server.
#[derive(Debug)]
pub struct LspClient {
    request_tx: std::sync::mpsc::Sender<OutboundMessage>,
    response_rx: std::sync::mpsc::Receiver<InboundMessage>,
    server_capabilities: ServerCapabilities,
    initialized: bool,
}

impl LspClient {
    fn initialize(&mut self) -> anyhow::Result<()> {
        let initialize_params: InitializeParams = get_client_capabilities();

        let initialize_params = match serde_json::to_value(&initialize_params) {
            Ok(params) => params,
            Err(_) => anyhow::bail!("Error initializing LSP Client"),
        };

        let init_req_id = self.send_request("initialize", initialize_params)?;

        loop {
            let msg = match self.recv_message() {
                Ok(msg) => msg,
                Err(err) => anyhow::bail!("Could not initialize the client: {}", err),
            };

            match msg {
                InboundMessage::Response(resp) => {
                    if resp.id != init_req_id {
                        // If response id does not eq initialize req id continue.
                        continue;
                    }

                    let result = resp.result.ok_or_else(|| {
                        anyhow::anyhow!("Did not get any result attribute on initialize response")
                    })?;

                    let capabilities = result.get("capabilities").ok_or_else(|| {
                        anyhow::anyhow!(
                            "No `capabilities` attribute in initialize response `result` attribute."
                        )
                    })?;

                    let server_caps: ServerCapabilities =
                        serde_json::from_value(capabilities.clone()).map_err(|err| {
                            anyhow::anyhow!("Could not serialize server capabilities: {}", err)
                        })?;

                    self.server_capabilities = server_caps;
                    self.initialized = true;

                    // Send Initialized notification
                    self.send_notification("initialized", json!({}))?;
                    return Ok(());
                }
                InboundMessage::Error(err) => {
                    anyhow::bail!("Got a ResponseError on initialize request: {:#?}", err);
                }
                _ => continue, // Ignore notifications
            }
        }
    }

    fn did_open(&mut self, file_path: &str, file_contents: &str) -> anyhow::Result<()> {
        let params = json!({
            "textDocument": TextDocumentItem {
                uri: format!("file://{file_path}"),
                language_id: "rust".to_owned(),
                version: 1,
                text: file_contents.to_owned(),
            }
        });

        self.send_notification("textDocument/didOpen", serde_json::to_value(params)?)?;
        Ok(())
    }

    fn send_request(&mut self, method: &str, params: serde_json::Value) -> anyhow::Result<i64> {
        let req = Request {
            id: next_id() as i64,
            method: method.to_owned(),
            params: Some(params),
        };

        let id = req.id;

        self.request_tx.send(OutboundMessage::Request(req))?;

        Ok(id)
    }

    fn send_notification(&mut self, method: &str, params: serde_json::Value) -> anyhow::Result<()> {
        let noti = Notification {
            method: method.to_owned(),
            params: Some(params),
        };

        self.request_tx.send(OutboundMessage::Notification(noti))?;

        Ok(())
    }

    fn recv_message(&mut self) -> anyhow::Result<InboundMessage> {
        if let Ok(msg) = self.response_rx.recv() {
            Ok(msg)
        } else {
            anyhow::bail!("Error trying to recieve message");
        }
    }
}

/// Messages that the cliend sends to the LSP Server
pub enum OutboundMessage {
    Request(Request),
    Notification(Notification),
}

/// Messages that the client can receive from the LSP Server
#[derive(Debug)]
pub enum InboundMessage {
    Response(Response),
    Error(ResponseError),
    Notification(Notification),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next_id;
    use crate::types::*;

    #[test]
    fn initialize_lsp() {
        let mut lsp = start_lsp().unwrap();
        lsp.initialize().unwrap();
    }

    #[test]
    fn test_did_open() {
        let mut lsp = start_lsp().unwrap();
        lsp.initialize().unwrap();
        if lsp.initialized {
            let fp = String::from("/home/beri/dev/oxid/oxid-lsp/src/lib.rs");
            let fc = String::from("pub fn main() {\n println!(\"hello\");\n}\n");
            lsp.did_open(&fp, &fc).unwrap();
        }
    }

    // #[test]
    // fn hover_request() {
    //     let lsp = start_lsp().unwrap();
    //
    //     let initialization_params = InitializeParams {
    //         process_id: None,
    //         client_info: Some(ClientInfo {
    //             name: "oxid".to_string(),
    //             version: Some("0.1.0".to_string()),
    //         }),
    //         locale: None,
    //         root_uri: Some("file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs".into()),
    //         initialization_options: None,
    //         capabilities: ClientCapabilities {
    //             text_document: Some(TextDocumentClientCapabilities {
    //                 hover: Some(HoverClientCapabilities {
    //                     dynamic_registration: Some(false),
    //                     content_format: Some(vec![MarkupKind::Markdown, MarkupKind::PlainText]),
    //                 }),
    //             }),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     };
    //
    //     let initialize_request = Request {
    //         id: next_id() as i64,
    //         method: "initialize".to_string(),
    //         params: Some(serde_json::to_value(&initialization_params).unwrap()),
    //     };
    //     lsp.request_tx
    //         .send(OutboundMessage::Request(initialize_request))
    //         .unwrap();
    //
    //     let _ = lsp.response_rx.recv().unwrap(); // recieve initialize response
    //
    //     let initialized_notification = Notification {
    //         method: "initialized".to_string(),
    //         params: None,
    //     };
    //     lsp.request_tx
    //         .send(OutboundMessage::Notification(initialized_notification))
    //         .unwrap();
    //
    //     let notif = Notification {
    //         method: "textDocument/didOpen".to_string(),
    //         params: Some(json!({
    //           "textDocument": {
    //             "uri": "file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs",
    //             "languageId": "rust",
    //             "version": 1,
    //             "text": "use std::sync::atomic::AtomicUsize;\n\npub mod client;\npub mod types;\n\nstatic ID: AtomicUsize = AtomicUsize::new(1);\n\nfn next_id() -> usize {\n    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)\n}\n"
    //           }
    //         })),
    //     };
    //     lsp.request_tx
    //         .send(OutboundMessage::Notification(notif))
    //         .unwrap();
    //
    //     let _ = lsp.response_rx.recv().unwrap(); // ignore publish diagnostics
    //
    //     let req = Request {
    //         id: next_id() as i64,
    //         method: "textDocument/hover".to_string(),
    //         params: Some(json!({
    //             "textDocument": {
    //                 "uri": "file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs"
    //             },
    //             "position": {
    //                 "line": 5,
    //                 "character": 7,
    //             }
    //         })),
    //     };
    //     lsp.request_tx.send(OutboundMessage::Request(req)).unwrap();
    //
    //     loop {
    //         if let InboundMessage::Response(resp) = lsp.response_rx.recv().unwrap() {
    //             let resp_obj = serde_json::to_value(&resp).unwrap();
    //             println!("{resp_obj:#?}");
    //             if let Some(result) = resp_obj.get("result") {
    //                 if let Some(contents) = result.get("contents") {
    //                     if let Some(kind) = contents.get("kind") {
    //                         assert_eq!(kind.as_str().unwrap(), "markdown");
    //                         break;
    //                     }
    //                 }
    //             }
    //         } else {
    //             continue;
    //         }
    //     }
    // }
}
