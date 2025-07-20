use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    text::Text,
    widgets::Paragraph,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let terminal_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(2), Constraint::Fill(1)])
        .split(frame.area());

    // Render line numbers at the side of each line
    let numbar_area = terminal_area[0];
    let nums_of_lines = {
        let mut vec_nums: Vec<String> = Vec::new();
        for (i, _) in app.file_lines.iter().enumerate() {
            vec_nums.push(i.to_string())
        }
        vec_nums
    };
    let numbar_text = Paragraph::new(nums_of_lines.join("\n"));
    frame.render_widget(numbar_text, numbar_area);

    let editor_area = terminal_area[1];

    let editor_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(editor_area);

    // Iterate over the Vec<(linestring, length)> and just get the linestring out
    let file_string: Vec<String> = app.file_lines.iter().map(|s| s.0.to_string()).collect();

    let file_text = Paragraph::new(file_string.join("\n"));

    frame.set_cursor_position(Position {
        x: app.current_pos.char,
        y: app.current_pos.line,
    });

    frame.render_widget(file_text, editor_area_chunks[0]);

    let mode = format!(
        "{:?} Mode :: {}:{}",
        app.mode, app.current_pos.line, app.current_pos.char - 2
    );
    let text = Text::raw(mode);
    frame.render_widget(text, editor_area_chunks[1]);
}
