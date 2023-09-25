use once_cell::sync::Lazy;
use sqlx::{
    any::{install_default_drivers, AnyPoolOptions},
    AnyPool,
};
use sqlx::{migrate::MigrateDatabase, Any};
use std::env;

pub mod users;

static URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").unwrap());

pub async fn init() {
    install_default_drivers();

    Any::create_database(&URL).await.unwrap();

    let pool = create_connection_pool();

    sqlx::migrate!().run(&pool).await.unwrap();
}

pub fn create_connection_pool() -> AnyPool {
    AnyPoolOptions::new().connect_lazy(&URL).unwrap()
}
