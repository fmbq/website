use color_eyre::eyre::Context;
use once_cell::sync::OnceCell;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
    Sqlite, SqliteConnection, SqlitePool,
};
use std::env;

pub mod articles;
pub mod users;

static URL: OnceCell<String> = OnceCell::new();
static MIGRATOR: Migrator = sqlx::migrate!();

pub type Connection = SqliteConnection;
pub type Pool = SqlitePool;

pub async fn init() -> color_eyre::eyre::Result<()> {
    let url = URL
        .get_or_try_init(|| env::var("DATABASE_URL"))
        .wrap_err("DATABASE_URL environment variable must be set to find database")?;

    Sqlite::create_database(url).await?;

    let pool = create_connection_pool()?;

    MIGRATOR.run(&pool).await?;

    Ok(())
}

pub fn create_connection_pool() -> color_eyre::eyre::Result<Pool> {
    let url = URL.get_or_try_init(|| env::var("DATABASE_URL"))?;

    SqlitePoolOptions::new()
        .connect_lazy(url)
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
