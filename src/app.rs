#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub file: String,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub size_x: u16,
    pub size_y: u16,
}

impl App {
    pub fn new(buffer: String, size_x: u16, size_y: u16) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            file: buffer,
            cursor_x: 0,
            cursor_y: 0,
            size_x,
            size_y,
        }
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }
}
