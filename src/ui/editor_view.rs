use super::debug::DebugPopup;
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    prelude::Stylize,
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let terminal_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(app.buffers[0].numbar_space as u16),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    // Get visible lines for the current viewport
    let visible_lines = app.buffers[0].get_visible_lines();

    // Render line numbers for only the visible lines
    let numbar_area = terminal_area[0];
    let nums_of_lines = {
        let mut vec_nums: Vec<String> = Vec::new();
        let start_line = app.buffers[0].vertical_scroll;
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

    // // Render only the visible lines
    // let file_string: Vec<String> = visible_lines
    //     .iter()
    //     .map(|line| app.buffers[0].get_visible_line_content(line))
    //     .collect();
    // let file_text = Paragraph::new(file_string.join("\n"));

    let selection = &app.buffers[0].selection;
    let mut styled_lines: Vec<Line> = Vec::new();
    let start_line = app.buffers[0].vertical_scroll;
    let numbar_space = app.buffers[0].numbar_space;

    for (i, visible_line) in visible_lines.iter().enumerate() {
        let line_content = app.buffers[0].get_visible_line_content(visible_line);
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
                Span::styled(ch.to_string(), Style::default().on_dark_gray())
            } else {
                Span::raw(ch.to_string())
            };
            spans.push(styled_char);
        }

        styled_lines.push(Line::from(spans));
    }

    let file_text = Paragraph::new(styled_lines);

    // Get cursor position relative to the viewport
    let viewport_cursor = app.buffers[0].get_viewport_cursor_pos();
    frame.set_cursor_position(Position {
        x: viewport_cursor.character as u16,
        y: viewport_cursor.line as u16,
    });

    if !app.debug_mode {
        frame.render_widget(file_text, editor_area_chunks[0]);
    } else {
        let editor_subareas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(editor_area_chunks[0]);
        frame.render_widget(file_text, editor_subareas[0]);

        let mut curr_sel = format!("{:?}", app.buffers[0].selection.clone());
        let curr_selected_value = format!("{:?}", app.buffers[0].selected_string.clone());

        curr_sel.push('\n');
        curr_sel.push_str(&curr_selected_value);

        let default_register_contents = app.registers.get("default");
        curr_sel.push('\n');

        curr_sel.push_str(
            default_register_contents.unwrap_or(&"Nothing to default register yet.".to_string()),
        );

        let popup = DebugPopup::default()
            .content(&curr_sel)
            .style(Style::new().yellow())
            .title("Debug selection")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());
        frame.render_widget(popup, editor_subareas[1]);
    }

    // Status bar showing mode, absolute cursor position, and scroll info
    let mode = format!(
        "{:?} Mode :: {}:{}",
        app.mode,
        app.buffers[0].current_position.line,
        app.buffers[0]
            .current_position
            .character
            .saturating_sub(app.buffers[0].numbar_space),
    );
    let text = Text::raw(mode);
    frame.render_widget(text, editor_area_chunks[1]);
}
