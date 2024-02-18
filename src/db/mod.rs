use once_cell::sync::Lazy;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
    Sqlite, SqliteConnection, SqlitePool,
};
use std::env;

pub mod users;

static URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").unwrap());
static MIGRATOR: Migrator = sqlx::migrate!();

pub type Connection = SqliteConnection;
pub type Pool = SqlitePool;

pub async fn init() {
    Sqlite::create_database(&URL).await.unwrap();

    let pool = create_connection_pool();

    MIGRATOR.run(&pool).await.unwrap();
}

pub fn create_connection_pool() -> Pool {
    SqlitePoolOptions::new().connect_lazy(&URL).unwrap()
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
