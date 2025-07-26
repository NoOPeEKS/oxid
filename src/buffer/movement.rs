use super::core::Buffer;
use super::types::{BufferPosition, CharType, FileLine};

impl Buffer {
    fn get_next_word(&self) -> Option<BufferPosition> {
        let mut line_idx = self.current_position.line;
        let char_idx = self.current_position.character - self.numbar_space;

        let char_type = |c: char| -> CharType {
            if c.is_whitespace() {
                CharType::Whitespace
            } else if c.is_alphanumeric() || c == '_' {
                CharType::Word
            } else {
                CharType::Punctuation
            }
        };

        // Try first to move within current line
        if line_idx < self.file_lines.len() {
            let file_line = &self.file_lines[line_idx];
            let line_content = file_line.content.clone();
            let chars: Vec<char> = line_content.chars().collect();

            if char_idx < chars.len() {
                let current_char_type = char_type(chars[char_idx]);
                let mut i = char_idx;

                // Skip characters of the same type as current if not whitespace
                if current_char_type != CharType::Whitespace {
                    while i < chars.len() && char_type(chars[i]) == current_char_type {
                        i += 1;
                    }
                }

                // Skip whitespaces on current line
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }

                // If non-whitespace character on current line, return its position.
                if i < chars.len() {
                    return Some(BufferPosition {
                        line: line_idx,
                        character: i + self.numbar_space,
                    });
                }
            }
        }

        // Move to next line
        line_idx += 1;
        if line_idx < self.file_lines.len() {
            let file_line = &self.file_lines[line_idx];
            let line_content = file_line.content.clone();
            let chars: Vec<char> = line_content.chars().collect();

            // If empty line, move there
            if chars.is_empty() {
                return Some(BufferPosition {
                    line: line_idx,
                    character: self.numbar_space,
                });
            }

            // If it's a line with content, find first non-whitespace character
            for (i, &c) in chars.iter().enumerate() {
                if !c.is_whitespace() {
                    return Some(BufferPosition {
                        line: line_idx,
                        character: i + self.numbar_space,
                    });
                }
            }

            // If it's a line with only whitespace, stop at beginning of line
            return Some(BufferPosition {
                line: line_idx,
                character: self.numbar_space,
            });
        }
        None
    }

    pub fn move_to_next_word(&mut self) {
        // Only move to next word if actually some new position is found.
        if let Some(position) = self.get_next_word() {
            self.current_position = position;
        }
    }

    pub fn move_cursor_start_line(&mut self) {
        self.current_position.character = self.numbar_space;
    }
    pub fn move_cursor_end_line(&mut self) {
        self.current_position.character =
            self.file_lines[self.current_position.line].length + self.numbar_space;
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(lines);
        self.current_position.line = self.current_position.line.saturating_sub(lines);
    }

    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = (self.file_lines.len()).saturating_sub(self.viewport_height);
        self.vertical_scroll = std::cmp::min(self.vertical_scroll + lines, max_scroll);
        self.current_position.line = {
            if self.current_position.line.saturating_add(lines) > self.file_lines.len() {
                self.file_lines.len() - 1 // Respect the status line
            } else {
                self.current_position.line.saturating_add(lines)
            }
        };
    }

    pub fn move_cursor_left(&mut self) {
        if self.current_position.character > self.numbar_space {
            self.current_position.character = self.current_position.character.saturating_sub(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_down(&mut self) {
        // If current line is bigger than length of lines vector - 1, limit
        // it to last line available. Must be len(vec) - 1 because lines start at
        // 0.
        self.current_position.line = {
            if self.current_position.line >= (self.file_lines.len() - 1) {
                self.file_lines.len() - 1
            } else {
                self.current_position.line.saturating_add(1)
            }
        };

        // Edge case where when going down, the line is empty line. Then put cursor
        // right after numbar.
        if self.file_lines[self.current_position.line].length == 0 {
            self.current_position.character = self.numbar_space;
        }
        // If current char after going down would be bigger than the new line's
        // length, put it on max character of the line.
        else if self.current_position.character
            > self.file_lines[self.current_position.line].length + self.numbar_space
        {
            // -1 because lines start at 0 and length is always bigger.
            self.current_position.character =
                self.file_lines[self.current_position.line].length + self.numbar_space - 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_up(&mut self) {
        self.current_position.line = self.current_position.line.saturating_sub(1);
        // Edge case where when going up the line is empty line. Then put cursor
        // after numbar.
        if self.file_lines[self.current_position.line].length == 0 {
            self.current_position.character = self.numbar_space;
        }
        // If current char after going up would be bigger than the new line's
        // length, put it on max character of the line.
        else if self.current_position.character
            > self.file_lines[self.current_position.line].length + self.numbar_space
        {
            // -1 because lines start at 0 and length is always bigger.
            self.current_position.character =
                self.file_lines[self.current_position.line].length + self.numbar_space - 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_right(&mut self) {
        let line_len = self.file_lines[self.current_position.line].length;
        let max_cursor_pos = line_len + self.numbar_space;
        if self.current_position.character < max_cursor_pos {
            self.current_position.character = self.current_position.character.saturating_add(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_line_below(&mut self) {
        self.file_lines.insert(
            self.current_position.line + 1,
            FileLine {
                content: String::from(""),
                length: 0,
            },
        );
        self.current_position.line = self.current_position.line.saturating_add(1);
        self.current_position.character = self.numbar_space;
        // self.insert_mode();
        self.ensure_cursor_visible();
    }
}
