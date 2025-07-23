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
        .constraints([Constraint::Length(app.numbar_space), Constraint::Fill(1)])
        .split(frame.area());

    // Get visible lines for the current viewport
    let visible_lines = app.get_visible_lines();

    // Render line numbers for only the visible lines
    let numbar_area = terminal_area[0];
    let nums_of_lines = {
        let mut vec_nums: Vec<String> = Vec::new();
        let start_line = app.scroll_offset as usize;
        for (i, _) in visible_lines.iter().enumerate() {
            vec_nums.push((start_line + i).to_string())
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

    // Render only the visible lines
    let file_string: Vec<String> = visible_lines
        .iter()
        .map(|line| app.get_visible_line_content(line))
        .collect();
    let file_text = Paragraph::new(file_string.join("\n"));

    // Get cursor position relative to the viewport
    let viewport_cursor = app.get_viewport_cursor_pos();
    frame.set_cursor_position(Position {
        x: viewport_cursor.char,
        y: viewport_cursor.line,
    });

    frame.render_widget(file_text, editor_area_chunks[0]);

    // Status bar showing mode, absolute cursor position, and scroll info
    let mode = format!(
        "{:?} Mode :: {}:{}",
        app.mode,
        app.current_pos.line,
        app.current_pos.char.saturating_sub(app.numbar_space),
    );
    let text = Text::raw(mode);
    frame.render_widget(text, editor_area_chunks[1]);
}
