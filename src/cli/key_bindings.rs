use std::fmt;

use crate::cli::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub struct KeyBindings(Vec<Binding>);

#[macro_export]
macro_rules! bind_key {
    // The final parameter can be used to communicate constraints like current editor mode.
    ($key:expr, $action:expr, $constrain_fn:expr) => {
        //bind!(KeyCode::Char($key), KeyModifiers::NONE, $cmd, $filter)
        Binding {
            code: KeyCode::Char($key),
            action: $action,
            constrain: Box::new($constrain_fn),
        }
    };
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings(vec![bind_key!('q', Action::Exit, |_p| { true })])
    }
}

impl KeyBindings {
    pub fn key_event_to_action(&self, event: KeyEvent) -> Action {
        for binding in self.matches(event) {
            if binding.is_allowed(&ConstrainParams {}) {
                return binding.action.clone();
            }
        }

        Action::NoOp
    }

    fn matches(&self, event: KeyEvent) -> Vec<&Binding> {
        let KeyEvent { code, modifiers } = event;

        let mut ret = Vec::with_capacity(1);
        for binding in &self.0 {
            // TODO: modifiers not implemented
            if binding.code == code {
                ret.push(binding);
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct Binding {
    code: KeyCode,
    action: Action,
    constrain: Box<ConstrainFn>,
}

impl Binding {
    /// Checks whether this binding is active based on the constraints
    fn is_allowed(&self, p: &ConstrainParams) -> bool {
        (self.constrain)(p)
    }
}

#[derive(Debug)]
pub struct ConstrainParams;
pub trait ConstrainFnTrait: Fn(&ConstrainParams) -> bool {}
impl<F> ConstrainFnTrait for F where F: Fn(&ConstrainParams) -> bool {}

pub type ConstrainFn = dyn ConstrainFnTrait;
impl fmt::Debug for ConstrainFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "constraint fn")
    }
}
