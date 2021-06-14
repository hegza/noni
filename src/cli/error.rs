use thiserror::Error;

/// Represents all errors in the CLI.
#[derive(Error, Debug)]
pub enum Error {
    #[error("crossterm error")]
    Crossterm(#[from] crossterm::ErrorKind),
}
