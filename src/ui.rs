use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    text::Text,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let window_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    // Iterate over the Vec<(linestring, length)> and just get the linestring out 
    let file_string: Vec<String> = app.file_lines.iter().map(|s| s.0.to_string()).collect();

    let file_text = Text::raw(file_string.join("\n"));
    frame.set_cursor_position(Position {
        x: app.current_pos.char,
        y: app.current_pos.line,
    });
    frame.render_widget(file_text, window_area_chunks[0]);

    let mode = format!("{:?}", app.mode);
    let text = Text::raw(mode);
    frame.render_widget(text, window_area_chunks[1]);
}
