use poem::{
    error::Result,
    session::{CookieConfig, MemoryStorage, RedisStorage, ServerSession},
    Endpoint, EndpointExt, Response,
};
use redis::{aio::ConnectionManager, Client};
use std::env;

pub async fn configure_session<'a, T>(
    endpoint: T,
) -> Result<impl Endpoint<Output = Response> + 'a, Box<dyn std::error::Error>>
where
    T: Endpoint + 'a,
{
    let cookie_config = CookieConfig::default();

    if let Ok(redis_host) = env::var("REDIS_HOST") {
        let redis_port = env::var("REDIS_PORT").unwrap_or("6379".to_string());

        log::info!(
            "using redis session storage at {}:{}",
            redis_host,
            redis_port
        );

        let server_session = ServerSession::new(
            cookie_config,
            RedisStorage::new(
                ConnectionManager::new(Client::open(format!("redis://{redis_host}:{redis_port}"))?)
                    .await?,
            ),
        );

        Ok(endpoint.with(server_session).boxed())
    } else {
        log::warn!("redis not configured, using in-memory session storage");

        let server_session = ServerSession::new(cookie_config, MemoryStorage::new());

        Ok(endpoint.with(server_session).boxed())
    }
}
