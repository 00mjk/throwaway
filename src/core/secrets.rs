use serde::{Deserialize, Serialize};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

use crate::config::Config;
use crate::core::errors::ServerError;
use crate::models::secrets::cache::CacheSecrets;
use crate::models::secrets::database::DatabaseSecrets;
use crate::models::secrets::jwt::JwtSecrets;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secrets {
    pub cache: CacheSecrets,
    pub database: DatabaseSecrets,
    pub jwt: JwtSecrets,
}

pub async fn read(config: &Config) -> Result<Secrets, ServerError> {
    let settings = VaultClientSettingsBuilder::default()
        .address(&config.vault_url)
        .token(&config.vault_token)
        .build()
        .unwrap();

    let client = VaultClient::new(settings).unwrap();

    let cache_secrets: CacheSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_path_redis)
        .await
        .unwrap();

    let database_secrets: DatabaseSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_path_postgresql)
        .await
        .unwrap();

    let jwt_secrets: JwtSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_path_jwt)
        .await
        .unwrap();

    Ok(Secrets {
        cache: cache_secrets,
        database: database_secrets,
        jwt: jwt_secrets,
    })
}
