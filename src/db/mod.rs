use crate::config::Configuration;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
    Sqlite, SqliteConnection, SqlitePool,
};

pub mod articles;
pub mod users;

static MIGRATOR: Migrator = sqlx::migrate!();

pub type Connection = SqliteConnection;
pub type Pool = SqlitePool;

pub async fn init(config: &Configuration) -> color_eyre::eyre::Result<()> {
    Sqlite::create_database(&config.database_url).await?;

    let pool = create_connection_pool(config)?;

    MIGRATOR.run(&pool).await?;

    Ok(())
}

pub fn create_connection_pool(config: &Configuration) -> color_eyre::eyre::Result<Pool> {
    SqlitePoolOptions::new()
        .connect_lazy(&config.database_url)
        .map_err(Into::into)
}

#[cfg(test)]
pub(crate) async fn create_test_connection() -> Connection {
    use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions};
    use std::str::FromStr;

    let mut connection = SqliteConnectOptions::from_str("sqlite://")
        .unwrap()
        .connect()
        .await
        .unwrap();

    MIGRATOR.run(&mut connection).await.unwrap();

    connection
}
