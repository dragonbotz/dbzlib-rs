//! Character model
//!
//! This module contains the implementation of the Character model
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a Character
#[derive(Deserialize, Serialize, FromRow)]
pub struct Character {
    id: Option<i64>,
    name: String,
    image_url: String,
}
