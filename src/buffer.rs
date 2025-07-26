const STATUSBAR_SPACE: usize = 1;

#[derive(PartialEq)]
enum CharType {
    Word,
    Punctuation,
    Whitespace,
}

pub struct BufferPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Clone, Debug)]
pub struct FileLine {
    pub content: String,
    pub length: usize,
}

pub struct Buffer {
    pub file_path: Option<String>,
    pub file_lines: Vec<FileLine>,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub current_position: BufferPosition,
    pub numbar_space: usize,
}

impl Buffer {
    pub fn new(
        file_path: Option<String>,
        file_text: String,
        viewport_width: usize,
        viewport_height: usize,
    ) -> Self {
        let numbar_space = file_text.lines().count().to_string().len() + 1;
        Buffer {
            file_path,
            file_lines: file_text
                .lines()
                .map(|l| FileLine {
                    content: l.to_string(),
                    length: l.len(),
                })
                .collect(),
            viewport_width: viewport_width - numbar_space,
            viewport_height: viewport_height - STATUSBAR_SPACE,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            current_position: BufferPosition {
                line: 0,
                character: numbar_space,
            },
            numbar_space,
        }
    }

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

    pub fn save_file(&self) -> anyhow::Result<()> {
        let lines_vec: Vec<_> = self
            .file_lines
            .iter()
            .map(|fl| fl.content.clone())
            .collect();
        if let Some(filepath) = &self.file_path {
            std::fs::write(filepath.clone(), lines_vec.join("\n"))?;
        } else {
            anyhow::bail!("No filepath provided, cannot save file...")
        }
        Ok(())
    }

    fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.current_position.line;
        let viewport_bottom = self.vertical_scroll + self.viewport_height;

        // Vertical scrolling
        if cursor_line < self.vertical_scroll {
            self.vertical_scroll = cursor_line;
        } else if cursor_line >= viewport_bottom {
            self.vertical_scroll = cursor_line.saturating_sub(self.viewport_height) + 1;
        }

        // Horizontal scrolling
        let cursor_char = self.current_position.character;
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
        let start = self.vertical_scroll;
        let end = std::cmp::min(start + self.viewport_height, self.file_lines.len());

        self.file_lines[start..end].iter().collect()
    }

    pub fn get_visible_line_content(&self, line: &FileLine) -> String {
        let start_col = self.horizontal_scroll;
        if start_col >= line.content.len() {
            return String::new();
        }

        let end_col = std::cmp::min(start_col + self.viewport_width, line.content.len());

        line.content[start_col..end_col].to_string()
    }

    pub fn get_viewport_cursor_pos(&self) -> BufferPosition {
        BufferPosition {
            line: self.current_position.line - self.vertical_scroll,
            character: self
                .current_position
                .character
                .saturating_sub(self.horizontal_scroll),
        }
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

    pub fn enter_key(&mut self) {
        let curr_line = self.file_lines[self.current_position.line].clone();
        let current_character = self.current_position.character;
        // If cursor is at the end of the line, just include empty line below.
        if current_character - self.numbar_space == curr_line.length {
            self.file_lines.insert(
                self.current_position.line + 1,
                FileLine {
                    content: String::from(""),
                    length: 0,
                },
            );
            self.current_position.line = self.current_position.line.saturating_add(1);
            self.current_position.character = self.numbar_space;
        } else if current_character >= self.numbar_space && current_character < curr_line.length {
            // If cursor is anywhere between the line, move cursor + forward to next line.
            // So basically, current line should be line[0..cursor] and next line should be
            // line[cursor..]
            self.file_lines[self.current_position.line].content =
                curr_line.content[0..current_character - self.numbar_space].to_string();

            let new_line_content =
                curr_line.content[current_character - self.numbar_space..].to_string();

            self.file_lines.insert(
                self.current_position.line + 1,
                FileLine {
                    content: new_line_content.clone(),
                    length: new_line_content.len(),
                },
            );
            self.current_position.line = self.current_position.line.saturating_add(1);
            self.current_position.character = self.numbar_space;
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_char(&mut self, ch: char) {
        // TODO: Need to handle edge case where line is new line.
        // Probably gonna have to optimize this later as there are many clones.
        // In the future, handling '\n' or '<CR>' will be tricky.

        let mut curr_line = self.file_lines[self.current_position.line].content.clone();
        let insert_index = self.current_position.character - self.numbar_space;
        if insert_index <= curr_line.len() {
            curr_line.insert(insert_index, ch);
            self.file_lines[self.current_position.line].content = curr_line.clone();
            self.file_lines[self.current_position.line].length = curr_line.len();
            self.current_position.character = self.current_position.character.saturating_add(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn remove_char(&mut self) {
        // Probably gonna have to optimize this later as there are many clones.
        let mut curr_line = self.file_lines[self.current_position.line].content.clone();

        // If current position is and only if is bigger than the numbar, delete it.
        if self.current_position.character > self.numbar_space {
            let string_index = self.current_position.character - 1 - self.numbar_space;
            if string_index < curr_line.len() {
                curr_line.remove(string_index);
                self.file_lines[self.current_position.line].content = curr_line.clone();
                self.file_lines[self.current_position.line].length = curr_line.len();
                self.current_position.character = self.current_position.character.saturating_sub(1);
            }
        }

        // If current pos is just after the numbar, means we're deleting entire line.
        if self.current_position.character == self.numbar_space && self.current_position.line > 0 {
            let current_line_index = self.current_position.line;

            // If it's empty, should just delete the line and move cursor.
            if self.file_lines[current_line_index].content.is_empty() {
                self.file_lines.remove(current_line_index);
                self.current_position.line = self.current_position.line.saturating_sub(1);
                self.current_position.character =
                    self.file_lines[current_line_index - 1].length + self.numbar_space;
            }
            // If it's not empty, should join the current linestring with the previous unless it's the
            // first line, and move cursor to last char of previous line.
            else {
                let line = self.file_lines[current_line_index].clone();
                let mut top_line = self.file_lines[current_line_index - 1].clone();
                let top_line_old_len = top_line.length;

                top_line.content = top_line.content + &line.content;
                top_line.length = top_line.content.len();
                self.file_lines[current_line_index - 1] = top_line;
                self.file_lines.remove(current_line_index);

                self.current_position.line = self.current_position.line.saturating_sub(1);
                self.current_position.character = top_line_old_len + self.numbar_space;
            }
        }
        self.ensure_cursor_visible();
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
