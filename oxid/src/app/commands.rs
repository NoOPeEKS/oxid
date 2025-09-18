use ratatui::DefaultTerminal;
use ropey::Rope;

use std::{fs::OpenOptions, io::BufReader};

use super::App;
use crate::buffer::Buffer;
use crate::{app::modes::Mode, command::Command};

impl App {
    pub fn apply_command(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(cmd_str) = &self.command {
            match Command::parse(cmd_str) {
                Ok(cmd) => self.execute_command(cmd, terminal),
                Err(_) => self.reset_command(terminal),
            }
        } else {
            self.reset_command(terminal);
        }
    }

    fn reset_command(&mut self, terminal: &mut DefaultTerminal) {
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn execute_command(&mut self, cmd: Command, terminal: &mut DefaultTerminal) {
        match cmd {
            Command::SaveAll => self.save_all(terminal),
            Command::QuitAll => self.quit_all(terminal),
            Command::SaveQuitAll => self.save_quit_all(terminal),
            Command::QuitCurrentFile => self.quit_current_file(terminal),
            Command::SaveCurrentFile => self.save_current_file(terminal),
            Command::NextBuffer => self.next_buffer(terminal),
            Command::PreviousBuffer => self.previous_buffer(terminal),
            Command::OpenFile(path) => self.open_file(path, terminal),
            Command::GoToLine(line) => self.go_to_line(line, terminal),
        }
    }

    fn save_current_file(&mut self, terminal: &mut DefaultTerminal) {
        // TODO: Handle this better
        self.buffers[self.current_buf_index]
            .save_file()
            .expect("Could not save current file.");
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn save_all(&mut self, terminal: &mut DefaultTerminal) {
        self.buffers.iter().for_each(|buf| {
            // TODO: Handle this better
            buf.save_file().expect("Could not save all files");
        });
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn quit_current_file(&mut self, terminal: &mut DefaultTerminal) {
        if self.buffers.len() == 1 {
            if let Some(lsp) = self.lsp_client.as_mut() {
                _ = lsp.shutdown();
            }
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
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn quit_all(&mut self, terminal: &mut DefaultTerminal) {
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
        if let Some(lsp) = self.lsp_client.as_mut() {
            _ = lsp.shutdown();
        }
        self.quitting = true;
    }
    fn save_quit_all(&mut self, terminal: &mut DefaultTerminal) {
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
        self.buffers.iter().for_each(|buf| {
            // TODO: Handle this better
            buf.save_file().expect("Could not save all files.");
        });
        if let Some(lsp) = self.lsp_client.as_mut() {
            _ = lsp.shutdown();
        }
        self.quitting = true;
    }
    fn next_buffer(&mut self, terminal: &mut DefaultTerminal) {
        // .len() and not .len() - 1 bc we want only 0 when index would be
        // greater than allowed index (len() - 1).
        if self.current_buf_index + 1 == self.buffers.len() {
            self.current_buf_index = 0;
        } else {
            self.current_buf_index += 1;
        }
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn previous_buffer(&mut self, terminal: &mut DefaultTerminal) {
        if self.current_buf_index as isize - 1 == -1 {
            self.current_buf_index = self.buffers.len() - 1;
        } else {
            self.current_buf_index -= 1;
        }
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn open_file(&mut self, file: String, terminal: &mut DefaultTerminal) {
        if let Some(buffer) = self.create_new_buffer(file) {
            self.buffers.push(buffer);
            self.current_buf_index = self.buffers.len() - 1;
        }
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
    }

    fn go_to_line(&mut self, line_num: isize, terminal: &mut DefaultTerminal) {
        let max_buf_lines = self.buffers[self.current_buf_index].file_text.len_lines() - 1;

        if line_num == -1 || line_num > max_buf_lines as isize {
            self.buffers[self.current_buf_index].current_position.line = max_buf_lines;
            self.buffers[self.current_buf_index].ensure_cursor_visible();
        } else {
            self.buffers[self.current_buf_index].current_position.line = line_num as usize;
            self.buffers[self.current_buf_index].ensure_cursor_visible();
        }
        self.set_mode(terminal, Mode::Normal);
        self.command = None;
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
}
