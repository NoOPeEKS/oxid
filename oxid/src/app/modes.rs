use ratatui::DefaultTerminal;
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::execute;

use std::fmt::Display;

use super::App;
use crate::buffer::types::Selection;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Insert => write!(f, "Insert"),
            Self::Visual => write!(f, "Visual"),
            Self::Command => write!(f, "Command"),
        }
    }
}

impl App {
    pub fn set_mode(&mut self, terminal: &mut DefaultTerminal, mode: Mode) {
        match mode {
            Mode::Normal => {
                self.mode = Mode::Normal;
                execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBlock).unwrap();
            }
            Mode::Insert => {
                self.mode = Mode::Insert;
                execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBar).unwrap();
            }
            Mode::Visual => {
                if self.mode == Mode::Normal {
                    // First time on visual, so start saving selection.
                    self.mode = Mode::Visual;
                    self.buffers[self.current_buf_index].selection = Some(Selection {
                        start: self.buffers[self.current_buf_index]
                            .current_position
                            .clone(),
                        end: self.buffers[self.current_buf_index]
                            .current_position
                            .clone(),
                    });
                    self.buffers[self.current_buf_index].update_selected_string();
                } else {
                    // If on whatever mode but normal, stop selecting and reset.
                    self.mode = Mode::Normal;
                    self.buffers[self.current_buf_index].selection = None;
                    self.buffers[self.current_buf_index].update_selected_string();
                }
                execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBlock).unwrap();
            }
            Mode::Command => {
                if self.mode == Mode::Normal {
                    self.mode = Mode::Command;
                    self.command = None;
                } else {
                    // If we don't come from normal mode, just reset everything
                    // at least for now.
                    self.mode = Mode::Command;
                    self.buffers[self.current_buf_index].selection = None;
                    self.buffers[self.current_buf_index].update_selected_string();
                    self.command = None;
                }
                execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBlock).unwrap();
            }
        }
    }
}
