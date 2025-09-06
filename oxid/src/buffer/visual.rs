use super::core::Buffer;

impl Buffer {
    pub fn update_selected_string(&mut self) {
        if let Some(selection) = &self.selection {
            // Single line selection
            if selection.start.line == selection.end.line {
                let start = selection.start.character - self.numbar_space;
                let end = selection.end.character - self.numbar_space;
                // Normalize so that we don't get indexing errors if selection went backwards.
                let substr = if start >= end {
                    self.file_text
                        .line(selection.start.line)
                        .get_slice(end..start)
                        .unwrap()
                        .to_string()
                } else {
                    self.file_text
                        .line(selection.start.line)
                        .get_slice(start..end)
                        .unwrap()
                        .to_string()
                };
                self.selected_string = Some(substr);
            }
            // Multi-line selection going down
            else if selection.start.line < selection.end.line {
                let mut final_string = String::new();

                // First line: from start character to end of line
                let start_str = self
                    .file_text
                    .line(selection.start.line)
                    .slice(selection.start.character - self.numbar_space..)
                    .to_string();
                final_string.push_str(&start_str);

                // Add newline only if we're going to add more content
                if selection.end.line > selection.start.line {
                    final_string.push('\n');
                }

                // Middle lines: complete lines
                for line_num in (selection.start.line + 1)..selection.end.line {
                    let line = self.file_text.line(line_num).as_str().unwrap();
                    final_string.push_str(line);
                    final_string.push('\n');
                }

                // Last line: from beginning to end character
                if selection.end.line > selection.start.line {
                    let last_line = self
                        .file_text
                        .line(selection.end.line)
                        .slice(0..selection.end.character - self.numbar_space)
                        .to_string();
                    final_string.push_str(&last_line);
                }

                self.selected_string = Some(final_string);
            }
            // Multi-line selection going up (backwards selection)
            else {
                let mut final_string = String::new();

                // First line: from end character to end of line
                let start_str = self
                    .file_text
                    .line(selection.end.line)
                    .slice(selection.end.character - self.numbar_space..)
                    .to_string();
                final_string.push_str(&start_str);

                // Add newline only if we're going to add more content
                if selection.start.line > selection.end.line {
                    final_string.push('\n');
                }

                // Middle lines: complete lines
                for line_num in (selection.end.line + 1)..selection.start.line {
                    let line = self.file_text.line(line_num).to_string();
                    final_string.push_str(&line);
                    final_string.push('\n');
                }

                // Last line: from beginning to start character
                if selection.start.line > selection.end.line {
                    let last_line = self
                        .file_text
                        .line(selection.start.line)
                        .slice(0..selection.start.character - self.numbar_space)
                        .to_string();
                    final_string.push_str(&last_line);
                }

                self.selected_string = Some(final_string);
            }
        } else {
            self.selected_string = None;
        }
    }
}
