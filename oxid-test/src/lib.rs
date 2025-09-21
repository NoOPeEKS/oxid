use oxid::config::Config;
use oxid::events::EventKind;
use oxid::{app::App, buffer::Buffer};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle};
use std::time::Duration;

pub struct TestOxid {
    pub app: Arc<Mutex<App>>,
    pub sender: Sender<EventKind>,
    pub _handle: JoinHandle<()>,
}

impl TestOxid {
    pub fn new(
        path: Option<String>,
        text: ropey::Rope,
        width: usize,
        height: usize,
        config: Config,
    ) -> Self {
        let (sender, receiver) = channel::<EventKind>();

        let buffers = vec![Buffer::new(path, text, width, height)];
        let app = Arc::new(Mutex::new(App::new(buffers, width, height, config)));

        let app_clone = Arc::clone(&app);
        let handle = std::thread::spawn(move || {
            let backend = TestBackend::new(width as u16, height as u16);
            let mut terminal = Terminal::new(backend).unwrap();

            loop {
                let should_quit = {
                    let mut app_guard = app_clone.lock().unwrap();

                    match receiver.recv_timeout(Duration::from_millis(10)) {
                        Ok(event) => {
                            if app_guard.handle_event(event, &mut terminal).is_err() {
                                break;
                            }
                            app_guard.quitting
                        }
                        Err(_) => app_guard.quitting,
                    }
                };

                if should_quit {
                    break;
                }

                // Prevent busy waiting and allow threads to acquire lock
                std::thread::sleep(Duration::from_millis(1));
            }
        });

        Self {
            app,
            sender,
            _handle: handle,
        }
    }

    pub fn quit_editor(&self) {
        self.sender.send(EventKind::Quit).unwrap();
        std::thread::sleep(Duration::from_millis(20));
    }

    pub fn normal_mode(&self) {
        self.sender.send(EventKind::NormalMode).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }

    pub fn insert_mode(&self) {
        self.normal_mode();
        self.sender.send(EventKind::KeyPressed('i')).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }

    pub fn append_end_line(&self, text: &str) {
        self.normal_mode();
        self.sender.send(EventKind::ShiftedKey('A')).unwrap();
        std::thread::sleep(Duration::from_millis(15));

        for char in text.chars() {
            self.sender.send(EventKind::KeyPressed(char)).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }

        self.normal_mode();
    }

    pub fn get_buffer_text(&self) -> String {
        for _ in 0..50 {
            if let Ok(app) = self.app.try_lock() {
                return app.buffers[0].file_text.to_string();
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        panic!("Could not acquire lock to read buffer text");
    }

    pub fn get_mode(&self) -> String {
        for _ in 0..50 {
            if let Ok(app) = self.app.try_lock() {
                return app.mode.to_string();
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        panic!("Could not acquire lock to read mode");
    }
}

