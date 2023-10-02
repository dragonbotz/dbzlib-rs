//! Portal model
//!
//! This module contains the implementation of the portal models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PortalData {
    id: Option<i64>,
    name: String,
    description: String,
    image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PortalContent {
    portal: i64,
    characters: Vec<i64>,
}

impl PortalData {
    /// Returns a reference to portal's id
    pub fn id(&self) -> &Option<i64> { &self.id }

    /// Returns a reference to portal's name
    pub fn name(&self) -> &String { &self.name }

    /// Returns a reference to portal's description
    pub fn description(&self) -> &String { &self.description }

    /// Returns a reference to portal's image url
    pub fn image_url(&self) -> &String { &self.image_url }
}
