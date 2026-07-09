use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
};

use std::time::Duration;

pub enum KeyAction {
    None,
    Quit,
    Search,
    Character(char),
    Backspace,
    Enter,
    Esc,
    Up,
    Down,
}

pub fn read_key() -> KeyAction {
    if event::poll(Duration::from_millis(10)).unwrap() {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('q') => KeyAction::Quit,

                KeyCode::Char('/') => KeyAction::Search,

                KeyCode::Char(c) => KeyAction::Character(c),

                KeyCode::Backspace => KeyAction::Backspace,

                KeyCode::Enter => KeyAction::Enter,

                KeyCode::Esc => KeyAction::Esc,

                KeyCode::Up => KeyAction::Up,

                KeyCode::Down => KeyAction::Down,

                _ => KeyAction::None,
            }
        } else {
            KeyAction::None
        }
    } else {
        KeyAction::None
    }
}
