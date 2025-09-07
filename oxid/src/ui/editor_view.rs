use super::debug::DebugPopup;
use crate::app::{App, Mode};
use crate::buffer::STATUSBAR_SPACE;
use crate::ui::command::CommandPopup;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Position},
    prelude::Stylize,
    style::{Color, Style, palette::tailwind::PURPLE},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let background = Block::default().style(Style::default().bg(Color::Rgb(59, 34, 76)));
    frame.render_widget(background, frame.area());

    let terminal_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(STATUSBAR_SPACE as u16),
        ])
        .split(frame.area());

    let top_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(app.buffers[app.current_buf_index].numbar_space as u16),
            Constraint::Fill(1),
        ])
        .split(terminal_area[0]);

    // Get visible lines for the current viewport
    let visible_lines = app.buffers[app.current_buf_index].get_visible_lines();

    // Render line numbers for only the visible lines
    let numbar_area = top_area[0];
    let nums_of_lines = {
        let mut vec_nums: Vec<String> = Vec::new();
        let start_line = app.buffers[app.current_buf_index].vertical_scroll;
        for (i, _) in visible_lines.iter().enumerate() {
            vec_nums.push((start_line + i).to_string())
        }
        vec_nums
    };
    let numbar_text = Paragraph::new(nums_of_lines.join("\n")).style(Color::Rgb(164, 160, 232));
    frame.render_widget(numbar_text, numbar_area);

    let editor_area = top_area[1];
    let editor_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(STATUSBAR_SPACE as u16),
        ])
        .split(editor_area);

    let selection = &app.buffers[app.current_buf_index].selection;
    let mut styled_lines: Vec<Line> = Vec::new();
    let start_line = app.buffers[app.current_buf_index].vertical_scroll;
    let numbar_space = app.buffers[app.current_buf_index].numbar_space;

    for (i, visible_line) in visible_lines.iter().enumerate() {
        let line_content =
            app.buffers[app.current_buf_index].get_visible_line_content(visible_line.to_owned());
        let mut spans: Vec<Span> = Vec::new();

        for (char_idx, ch) in line_content.chars().enumerate() {
            let abs_line = start_line + i;
            // If selection, check if char is inside of it, if no selection, just pass.
            let in_selection = if let Some(sel) = selection {
                // Normalize selection, even if it went backwards, so that it's always start < end.
                // Always take into account that for rendering, we need to sub numbar_space to
                // compare in terms of line length!!
                let (start, end) = if sel.start.line < sel.end.line
                    || (sel.start.line == sel.end.line
                        && sel.start.character - numbar_space <= sel.end.character - numbar_space)
                {
                    (&sel.start, &sel.end)
                } else {
                    (&sel.end, &sel.start)
                };

                // Is in selection if line is bigger than start line or same line but char bigger
                // than start char, and if line is less than or equal to line end and character is
                // less than the end character.
                (abs_line > start.line
                    || (abs_line == start.line && char_idx >= start.character - numbar_space))
                    && (abs_line < end.line
                        || (abs_line == end.line && char_idx < end.character - numbar_space))
            } else {
                false
            };

            let styled_char = if in_selection {
                Span::styled(ch.to_string(), Style::default().bg(PURPLE.c900))
            } else {
                Span::raw(ch.to_string())
            };
            spans.push(styled_char);
        }

        styled_lines.push(Line::from(spans));
    }

    let file_text = Paragraph::new(styled_lines).style(Color::Rgb(164, 160, 232));

    // Get cursor position relative to the viewport
    let viewport_cursor = app.buffers[app.current_buf_index].get_viewport_cursor_pos();
    frame.set_cursor_position(Position {
        x: viewport_cursor.character as u16,
        y: viewport_cursor.line as u16,
    });

    // Handle different modes
    if app.mode == Mode::Command {
        // Render editor content first
        frame.render_widget(file_text, editor_area_chunks[0]);

        // Then render command popup on top
        let editor_subareas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(6),
                Constraint::Percentage(6),
                Constraint::Percentage(6),
                Constraint::Fill(1),
            ])
            .split(editor_area_chunks[0]);
        let main_area = editor_subareas[1];
        let popup_subareas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Fill(1),
            ])
            .split(main_area);

        let command_popup = CommandPopup::default()
            .content(app.command.as_deref().unwrap_or(""))
            .style(Color::Rgb(164, 160, 232).into())
            .title("Command")
            .title_style(Style::new().white().bold())
            .border_style(Color::Black.into());
        frame.render_widget(command_popup, popup_subareas[1]);
    } else if app.debug_mode {
        // Debug mode rendering
        let editor_subareas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(editor_area_chunks[0]);
        frame.render_widget(file_text, editor_subareas[0]);

        let mode = app.mode.to_string();
        let command = app.command.clone();
        let dbg_str = format!("MODE: {mode}\n CURRENT COMMAND: {command:#?}");

        let popup = DebugPopup::default()
            .content(&dbg_str)
            .style(Style::new().yellow())
            .title("Debug selection")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());
        frame.render_widget(popup, editor_subareas[1]);
    } else {
        // Normal mode rendering
        frame.render_widget(file_text, editor_area_chunks[0]);
    }

    let status_bar_area_bg = Block::default().style(Style::default().bg(Color::Rgb(40, 30, 51)));

    // Status bar showing mode, file, cursor position
    let mode = format!(
        "  {} Mode :: {}",
        app.mode,
        app.buffers[app.current_buf_index]
            .file_path
            .clone()
            .unwrap_or("New File".to_string())
    );
    let cursor_pos = format!(
        "{}:{}",
        app.buffers[app.current_buf_index].current_position.line,
        app.buffers[app.current_buf_index]
            .current_position
            .character
            .saturating_sub(app.buffers[app.current_buf_index].numbar_space),
    );
    let area_width = editor_area_chunks[1].width as usize;
    let mode_width = mode.chars().count();
    let position_width = cursor_pos.chars().count();
    let spacer_width = area_width.saturating_sub(mode_width + position_width);

    let text = Line::from(vec![
        Span::raw(mode),
        Span::raw(" ".repeat(spacer_width)),
        Span::raw(cursor_pos),
    ]);
    let sb_paragraph = Paragraph::new(text)
        .style(Style::default())
        .block(status_bar_area_bg)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(sb_paragraph, terminal_area[1]);
}
