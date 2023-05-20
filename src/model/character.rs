//! Character model
//!
//! This module contains the implementation of the Character model
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a Character
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Character {
    id: Option<i64>,
    name: String,
    image_url: String,
}

impl Character {
    /// Returns a reference to Character's id
    pub fn id(&self) -> &Option<i64> {
        &self.id
    }

    /// Returns a reference to Character's name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to Character's image url
    pub fn image_url(&self) -> &String {
        &self.image_url
    }
}
