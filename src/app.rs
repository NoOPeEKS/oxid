#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
        }
    }

    pub fn toggle_mode(&mut self) {
        match &self.mode {
            Mode::Normal => self.mode = Mode::Insert,
            Mode::Insert => self.mode = Mode::Normal,
        }
    }
}
