use crossterm::{InputEvent, KeyEvent, MouseButton, MouseEvent};

/// Accepts a crossterm InputEvent and spits out a an Event
pub fn process_input_event(key_event: InputEvent) -> Event {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => Event::Keyboard(KeyboardAction::Type(c)),
                KeyEvent::Esc => {
                    Event::Keyboard(KeyboardAction::Close)
                },
                KeyEvent::Alt(c) => {
                    Event::Keyboard(KeyboardAction::AltModifier(c))
                },
                KeyEvent::Ctrl(c) => {
                    Event::Keyboard(KeyboardAction::CtrlModifier(c))
                },
                KeyEvent::Delete => {
                    Event::Keyboard(KeyboardAction::Delete)
                },
                _ => {
                    Event::Unknown
                }
            }
        },
        InputEvent::Mouse(m) => match m {
            MouseEvent::Press(b, x, y) => match b {
                MouseButton::Left => {
                    Event::Mouse(MouseAction::Somthing)
                }
                MouseButton::Right => {
                    Event::Mouse(MouseAction::Somthing)
                }
                MouseButton::Middle => {
                    Event::Mouse(MouseAction::Somthing)
                }
                MouseButton::WheelUp => {
                    Event::Mouse(MouseAction::Somthing)
                },
                MouseButton::WheelDown => {
                    Event::Mouse(MouseAction::Somthing)
                },
                _ => {
                    Event::Unknown
                }
            },
            MouseEvent::Release(x, y) => {
                Event::Mouse(MouseAction::Somthing)
            },
            MouseEvent::Hold(x, y) => {
                Event::Mouse(MouseAction::Somthing)
            },
            _ => {
                Event::Unknown
            }
        },
        _ => Event::Unknown,
    }
}

pub enum Event {
    Keyboard(KeyboardAction),
    Mouse(MouseAction),
    Unknown
}

pub enum KeyboardAction {
    Close,
    Type(char),
    AltModifier(char),
    CtrlModifier(char),
    Delete
}

pub enum MouseAction {
    Somthing
}
