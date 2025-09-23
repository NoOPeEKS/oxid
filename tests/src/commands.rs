#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::{Command, TestOxid};
    use ropey::Rope;

    #[test]
    fn quit_single_buffer() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_command(Command::QuitCurrentBuffer);
        assert!(test_oxid.app.lock().unwrap().buffers.is_empty());
    }
}
