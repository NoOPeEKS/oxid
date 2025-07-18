use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw)?;

        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello, World!");
    frame.render_widget(text, frame.area());
}
