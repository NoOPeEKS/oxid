#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::{Motion, TestOxid};
    use ropey::Rope;

    #[test]
    fn visual_select_text() {
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
        test_oxid.visual_mode();
        test_oxid.execute_motion(Motion::NextWord, 1);
        let app_guard = test_oxid.app.lock().unwrap();
        let selection = app_guard.buffers[0].selection.clone();
        let selected_string = app_guard.buffers[0].selected_string.clone();
        assert!(selection.is_some());
        assert!(selected_string.is_some());
        assert_eq!(selected_string.unwrap(), "Hola ".to_string());
    }

    #[test]
    fn visual_yank_text() {
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
        test_oxid.visual_mode();
        test_oxid.execute_motion(Motion::NextWord, 1);
        test_oxid.yank();
        let app_guard = test_oxid.app.lock().unwrap();
        let yanked_string = app_guard.registers.get("default").unwrap();
        assert_eq!(*yanked_string, "Hola ".to_string());
    }
}
