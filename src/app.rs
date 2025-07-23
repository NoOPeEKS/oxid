const STATUSBAR_SPACE: u16 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct EditorPosition {
    pub line: u16, // Good idea to change to usize?
    pub char: u16, // Good idea to change to usize?
}

#[derive(Clone, Debug)]
pub struct FileLine {
    pub content: String,
    pub length: u16,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub file_lines: Vec<FileLine>,
    pub viewport_width: u16,
    pub viewport_height: u16,
    pub scroll_offset: u16,
    pub horizontal_scroll: u16,
    pub current_pos: EditorPosition,
    pub numbar_space: u16,
}

impl App {
    pub fn new(file_text: String, viewport_width: u16, viewport_height: u16) -> Self {
        let numbar_space = file_text.lines().count().to_string().len() + 1; // Leave a space
        // between column and first char.
        App {
            mode: Mode::Normal,
            quitting: false,
            file_lines: file_text
                .lines()
                .map(|l| FileLine {
                    content: l.to_string(),
                    length: l.len() as u16,
                })
                .collect(),
            viewport_width: viewport_width - numbar_space as u16,
            viewport_height: viewport_height - STATUSBAR_SPACE,
            scroll_offset: 0,
            horizontal_scroll: 0,
            current_pos: EditorPosition {
                line: 0,
                char: numbar_space as u16,
            },
            numbar_space: numbar_space as u16,
        }
    }

    fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.current_pos.line;
        let viewport_bottom = self.scroll_offset + self.viewport_height;

