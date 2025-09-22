#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::TestOxid;
    use ropey::Rope;

    #[test]
    fn append_end_line_text() {
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

    #[test]
    fn insert_start_line_text() {
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
        test_oxid.insert_start_line("uwu");
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .file_text
                .to_string(),
            "uwuHola mundo".to_string()
        );
    }

    #[test]
    fn backspace_remove_text() {
        let mut test_oxid = TestOxid::new(
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
        test_oxid.place_cursor(5, 0);
        test_oxid.backspace(2);
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .file_text
                .to_string(),
            "Holmundo".to_string()
        );
    }

    #[test]
    fn backspace_joins_lines() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis is new"),
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
        test_oxid.place_cursor(0, 1);
        test_oxid.backspace(2);
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .file_text
                .to_string(),
            "Hola mundThis is new".to_string()
        );
    }
}
