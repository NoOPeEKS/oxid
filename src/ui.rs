use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    text::Text,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let window_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(frame.area());

    let file_text = Text::raw(app.file.clone());
    frame.set_cursor_position(Position {
        x: app.cursor_x,
        y: app.cursor_y,
    });
    frame.render_widget(file_text, window_area_chunks[0]);

    let mode = format!("{:?}", app.mode);
    let text = Text::raw(mode);
    frame.render_widget(text, window_area_chunks[1]);
}
