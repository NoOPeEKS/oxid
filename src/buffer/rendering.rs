use super::core::Buffer;
use super::types::{BufferPosition, FileLine};

impl Buffer {
    pub(super) fn ensure_cursor_visible(&mut self) {
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
}

