use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
};

use std::time::Duration;

pub enum KeyAction {
    None,

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
		    KeyCode::Up => KeyAction::Up,

		    KeyCode::Down => KeyAction::Down,

		    KeyCode::Backspace => KeyAction::Backspace,

		    KeyCode::Enter => KeyAction::Enter,

		    KeyCode::Esc => KeyAction::Esc,

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
