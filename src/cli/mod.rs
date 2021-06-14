mod action;
mod error;
mod key_bindings;
mod raw_mode;

pub use action::Action;
pub use error::Error;
pub use key_bindings::KeyBindings;

use std::result;

pub type Result<T> = result::Result<T, Error>;

pub fn run() -> Result<()> {
    raw_mode::wrap(|| {
        let mut cli = Cli::default();

        cli.block()
    })
}

/// Call [`Self::block`] to enter event loop.
pub struct Cli {
    key_bindings: KeyBindings,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            key_bindings: KeyBindings::default(),
        }
    }
}

enum ContinueStatus {
    Stop,
    Continue,
}

impl Cli {
    /// Main entry point to CLI.
    fn block(&mut self) -> Result<()> {
        loop {
            // Update user's view
            self.display()?;

            // Block on input, then mutate the CLI
            match self.block_on_input()? {
                ContinueStatus::Stop => return Ok(()),
                ContinueStatus::Continue => {}
            }
        }
    }

    /// Updates the user's view
    fn display(&mut self) -> Result<()> {
        Ok(())
    }

    fn block_on_input(&mut self) -> Result<ContinueStatus> {
        // Loop over crossterm::event::read() until a key event is converted into something actionable
        loop {
            // This blocks until crossterm returns an input event
            let event = crossterm::event::read()?;

            // Convert key input into a CLI action; other kinds of events are discarded
            if let crossterm::event::Event::Key(key_ev) = event {
                let action = Action::from_key_event(key_ev, &self.key_bindings);

                // Mutate CLI based on action
                self.act(&action)?;

                return Ok(if action.exit() {
                    ContinueStatus::Stop
                } else {
                    ContinueStatus::Continue
                });
            }
        }
    }

    /// Take action based on user input
    fn act(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::Exit | Action::NoOp => {}
        }

        Ok(())
    }
}
