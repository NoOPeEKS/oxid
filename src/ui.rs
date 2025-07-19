use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Text,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let window_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(frame.area());

    let file_text = Text::raw(app.file.clone());
    frame.render_widget(file_text, window_area_chunks[0]);

    let mode = format!("{:?}", app.mode);
    let text = Text::raw(mode);
    frame.render_widget(text, window_area_chunks[1]);
}
