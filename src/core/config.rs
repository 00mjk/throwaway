use dotenv::dotenv;
use envconfig::Envconfig;
use serde::Serialize;
use tracing::error;

use crate::errors::core::ServerError;

#[derive(Envconfig, Clone, Debug, Serialize)]
pub struct Config {
    #[envconfig(from = "USE_LOCAL", default = "false")]
    pub use_local: bool,

    #[envconfig(from = "VAULT_URL")]
    pub vault_url: String,
    #[envconfig(from = "VAULT_TOKEN")]
    pub vault_token: String,

    #[envconfig(from = "VAULT_KV_MOUNT")]
    pub vault_kv_mount: String,
    #[envconfig(from = "VAULT_KV_PATH_REDIS")]
    pub vault_kv_path_redis: String,
    #[envconfig(from = "VAULT_KV_PATH_JWT")]
    pub vault_kv_path_jwt: String,

    #[envconfig(from = "VAULT_DATABASE_MOUNT")]
    pub vault_database_mount: String,
    #[envconfig(from = "VAULT_DATABASE_DEPLOYMENT_ROLE")]
    pub vault_database_deployment_role: String,
    #[envconfig(from = "VAULT_DATABASE_ROLE")]
    pub vault_database_role: String,

    #[envconfig(from = "DATABASE_HOST")]
    pub database_host: String,
    #[envconfig(from = "DATABASE_PORT")]
    pub database_port: String,
    #[envconfig(from = "DATABASE_DB")]
    pub database_db: String,
    #[envconfig(from = "DATABASE_CONNECTIONS", default = "50")]
    pub database_connections: u32,
    #[envconfig(from = "DATABASE_LIFETIME", default = "3600")]
    pub database_lifetime: u64,
}

pub fn read() -> Result<Config, ServerError> {
    dotenv().ok();
    Config::init_from_env().map_err(|error| {
        error!("Failed to load config: {error:#?}");
        ServerError::EnvConfigError(error)
    })
}
