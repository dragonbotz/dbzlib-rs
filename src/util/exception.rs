//! Exception module
//!
//! In Dragon Bot Z, an Exception is a recoverable error, you should not panic
//! after catching an Exception, but instead, you may catch it and go on.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use core::fmt;

/// Recoverable errors
pub enum Exception {
    InsertNewCharacter(String),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self {
            Exception::InsertNewCharacter(error) => format!("[Exception][Insert New Character] An error occured while trying to add a new character to the database: {error}"),
        };

        write!(f, "{}", content)
    }
}
