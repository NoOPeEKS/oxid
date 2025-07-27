use crate::buffer::Buffer;
use crate::buffer::types::Selection;
use crate::events::EventKind;
use crate::ui::ui;
use std::sync::mpsc::Receiver;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub buffers: Vec<Buffer>,
    pub debug_mode: bool,
}

impl App {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            buffers,
            debug_mode: false,
        }
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
    }

    pub fn visual_mode(&mut self) {
        if self.mode == Mode::Normal {
            // First time on visual, so start saving selection.
            self.mode = Mode::Visual;
            self.buffers[0].selection = Some(Selection {
                start: self.buffers[0].current_position.clone(),
                end: self.buffers[0].current_position.clone(),
            });
        } else {
            self.mode = Mode::Normal;
        }
    }

    pub fn run(
        &mut self,
        event_receiver: Receiver<EventKind>,
        terminal: &mut ratatui::DefaultTerminal,
    ) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if let Ok(event) = event_receiver.recv() {
                match event {
                    EventKind::SaveFile => {
                        self.buffers[0].save_file()?;
                        self.normal_mode();
                    }
                    EventKind::Quit => self.quitting = true,
                    EventKind::NormalMode => self.normal_mode(),
                    EventKind::ScrollUp => self.buffers[0].scroll_up(5),
                    EventKind::ScrollDown => self.buffers[0].scroll_down(5),
                    EventKind::KeyPressed(ch) => {
                        if self.mode == Mode::Normal || self.mode == Mode::Visual {
                            let vis = self.mode == Mode::Visual;
                            match ch {
                                'v' => self.visual_mode(),
                                'h' => {
                                    self.buffers[0].move_cursor_left();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'j' => {
                                    self.buffers[0].move_cursor_down();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'k' => {
                                    self.buffers[0].move_cursor_up();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'l' => {
                                    self.buffers[0].move_cursor_right();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'w' => {
                                    self.buffers[0].move_to_next_word();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'b' => {
                                    self.buffers[0].move_to_previous_word();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'e' => {
                                    self.buffers[0].move_to_end_of_word();
                                    if vis {
                                        if let Some(selection) = &self.buffers[0].selection {
                                            let start = selection.start.clone();
                                            let end = self.buffers[0].current_position.clone();
                                            self.buffers[0].selection =
                                                Some(Selection { start, end });
                                        }
                                    }
                                }
                                'i' => self.insert_mode(),
                                'o' => {
                                    self.buffers[0].insert_line_below();
                                    self.insert_mode();
                                }
                                '0' => self.buffers[0].move_cursor_start_line(),
                                '$' => self.buffers[0].move_cursor_end_line(),
                                _ => {}
                            }
                        } else if self.mode == Mode::Insert {
                            self.buffers[0].insert_char(ch);
                        }
                    }
                    EventKind::ShiftedKey(ch) => {
                        if self.mode == Mode::Normal {
                            match ch {
                                'I' => {
                                    self.buffers[0].move_cursor_start_line();
                                    self.insert_mode();
                                }
                                'A' => {
                                    self.buffers[0].move_cursor_end_line();
                                    self.insert_mode();
                                }
                                _ => {}
                            }
                        } else if self.mode == Mode::Insert
                            && (ch.is_alphanumeric() || ch.is_ascii_punctuation())
                        {
                            self.buffers[0].insert_char(ch);
                        }
                    }
                    EventKind::Backspace => {
                        if self.mode == Mode::Insert {
                            self.buffers[0].remove_char();
                        }
                    }
                    EventKind::EnterKey => {
                        if self.mode == Mode::Insert {
                            self.buffers[0].enter_key();
                        }
                    }
                }
            }

            if self.quitting {
                return Ok(());
            }
        }
    }
}
