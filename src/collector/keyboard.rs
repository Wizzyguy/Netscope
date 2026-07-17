use crossterm::event::{self, Event, KeyCode, KeyEvent};

use std::time::Duration;

pub enum KeyAction {
    None,

    Character(char),

    Backspace,

    Enter,

    Esc,

    Up,

    Down,

    Left,

    Right,

    Reset,
}

pub fn read_key() -> KeyAction {
    if event::poll(Duration::from_millis(10)).unwrap() {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Up => KeyAction::Up,

                KeyCode::Down => KeyAction::Down,

                KeyCode::Left => KeyAction::Left,

                KeyCode::Right => KeyAction::Right,

                KeyCode::Backspace => KeyAction::Backspace,

                KeyCode::Enter => KeyAction::Enter,

                KeyCode::Esc => KeyAction::Esc,

                KeyCode::Char('r') | KeyCode::Char('R') => KeyAction::Reset,

                KeyCode::Char(c) => KeyAction::Character(c),

                _ => KeyAction::None,
            }
        } else {
            KeyAction::None
        }
    } else {
        KeyAction::None
    }
}
