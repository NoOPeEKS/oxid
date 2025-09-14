use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style, palette::tailwind::VIOLET},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::app::App;

pub fn render_completion_table(frame: &mut Frame, app: &App, editor_area: Rect) {
    if let Some(completion_list) = &app.completion_list {
        let max_items = 6; // fixed number of rows in popup
        let total_items = completion_list.items.len();
        if total_items == 0 {
            return;
        }

        let max_label_width = completion_list
            .items
            .iter()
            .map(|item| item.label.len())
            .max()
            .unwrap_or(10);
        let max_kind_width = completion_list
            .items
            .iter()
            .map(|item| {
                let kind = {
                    if let Some(knd) = &item.kind {
                        String::from(knd.clone())
                    } else {
                        String::from("unknown")
                    }
                };
                kind.len()
            })
            .max()
            .unwrap_or(5);

        let label_column_width = max_label_width + 2;
        let kind_column_width = max_kind_width + 2;
        let total_width = label_column_width + kind_column_width + 2; // +2 for borders

        // Position popup under cursor
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
            height: (max_items.min(total_items) as u16) + 2, // +2 for borders
        };

        // Slice only visible rows
        let start = app.completion_offset;
        let end = (start + max_items).min(total_items);

        let rows: Vec<Row> = completion_list.items[start..end]
            .iter()
            .map(|item| {
                let kind = {
                    if let Some(knd) = &item.kind {
                        String::from(knd.clone())
                    } else {
                        String::from("unknown")
                    }
                };
                Row::new(vec![
                    Cell::from(item.label.clone()),
                    Cell::from(kind),
                    Cell::from("function"),
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
        .style(Style::default().fg(Color::Rgb(164, 160, 232)))
        .row_highlight_style(Style::default().bg(VIOLET.c700));

        // Adjust selected index relative to offset
        let mut state = app.table_state.clone();
        if let Some(selected) = app.table_state.selected() {
            if selected >= start && selected < end {
                state.select(Some(selected - start));
            } else {
                state.select(None);
            }
        }

        // Render with stateful widget (for highlight)
        frame.render_stateful_widget(table, popup_area, &mut state);
    }
}
