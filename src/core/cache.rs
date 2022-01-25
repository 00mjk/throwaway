use std::fmt::Debug;

use anyhow::Result;
use deadpool_redis::{Config as RedisConfig, Runtime};
use deadpool_redis::{Connection, Pool};
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use tracing::{error, info};

use crate::config::Config;
use crate::errors::internal::ServerError;
use crate::models::secrets::cache::CacheSecrets;

pub type CachePool = Pool;

pub async fn connect(config: &Config, cache_secrets: &CacheSecrets) -> Result<CachePool> {
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
        .map_err(|error| {
            error!("Failed to connect to Redis: {error:#?}");
            ServerError::GenericError.into()
        })
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
            let result = cache.get(&key).await.unwrap();
            info!("HIT | {key} | {result:#?}");
            return Some(result);
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
        cache
            .set_ex::<_, _, ()>(&key, value, expiry)
            .await
            .unwrap();
    }

    pub async fn delete(&self, key: &str) {
        let mut cache = self.connection().await;

        info!("DELETE | {key}");
        cache.del::<_, ()>(&key).await.unwrap();
    }

    pub async fn exists(&self, key: &str) -> bool {
        let mut cache = self.connection().await;

        if cache.exists(&key).await.unwrap() {
            info!("EXISTS | {key} | true");
            return true;
        }

        info!("EXISTS | {key} | false");
        false
    }
}
