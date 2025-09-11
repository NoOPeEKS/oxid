use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::app::App;

pub fn render_completion_table(frame: &mut Frame, app: &App, editor_area: Rect) {
    if let Some(completion_list) = &app.completion_list {
        let max_items = completion_list.len().min(6);
        let max_label_width = completion_list
            .iter()
            .map(|item| item.label.len())
            .max()
            .unwrap_or(10);
        let max_kind_width = completion_list
            .iter()
            // .map(|item| String::from(item.kind).len())
            .map(|_| 8)
            .max()
            .unwrap_or(5);

        let label_column_width = max_label_width + 2;
        let kind_column_width = max_kind_width + 2;
        let total_width = label_column_width + kind_column_width + 2; // +2 for borders

        let cursor_pos = app.buffers[app.current_buf_index].get_viewport_cursor_pos();

        let popup_x = cursor_pos
            .character
            .min(editor_area.width.saturating_sub(total_width as u16).into());
        let popup_y = (cursor_pos.line + 1).min(
            editor_area
                .height
                .saturating_sub(max_items as u16 + 2)
                .into(),
        ); // +2 for borders

        let popup_area = Rect {
            x: editor_area.x + popup_x as u16,
            y: editor_area.y + popup_y as u16,
            width: total_width as u16,
            height: max_items as u16 + 2, // +2 for borders
        };

        let rows: Vec<Row> = completion_list
            .iter()
            .take(6) // Clamp to 6 items
            .map(|item| {
                Row::new(vec![
                    Cell::from(item.label.clone()),
                    // Cell::from(String::from(item.kind)),
                    Cell::from(String::from("function")),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(label_column_width as u16),
                Constraint::Length(kind_column_width as u16),
            ],
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Completions")
                .style(Style::default().bg(Color::Rgb(40, 30, 51))),
        )
        .style(Style::default().fg(Color::Rgb(164, 160, 232)));

        frame.render_widget(table, popup_area);
    }
}
