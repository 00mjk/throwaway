use std::fmt::Debug;

use deadpool_redis::{Config as RedisConfig, Runtime};
use deadpool_redis::{Connection, Pool};
use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};
use tracing::info;

use crate::config::Config;
use crate::errors::core::ServerError;
use crate::models::secrets::cache::CacheSecrets;

pub type CachePool = Pool;

pub fn connect(config: &Config, cache_secrets: &CacheSecrets) -> Result<CachePool, ServerError> {
    let dsn = if config.use_local {
        cache_secrets.local_dsn()
    } else {
        cache_secrets.dsn()
    };

    info!("Redis DSN: {dsn}");

    let redis_config = RedisConfig {
        url: Some(dsn),
        connection: None,
        pool: None,
    };

    redis_config
        .create_pool(Some(Runtime::Tokio1))
        .map_err(ServerError::CacheError)
}

#[derive(Clone)]
pub struct Cache {
    pool: CachePool,
}

impl Cache {
    pub const fn new(pool: CachePool) -> Self {
        Self {
            pool,
        }
    }

    async fn connection(&self) -> Connection {
        self.pool.get().await.unwrap()
    }

    pub async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: FromRedisValue + Debug + Send + Sync,
    {
        let mut cache = self.connection().await;

        if self.exists(key).await {
            let result = cache.get(&key).await;
            if let Ok(inner) = result {
                info!("HIT | {key} | {inner:#?}");
                return Some(inner);
            }
        }

        info!("MISS | {key}");
        None
    }

    pub async fn set<T>(&self, key: &str, value: T, expiry: usize)
    where
        T: ToRedisArgs + Debug + Send + Sync,
    {
        let mut cache = self.connection().await;

        info!("SET | {key} | {value:#?}");
        let result = cache
            .set_ex::<_, _, ()>(&key, value, expiry)
            .await;

        if result.is_err() {
            info!("Failed to set");
        }
    }

    pub async fn delete(&self, key: &str) {
        let mut cache = self.connection().await;

        info!("DELETE | {key}");
        let result = cache.del::<_, ()>(&key).await;

        if result.is_err() {
            info!("Failed to delete");
        }
    }

    pub async fn exists(&self, key: &str) -> bool {
        let mut cache = self.connection().await;

        let cache_exists: RedisResult<bool> = cache.exists(&key).await;
        if let Ok(exists) = cache_exists {
            info!("EXISTS | {key} | {exists}");
            return exists;
        }

        info!("EXISTS | {key} | false");
        false
    }
}
