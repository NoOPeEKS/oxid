use ratatui::DefaultTerminal;

use crate::buffer::types::Selection;
use crate::events::EventKind;

use super::App;
use super::modes::Mode;

impl App {
    pub fn handle_event(
        &mut self,
        event: EventKind,
        terminal: &mut DefaultTerminal,
    ) -> anyhow::Result<()> {
        match event {
            EventKind::RequestCompletion => self.handle_completion()?,
            EventKind::SaveFile => self.handle_save_file(terminal)?,
            EventKind::Quit => self.handle_quit()?,
            EventKind::NormalMode => self.handle_normal_mode(terminal),
            EventKind::ScrollUp => self.scroll_up(),
            EventKind::ScrollDown => self.scroll_down(),
            EventKind::KeyPressed(ch) => self.handle_key(ch, terminal),
            EventKind::ShiftedKey(ch) => self.handle_shifted_key(ch, terminal),
            EventKind::Backspace => self.handle_backspace(),
            EventKind::EnterKey => self.handle_enter(terminal),
            EventKind::Tab => {
                if self.completion_list.is_some() {
                    self.next_table_row();
                }
            }
            EventKind::ShiftTab => {
                if self.completion_list.is_some() {
                    self.previous_table_row();
                }
            }
        }
        Ok(())
    }

    fn handle_completion(&mut self) -> anyhow::Result<()> {
        if self.mode == Mode::Insert {
            if let Some(file_path) = self.buffers[self.current_buf_index].file_path.clone() {
                let file_contents = self.buffers[self.current_buf_index].file_text.to_string();

                self.lsp_client.did_change(&file_path, &file_contents)?;

                self.completion_list = match self.lsp_client.request_completion(
                    &file_path,
                    self.buffers[self.current_buf_index].current_position.line,
                    self.buffers[self.current_buf_index]
                        .current_position
                        .character,
                ) {
                    Ok(opt) => opt,
                    Err(err) => {
                        self.error = Some(err.to_string());
                        None
                    }
                };
                if self.completion_list.is_some() {
                    self.choose_completion(0);
                    self.table_state.select(Some(0));
                    let file_contents = self.buffers[self.current_buf_index].file_text.to_string();
                    self.lsp_client.did_change(&file_path, &file_contents)?;
                }
            }
        }
        Ok(())
    }

    fn handle_save_file(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        if self.buffers[self.current_buf_index].save_file().is_ok() {
            let fp = self.buffers[self.current_buf_index]
                .file_path
                .clone()
                .expect("Filepath should be Some(fp)"); // As we're inside Ok, shouldn't fail.
            let fc = self.buffers[self.current_buf_index].file_text.to_string();
            self.lsp_client.did_save(&fp, &fc)?;
            self.set_mode(terminal, Mode::Normal);
            self.get_diagnostics();
        }
        Ok(())
    }

    fn handle_quit(&mut self) -> anyhow::Result<()> {
        _ = self.lsp_client.shutdown();
        self.quitting = true;
        Ok(())
    }

    fn handle_normal_mode(&mut self, terminal: &mut DefaultTerminal) {
        self.set_mode(terminal, Mode::Normal);
        self.buffers[self.current_buf_index].selection = None;
        self.buffers[self.current_buf_index].selected_string = None;
        self.completion_offset = 0;
        self.selected_completion = None;
        self.completion_list = None;
        self.table_state.select(Some(0));
    }

    fn scroll_up(&mut self) {
        self.buffers[self.current_buf_index].scroll_up(5);
    }

    fn scroll_down(&mut self) {
        self.buffers[self.current_buf_index].scroll_down(5);
    }

