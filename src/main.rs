use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, text::Text};

use crate::app::App;

mod app;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('m') => app.toggle_mode(),
                KeyCode::Esc => app.quitting = true,
                _ => {}
            }
        }
        if app.quitting {
            return Ok(());
        }
    }
}

fn draw(frame: &mut Frame, app: &App) {
    let mode = format!("{:?}", app.mode);
    let text = Text::raw(mode);
    frame.render_widget(text, frame.area());
}
