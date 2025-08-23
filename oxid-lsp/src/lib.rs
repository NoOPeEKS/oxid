use std::{
    fmt::format,
    io::{BufRead, BufReader, Read, Write},
    process::{ChildStdin, Command, Stdio},
    sync::atomic::AtomicUsize,
};

use serde_json::json;

use crate::types::{
    ClientCapabilities, ClientInfo, HoverClientCapabilities, InitializeParams, Notification,
    Request, Response, TextDocumentClientCapabilities,
};

mod types;

static ID: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> usize {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn lsp_send_request(stdin: &mut ChildStdin, req: &Request) -> anyhow::Result<()> {
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

fn lsp_send_notification(
    stdin: &mut ChildStdin,
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

fn run_lsp() -> anyhow::Result<()> {
    let mut lsp = Command::new("rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = lsp.stdin.take().unwrap();
    let stdout = lsp.stdout.take().unwrap();

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
        params: Some(serde_json::to_value(&initialization_params)?),
    };

    lsp_send_request(&mut stdin, &request)?;

    let mut reader = BufReader::new(stdout);

    let mut content_length = String::new();
    reader.read_line(&mut content_length)?;

    let content_length = content_length
        .trim_start_matches("Content-Length: ")
        .trim()
        .parse::<usize>()?;

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line)?;

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;

    let body = String::from_utf8_lossy(&body);
    let deserialized: Response = serde_json::from_str(&body)?;
    println!("{deserialized:#?}");

    let initialized_notification = Notification {
        method: "initialized".to_string(),
        params: None,
    };

    lsp_send_notification(&mut stdin, &initialized_notification)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        run_lsp().unwrap();
    }
}
