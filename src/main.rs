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
    let mut app = App::new(file_text);
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
                    KeyCode::Char('j') => app.cursor_y = app.cursor_y.saturating_add(1),
                    KeyCode::Char('k') => app.cursor_y = app.cursor_y.saturating_sub(1),
                    KeyCode::Char('l') => app.cursor_x = app.cursor_x.saturating_add(1),
                    _ => {}
                }
            }
        }
        if app.quitting {
            return Ok(());
        }
    }
}
