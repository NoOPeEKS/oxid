use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::fs::read_to_string;

use crate::app::{App, Mode};
use crate::ui::ui;

mod app;
mod ui;

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
                    KeyCode::Char('h') => app.cursor_x = app.cursor_x.saturating_sub(1),
                    KeyCode::Char('j') => {
                        // If you go down and try to go deeper down than allowed, just set to
                        // terminal limit.
                        app.cursor_y = {
                            if app.cursor_y >= app.size_y {
                                app.size_y
                            } else {
                                app.cursor_y.saturating_add(1)
                            }
                        }
                    }
                    KeyCode::Char('k') => app.cursor_y = app.cursor_y.saturating_sub(1),
                    KeyCode::Char('l') => {
                        // If you go right and try to go farther than allowed, just set to
                        // terminal limit.
                        app.cursor_x = {
                            if app.cursor_x >= app.size_x {
                                app.size_x
                            } else {
                                app.cursor_x.saturating_add(1)
                            }
                        }
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
