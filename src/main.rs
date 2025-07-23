use anyhow::Result;
use std::fs::read_to_string;
use std::sync::mpsc::{Receiver, channel};

use crate::app::{App, Mode};
use crate::events::{EventKind, handle_events};
use crate::ui::ui;

mod app;
mod cli;
mod events;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let file_path = cli::get_file_name_arg()?;
    let file_text = read_to_string(file_path)?;
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
        if let Ok(event) = event_receiver.recv() {
            match event {
                EventKind::Quit => app.quitting = true,
                EventKind::NormalMode => app.normal_mode(),
                EventKind::InsertMode => app.insert_mode(),
                EventKind::ScrollUp => app.scroll_up(5),
                EventKind::ScrollDown => app.scroll_down(5),
                EventKind::KeyPressed(ch) => {
                    if app.mode == Mode::Normal {
                        match ch {
                            'h' => app.move_cursor_left(),
                            'j' => app.move_cursor_down(),
                            'k' => app.move_cursor_up(),
                            'l' => app.move_cursor_right(),
                            'o' => app.insert_line_below(),
                            _ => {}
                        }
                    } else if app.mode == Mode::Insert {
                        app.insert_char(ch);
                    }
                }
                EventKind::Backspace => {
                    if app.mode == Mode::Insert {
                        app.remove_char();
                    }
                }
            }
        }

        if app.quitting {
            return Ok(());
        }
    }
}
