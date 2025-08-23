use std::{
    io::{BufRead, BufReader, Read, Write},
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

    let content_length = content_length
        .trim_start_matches("Content-Length: ")
        .trim()
        .parse::<usize>()?;

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line)?;

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;
    
    let body = String::from_utf8_lossy(&body);
    println!("{body}");

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
