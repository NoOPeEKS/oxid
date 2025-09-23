#[cfg(test)]
mod tests {
    use oxid::config::{Config, LspConfig};
    use oxid_test::{Command, TestOxid};
    use ropey::Rope;
    use std::time::Duration;

    #[test]
    fn save_single_buffer() {
        let test_oxid = TestOxid::new(
            Some("/tmp/dummy.rs".into()),
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
        test_oxid.execute_command(Command::SaveCurrentBuffer);
        std::thread::sleep(Duration::from_millis(50));
        assert!(!std::fs::read_to_string("/tmp/dummy.rs").unwrap().is_empty());
        std::thread::sleep(Duration::from_millis(50));
        std::fs::remove_file("/tmp/dummy.rs").unwrap();
    }

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

    #[test]
    fn open_multiple_buffers() {
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
        test_oxid.execute_command(Command::OpenBuffer("/tmp/path2.rs"));
        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 2);
        assert!(std::fs::exists("/tmp/path2.rs").unwrap());
        std::fs::remove_file("/tmp/path2.rs").unwrap();
    }

    #[test]
    fn close_multiple_buffers() {
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
        test_oxid.execute_command(Command::OpenBuffer("/tmp/path3.rs"));
        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 2);
        test_oxid.execute_command(Command::QuitAllBuffers);
        assert!(std::fs::exists("/tmp/path3.rs").unwrap());
        std::fs::remove_file("/tmp/path3.rs").unwrap();
        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 0);
    }

    #[test]
    fn save_multiple_buffers() {
        let test_oxid = TestOxid::new(
            Some("/tmp/whatever.rs".into()),
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
        test_oxid.execute_command(Command::OpenBuffer("/tmp/path3.rs"));
        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 2);
        test_oxid.execute_command(Command::SaveAllBuffers);
        std::thread::sleep(Duration::from_millis(50));
        assert!(std::fs::exists("/tmp/path3.rs").unwrap());
        std::fs::remove_file("/tmp/path3.rs").unwrap();
        assert!(std::fs::exists("/tmp/whatever.rs").unwrap());
        assert!(
            !std::fs::read_to_string("/tmp/whatever.rs")
                .unwrap()
                .is_empty()
        );
        std::fs::remove_file("/tmp/whatever.rs").unwrap();
    }

    #[test]
    fn save_quit_multiple_buffers() {
        let test_oxid = TestOxid::new(
            Some("/tmp/save.rs".into()),
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
        test_oxid.execute_command(Command::OpenBuffer("/tmp/new.rs"));
        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 2);
        test_oxid.execute_command(Command::SaveQuitAllBuffers);

        std::thread::sleep(Duration::from_millis(50));

        assert!(std::fs::exists("/tmp/new.rs").unwrap());
        std::fs::remove_file("/tmp/new.rs").unwrap();

        assert!(
            !std::fs::read_to_string("/tmp/save.rs")
                .unwrap()
                .is_empty()
        );
        std::fs::remove_file("/tmp/save.rs").unwrap();

        assert_eq!(test_oxid.app.lock().unwrap().buffers.len(), 0);
    }

    #[test]
    fn next_and_previous_buffer() {
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
        test_oxid.execute_command(Command::OpenBuffer("/tmp/path4.rs"));
        // When editor opens a buffer, it automatically changes to that buffer.
        test_oxid.execute_command(Command::PreviousBuffer); // Reset to first (0)
        assert_eq!(test_oxid.app.lock().unwrap().current_buf_index, 0);
        test_oxid.execute_command(Command::NextBuffer); // Go to next buffer (new one, 1)
        assert_eq!(test_oxid.app.lock().unwrap().current_buf_index, 1);
        assert!(std::fs::exists("/tmp/path4.rs").unwrap());
        std::fs::remove_file("/tmp/path4.rs").unwrap();
    }

    #[test]
    fn go_to_line() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis is a new line\nThis is another new line"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        test_oxid.execute_command(Command::GoToLine(2));
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .current_position
                .line,
            2
        );
        test_oxid.execute_command(Command::GoToLine(256));
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .current_position
                .line,
            2
        );
        test_oxid.execute_command(Command::GoToLine(0));
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .current_position
                .line,
            0
        );
        test_oxid.execute_command(Command::GoToLine(-1));
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .current_position
                .line,
            0
        );
        test_oxid.execute_command(Command::GoToLine(1));
        assert_eq!(
            test_oxid.app.lock().unwrap().buffers[0]
                .current_position
                .line,
            1
        );
    }

    #[test]
    fn lsp_stop() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.py".into()),
            Rope::from_str("Hola mundo\nThis is a new line\nThis is another new line"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_some());
        test_oxid.execute_command(Command::LspStop);
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_none());
    }

    #[test]
    fn lsp_start() {
        let test_oxid = TestOxid::new(
            Some("/dummy/path.rs".into()),
            Rope::from_str("Hola mundo\nThis is a new line\nThis is another new line"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyrefly lsp".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_none());
        test_oxid.execute_command(Command::LspStart("rust-analyzer"));
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_some());
    }
}
