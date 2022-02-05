use serde::{Deserialize, Serialize};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::database::role::creds;
use vaultrs::kv2;

use crate::config::Config;
use crate::models::secrets::cache::CacheSecrets;
use crate::models::secrets::database::DatabaseSecrets;
use crate::models::secrets::jwt::JwtSecrets;
use crate::ServerError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secrets {
    pub cache: CacheSecrets,
    pub database: DatabaseSecrets,
    pub database_deployment: DatabaseSecrets,
    pub jwt: JwtSecrets,
}

// FIXME: Support reading Vault token from filepath (K8s injection method)
pub async fn read(config: &Config) -> Result<Secrets, ServerError> {
    let settings = VaultClientSettingsBuilder::default()
        .address(&config.vault_url)
        .token(&config.vault_token)
        .build()
        .map_err(ServerError::VaultClientConfigError)?;

    let client = VaultClient::new(settings).map_err(ServerError::VaultClientError)?;

    let cache_secrets: CacheSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_kv_path_redis).await?;

    let database_deployment_credentials = creds(
        &client,
        &config.vault_database_mount,
        &config.vault_database_deployment_role,
    )
    .await?;
    let database_deployment_secrets = DatabaseSecrets {
        host: config.database_host.clone(),
        port: config.database_port.clone(),
        db: config.database_db.clone(),
        user: database_deployment_credentials.username,
        password: database_deployment_credentials.password,
    };

    let database_credentials = creds(&client, &config.vault_database_mount, &config.vault_database_role).await?;
    let database_secrets = DatabaseSecrets {
        host: config.database_host.clone(),
        port: config.database_port.clone(),
        db: config.database_db.clone(),
        user: database_credentials.username,
        password: database_credentials.password,
    };

    let jwt_secrets: JwtSecrets = kv2::read(&client, &config.vault_kv_mount, &config.vault_kv_path_jwt).await?;

    Ok(Secrets {
        cache: cache_secrets,
        database: database_secrets,
        database_deployment: database_deployment_secrets,
        jwt: jwt_secrets,
    })
}
