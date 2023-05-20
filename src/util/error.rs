//! Error handling module
//!
//! In Dragon Bot Z, an Error is a non-recoverable error, you should panic.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use core::fmt;

// Custom Result type containing an error
pub type ErrResult<T> = std::result::Result<T, Error>;

/// Not recoverable errors
#[derive(Debug)]
pub enum Error {
    // Error thrown when the program is unable to connect to a Database
    DatabaseConnection(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // content to write
        let content = match &self {
            Error::DatabaseConnection(error) => format!("[Error][Database Connection] An error occured while trying to connect to Database: {error}"),
        };

        write!(f, "{}", content)
    }
}
