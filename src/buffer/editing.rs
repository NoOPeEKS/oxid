use super::core::Buffer;
use super::types::FileLine;

impl Buffer {
    pub fn paste(&mut self, paste_string: String) {
        let curr_line = self.current_position.line;
        let curr_char = self.current_position.character - self.numbar_space;
        let curr_string = self.file_lines[curr_line].content.clone();
        let start_until_cursor = &curr_string[0..curr_char];
        let rest_original_string = &curr_string[curr_char..];
        let newlines: Vec<_> = paste_string.lines().collect();

        // We only handle two cases, if len == 1 or len > 1. We should never receive an empty
        // pastestring here.
        if newlines.len() == 1 {
            // If we have len == 1 this means there were no newlines and we can just append on
            // cursor position and don't have to handle newlines after.
            let mut new_str = String::from(start_until_cursor);
            new_str.push_str(&paste_string);
            new_str.push_str(rest_original_string);
            self.file_lines[curr_line].length = new_str.len();
            self.file_lines[curr_line].content = new_str;
        } else if newlines.len() > 1 {
            // If we enter here, means we need to do multiline selection handling.
            // Update the current line with the first part
            let mut first_line = String::from(start_until_cursor);
            first_line.push_str(newlines[0]);
            self.file_lines[curr_line].length = first_line.len();
            self.file_lines[curr_line].content = first_line;

            // Insert middle lines (if any)
            for (i, str_text) in newlines[1..newlines.len() - 1].iter().enumerate() {
                self.file_lines.insert(
                    curr_line + i + 1,
                    FileLine {
                        content: str_text.to_string(),
                        length: str_text.len(),
                    },
                );
            }

            // Handle the last line
            let last_newline = newlines[newlines.len() - 1];
            let mut last_line = String::from(last_newline);
            last_line.push_str(rest_original_string);
            self.file_lines.insert(
                curr_line + newlines.len() - 1,
                FileLine {
                    content: last_line.clone(),
                    length: last_line.len(),
                },
            );
        }
    }
    pub fn update_numbar_space(&mut self) {
        let numbar_space = self.file_lines.len().to_string().len() + 1;
        if self.numbar_space != numbar_space {
            self.numbar_space = self.file_lines.len().to_string().len() + 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        // TODO: Need to handle edge case where line is new line.
        // Probably gonna have to optimize this later as there are many clones.
        // In the future, handling '\n' or '<CR>' will be tricky.

        let mut curr_line = self.file_lines[self.current_position.line].content.clone();
        let insert_index = self.current_position.character - self.numbar_space;
        if insert_index <= curr_line.len() {
            curr_line.insert(insert_index, ch);
            self.update_numbar_space();
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
                self.update_numbar_space();
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
                self.update_numbar_space();
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
                self.update_numbar_space();

                self.current_position.line = self.current_position.line.saturating_sub(1);
                self.current_position.character = top_line_old_len + self.numbar_space;
            }
        }
        self.ensure_cursor_visible();
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
            self.update_numbar_space();
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
            self.update_numbar_space();
            self.current_position.line = self.current_position.line.saturating_add(1);
            self.current_position.character = self.numbar_space;
        }
        self.ensure_cursor_visible();
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
}
