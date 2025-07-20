use anyhow::Result;
use std::fs::read_to_string;
use std::sync::mpsc::{Receiver, channel};

use crate::app::App;
use crate::events::{EventKind, handle_events};
use crate::ui::ui;

mod app;
mod events;
mod ui;

const NUMBAR_SPACE: u16 = 2;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let file_text = read_to_string("./testfiles/exceptions.py")?;
    let tsize_x = terminal.size()?.width;
    let tsize_y = terminal.size()?.height;
    let (event_sender, event_receiver) = channel::<EventKind>();
    let mut app = App::new(file_text, tsize_x, tsize_y);
    std::thread::spawn(move || handle_events(event_sender));
    let result = run(event_receiver, &mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run(
    event_receiver: Receiver<EventKind>,
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        if let Ok(event) = event_receiver.try_recv() {
            match event {
                EventKind::Quit => app.quitting = true,
                EventKind::NormalMode => app.normal_mode(),
                EventKind::InsertMode => app.insert_mode(),
                EventKind::InsertLineBelow => {
                    app.file_lines.insert(
                        (app.current_pos.line + 1) as usize,
                        (String::from(""), 0_u16),
                    );
                    app.current_pos.line = app.current_pos.line.saturating_add(1);
                    app.current_pos.char = NUMBAR_SPACE;
                    app.insert_mode();
                }
                EventKind::MoveCursorLeft => {
                    app.current_pos.char = {
                        // Never let current cursor position be less than 2, because char
                        // positions 0 and 1 are occupied by line number bar's rendering.
                        if app.current_pos.char == NUMBAR_SPACE {
                            app.current_pos.char
                        } else if app.current_pos.char
                            > app.file_lines[app.current_pos.line as usize].1
                        {
                            app.file_lines[app.current_pos.line as usize].1 + 1
                        } else {
                            app.current_pos.char.saturating_sub(1)
                        }
                    }
                }
                EventKind::MoveCursorDown => {
                    // If current line is bigger than length of lines vector - 1, limit
                    // it to last line available. Must be len(vec) - 1 because lines start at
                    // 0.
                    app.current_pos.line = {
                        if app.current_pos.line >= (app.file_lines.len() - 1) as u16 {
                            (app.file_lines.len() - 1) as u16
                        } else {
                            app.current_pos.line.saturating_add(1)
                        }
                    };

                    // Edge case where when going down, the line is empty line. Then put cursor
                    // right after numbar.
                    if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                        app.current_pos.char = NUMBAR_SPACE;
                    }
                    // If current char after going down would be bigger than the new line's
                    // length, put it on max character of the line.
                    else if app.current_pos.char > app.file_lines[app.current_pos.line as usize].1
                    {
                        app.current_pos.char =
                            app.file_lines[app.current_pos.line as usize].1 + 1_u16;
                    }
                }
                EventKind::MoveCursorUp => {
                    app.current_pos.line = app.current_pos.line.saturating_sub(1);
                    // Edge case where when going up the line is empty line. Then put cursor
                    // after numbar.
                    if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                        app.current_pos.char = NUMBAR_SPACE;
                    }
                    // If current char after going up would be bigger than the new line's
                    // length, put it on max character of the line.
                    else if app.current_pos.char > app.file_lines[app.current_pos.line as usize].1
                    {
                        app.current_pos.char =
                            app.file_lines[app.current_pos.line as usize].1 + 1_u16;
                    }
                }
                EventKind::MoveCursorRight => {
                    app.current_pos.char = {
                        // If current line is a newly added line, default to first editor
                        // character that starts at NUMBAR_SPACE
                        if app.file_lines[app.current_pos.line as usize].1 == 0_u16 {
                            NUMBAR_SPACE
                        } else if app.current_pos.char
                            > app.file_lines[app.current_pos.line as usize].1
                        {
                            // Don't really know why this works but this keeps the cursor at
                            // the end of the line that's editing.
                            app.file_lines[app.current_pos.line as usize].1 + 1
                        } else {
                            // If no constaints are being met, means we can freely add one
                            // position to right.
                            app.current_pos.char.saturating_add(1)
                        }
                    }
                }
            }
        }

        if app.quitting {
            return Ok(());
        }
    }
}
