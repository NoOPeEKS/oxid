use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use serde_json::json;

mod types;

fn run_lsp() -> anyhow::Result<()> {
    let mut lsp = Command::new("rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = lsp.stdin.take().unwrap();
    let stdout = lsp.stdout.take().unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "capabilities": {}
        }
    });

    let body = request.to_string();
    let header = format!("Content-Length: {}\r\n\r\n", body.len());

    stdin.write_all(header.as_bytes())?;
    stdin.write_all(body.as_bytes())?;
    stdin.flush()?;

    let mut reader = BufReader::new(stdout);

    let mut content_length = String::new();
    reader.read_line(&mut content_length)?;
    println!("{content_length:?}");

    // "Content-Length: 2476\r\n" is the first line
    // content_length.to_string().split("")
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