    fn handle_key(&mut self, ch: char, terminal: &mut DefaultTerminal) {
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
                '[' => {
                    self.show_diagnostics = !self.show_diagnostics;
                }
                ':' => self.set_mode(terminal, Mode::Command),
                'v' => self.set_mode(terminal, Mode::Visual),
                'h' => {
                    self.buffers[self.current_buf_index].move_cursor_left();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'j' => {
                    self.buffers[self.current_buf_index].move_cursor_down();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'k' => {
                    self.buffers[self.current_buf_index].move_cursor_up();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'l' => {
                    self.buffers[self.current_buf_index].move_cursor_right();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'w' => {
                    self.buffers[self.current_buf_index].move_to_next_word();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'b' => {
                    self.buffers[self.current_buf_index].move_to_previous_word();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'e' => {
                    self.buffers[self.current_buf_index].move_to_end_of_word();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'i' => {
                    if !vis {
                        self.set_mode(terminal, Mode::Insert);
                    }
                }
                'o' => {
                    if !vis {
                        self.buffers[self.current_buf_index].insert_line_below();
                        self.set_mode(terminal, Mode::Insert);
                    }
                }
                '0' => {
                    self.buffers[self.current_buf_index].move_cursor_start_line();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                '$' => {
                    self.buffers[self.current_buf_index].move_cursor_end_line();
                    if vis {
                        self.update_visual_selection();
                    }
                }
                'y' => {
                    if vis {
                        if let Some(selection) =
                            &self.buffers[self.current_buf_index].selected_string
                        {
                            self.registers
                                .insert(String::from("default"), selection.to_string());
                            self.set_mode(terminal, Mode::Normal);
                            self.buffers[self.current_buf_index].selection = None;
                        }
                    }
                }
                'p' => {
                    if !vis {
                        if let Some(paste_string) = self.registers.get("default") {
                            if !paste_string.is_empty() {
                                self.buffers[self.current_buf_index].paste(paste_string.to_owned());
                            }
                        }
                    }
                }
                _ => {}
            }
        } else if self.mode == Mode::Insert {
            self.buffers[self.current_buf_index].insert_char(ch);
            if let Some(fp) = &self.buffers[self.current_buf_index].file_path {
                _ = self.lsp_client.did_change(
                    fp,
                    &self.buffers[self.current_buf_index].file_text.to_string(),
                );
            }
        }
    }

    fn handle_shifted_key(&mut self, ch: char, terminal: &mut DefaultTerminal) {
        if self.mode == Mode::Normal {
            match ch {
                'I' => {
                    self.buffers[self.current_buf_index].move_cursor_start_line();
                    self.set_mode(terminal, Mode::Insert);
                }
                'A' => {
                    self.buffers[self.current_buf_index].move_cursor_end_line();
                    self.set_mode(terminal, Mode::Insert);
                }
                'K' => {
                    self.hover();
                }
                _ => {}
            }
        } else if self.mode == Mode::Insert && (ch.is_alphanumeric() || ch.is_ascii_punctuation()) {
            self.buffers[self.current_buf_index].insert_char(ch);
        } else if self.mode == Mode::Command {
            if let Some(cmd_str) = &mut self.command {
                cmd_str.push(ch);
            } else {
                self.command = Some(String::from(ch));
            }
        }
    }

    fn handle_backspace(&mut self) {
        if self.mode == Mode::Insert {
            self.buffers[self.current_buf_index].remove_char();
        }
        if self.mode == Mode::Command {
            if let Some(command) = &mut self.command {
                command.pop();
            }
        }
    }

    fn handle_enter(&mut self, terminal: &mut DefaultTerminal) {
        if self.mode == Mode::Insert {
            if let Some(completion_item) = &self.selected_completion {
                let buffer_pos = self.buffers[self.current_buf_index].get_viewport_cursor_pos();
                self.insert_completion(completion_item.clone(), buffer_pos);
                self.selected_completion = None;
                self.completion_list = None;
                self.completion_offset = 0;
                let fp = self.buffers[self.current_buf_index]
                    .file_path
                    .clone()
                    .unwrap_or(String::from(""));
                let contents = self.buffers[self.current_buf_index].file_text.to_string();
                _ = self.lsp_client.did_change(&fp, &contents);
            } else {
                self.buffers[self.current_buf_index].enter_key();
            }
        } else if self.mode == Mode::Command {
            self.apply_command(terminal);
        }
    }

    fn update_visual_selection(&mut self) {
        if let Some(selection) = &self.buffers[self.current_buf_index].selection {
            let start = selection.start.clone();
            let end = self.buffers[self.current_buf_index]
                .current_position
                .clone();
            self.buffers[self.current_buf_index].selection = Some(Selection { start, end });
            self.buffers[self.current_buf_index].update_selected_string();
        }
    }
}
