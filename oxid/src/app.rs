use std::collections::HashMap;
use std::fmt::Display;
use std::sync::mpsc::Receiver;

use crate::buffer::Buffer;
use crate::buffer::types::Selection;
use crate::command::Command;
use crate::events::EventKind;
use crate::ui::ui;

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

pub struct App {
    pub mode: Mode,
    pub quitting: bool,
    pub buffers: Vec<Buffer>,
    pub current_buf_index: usize,
    pub registers: HashMap<String, String>,
    pub command: Option<String>,
    pub debug_mode: bool,
}

impl App {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        App {
            mode: Mode::Normal,
            quitting: false,
            buffers,
            current_buf_index: 0,
            registers: HashMap::from([(String::from("default"), String::new())]),
            command: None,
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
    }
    pub fn command_mode(&mut self) {
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
    }

    pub fn apply_command(&mut self) {
        if let Some(cmd_str) = &self.command {
            match Command::parse(cmd_str) {
                Ok(cmd_type) => {
                    match cmd_type {
                        Command::SaveCurrentFile(file_name) => {
                            self.buffers.iter().for_each(|buf| {
                                if let Some(file_path) = &buf.file_path {
                                    if *file_path == file_name {
                                        buf.save_file().unwrap();
                                    }
                                }
                            });
                            self.mode = Mode::Normal;
                            self.command = None;
                        }
                        Command::SaveAll => {
                            self.buffers.iter().for_each(|buf| {
                                // TODO: Handle this better
                                buf.save_file().expect("Could not save all files.");
                            });
                            self.mode = Mode::Normal;
                            self.command = None;
                        }
                        Command::QuitAll => {
                            self.mode = Mode::Normal;
                            self.command = None;
                            self.quitting = true;
                        }
                        Command::SaveQuitAll => {
                            self.mode = Mode::Normal;
                            self.command = None;
                            self.buffers.iter().for_each(|buf| {
                                // TODO: Handle this better
                                buf.save_file().expect("Could not save all files.");
                            });
                            self.quitting = true;
                        }
                        // TODO: Implement the rest of these commands.
                        Command::OpenFile(_) => todo!(":e command is not implemented yet!"),
                        Command::QuitCurrentFile(_) => {
                            todo!(":q <file_name> is not implemented yet!")
                        }
                        Command::NextBuffer => todo!(":bn is not implemented yet!"),
                        Command::PreviousBuffer => todo!(":bp is not implemented yet!"),
                    }
                }
                Err(_) => {
                    // TODO: handle showing command error to editor
                    self.mode = Mode::Normal;
                    self.command = None;
                }
            }
        } else {
            self.mode = Mode::Normal;
            self.command = None;
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
                        self.buffers[self.current_buf_index].save_file()?;
                        self.normal_mode();
                    }
                    EventKind::Quit => self.quitting = true,
                    EventKind::NormalMode => {
                        self.normal_mode();
                        self.buffers[self.current_buf_index].selection = None;
                        self.buffers[self.current_buf_index].selected_string = None;
                    }
                    EventKind::ScrollUp => self.buffers[self.current_buf_index].scroll_up(5),
                    EventKind::ScrollDown => self.buffers[self.current_buf_index].scroll_down(5),
                    EventKind::KeyPressed(ch) => {
                        if self.mode == Mode::Command {
                            if let Some(cmd_str) = &mut self.command {
                                cmd_str.push(ch);
                            } else {
                                self.command = Some(String::from(ch));
                            }
                        }
                        if self.mode == Mode::Normal || self.mode == Mode::Visual {
                            let vis = self.mode == Mode::Visual;
                            match ch {
                                ':' => self.command_mode(),
                                'v' => self.visual_mode(),
                                'h' => {
                                    self.buffers[self.current_buf_index].move_cursor_left();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'j' => {
                                    self.buffers[self.current_buf_index].move_cursor_down();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'k' => {
                                    self.buffers[self.current_buf_index].move_cursor_up();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'l' => {
                                    self.buffers[self.current_buf_index].move_cursor_right();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'w' => {
                                    self.buffers[self.current_buf_index].move_to_next_word();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'b' => {
                                    self.buffers[self.current_buf_index].move_to_previous_word();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'e' => {
                                    self.buffers[self.current_buf_index].move_to_end_of_word();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'i' => {
                                    if !vis {
                                        self.insert_mode()
                                    }
                                }
                                'o' => {
                                    if !vis {
                                        self.buffers[self.current_buf_index].insert_line_below();
                                        self.insert_mode();
                                    }
                                }
                                '0' => {
                                    self.buffers[self.current_buf_index].move_cursor_start_line();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                '$' => {
                                    self.buffers[self.current_buf_index].move_cursor_end_line();
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selection
                                        {
                                            let start = selection.start.clone();
                                            let end = self.buffers[self.current_buf_index]
                                                .current_position
                                                .clone();
                                            self.buffers[self.current_buf_index].selection =
                                                Some(Selection { start, end });
                                            self.buffers[self.current_buf_index]
                                                .update_selected_string();
                                        }
                                    }
                                }
                                'y' => {
                                    if vis {
                                        if let Some(selection) =
                                            &self.buffers[self.current_buf_index].selected_string
                                        {
                                            self.registers.insert(
                                                String::from("default"),
                                                selection.to_string(),
                                            );
                                            self.normal_mode();
                                            self.buffers[self.current_buf_index].selection = None;
                                        }
                                    }
                                }
                                'p' => {
                                    if !vis {
                                        if let Some(paste_string) = self.registers.get("default") {
                                            if !paste_string.is_empty() {
                                                self.buffers[self.current_buf_index]
                                                    .paste(paste_string.to_owned());
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        } else if self.mode == Mode::Insert {
                            self.buffers[self.current_buf_index].insert_char(ch);
                        }
                    }
                    EventKind::ShiftedKey(ch) => {
                        if self.mode == Mode::Normal {
                            match ch {
                                'I' => {
                                    self.buffers[self.current_buf_index].move_cursor_start_line();
                                    self.insert_mode();
                                }
                                'A' => {
                                    self.buffers[self.current_buf_index].move_cursor_end_line();
                                    self.insert_mode();
                                }
                                _ => {}
                            }
                        } else if self.mode == Mode::Insert
                            && (ch.is_alphanumeric() || ch.is_ascii_punctuation())
                        {
                            self.buffers[self.current_buf_index].insert_char(ch);
                        } else if self.mode == Mode::Command {
                            if let Some(cmd_str) = &mut self.command {
                                cmd_str.push(ch);
                            } else {
                                self.command = Some(String::from(ch));
                            }
                        }
                    }
                    EventKind::Backspace => {
                        if self.mode == Mode::Insert {
                            self.buffers[self.current_buf_index].remove_char();
                        }
                        if self.mode == Mode::Command {
                            if let Some(command) = &mut self.command {
                                command.pop();
                            }
                        }
                    }
                    EventKind::EnterKey => {
                        if self.mode == Mode::Insert {
                            self.buffers[self.current_buf_index].enter_key();
                        } else if self.mode == Mode::Command {
                            self.apply_command();
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
