use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;

pub enum EventKind {
    NormalMode,
    Quit,
    KeyPressed(char),
    Backspace,
    ScrollUp,
    ScrollDown,
    SaveFile,
    EnterKey,
}

pub fn handle_events(sender: Sender<EventKind>) -> Result<()> {
    loop {
        if let event::Event::Key(key) = event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => sender.send(EventKind::Quit)?,
                    KeyCode::Char('u') => sender.send(EventKind::ScrollUp)?,
                    KeyCode::Char('d') => sender.send(EventKind::ScrollDown)?,
                    KeyCode::Char('s') => sender.send(EventKind::SaveFile)?,
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc => sender.send(EventKind::NormalMode)?,
                    KeyCode::Char(ch) => sender.send(EventKind::KeyPressed(ch))?,
                    KeyCode::Backspace => sender.send(EventKind::Backspace)?,
                    KeyCode::Enter => sender.send(EventKind::EnterKey)?,
                    _ => {}
                }
            }
        }
    }
}