        // Vertical scrolling
        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        } else if cursor_line >= viewport_bottom {
            self.scroll_offset = cursor_line.saturating_sub(self.viewport_height) + 1;
        }

        // Horizontal scrolling
        let cursor_char = self.current_pos.char;
        let viewport_right = self.horizontal_scroll + self.viewport_width + self.numbar_space;

        // If cursor is to the left of visible area, scroll left
        if cursor_char < self.horizontal_scroll + self.numbar_space {
            self.horizontal_scroll = cursor_char.saturating_sub(self.numbar_space);
        }
        // If cursor is to the right of visible area, scroll right
        else if cursor_char >= viewport_right {
            self.horizontal_scroll = cursor_char
                .saturating_sub(self.viewport_width)
                .saturating_sub(self.numbar_space)
                + 1;
        }
    }

    pub fn get_visible_lines(&self) -> Vec<&FileLine> {
        let start = self.scroll_offset as usize;
        let end = std::cmp::min(start + self.viewport_height as usize, self.file_lines.len());

        self.file_lines[start..end].iter().collect()
    }

    pub fn get_visible_line_content(&self, line: &FileLine) -> String {
        let start_col = self.horizontal_scroll as usize;
        if start_col >= line.content.len() {
            return String::new();
        }

        let end_col = std::cmp::min(start_col + self.viewport_width as usize, line.content.len());

        line.content[start_col..end_col].to_string()
    }

    pub fn get_viewport_cursor_pos(&self) -> EditorPosition {
        EditorPosition {
            line: self.current_pos.line - self.scroll_offset,
            char: self.current_pos.char.saturating_sub(self.horizontal_scroll),
        }
    }

    pub fn scroll_up(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
        self.current_pos.line = self.current_pos.line.saturating_sub(lines);
    }

    pub fn scroll_down(&mut self, lines: u16) {
        let max_scroll = (self.file_lines.len() as u16).saturating_sub(self.viewport_height);
        self.scroll_offset = std::cmp::min(self.scroll_offset + lines, max_scroll);
        self.current_pos.line = {
            if self.current_pos.line.saturating_add(lines) > self.file_lines.len() as u16 {
                self.file_lines.len() as u16 - 1 // Respect the status line
            } else {
                self.current_pos.line.saturating_add(lines)
            }
        };
    }

    pub fn insert_char(&mut self, ch: char) {
        // TODO: Need to handle edge case where line is new line.
        // Probably gonna have to optimize this later as there are many clones.
        // In the future, handling '\n' or '<CR>' will be tricky.

        let mut curr_line = self.file_lines[self.current_pos.line as usize]
            .content
            .clone();
        let insert_index = (self.current_pos.char - self.numbar_space) as usize;
        if insert_index <= curr_line.len() {
            curr_line.insert(insert_index, ch);
            self.file_lines[self.current_pos.line as usize].content = curr_line.clone();
            self.file_lines[self.current_pos.line as usize].length = curr_line.len() as u16;
            self.current_pos.char = self.current_pos.char.saturating_add(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn remove_char(&mut self) {
        // Probably gonna have to optimize this later as there are many clones.
        let mut curr_line = self.file_lines[self.current_pos.line as usize]
            .content
            .clone();

        // If current position is and only if is bigger than the numbar, delete it.
        if self.current_pos.char > self.numbar_space {
            let string_index = (self.current_pos.char - 1 - self.numbar_space) as usize;
            if string_index < curr_line.len() {
                curr_line.remove(string_index);
                self.file_lines[self.current_pos.line as usize].content = curr_line.clone();
                self.file_lines[self.current_pos.line as usize].length = curr_line.len() as u16;
                self.current_pos.char = self.current_pos.char.saturating_sub(1);
            }
        }

        // If current pos is just after the numbar, means we're deleting entire line.
        if self.current_pos.char == self.numbar_space && self.current_pos.line > 0 {
            let current_line_index = self.current_pos.line as usize;

            // If it's empty, should just delete the line and move cursor.
            if self.file_lines[current_line_index].content.is_empty() {
                self.file_lines.remove(current_line_index);
                self.current_pos.line = self.current_pos.line.saturating_sub(1);
                self.current_pos.char =
                    self.file_lines[current_line_index - 1].length + self.numbar_space;
            }
            // If it's not empty, should join the current linestring with the previous unless it's the
            // first line.
            else {
                let line = self.file_lines[current_line_index].clone();
                let mut top_line = self.file_lines[current_line_index - 1].clone();
                let top_line_old_len = top_line.length;

                top_line.content = top_line.content + &line.content;
                top_line.length = top_line.content.len() as u16;
                self.file_lines[current_line_index - 1] = top_line;
                self.file_lines.remove(current_line_index);

                self.current_pos.line = self.current_pos.line.saturating_sub(1);
                self.current_pos.char = top_line_old_len + self.numbar_space;
            }
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }

    pub fn move_cursor_left(&mut self) {
        if self.mode == Mode::Normal && self.current_pos.char > self.numbar_space {
            self.current_pos.char = self.current_pos.char.saturating_sub(1);
        }
        self.ensure_cursor_visible();
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
            if self.file_lines[self.current_pos.line as usize].length == 0_u16 {
                self.current_pos.char = self.numbar_space;
            }
            // If current char after going down would be bigger than the new line's
            // length, put it on max character of the line.
            else if self.current_pos.char
                > self.file_lines[self.current_pos.line as usize].length + self.numbar_space
            {
                // -1 because lines start at 0 and length is always bigger.
                self.current_pos.char =
                    self.file_lines[self.current_pos.line as usize].length + self.numbar_space - 1;
            }
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_up(&mut self) {
        if self.mode == Mode::Normal {
            self.current_pos.line = self.current_pos.line.saturating_sub(1);
            // Edge case where when going up the line is empty line. Then put cursor
            // after numbar.
            if self.file_lines[self.current_pos.line as usize].length == 0_u16 {
                self.current_pos.char = self.numbar_space;
            }
            // If current char after going up would be bigger than the new line's
            // length, put it on max character of the line.
            else if self.current_pos.char
                > self.file_lines[self.current_pos.line as usize].length + self.numbar_space
            {
                // -1 because lines start at 0 and length is always bigger.
                self.current_pos.char =
                    self.file_lines[self.current_pos.line as usize].length + self.numbar_space - 1;
            }
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_right(&mut self) {
        if self.mode == Mode::Normal {
            let line_len = self.file_lines[self.current_pos.line as usize].length;
            let max_cursor_pos = line_len + self.numbar_space;
            if self.current_pos.char < max_cursor_pos {
                self.current_pos.char = self.current_pos.char.saturating_add(1);
            }
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_line_below(&mut self) {
        self.file_lines.insert(
            (self.current_pos.line + 1) as usize,
            FileLine {
                content: String::from(""),
                length: 0_u16,
            },
        );
        self.current_pos.line = self.current_pos.line.saturating_add(1);
        self.current_pos.char = self.numbar_space;
        self.insert_mode();
        self.ensure_cursor_visible();
    }
}
