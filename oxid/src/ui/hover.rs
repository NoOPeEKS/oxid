use oxid_lsp::types::MarkupKind;
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render_hover_area(frame: &mut Frame, app: &App, editor_area: Rect) {
    if let Some(hover) = &app.hover {
        match hover.contents.kind {
            MarkupKind::PlainText => {
                let text_width = hover
                    .contents
                    .value
                    .lines()
                    .map(|line| line.len())
                    .max()
                    .unwrap_or(25)
                    + 2; // +2 for borders.
                let text_height = hover.contents.value.lines().count() + 2; // +2 for borders
                let cursor_pos = app.buffers[app.current_buf_index].get_viewport_cursor_pos();
                let popup_x = cursor_pos
                    .character
                    .min(editor_area.width.saturating_sub(text_width as u16).into());
                let popup_y = cursor_pos
                    .line
                    .saturating_sub(1)
                    .min(editor_area.height.saturating_sub(text_height as u16).into());
                let popup_area = Rect {
                    x: editor_area.x + popup_x as u16,
                    y: editor_area.y + popup_y as u16,
                    width: text_width as u16,
                    height: text_height as u16,
                };

                let pg_text: Vec<Line> = hover.contents.value.lines().map(Line::from).collect();
                let block = Block::default().borders(Borders::ALL);
                let paragraph = Paragraph::new(pg_text).block(block);
                frame.render_widget(paragraph, popup_area);
            }
            MarkupKind::Markdown => {
                // TODO: Treat markdown differently, parse it, represent it with different
                // fontsizes, etc...
                let text_width = hover
                    .contents
                    .value
                    .lines()
                    .map(|line| line.len())
                    .max()
                    .unwrap_or(25)
                    + 2; // +2 for borders.
                let text_height = hover.contents.value.lines().count() + 2; // +2 for borders
                let cursor_pos = app.buffers[app.current_buf_index].get_viewport_cursor_pos();
                let popup_x = cursor_pos
                    .character
                    .min(editor_area.width.saturating_sub(text_width as u16).into());
                let popup_y = cursor_pos
                    .line
                    .saturating_sub(1)
                    .min(editor_area.height.saturating_sub(text_height as u16).into());
                let popup_area = Rect {
                    x: editor_area.x + popup_x as u16,
                    y: editor_area.y + popup_y as u16,
                    width: text_width as u16,
                    height: text_height as u16,
                };

                let pg_text: Vec<Line> = hover.contents.value.lines().map(Line::from).collect();
                let block = Block::default().borders(Borders::ALL);
                let paragraph = Paragraph::new(pg_text).block(block);
                frame.render_widget(paragraph, popup_area);
            }
        }
    }
}
