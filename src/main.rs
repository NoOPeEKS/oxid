use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::fs::read_to_string;

use crate::app::{App, Mode};
use crate::ui::ui;

mod app;
mod ui;

const NUMBAR_SPACE: u16 = 2;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let file_text = read_to_string("./testfiles/exceptions.py")?;
    let tsize_x = terminal.size()?.width;
    let tsize_y = terminal.size()?.height;
    let mut app = App::new(file_text, tsize_x, tsize_y);
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc && app.mode == Mode::Insert {
                app.normal_mode();
            } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('e') => {
                        app.insert_mode();
                    }
                    KeyCode::Char('c') => app.quitting = true,
                    _ => {}
                }
            } else if app.mode == Mode::Normal {
                match key.code {
                    KeyCode::Char('h') => {
                        app.current_pos.char = {
                            // Never let current cursor position be less than 2, because char
                            // positions 0 and 1 are occupied by line number bar's rendering.
                            if app.current_pos.char == NUMBAR_SPACE {
                                app.current_pos.char
                            } else if app.current_pos.char
                                > app.file_lines[app.current_pos.line as usize].1
                            {
                                app.file_lines[app.current_pos.line as usize].1 + 1
                            } else {
                                app.current_pos.char.saturating_sub(1)
                            }
                        }
                    }

                    KeyCode::Char('j') => {
                        // If current line is bigger than length of lines vector - 1, limit
                        // it to last line available. Must be len(vec) - 1 because lines start at
                        // 0.
                        app.current_pos.line = {
                            if app.current_pos.line >= (app.file_lines.len() - 1) as u16 {
                                (app.file_lines.len() - 1) as u16
                            } else {
                                app.current_pos.line.saturating_add(1)
                            }
                        };

                        // Edge case where when going down, the line is empty line. Then put cursor
                        // right after numbar.
                        if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                            app.current_pos.char = NUMBAR_SPACE;
                        }
                        // If current char after going down would be bigger than the new line's
                        // length, put it on max character of the line.
                        else if app.current_pos.char
                            > app.file_lines[app.current_pos.line as usize].1
                        {
                            app.current_pos.char =
                                app.file_lines[app.current_pos.line as usize].1 + 1_u16;
                        }
                    }
                    KeyCode::Char('k') => {
                        app.current_pos.line = app.current_pos.line.saturating_sub(1);
                        // Edge case where when going up the line is empty line. Then put cursor
                        // after numbar.
                        if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                            app.current_pos.char = NUMBAR_SPACE;
                        }
                        // If current char after going up would be bigger than the new line's
                        // length, put it on max character of the line.
                        else if app.current_pos.char
                            > app.file_lines[app.current_pos.line as usize].1
                        {
                            app.current_pos.char =
                                app.file_lines[app.current_pos.line as usize].1 + 1_u16;
                        }
                    }
                    KeyCode::Char('l') => {
                        app.current_pos.char = {
                            // If current line is a newly added line, default to first editor
                            // character that starts at NUMBAR_SPACE
                            if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                                NUMBAR_SPACE
                            } else if app.current_pos.char
                                > app.file_lines[app.current_pos.line as usize].1
                            {
                                // Don't really know why this works but this keeps the cursor at
                                // the end of the line that's editing.
                                app.file_lines[app.current_pos.line as usize].1 + 1
                            } else {
                                // If no constaints are being met, means we can freely add one
                                // position to right.
                                app.current_pos.char.saturating_add(1)
                            }
                        }
                    }
                    // Create new line under the cursor and move to it
                    KeyCode::Char('o') => {
                        app.file_lines.insert(
                            (app.current_pos.line + 1) as usize,
                            (String::from(""), 0_u16),
                        );
                        // app.file_lines.push((String::new(), 0_u16));
                        app.current_pos.line = app.current_pos.line.saturating_add(1);
                        app.current_pos.char = NUMBAR_SPACE;
                        app.insert_mode();
                    }
                    _ => {}
                }
            }
        }
        if app.quitting {
            return Ok(());
        }
    }
}
