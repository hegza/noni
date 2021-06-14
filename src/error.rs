use thiserror::Error;

use std::io;

/// Represents all errors in this application.
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
}
