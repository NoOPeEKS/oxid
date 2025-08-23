use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
    vec,
};

use serde_json::json;

use crate::types::{Notification, Request};

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

                // TEMPORARY
                let response_body = String::from_utf8_lossy(&response_body);
                let response_json: serde_json::Value =
                    serde_json::from_str(&response_body).unwrap();
                response_tx
                    .send(InboundMessage::Value(response_json))
                    .unwrap();
            } else {
                // Invalid message, just continue to next iter
                continue;
            }
        }
    });

    let lsp_client = LspClient {
        request_tx: request_tx.clone(),
        response_rx,
    };

    Ok(lsp_client)
}

/// LspClient struct containing everything necessary to communicate with
/// an LSP Server.
#[derive(Debug)]
pub struct LspClient {
    request_tx: std::sync::mpsc::Sender<OutboundMessage>,
    response_rx: std::sync::mpsc::Receiver<InboundMessage>,
}

/// Messages that the cliend sends to the LSP Server
pub enum OutboundMessage {
    Request(Request),
    Notification(Notification),
}

/// Messages that the client can receive from the LSP Server
#[derive(Debug)]
pub enum InboundMessage {
    Value(serde_json::Value),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        next_id,
        types::{
            ClientCapabilities, ClientInfo, HoverClientCapabilities, InitializeParams,
            TextDocumentClientCapabilities,
        },
    };

    #[test]
    fn initialize_lsp() {
        let lsp = start_lsp().unwrap();

        let initialization_params = InitializeParams {
            process_id: None,
            client_info: Some(ClientInfo {
                name: "oxid".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            locale: None,
            root_uri: Some("file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs".into()),
            initialization_options: None,
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    hover: Some(HoverClientCapabilities {
                        dynamic_registration: Some(true),
                        content_format: Some(Vec::new()),
                    }),
                }),
            },
            trace: None,
        };

        let request = Request {
            id: next_id() as i64,
            method: "initialize".to_string(),
            params: Some(serde_json::to_value(&initialization_params).unwrap()),
        };
        lsp.request_tx
            .send(OutboundMessage::Request(request))
            .unwrap();

        let response = lsp.response_rx.recv().unwrap();
        let InboundMessage::Value(val) = response;
        let response_str = serde_json::to_string_pretty(&val).unwrap();
        println!("{response_str}");

        let initialized_notification = Notification {
            method: "initialized".to_string(),
            params: None,
        };
        lsp.request_tx
            .send(OutboundMessage::Notification(initialized_notification))
            .unwrap();

        let notif = Notification {
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
              "textDocument": {
                "uri": "file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs",
                "languageId": "rust",
                "version": 1,
                "text": "pub fn main() {\n    println!(\"hello\");\n}\n"
              }
            })),
        };
        lsp.request_tx
            .send(OutboundMessage::Notification(notif))
            .unwrap();

        let response = lsp.response_rx.recv().unwrap();
        let InboundMessage::Value(val) = response;
        let response_str = serde_json::to_string_pretty(&val).unwrap();
        println!("{response_str}");
    }
}
