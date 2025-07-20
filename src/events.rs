use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
use std::{sync::mpsc::Sender, time::Duration};

pub enum EventKind {
    NormalMode,
    InsertMode,
    Quit,
    KeyPressed(char),
}

pub fn handle_events(sender: Sender<EventKind>) -> Result<()> {
    loop {
        if event::poll(Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('e') => sender.send(EventKind::InsertMode)?,
                        KeyCode::Char('c') => sender.send(EventKind::Quit)?,
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => sender.send(EventKind::NormalMode)?,
                        KeyCode::Char(ch) => sender.send(EventKind::KeyPressed(ch))?,
                        _ => {}
                    }
                }
            }
        }
    }
}
