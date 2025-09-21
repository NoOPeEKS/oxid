#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::TestOxid;
    use ropey::Rope;

    #[test]
    fn edit_char() {
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
        test_oxid.normal_mode();
        test_oxid.append_end_line("uwu");
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .file_text
                .to_string(),
            "Hola mundouwu".to_string()
        );
    }
}
