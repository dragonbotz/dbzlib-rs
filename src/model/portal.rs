//! Portal model
//!
//! This module contains the implementation of the portal models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PortalData {
    id: Option<i64>,
    name: String,
    description: String,
    image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortalContent {
    portal: i64,
    characters: Vec<i64>,
}

impl PortalData {
    /// Creates an instance of PortalData
    ///
    /// # Arguments
    /// * id - the portal id
    /// * name - the portal name
    /// * description - the portal description
    /// * image_url - the portal image url
    pub fn new(id: Option<i64>, name: String, description: String, image_url: String) -> Self {
        Self {
            id,
            name,
            description,
            image_url,
        }
    }

    /// Returns a reference to portal's id
    pub fn id(&self) -> &Option<i64> {
        &self.id
    }

    /// Returns a reference to portal's name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to portal's description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns a reference to portal's image url
    pub fn image_url(&self) -> &String {
        &self.image_url
    }
}

impl PortalContent {
    /// Creates an instance of PortalContent
    ///
    /// # Arguments
    /// * portal - the portal id
    /// * characters - the portal characters
    pub fn new(portal: i64, characters: Vec<i64>) -> Self {
        Self { portal, characters }
    }

    /// Returns the portal id
    pub fn portal(&self) -> i64 {
        self.portal
    }

    /// Returns a reference to the characters contained in the portal
    pub fn characters(&self) -> &Vec<i64> {
        &self.characters
    }
}
