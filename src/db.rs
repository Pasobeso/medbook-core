use anyhow::Result;
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::time::Duration;
use tracing::info;

/// Use diesel_async for pooled async connection, instead of
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

// Use async PG connection instead of
// pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>
pub type DbPool = Pool<AsyncPgConnection>;

pub async fn connect(database_url: &str) -> Result<DbPool> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(config)
        .await;
    info!("Connected to database");
    Ok(pool?)
}

/// รัน migrations แบบ synchronous ใน thread แยก (เสถียร/ชัวร์)
pub async fn run_migrations_blocking(
    migrations: EmbeddedMigrations,
    database_url: &str,
) -> Result<usize> {
    let url = database_url.to_string();

    let applied = tokio::task::spawn_blocking(move || -> Result<usize> {
        // 1) ต่อ DB แบบ sync
        let mut conn = PgConnection::establish(&url)
            .map_err(|e| anyhow::anyhow!("connect for migrations failed: {e}"))?;

        // 2) รันทุก migration ที่ยังไม่ถูก apply
        let versions = conn
            .run_pending_migrations(migrations)
            .map_err(|e| anyhow::anyhow!("running migrations failed: {e}"))?;

        Ok(versions.len())
    })
    .await
    .map_err(|e| anyhow::anyhow!("spawn_blocking join error: {e}"))??;

    Ok(applied)
}
