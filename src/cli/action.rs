use crate::cli::KeyBindings;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Exit,
    NoOp,
}

use crossterm::event::KeyEvent;

impl Action {
    /// Uses key bindings to determine what action the key event should become
    pub fn from_key_event(event: KeyEvent, bindings: &KeyBindings) -> Self {
        bindings.key_event_to_action(event)
    }

    pub fn exit(&self) -> bool {
        self == &Action::Exit
    }
}
