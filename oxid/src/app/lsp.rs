use oxid_lsp::types::{CompletionItem, Position};

use crate::buffer::types::BufferPosition;

use super::App;

impl App {
    pub fn get_diagnostics(&mut self) {
        if let Some(filepath) = &self.buffers[self.current_buf_index].file_path
            && let Some(lsp) = self.lsp_client.as_mut()
        {
            match lsp.get_file_diagnostic(filepath) {
                Ok(diag_opt) => match diag_opt {
                    Some(diag_vec) => self.diagnostics = Some(diag_vec),
                    None => self.diagnostics = None,
                },
                Err(_) => self.diagnostics = None,
            }
        }
    }

    pub fn hover(&mut self) {
        if let Some(curr_file_path) = &self.buffers[self.current_buf_index].file_path
            && let Some(lsp) = self.lsp_client.as_mut()
        {
            let mut pos = self.buffers[self.current_buf_index]
                .current_position
                .clone();

            pos.character = pos
                .character
                .saturating_sub(self.buffers[self.current_buf_index].numbar_space);

            match lsp.hover(
                curr_file_path,
                Position {
                    line: pos.line,
                    character: pos.character,
                },
            ) {
                Ok(hover) => self.hover = Some(hover),
                Err(_) => self.hover = None,
            }
        } else {
            self.hover = None;
        }
    }

    pub fn choose_completion(&mut self, comp_list_idx: usize) {
        if let Some(completion_list) = &self.completion_list {
            if let Some(comp_item) = completion_list.items.get(comp_list_idx) {
                self.selected_completion = Some(comp_item.clone());
            } else {
                self.selected_completion = None;
                self.error = Some("Not get items index".to_string());
            }
        } else {
            self.selected_completion = None;
            self.error = Some("No completion list yet".to_string());
        }
    }

    pub fn next_table_row(&mut self) {
        if let Some(completion_list) = &self.completion_list {
            let len = completion_list.items.len();
            if len == 0 {
                return;
            }

            let i = match self.table_state.selected() {
                Some(i) if i + 1 < len => i + 1,
                _ => len - 1,
            };

            self.table_state.select(Some(i));
            self.choose_completion(i);

            // keep offset within window
            let max_items = 6;
            if i >= self.completion_offset + max_items {
                self.completion_offset = i + 1 - max_items;
            }
        }
    }

    pub fn previous_table_row(&mut self) {
        if let Some(completion_list) = &self.completion_list {
            let len = completion_list.items.len();
            if len == 0 {
                return;
            }

            let i = match self.table_state.selected() {
                Some(i) if i > 0 => i - 1,
                _ => 0,
            };

            self.table_state.select(Some(i));
            self.choose_completion(i);

            // keep offset within window
            if i < self.completion_offset {
                self.completion_offset = i;
            }
        }
    }

    pub fn insert_completion(&mut self, completion: CompletionItem, buffer_pos: BufferPosition) {
        let buffer = &mut self.buffers[self.current_buf_index];
        let line_start_idx = buffer.file_text.line_to_char(buffer_pos.line);
        let mut start_idx = line_start_idx + buffer_pos.character - buffer.numbar_space;

        // Scan backwards to find the start of the current identifier
        while start_idx > line_start_idx {
            let c = buffer.file_text.char(start_idx - 1);
            if !c.is_alphanumeric() && c != '_' {
                // Stop BEFORE a separator, but keep :: intact
                if c == ':'
                    && start_idx > line_start_idx + 1
                    && buffer.file_text.char(start_idx - 2) == ':'
                {
                    // cursor is after ::, so we stop without including ::
                    break;
                }
                break;
            }
            start_idx -= 1;
        }

        let end_idx = line_start_idx + buffer_pos.character - buffer.numbar_space;

        let start_byte = buffer.file_text.char_to_byte(start_idx);
        let end_byte = buffer.file_text.char_to_byte(end_idx);

        buffer.file_text.remove(start_byte..end_byte);
        buffer.file_text.insert(start_byte, &completion.label);

        buffer.current_position.character =
            start_idx + completion.label.chars().count() - line_start_idx + buffer.numbar_space;
    }
}
