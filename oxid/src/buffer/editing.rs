use super::core::Buffer;

impl Buffer {
    pub fn paste(&mut self, paste_string: String) {
        let curr_line = self.current_position.line;
        let curr_char = self.current_position.character - self.numbar_space;

        let curr_string = self.file_text.line(curr_line).to_string();

        // Ensure curr_char doesn't exceed the line length
        let safe_curr_char = curr_char.min(curr_string.len());

        let start_until_cursor = &curr_string[0..safe_curr_char];
        let rest_original_string = &curr_string[safe_curr_char..];

        let newlines: Vec<_> = paste_string.lines().collect();

        // Handle empty paste string
        if newlines.is_empty() {
            return;
        }

        if newlines.len() == 1 {
            // Single line paste means just insert at cursor position.
            let mut new_str = String::from(start_until_cursor);
            new_str.push_str(&paste_string);
            new_str.push_str(rest_original_string);

            let line_len = self.file_text.line(curr_line).len_chars();
            let start_line_char = self.file_text.line_to_char(curr_line);
            let end_line_char = start_line_char + line_len;

            if start_line_char <= end_line_char && end_line_char <= self.file_text.len_chars() {
                self.file_text.remove(start_line_char..end_line_char);
                self.file_text.insert(start_line_char, &new_str);
            }
        } else {
            // Multi-line paste
            let mut new_text = String::from(start_until_cursor);
            new_text.push_str(&paste_string);
            new_text.push_str(rest_original_string);

            let line_len = self.file_text.line(curr_line).len_chars();
            let start_line_char = self.file_text.line_to_char(curr_line);
            let end_line_char = start_line_char + line_len;

            self.file_text.remove(start_line_char..end_line_char);
            self.file_text.insert(start_line_char, &new_text);
        }
        self.update_numbar_space();
        self.ensure_cursor_visible();
    }

    pub fn update_numbar_space(&mut self) {
        let numbar_space = self.file_text.len_lines().to_string().len() + 1;
        if self.numbar_space != numbar_space {
            self.numbar_space = self.file_text.len_lines().to_string().len() + 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        let line = self.current_position.line;
        let character = self.current_position.character - self.numbar_space;
        let mut char_idx = self.file_text.line_to_char(line);
        char_idx = char_idx.saturating_add(character);
        self.file_text.insert_char(char_idx, ch);
        self.update_numbar_space();
        self.current_position.character = self.current_position.character.saturating_add(1);

        self.ensure_cursor_visible();
    }

    pub fn remove_char(&mut self) {
        let curr_line = self.current_position.line;
        let curr_char = self
            .current_position
            .character
            .saturating_sub(self.numbar_space);
        let line_start_char = self.file_text.line_to_char(curr_line);

        // Nothing to delete if at start of file
        if curr_line == 0 && curr_char == 0 {
            return;
        }

        if curr_char > 0 {
            // Regular backspace: delete the character just before the cursor
            self.file_text
                .remove(line_start_char + curr_char - 1..line_start_char + curr_char);
            self.current_position.character = self.current_position.character.saturating_sub(1);
        } else {
            // We're at the start of a line -> merge with previous line
            let prev_line = curr_line - 1;
            let prev_line_len = self.file_text.line(prev_line).len_chars() -1;

            // Remove the line break between prev_line and curr_line
            let prev_line_end = self.file_text.line_to_char(prev_line) + prev_line_len;
            let curr_line_start = line_start_char;
            self.file_text.remove(prev_line_end..curr_line_start);

            // Update cursor: move to end of previous line
            self.current_position.line = prev_line;
            self.current_position.character = prev_line_len + self.numbar_space;
        }

        self.ensure_cursor_visible();
        // let curr_line = self.current_position.line;
        // let curr_char = self.current_position.character - self.numbar_space;
        // let line_to_char = self.file_text.line_to_char(curr_line);
        //
        // if curr_line == 0 && curr_char == 0 {
        //     return;
        // }
        //
        // // TODO: Deleting in same line works, but need to move cursor to other lines if deleting
        // // goes to above line.
        // self.file_text.remove(
        //     line_to_char.saturating_add(curr_char).saturating_sub(1)
        //         ..line_to_char.saturating_add(curr_char),
        // );
        // self.current_position.character = self.current_position.character.saturating_sub(1);
        // if self.current_position.character - self.numbar_space == 0 {
        //     self.current_position.line = curr_line.saturating_sub(1);
        //     self.current_position.character =
        //         self.file_text.line(self.current_position.line).len_chars() + self.numbar_space;
        // }
        // self.ensure_cursor_visible();
    }

    pub fn enter_key(&mut self) {
        let line = self.current_position.line;
        let character = self.current_position.character - self.numbar_space;
        let mut char_idx = self.file_text.line_to_char(line);
        char_idx = char_idx.saturating_add(character);
        self.file_text.insert(char_idx, "\n");
        self.update_numbar_space();
        self.current_position.line = self.current_position.line.saturating_add(1);
        self.current_position.character = self.numbar_space;
        self.ensure_cursor_visible();
        // let curr_line = self.file_lines[self.current_position.line].clone();
        // let current_character = self.current_position.character;
        // // If cursor is at the end of the line, just include empty line below.
        // if current_character - self.numbar_space == curr_line.length {
        //     self.file_lines.insert(
        //         self.current_position.line + 1,
        //         FileLine {
        //             content: String::from(""),
        //             length: 0,
        //         },
        //     );
        //     self.update_numbar_space();
        //     self.current_position.line = self.current_position.line.saturating_add(1);
        //     self.current_position.character = self.numbar_space;
        // } else if current_character >= self.numbar_space && current_character < curr_line.length {
        //     // If cursor is anywhere between the line, move cursor + forward to next line.
        //     // So basically, current line should be line[0..cursor] and next line should be
        //     // line[cursor..]
        //     self.file_lines[self.current_position.line].content =
        //         curr_line.content[0..current_character - self.numbar_space].to_string();
        //
        //     let new_line_content =
        //         curr_line.content[current_character - self.numbar_space..].to_string();
        //
        //     self.file_lines.insert(
        //         self.current_position.line + 1,
        //         FileLine {
        //             content: new_line_content.clone(),
        //             length: new_line_content.len(),
        //         },
        //     );
        //     self.update_numbar_space();
        //     self.current_position.line = self.current_position.line.saturating_add(1);
        //     self.current_position.character = self.numbar_space;
        // }
        // self.ensure_cursor_visible();
    }

    pub fn save_file(&self) -> anyhow::Result<()> {
        // let lines_vec: Vec<_> = self
        //     .file_lines
        //     .iter()
        //     .map(|fl| fl.content.clone())
        //     .collect();
        let text = self.file_text.to_string();
        if let Some(filepath) = &self.file_path {
            std::fs::write(filepath, text)?;
        } else {
            anyhow::bail!("No filepath provided, cannot save file...")
        }
        Ok(())
    }
}
