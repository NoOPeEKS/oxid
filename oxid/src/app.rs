use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::execute;
use ropey::Rope;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::BufReader;
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
    pub tsize_x: usize,
    pub tsize_y: usize,
    pub quitting: bool,
    pub buffers: Vec<Buffer>,
    pub current_buf_index: usize,
    pub registers: HashMap<String, String>,
    pub command: Option<String>,
    pub debug_mode: bool,
}

impl App {
    pub fn new(buffers: Vec<Buffer>, tsize_x: usize, tsize_y: usize) -> Self {
        App {
            mode: Mode::Normal,
            tsize_x,
            tsize_y,
            quitting: false,
            buffers,
            current_buf_index: 0,
            registers: HashMap::from([(String::from("default"), String::new())]),
            command: None,
            debug_mode: false,
        }
    }

    pub fn insert_mode(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        self.mode = Mode::Insert;
        execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBar).unwrap();
    }

    pub fn normal_mode(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        self.mode = Mode::Normal;
        execute!(terminal.backend_mut(), SetCursorStyle::BlinkingBlock).unwrap();
    }

    pub fn visual_mode(&mut self, terminal: &mut ratatui::DefaultTerminal) {
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
            self.normal_mode(terminal);
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

    pub fn apply_command(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        if let Some(cmd_str) = &self.command {
            match Command::parse(cmd_str) {
                Ok(cmd_type) => {
                    match cmd_type {
                        Command::SaveCurrentFile => {
                            // TODO: Handle this gracefully.
                            self.buffers[self.current_buf_index]
                                .save_file()
                                .expect("Could not save current file.");
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::SaveAll => {
                            self.buffers.iter().for_each(|buf| {
                                // TODO: Handle this better
                                buf.save_file().expect("Could not save all files.");
                            });
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::QuitCurrentFile => {
                            // If we only have 1 buffer, quit the app directly
                            if self.buffers.len() == 1 {
                                self.quitting = true;
                            }
                            // -2 because we are gonna remove one more right now, to avoid an extra assign.
                            let num_buffers = self.buffers.len() - 2;
                            _ = self.buffers.remove(self.current_buf_index);
                            if self.current_buf_index + 1 >= num_buffers {
                                self.current_buf_index = 0;
                            } else {
                                self.current_buf_index += 1;
                            }
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::QuitAll => {
                            self.normal_mode(terminal);
                            self.command = None;
                            self.quitting = true;
                        }
                        Command::SaveQuitAll => {
                            self.normal_mode(terminal);
                            self.command = None;
                            self.buffers.iter().for_each(|buf| {
                                // TODO: Handle this better
                                buf.save_file().expect("Could not save all files.");
                            });
                            self.quitting = true;
                        }
                        Command::NextBuffer => {
                            // .len() and not .len() - 1 bc we want only 0 when index would be
                            // greater than allowed index (len() - 1).
                            if self.current_buf_index + 1 == self.buffers.len() {
                                self.current_buf_index = 0;
                            } else {
                                self.current_buf_index += 1;
                            }
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::PreviousBuffer => {
                            if self.current_buf_index as isize - 1 == -1 {
                                self.current_buf_index = self.buffers.len() - 1;
                            } else {
                                self.current_buf_index -= 1;
                            }
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::OpenFile(file_path) => {
                            if let Some(buffer) = self.create_new_buffer(file_path) {
                                self.buffers.push(buffer);
                                self.current_buf_index = self.buffers.len() - 1;
                            }
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                        Command::GoToLine(line_num) => {
                            let max_buf_lines =
                                self.buffers[self.current_buf_index].file_text.len_lines() - 1;

                            if line_num == -1 || line_num > max_buf_lines as isize {
                                self.buffers[self.current_buf_index].current_position.line =
                                    max_buf_lines;
                                self.buffers[self.current_buf_index].ensure_cursor_visible();
                            } else {
                                self.buffers[self.current_buf_index].current_position.line =
                                    line_num as usize;
                                self.buffers[self.current_buf_index].ensure_cursor_visible();
                            }
                            self.normal_mode(terminal);
                            self.command = None;
                        }
                    }
                }
                Err(_) => {
                    // TODO: handle showing command error to editor
                    self.normal_mode(terminal);
                    self.command = None;
                }
            }
        } else {
            self.normal_mode(terminal);
            self.command = None;
        }
    }

    fn create_new_buffer(&self, file_path: String) -> Option<Buffer> {
        if let Ok(file_handler) = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)
        {
            if let Ok(file_text) = Rope::from_reader(BufReader::new(file_handler)) {
                let buf = Buffer::new(Some(file_path), file_text, self.tsize_x, self.tsize_y);
                return Some(buf);
            }
            return None;
        }
        None
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
                        self.normal_mode(terminal);
                    }
                    EventKind::Quit => self.quitting = true,
                    EventKind::NormalMode => {
                        self.normal_mode(terminal);
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
                                'v' => self.visual_mode(terminal),
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
                                        self.insert_mode(terminal)
                                    }
                                }
                                'o' => {
                                    if !vis {
                                        self.buffers[self.current_buf_index].insert_line_below();
                                        self.insert_mode(terminal);
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
                                            self.normal_mode(terminal);
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
                                    self.insert_mode(terminal);
                                }
                                'A' => {
                                    self.buffers[self.current_buf_index].move_cursor_end_line();
                                    self.insert_mode(terminal);
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
                            self.apply_command(terminal);
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
