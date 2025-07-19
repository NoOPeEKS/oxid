#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub file: String,
}

impl App {
    pub fn new(buffer: String) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            file: buffer,
        }
    }
    
    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }
}
