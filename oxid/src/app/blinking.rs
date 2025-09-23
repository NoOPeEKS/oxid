use ratatui::backend::{CrosstermBackend, TestBackend};
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::execute;
use std::io::Stdout;

/// Backend support for setting the cursor style.
pub trait CursorStyleSupport {
    fn set_cursor_style(&mut self, style: SetCursorStyle);
}

impl CursorStyleSupport for CrosstermBackend<Stdout> {
    fn set_cursor_style(&mut self, style: SetCursorStyle) {
        execute!(std::io::stdout(), style).unwrap_or_default();
    }
}

impl CursorStyleSupport for TestBackend {
    fn set_cursor_style(&mut self, _style: SetCursorStyle) {
        // no-op because test backend does not implement std::io::Write.
    }
}
