use std::sync::mpsc::channel;
use std::time::Duration;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ropey::Rope;

use oxid::app::App;
use oxid::buffer::Buffer;
use oxid::config::{Config, LspConfig};
use oxid::events::EventKind;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_init() {
        let mut terminal = Terminal::new(TestBackend::new(80, 40)).unwrap();
        let config = Config {
            lsp: vec![LspConfig {
                filetype: "py".to_string(),
                command: "pyrefly lsp".to_string(),
            }],
        };
        let text = Rope::from_str("Hello, world this is a text!");
        let buffers: Vec<Buffer> = vec![Buffer::new(
            Some(String::from("/dummy/path.py")),
            text,
            80,
            40,
        )];
        let mut app = App::new(buffers, 80, 40, config);
        let (event_sender, event_receiver) = channel::<EventKind>();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(50));
            event_sender.send(EventKind::Quit).unwrap();
        });
        let result = app.run(event_receiver, &mut terminal);
        assert!(result.is_ok());
        assert!(app.lsp_client.is_some());
        assert_eq!(
            app.buffers[0].file_text,
            Rope::from_str("Hello, world this is a text!")
        );
    }
}
