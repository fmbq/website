use crate::config::Configuration;
use poem::{
    session::{CookieConfig, MemoryStorage, RedisStorage, ServerSession},
    Endpoint, EndpointExt, Response,
};
use redis::{aio::ConnectionManager, Client};

pub async fn configure_session<'a, T>(
    endpoint: T,
    config: &Configuration,
) -> color_eyre::eyre::Result<impl Endpoint<Output = Response> + 'a>
where
    T: Endpoint + 'a,
{
    let cookie_config = CookieConfig::default();

    if let Some(redis_host) = &config.redis_host {
        let redis_port = config.redis_port.unwrap_or(6379);

        tracing::info!(
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
        tracing::warn!("redis not configured, using in-memory session storage");

        let server_session = ServerSession::new(cookie_config, MemoryStorage::new());

        Ok(endpoint.with(server_session).boxed())
    }
}
