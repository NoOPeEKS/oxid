#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::{CursorDirection, Motion, TestOxid};
    use ropey::Rope;

    #[test]
    fn move_cursor_down() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.move_cursor(CursorDirection::Down, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 1);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_up() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(0, 1);
        test_oxid.move_cursor(CursorDirection::Up, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_left() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(3, 0);
        test_oxid.move_cursor(CursorDirection::Left, 2);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            1 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_right() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.move_cursor(CursorDirection::Right, 2);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            2 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_righter_than_last_char_of_line() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.move_cursor(CursorDirection::Right, 20);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            10 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_lefter_than_first_char_of_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(3, 0);
        test_oxid.move_cursor(CursorDirection::Left, 20);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_upper_than_first_line() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.move_cursor(CursorDirection::Up, 20);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_cursor_lower_than_last_line() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.move_cursor(CursorDirection::Down, 20);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 1);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_word() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_motion(Motion::NextWord, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            5 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_end_word() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_motion(Motion::EndWord, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            3 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_start_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(7, 0);
        test_oxid.execute_motion(Motion::StartLine, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_end_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_motion(Motion::EndLine, 1);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            10 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_word_goes_to_next_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_motion(Motion::NextWord, 2);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 1);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_end_word_goes_to_next_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_motion(Motion::EndWord, 3);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 1);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            3 + app_handle.buffers[0].numbar_space
        );
    }

    #[test]
    fn move_back_word_goes_to_prev_line() {
        let mut test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis shouldn't compile xd."),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(3, 1);
        test_oxid.execute_motion(Motion::PrevWord, 2);
        let app_handle = test_oxid.app.lock().unwrap();
        assert_eq!(app_handle.buffers[0].current_position.line, 0);
        assert_eq!(
            app_handle.buffers[0].current_position.character,
            5 + app_handle.buffers[0].numbar_space
        );
    }
}
