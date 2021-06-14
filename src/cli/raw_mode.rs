use crate::cli::Result;
use backtrace::Backtrace;

/// Enters raw mode, calls function, makes sure raw mode is disabled before return.
pub fn wrap(f: impl Fn() -> Result<()>) -> Result<()> {
    // Enable raw mode: exit on failure
    enable()?;

    // Capture result from call, making sure to allow exiting raw mode
    let result = f();

    // Always exit raw mode.
    disable()?;

    // Return result
    result
}

/// Disables raw mode.
pub fn disable() -> crossterm::Result<()> {
    crossterm::terminal::disable_raw_mode().map_err(|err| {
        eprintln!("could not disable raw mode");
        eprintln!("err: {}", err);

        let bt = Backtrace::new();
        eprintln!("backtrace:\n{:?}", bt);

        err
    })
}

use std::panic;

/// Enables raw mode.
pub fn enable() -> crossterm::Result<()> {
    panic::set_hook(Box::new(|panic_info| {
        // On panic, try to disable raw mode...
        let result = disable();
        // ... then print panic info...
        eprintln!("{}", panic_info);
        // ... then unwrap possible error from disabling raw mode.
        result.unwrap();
    }));

    crossterm::terminal::enable_raw_mode()
}
