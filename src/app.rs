#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct EditorPosition {
    pub line: u16,
    pub char: u16,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub file_lines: Vec<(String, u16)>,
    pub size_x: u16,
    pub size_y: u16,
    pub current_pos: EditorPosition,
}

impl App {
    pub fn new(file_text: String, size_x: u16, size_y: u16) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            file_lines: file_text
                .lines()
                .map(|l| (l.to_string(), l.len() as u16))
                .collect(),
            size_x: size_x - 2, // Status line is on size -1 so cursor must be on -2 max.
            size_y: size_y - 2, // Status line is on size -1 so cursor must be on -2 max.
            current_pos: EditorPosition { line: 0, char: 0 },
        }
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }
}
