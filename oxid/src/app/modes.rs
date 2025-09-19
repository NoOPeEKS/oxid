use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::prelude::{Backend, Terminal};

use std::fmt::Display;

use super::App;
use crate::buffer::types::Selection;
use super::blinking::CursorStyleSupport;

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
    pub fn set_mode<B>(&mut self, terminal: &mut Terminal<B>, mode: Mode)
    where
        B: Backend + CursorStyleSupport,
    {
        match mode {
            Mode::Normal => {
                self.mode = Mode::Normal;
                self.completion_list = None;
                self.hover = None;
                self.selected_completion = None;
                self.set_cursor_style(terminal, SetCursorStyle::BlinkingBlock);
            }
            Mode::Insert => {
                self.mode = Mode::Insert;
                self.set_cursor_style(terminal, SetCursorStyle::BlinkingBar);
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
                self.set_cursor_style(terminal, SetCursorStyle::BlinkingBlock);
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
                self.set_cursor_style(terminal, SetCursorStyle::BlinkingBlock);
            }
        }
    }

    fn set_cursor_style<B>(&self, terminal: &mut Terminal<B>, style: SetCursorStyle)
    where
        B: Backend + CursorStyleSupport,
    {
        terminal.backend_mut().set_cursor_style(style);
    }
}
