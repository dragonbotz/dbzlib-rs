//! Database module
//!
//! This module contains implementation of Database used by Dragon Bot Z
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::util::error::{ErrResult, Error};

pub struct PgDatabase {
    pool: sqlx::Pool<Postgres>,
}

impl PgDatabase {
    /// Creates an instance of PgDatabase
    ///
    /// # Arguments
    /// * url - the connection url
    pub async fn new(url: &str) -> ErrResult<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await;

        if let Err(error) = pool {
            return Err(Error::DatabaseConnection(error.to_string()));
        }

        Ok(Self {
            pool: pool.unwrap(),
        })
    }

    /// Returns a reference to the Database's connection pool
    pub fn pool(&self) -> &sqlx::Pool<Postgres> {
        &self.pool
    }
}

#[cfg(test)]
mod PgDatabaseTest {
    use super::*;

    /// To make this test work properly, you must start a service's
    /// docker compose
    #[tokio::test]
    async fn ok_on_good_connection_url() {
        // works with character service
        let database = PgDatabase::new("postgresql://postgres@127.0.0.1:5432/characterdb").await;
        assert!(database.is_ok())
    }
}
