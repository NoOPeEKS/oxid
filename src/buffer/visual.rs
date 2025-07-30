use super::core::Buffer;

impl Buffer {
    pub fn update_selected_string(&mut self) {
        if let Some(selection) = &self.selection {
            // This means we're just selecting some substring of that line.
            if selection.start.line == selection.end.line {
                let start = selection.start.character - self.numbar_space;
                let end = selection.end.character - self.numbar_space;
                // Normalize so that we don't get indexing errors if selection went backwards.
                let substr = if start >= end {
                    &self.file_lines[selection.start.line].content[end..start]
                } else {
                    &self.file_lines[selection.start.line].content[start..end]
                };
                self.selected_string = Some(String::from(substr));
            }
            // This means selection starts in one line and finishes in another one.
            else {
                let start_str = &self.file_lines[selection.start.line].content
                    [selection.start.character - self.numbar_space..];

                let mut final_string = String::from(start_str);

                // We can do this because selection is always normalized.
                let num_lines = selection.end.line - selection.start.line;

                // We get each other line completely besides the last line.
                for i in 0..num_lines - 1 {
                    let line = &self.file_lines[selection.start.line + i + 1].content;
                    final_string.push_str(line);
                }
                // Last line is handled by only taking from 0 to selection.end.char.
                let last_line = &self.file_lines[selection.end.line].content
                    [0..selection.end.character - self.numbar_space];

                final_string.push_str(last_line);
                self.selected_string = Some(final_string);
            }
        } else {
            self.selected_string = None;
        }
    }
}
