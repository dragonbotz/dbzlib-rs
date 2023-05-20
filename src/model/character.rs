//! Character model
//!
//! This module contains the implementation of the Character model
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a Character
#[derive(Debug, Default, Deserialize, Serialize, FromRow, Clone)]
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

pub struct CharacterBuilder {
    character: Character,
}

impl CharacterBuilder {
    /// Creates an instance of CharacterBuilder
    pub fn new() -> Self {
        Self {
            character: Character::default(),
        }
    }

    /// Build the Character instance and returns it
    pub fn build(&self) -> Character {
        self.character.clone()
    }

    /// Sets the id value of the character instance
    ///
    /// # Argument
    /// * id - the id value to set
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.character.id = Some(id);
        self
    }

    /// Sets the name value of the character instance
    ///
    /// # Argument
    /// * name - the name value to set
    pub fn name(&mut self, name: String) -> &mut Self {
        self.character.name = name;
        self
    }

    /// Sets the image url value of the character instance
    ///
    /// # Argument
    /// * image_url - the image url value to set
    pub fn image_url(&mut self, image_url: String) -> &mut Self {
        self.character.image_url = image_url;
        self
    }
}
