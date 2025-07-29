use super::types::{BufferPosition, FileLine, Selection};

pub const STATUSBAR_SPACE: usize = 1;

pub struct Buffer {
    pub file_path: Option<String>,
    pub file_lines: Vec<FileLine>,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub current_position: BufferPosition,
    pub numbar_space: usize,
    pub selection: Option<Selection>,
    pub selected_string: Option<String>,
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
            selection: None,
            selected_string: None,
        }
    }
}
