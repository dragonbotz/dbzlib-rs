//! Exception module
//!
//! In Dragon Bot Z, an Exception is a recoverable error, you should not panic
//! after catching an Exception, but instead, you may catch it and go on.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use core::fmt;

// Custom Result type containing an exception
pub type ExcResult<T> = std::result::Result<T, Exception>;

/// Recoverable errors
#[derive(Debug)]
pub enum Exception {
    // character service related exceptions
    InsertNewCharacter(String),
    RetrieveCharacter(String),
    RetrieveMultipleCharacters(String),

    // portal service related exceptions
    RetrievePortal(String),
    RetrievePortalContent(String),

    // summon service related exceptions
    DrawCharacter(String),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self {
            Exception::InsertNewCharacter(error) => format!("[Exception][Insert New Character] An error occured while trying to add a new character to the database: {error}"),
            Exception::RetrieveCharacter(error) => format!("[Exception][Retrieve Character] An error occured while trying to retrieve a character from the database: {error}"),
            Exception::RetrieveMultipleCharacters(error) => format!("[Exception][Retrieve Multiple Characters] An error occured while trying to retrieve multiple characters from database: {error}"),

            Exception::RetrievePortal(error) => format!("[Exception][Retrieve Portal] An error occured while trying to retrieve a portal from the database: {error}"),
            Exception::RetrievePortalContent(error) => format!("[Exception][Retrieve Portal Content] An error occured while trying to retrieve a portal's content: {error}"),
            
            Exception::DrawCharacter(error) => format!("[Exception][Draw Character] An error occured while trying to draw a character: {error}"),
        };

        write!(f, "{}", content)
    }
}
