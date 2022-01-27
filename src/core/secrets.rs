use anyhow::Result;
use serde::{Deserialize, Serialize};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

use crate::config::Config;
use crate::models::secrets::cache::CacheSecrets;
use crate::models::secrets::database::DatabaseSecrets;
use crate::models::secrets::jwt::JwtSecrets;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secrets {
    pub cache: CacheSecrets,
    pub database: DatabaseSecrets,
    pub jwt: JwtSecrets,
}

pub async fn read(config: &Config) -> Result<Secrets> {
    let settings = VaultClientSettingsBuilder::default()
        .address(&config.vault_url)
        .token(&config.vault_token)
        .build()?;

    let client = VaultClient::new(settings)?;

    let cache_secrets: CacheSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_path_redis).await?;

    let database_secrets: DatabaseSecrets =
        kv2::read(&client, &config.vault_kv_mount, &config.vault_path_postgresql).await?;

    let jwt_secrets: JwtSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_path_jwt).await?;

    Ok(Secrets {
        cache: cache_secrets,
        database: database_secrets,
        jwt: jwt_secrets,
    })
}
