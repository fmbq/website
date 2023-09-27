use once_cell::sync::Lazy;
use sqlx::{
    any::{install_default_drivers, AnyPoolOptions},
    migrate::Migrator,
    AnyPool,
};
use sqlx::{migrate::MigrateDatabase, Any};
use std::env;

pub mod users;

static URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").unwrap());
static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init() {
    install_default_drivers();

    Any::create_database(&URL).await.unwrap();

    let pool = create_connection_pool();

    MIGRATOR.run(&pool).await.unwrap();
}

pub fn create_connection_pool() -> AnyPool {
    AnyPoolOptions::new().connect_lazy(&URL).unwrap()
}

#[cfg(test)]
pub(crate) async fn create_test_connection() -> sqlx::AnyConnection {
    use sqlx::{any::AnyConnectOptions, ConnectOptions};
    use std::str::FromStr;

    install_default_drivers();

    let mut connection = AnyConnectOptions::from_str("sqlite://")
        .unwrap()
        .connect()
        .await
        .unwrap();

    MIGRATOR.run(&mut connection).await.unwrap();

    connection
}
