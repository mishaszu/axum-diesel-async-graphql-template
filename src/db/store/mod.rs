use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};
use r2d2::PooledConnection;

pub use self::error::{Error, Result};

mod error;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub async fn new_db_pool(database_url: &str) -> Result<Pool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .map_err(|e| Error::FailToCreatePool(e.to_string()))
}