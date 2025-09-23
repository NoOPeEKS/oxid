use oxid::config::Config;
use oxid::events::EventKind;
use oxid::{app::App, buffer::Buffer};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

pub enum Motion {
    NextWord,
    PrevWord,
    EndWord,
    StartLine,
    EndLine,
}

pub enum Command<'a> {
    SaveCurrentBuffer,
    SaveAllBuffers,
    QuitCurrentBuffer,
    QuitAllBuffers,
    SaveQuitAllBuffers,
    OpenBuffer(&'a str),
    NextBuffer,
    PreviousBuffer,
    GoToLine(usize),
    LspStart(&'a str),
    LspStop,
}

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

    pub fn visual_mode(&self) {
        self.normal_mode();
        self.sender.send(EventKind::KeyPressed('v')).unwrap();
        std::thread::sleep(Duration::from_millis(10))
    }
    
    pub fn command_mode(&self) {
        self.normal_mode();
        self.sender.send(EventKind::KeyPressed(':')).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }

    pub fn execute_command(&self, command: Command) {
        fn send_cmd(cmd: &str, sender: Sender<EventKind>) {
            for char in cmd.chars() {
                sender.send(EventKind::KeyPressed(char)).unwrap();
                std::thread::sleep(Duration::from_millis(5));
            }
            sender.send(EventKind::EnterKey).unwrap();
        }
        self.command_mode();
        match command {
            Command::SaveCurrentBuffer => {
                send_cmd("w", self.sender.clone());
            }
            Command::SaveAllBuffers => {
                send_cmd("wa", self.sender.clone());
            }
            Command::QuitCurrentBuffer => {
                send_cmd("q", self.sender.clone());
            }
            Command::QuitAllBuffers => {
                send_cmd("qa", self.sender.clone());
            }
            Command::SaveQuitAllBuffers => {
                send_cmd("wqa", self.sender.clone());
            }
            Command::OpenBuffer(path) => {
                send_cmd(&format!("e {path}"), self.sender.clone());
            }
            Command::NextBuffer => {
                send_cmd("bn", self.sender.clone());
            }
            Command::PreviousBuffer => {
                send_cmd("bp", self.sender.clone());
            }
            Command::GoToLine(line_num) => {
                send_cmd(&line_num.to_string(), self.sender.clone());
            }
            Command::LspStart(lsp_cmd) => {
                send_cmd(&format!("LspStart {lsp_cmd}"), self.sender.clone());
            }
            Command::LspStop => {
                send_cmd("LspStop", self.sender.clone());
            }
        }
    }

    pub fn yank(&self) {
        self.sender.send(EventKind::KeyPressed('y')).unwrap();
        std::thread::sleep(Duration::from_millis(5));
    }

    pub fn insert_line_below(&self) {
        self.normal_mode();
        self.sender.send(EventKind::KeyPressed('o')).unwrap();
        std::thread::sleep(Duration::from_millis(5));
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

    pub fn insert_start_line(&self, text: &str) {
        self.normal_mode();
        self.sender.send(EventKind::ShiftedKey('I')).unwrap();
        std::thread::sleep(Duration::from_millis(15));

        for char in text.chars() {
            self.sender.send(EventKind::KeyPressed(char)).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }

        self.normal_mode();
    }

    pub fn place_cursor(&mut self, x: usize, y: usize) {
        let mut app_guard = self.app.lock().unwrap();
        let current_buf = app_guard.current_buf_index;
        let numbar_space = app_guard.buffers[current_buf].numbar_space;
        app_guard.buffers[current_buf].current_position.character = x + numbar_space;
        app_guard.buffers[current_buf].current_position.line = y;
    }

    pub fn execute_motion(&mut self, motion: Motion, times: usize) {
        let key = match motion {
            Motion::NextWord => 'w',
            Motion::PrevWord => 'b',
            Motion::EndWord => 'e',
            Motion::StartLine => '0',
            Motion::EndLine => '$',
        };

        for _ in 0..times {
            self.sender.send(EventKind::KeyPressed(key)).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn move_cursor(&self, direction: CursorDirection, times: usize) {
        self.normal_mode();
        let key = match direction {
            CursorDirection::Up => 'k',
            CursorDirection::Down => 'j',
            CursorDirection::Left => 'h',
            CursorDirection::Right => 'l',
        };

        for _ in 0..times {
            self.sender.send(EventKind::KeyPressed(key)).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn scroll_down(&self, times: usize) {
        self.normal_mode();
        for _ in 0..times {
            self.sender.send(EventKind::ScrollDown).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn scroll_up(&self, times: usize) {
        self.normal_mode();
        for _ in 0..times {
            self.sender.send(EventKind::ScrollUp).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn backspace(&self, times: usize) {
        self.insert_mode();
        for _ in 0..times {
            self.sender.send(EventKind::Backspace).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn enter(&self, times: usize) {
        self.insert_mode();
        for _ in 0..times {
            self.sender.send(EventKind::EnterKey).unwrap();
            std::thread::sleep(Duration::from_millis(5));
        }
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
