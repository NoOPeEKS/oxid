const NUMBAR_SPACE: u16 = 2;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct EditorPosition {
    pub line: u16, // Good idea to change to usize?
    pub char: u16, // Good idea to change to usize?
}

pub struct App {
    pub mode: Mode,

    pub quitting: bool,

    // Maybe convert this to a struct for easy use?
    pub file_lines: Vec<(String, u16)>,

    #[allow(dead_code)]
    pub size_x: u16, // Will be used later on.

    #[allow(dead_code)]
    pub size_y: u16, // Will be used later on.

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
            current_pos: EditorPosition { line: 0, char: 2 },
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        // TODO: Need to handle edge case where line is new line.
        // Probably gonna have to optimize this later as there are many clones.
        // In the future, handling '\n' or '<CR>' will be tricky.

        let mut curr_line = self.file_lines[self.current_pos.line as usize].0.clone();
        // Subtract NUMBAR_SPACE beacuse of the numbar taking 2 chars of render space.
        curr_line.insert((self.current_pos.char - NUMBAR_SPACE).into(), ch);

        self.file_lines[self.current_pos.line as usize].0 = curr_line.clone();
        self.file_lines[self.current_pos.line as usize].1 = curr_line.len() as u16;
        self.current_pos.char = self.current_pos.char.saturating_add(1);
    }

    pub fn remove_char(&mut self) {
        // Probably gonna have to optimize this later as there are many clones.
        let mut curr_line = self.file_lines[self.current_pos.line as usize].0.clone();

        // If current position is and only if is bigger than the numbar, delete it.
        if self.current_pos.char > NUMBAR_SPACE {
            let string_index = (self.current_pos.char - 1 - NUMBAR_SPACE) as usize;
            if string_index < curr_line.len() {
                curr_line.remove(string_index);
                self.file_lines[self.current_pos.line as usize].0 = curr_line.clone();
                self.file_lines[self.current_pos.line as usize].1 = curr_line.len() as u16;
                self.current_pos.char = self.current_pos.char.saturating_sub(1);
            }
        }
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }

    pub fn move_cursor_left(&mut self) {
        if self.mode == Mode::Normal
            && self.current_pos.char > NUMBAR_SPACE {
                self.current_pos.char = self.current_pos.char.saturating_sub(1);
            }
    }

    pub fn move_cursor_down(&mut self) {
        if self.mode == Mode::Normal {
            // If current line is bigger than length of lines vector - 1, limit
            // it to last line available. Must be len(vec) - 1 because lines start at
            // 0.
            self.current_pos.line = {
                if self.current_pos.line >= (self.file_lines.len() - 1) as u16 {
                    (self.file_lines.len() - 1) as u16
                } else {
                    self.current_pos.line.saturating_add(1)
                }
            };

            // Edge case where when going down, the line is empty line. Then put cursor
            // right after numbar.
            if self.file_lines[self.current_pos.line as usize].1 == 0_u16 {
                self.current_pos.char = NUMBAR_SPACE;
            }
            // If current char after going down would be bigger than the new line's
            // length, put it on max character of the line.
            else if self.current_pos.char > self.file_lines[self.current_pos.line as usize].1 {
                self.current_pos.char = self.file_lines[self.current_pos.line as usize].1 + 1_u16;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.mode == Mode::Normal {
            self.current_pos.line = self.current_pos.line.saturating_sub(1);
            // Edge case where when going up the line is empty line. Then put cursor
            // after numbar.
            if self.file_lines[self.current_pos.line as usize].1 == 0_u16 {
                self.current_pos.char = NUMBAR_SPACE;
            }
            // If current char after going up would be bigger than the new line's
            // length, put it on max character of the line.
            else if self.current_pos.char > self.file_lines[self.current_pos.line as usize].1 {
                self.current_pos.char = self.file_lines[self.current_pos.line as usize].1 + 1_u16;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.mode == Mode::Normal {
            self.current_pos.char = {
                // If current line is a newly added line, default to first editor
                // character that starts at NUMBAR_SPACE
                if self.file_lines[self.current_pos.line as usize].1 == 0_u16 {
                    NUMBAR_SPACE
                } else if self.current_pos.char > self.file_lines[self.current_pos.line as usize].1
                {
                    // Don't really know why this works but this keeps the cursor at
                    // the end of the line that's editing.
                    self.file_lines[self.current_pos.line as usize].1 + 1
                } else {
                    // If no constaints are being met, means we can freely add one
                    // position to right.
                    self.current_pos.char.saturating_add(1)
                }
            }
        }
    }

    pub fn insert_line_below(&mut self) {
        self.file_lines.insert(
            (self.current_pos.line + 1) as usize,
            (String::from(""), 0_u16),
        );
        self.current_pos.line = self.current_pos.line.saturating_add(1);
        self.current_pos.char = NUMBAR_SPACE;
        self.insert_mode();
    }
}
