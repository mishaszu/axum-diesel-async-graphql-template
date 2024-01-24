use crate::config::config;

pub use super::error::{Error, Result};
use super::store::{new_db_pool, run_migration, DbConn, Pool};

#[derive(Clone)]
pub struct ModelManager {
    db: Pool,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool(&config().DB_URL).await?;

        Ok(Self { db })
    }

    pub fn run_migration(&self) {
        let mut conn = self.db.get().unwrap();
        run_migration(&mut conn);
    }

    pub fn conn(&self) -> Result<DbConn> {
        self.db.get().map_err(|_| Error::DbPoolConnectionFailed)
    }
}
