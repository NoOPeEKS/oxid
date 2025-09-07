use super::core::Buffer;
use super::types::BufferPosition;

impl Buffer {
    pub fn ensure_cursor_visible(&mut self) {
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

    pub fn get_visible_lines(&self) -> Vec<ropey::RopeSlice> {
        let start = self.vertical_scroll;
        let end = std::cmp::min(start + self.viewport_height, self.file_text.len_lines());

        (start..end).map(|i| self.file_text.line(i)).collect()
    }

    pub fn get_visible_line_content(&self, line: ropey::RopeSlice) -> String {
        let start_col = self.horizontal_scroll;
        let line_len = line.len_chars();

        if start_col >= line_len {
            return String::new();
        }

        let end_col = std::cmp::min(start_col + self.viewport_width, line_len);

        // TODO: Check this unwrap
        line.get_slice(start_col..end_col).unwrap().to_string()
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
}
