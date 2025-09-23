#[cfg(test)]
mod tests {
    use oxid::{
        config::{Config, LspConfig},
        events::EventKind,
    };
    use oxid_test::{Command, TestOxid};
    use ropey::Rope;
    use std::time::Duration;

    #[test]
    fn attach_lsp_on_init_and_filetype() {
        let test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("from pathlib import Path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_some());
    }

    #[test]
    fn dont_attach_lsp_on_init() {
        let test_oxid = TestOxid::new(
            Some("/tmp/lsp.rs".into()),
            Rope::from_str("from pathlib import Path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().lsp_client.is_none());
    }

    #[test]
    fn hover_on_correct_zone() {
        let mut test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("from pathlib import Path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(21, 0);
        test_oxid.hover();
        std::thread::sleep(Duration::from_millis(50));
        assert!(test_oxid.app.lock().unwrap().hover.is_some());
    }

    #[test]
    fn hover_on_incorrect_zone() {
        let mut test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("from pathlib import Path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(19, 0);
        test_oxid.hover();
        std::thread::sleep(Duration::from_millis(50));
        assert!(test_oxid.app.lock().unwrap().hover.is_none());
    }

    #[test]
    fn request_completion_on_correct_zone() {
        let mut test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("from path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(10, 0);
        test_oxid.request_completion();
        std::thread::sleep(Duration::from_millis(50));
        assert!(test_oxid.app.lock().unwrap().completion_list.is_some());
        assert!(
            !test_oxid
                .app
                .lock()
                .unwrap()
                .completion_list
                .clone()
                .unwrap()
                .items
                .is_empty()
        );
    }

    #[test]
    fn request_completion_on_incorrect_zone() {
        let mut test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str(""),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        test_oxid.place_cursor(0, 0);
        test_oxid.request_completion();
        std::thread::sleep(Duration::from_millis(50));
        assert!(
            test_oxid
                .app
                .lock()
                .unwrap()
                .completion_list
                .clone()
                .unwrap()
                .items
                .is_empty()
        );
    }

    #[test]
    fn choose_completion() {
        let mut test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("from path"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().selected_completion.is_none());
        test_oxid.place_cursor(10, 0);
        test_oxid.request_completion();
        test_oxid.sender.send(EventKind::Tab).unwrap();
        std::thread::sleep(Duration::from_millis(50));
        test_oxid.enter(1);
        assert!(test_oxid.app.lock().unwrap().selected_completion.is_some());
    }

    #[test]
    fn get_diagnostics() {
        let test_oxid = TestOxid::new(
            Some("/tmp/lsp.py".into()),
            Rope::from_str("doesn't make any sense&should see diagnostics"),
            80,
            20,
            Config {
                lsp: vec![LspConfig {
                    filetype: "py".to_string(),
                    command: "pyright-langserver --stdio".to_string(),
                }],
            },
        );
        assert!(test_oxid.app.lock().unwrap().diagnostics.is_none());
        test_oxid.execute_command(Command::SaveAllBuffers);
        test_oxid.update_diagnostics();
        println!("{:?}", test_oxid.app.lock().unwrap().diagnostics);
        // assert!(test_oxid.app.lock().unwrap().diagnostics.is_some());
        // assert!(!test_oxid.app.lock().unwrap().diagnostics.clone().unwrap().is_empty());
    }
}
