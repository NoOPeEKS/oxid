use crate::buffer::Buffer;
use crate::events::EventKind;
use crate::ui::ui;
use std::sync::mpsc::Receiver;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub buffers: Vec<Buffer>,
}

impl App {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            buffers,
        }
    }

    pub fn insert_mode(&mut self) {
        self.mode = Mode::Insert
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal
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
                        if self.mode == Mode::Normal {
                            match ch {
                                'h' => self.buffers[0].move_cursor_left(),
                                'j' => self.buffers[0].move_cursor_down(),
                                'k' => self.buffers[0].move_cursor_up(),
                                'l' => self.buffers[0].move_cursor_right(),
                                'i' => self.insert_mode(),
                                'o' => {
                                    self.buffers[0].insert_line_below();
                                    self.insert_mode();
                                }
                                _ => {}
                            }
                        } else if self.mode == Mode::Insert {
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
                    },
                }
            }

            if self.quitting {
                return Ok(());
            }
        }
    }
}
