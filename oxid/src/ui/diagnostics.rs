use oxid_lsp::types::DiagnosticSeverity;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;

pub fn render_diagnostics(frame: &mut Frame, app: &App, editor_area: Rect) {
    if app.show_diagnostics {
        if let Some(diagnostics_vec) = &app.diagnostics {
            for diag in diagnostics_vec {
                let cur_pos = app.buffers[app.current_buf_index].get_viewport_cursor_pos();
                if diag.range.is_inside(cur_pos.line, cur_pos.character) {
                    let text_height = diag.message.lines().count() + 2 + 1; // +2 for borders + 1 for
                    // error severity
                    let text_width = diag.message.lines().map(|l| l.len()).max().unwrap_or(25) + 2; // +2 for borders
                    let popup_x = cur_pos
                        .character
                        .min(editor_area.width.saturating_sub(text_width as u16).into());
                    let popup_y = cur_pos
                        .line
                        .saturating_sub(1)
                        .min(editor_area.height.saturating_sub(text_height as u16).into());
                    let popup_area = Rect {
                        x: editor_area.x + popup_x as u16,
                        y: editor_area.y + popup_y as u16,
                        width: text_width as u16,
                        height: text_height as u16,
                    };
                    let mut pg_text: Vec<Line> = diag.message.lines().map(Line::from).collect();
                    pg_text.insert(
                        0,
                        Line::from(format!(
                            "[{:?}]",
                            diag.severity
                                .clone()
                                .unwrap_or(DiagnosticSeverity::Information)
                        )),
                    );
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Rgb(40, 30, 51)))
                        .border_style(Color::Rgb(164, 160, 232));
                    // TODO: Print message in different colors based on err severity.
                    let paragraph = Paragraph::new(pg_text)
                        .block(block)
                        .style(Color::Rgb(164, 160, 232));
                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(paragraph, popup_area);
                }
            }
        }
    }
}
