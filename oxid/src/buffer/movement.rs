use super::core::Buffer;
use super::types::{BufferPosition, CharType};

impl Buffer {
    pub fn move_to_end_of_word(&mut self) {
        if let Some(position) = self.get_end_of_word() {
            self.current_position = position;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_to_previous_word(&mut self) {
        if let Some(position) = self.get_previous_word() {
            self.current_position = position;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_to_next_word(&mut self) {
        // Only move to next word if actually some new position is found.
        if let Some(position) = self.get_next_word() {
            self.current_position = position;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_cursor_start_line(&mut self) {
        self.current_position.character = self.numbar_space;
    }
    pub fn move_cursor_end_line(&mut self) {
        self.current_position.character =
            // self.file_text[self.current_position.line].length + self.numbar_space;
            (self.file_text.line(self.current_position.line).len_chars() - 1) + self.numbar_space;
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(lines);
        self.current_position.line = self.current_position.line.saturating_sub(lines);
        // if self.file_lines[self.current_position.line].length
        if self.file_text.line(self.current_position.line).len_chars()
            < self.current_position.character - self.numbar_space
        {
            self.current_position.character =
                // self.file_lines[self.current_position.line].length + self.numbar_space;
                self.file_text.line(self.current_position.line).len_chars() + self.numbar_space;
        }
    }

    pub fn scroll_down(&mut self, lines: usize) {
        // let max_scroll = (self.file_lines.len()).saturating_sub(self.viewport_height);
        let max_scroll = (self.file_text.len_lines()).saturating_sub(self.viewport_height);
        self.vertical_scroll = std::cmp::min(self.vertical_scroll + lines, max_scroll);
        self.current_position.line = {
            // if self.current_position.line.saturating_add(lines) > self.file_lines.len() {
            if self.current_position.line.saturating_add(lines) > self.file_text.len_lines() {
                // self.file_lines.len() - 1 // Respect the status line
                self.file_text.len_lines() - 1 // Respect the status line
            } else {
                self.current_position.line.saturating_add(lines)
            }
        };
        // if self.file_lines[self.current_position.line].length
        if self.file_text.line(self.current_position.line).len_chars()
            < self.current_position.character - self.numbar_space
        {
            self.current_position.character =
                // self.file_lines[self.current_position.line].length + self.numbar_space;
                self.file_text.line(self.current_position.line).len_chars() + self.numbar_space;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.current_position.character > self.numbar_space {
            self.current_position.character = self.current_position.character.saturating_sub(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_down(&mut self) {
        // If current line is bigger or eq than num of total lines, limit
        // it to last line available. Must be len_lines() - 1 because lines start at 0.
        self.current_position.line = {
            if self.current_position.line >= (self.file_text.len_lines() - 1) {
                self.file_text.len_lines() - 1
            } else {
                self.current_position.line.saturating_add(1)
            }
        };

        // Edge case where when going down, the line is empty line. Then put cursor
        // right after numbar.
        if self.file_text.line(self.current_position.line).len_chars() == 0 {
            self.current_position.character = self.numbar_space;
        }
        // If current char after going down would be bigger than the new line's
        // length, put it on max character of the line.
        else if self.current_position.character
            > self.file_text.line(self.current_position.line).len_chars() + self.numbar_space
        {
            self.current_position.character =
                self.file_text.line(self.current_position.line).len_chars() + self.numbar_space - 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_up(&mut self) {
        self.current_position.line = self.current_position.line.saturating_sub(1);
        // Edge case where when going up the line is empty line. Then put cursor
        // after numbar.
        // if self.file_lines[self.current_position.line].length == 0 {
        if self.file_text.line(self.current_position.line).len_chars() == 0 {
            self.current_position.character = self.numbar_space;
        }
        // If current char after going up would be bigger than the new line's
        // length, put it on max character of the line.
        else if self.current_position.character
            // > self.file_lines[self.current_position.line].length + self.numbar_space
            > self.file_text.line(self.current_position.line).len_chars() + self.numbar_space
        {
            // -1 because lines start at 0 and length is always bigger.
            self.current_position.character =
                // self.file_lines[self.current_position.line].length + self.numbar_space - 1;
                self.file_text.line(self.current_position.line).len_chars() + self.numbar_space - 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn move_cursor_right(&mut self) {
        let line_len = self.file_text.line(self.current_position.line).len_chars();
        // For some reason the -1 must be on this line and not after len_chars() otherwise it
        // crashes if cursor is on last line, it's empty and tries to go to the right.
        let max_cursor_pos = line_len + self.numbar_space - 1;
        if self.current_position.character < max_cursor_pos {
            self.current_position.character = self.current_position.character.saturating_add(1);
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_line_below(&mut self) {
        let curr_line = self.current_position.line;
        let curr_idx = self.file_text.line_to_char(curr_line + 1);

        self.file_text.insert(curr_idx, "\n");

        self.update_numbar_space();

        self.current_position.line = self.current_position.line.saturating_add(1);
        self.current_position.character = self.numbar_space;
        self.ensure_cursor_visible();
    }

    fn get_next_word(&self) -> Option<BufferPosition> {
        let line_idx = self.current_position.line;
        let char_idx = self
            .current_position
            .character
            .saturating_sub(self.numbar_space);

        // Try current line first
        if let Some(chars) = self.get_line_chars(line_idx) {
            if char_idx < chars.len() {
                let current_type = CharType::from_char(chars[char_idx]);
                let mut pos = char_idx;

                // Skip characters of same type if not whitespace
                if current_type != CharType::Whitespace {
                    while pos < chars.len() && CharType::from_char(chars[pos]) == current_type {
                        pos += 1;
                    }
                }

                // Skip whitespace
                pos = self.skip_whitespace_forward(&chars, pos);

                // Return position if we found non-whitespace
                if pos < chars.len() {
                    return Some(BufferPosition {
                        line: line_idx,
                        character: pos + self.numbar_space,
                    });
                }
            }
        }

        // Try next lines
        // for next_line_idx in (line_idx + 1)..self.file_lines.len() {
        for next_line_idx in (line_idx + 1)..self.file_text.len_lines() {
            if let Some(chars) = self.get_line_chars(next_line_idx) {
                if chars.is_empty() {
                    return Some(BufferPosition {
                        line: next_line_idx,
                        character: self.numbar_space,
                    });
                }

                // Find first non-whitespace
                let pos = self.skip_whitespace_forward(&chars, 0);
                if pos < chars.len() {
                    return Some(BufferPosition {
                        line: next_line_idx,
                        character: pos + self.numbar_space,
                    });
                }

                // Line with only whitespace
                return Some(BufferPosition {
                    line: next_line_idx,
                    character: self.numbar_space,
                });
            }
        }

        None
    }

    fn get_previous_word(&self) -> Option<BufferPosition> {
        let line_idx = self.current_position.line;
        let char_idx = self
            .current_position
            .character
            .saturating_sub(self.numbar_space);

        // Try current line first
        if let Some(chars) = self.get_line_chars(line_idx) {
            if char_idx > 0 {
                let mut pos = char_idx - 1;

                // Skip whitespace backward
                pos = self.skip_whitespace_backward(&chars, pos);

                // If we found non-whitespace, find start of word
                if pos < chars.len() && !chars[pos].is_whitespace() {
                    if let Some(word_start) = self.find_word_start(&chars, pos) {
                        return Some(BufferPosition {
                            line: line_idx,
                            character: word_start + self.numbar_space,
                        });
                    }
                }
            }
        }

        // Try previous lines
        for prev_line_idx in (0..line_idx).rev() {
            if let Some(chars) = self.get_line_chars(prev_line_idx) {
                if chars.is_empty() {
                    return Some(BufferPosition {
                        line: prev_line_idx,
                        character: self.numbar_space,
                    });
                }

                // Start from end and skip trailing whitespace
                let mut pos = chars.len() - 1;
                pos = self.skip_whitespace_backward(&chars, pos);

                if pos < chars.len() && !chars[pos].is_whitespace() {
                    if let Some(word_start) = self.find_word_start(&chars, pos) {
                        return Some(BufferPosition {
                            line: prev_line_idx,
                            character: word_start + self.numbar_space,
                        });
                    }
                }

                // Line with only whitespace
                return Some(BufferPosition {
                    line: prev_line_idx,
                    character: self.numbar_space,
                });
            }
        }

        None
    }

    fn get_end_of_word(&self) -> Option<BufferPosition> {
        let line_idx = self.current_position.line;
        let char_idx = self
            .current_position
            .character
            .saturating_sub(self.numbar_space);

        // Try current line first
        if let Some(chars) = self.get_line_chars(line_idx) {
            if char_idx < chars.len() {
                let current_type = CharType::from_char(chars[char_idx]);

                if current_type == CharType::Whitespace {
                    // Skip whitespace and find end of next word
                    let pos = self.skip_whitespace_forward(&chars, char_idx);
                    if let Some(word_end) = self.find_word_end(&chars, pos) {
                        return Some(BufferPosition {
                            line: line_idx,
                            character: word_end + self.numbar_space,
                        });
                    }
                } else if self.is_at_word_end(&chars, char_idx) {
                    // At word end, find next word's end
                    let mut pos = char_idx + 1;
                    pos = self.skip_whitespace_forward(&chars, pos);

                    if let Some(word_end) = self.find_word_end(&chars, pos) {
                        return Some(BufferPosition {
                            line: line_idx,
                            character: word_end + self.numbar_space,
                        });
                    }
                } else {
                    // In middle of word, find end of current word
                    if let Some(word_end) = self.find_word_end(&chars, char_idx) {
                        return Some(BufferPosition {
                            line: line_idx,
                            character: word_end + self.numbar_space,
                        });
                    }
                }
            }
        }

        // Try next lines
        // for next_line_idx in (line_idx + 1)..self.file_lines.len() {
        for next_line_idx in (line_idx + 1)..self.file_text.len_lines() {
            if let Some(chars) = self.get_line_chars(next_line_idx) {
                if chars.is_empty() {
                    continue;
                }

                let pos = self.skip_whitespace_forward(&chars, 0);
                if let Some(word_end) = self.find_word_end(&chars, pos) {
                    return Some(BufferPosition {
                        line: next_line_idx,
                        character: word_end + self.numbar_space,
                    });
                }
            }
        }

        None
    }

    fn get_line_chars(&self, line_idx: usize) -> Option<Vec<char>> {
        // self.file_lines
        //     .get(line_idx)
        //     .map(|line| line.content.chars().collect())
        Some(self.file_text.line(line_idx).chars().collect())
    }

    fn skip_whitespace_forward(&self, chars: &[char], mut pos: usize) -> usize {
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        pos
    }

    fn skip_whitespace_backward(&self, chars: &[char], mut pos: usize) -> usize {
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }
        pos
    }

    fn find_word_end(&self, chars: &[char], start: usize) -> Option<usize> {
        if start >= chars.len() {
            return None;
        }

        let char_type = CharType::from_char(chars[start]);
        if char_type == CharType::Whitespace {
            return None;
        }

        let mut pos = start;
        while pos < chars.len() && CharType::from_char(chars[pos]) == char_type {
            pos += 1;
        }
        Some(pos - 1)
    }

    fn find_word_start(&self, chars: &[char], start: usize) -> Option<usize> {
        if start >= chars.len() {
            return None;
        }

        let char_type = CharType::from_char(chars[start]);
        if char_type == CharType::Whitespace {
            return None;
        }

        let mut pos = start;
        while pos > 0 && CharType::from_char(chars[pos]) == char_type {
            pos -= 1;
        }

        // Adjust if we stopped due to character type change
        if (pos > 0 || CharType::from_char(chars[pos]) != char_type)
            && CharType::from_char(chars[pos]) != char_type
        {
            pos += 1;
        }
        Some(pos)
    }

    fn is_at_word_end(&self, chars: &[char], pos: usize) -> bool {
        if pos >= chars.len() {
            return false;
        }

        let current_type = CharType::from_char(chars[pos]);
        if current_type == CharType::Whitespace {
            return false;
        }

        // At end if next char is different type or we're at line end
        pos + 1 >= chars.len() || CharType::from_char(chars[pos + 1]) != current_type
    }
}
