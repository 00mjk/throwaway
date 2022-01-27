#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::must_use_candidate)]
#![feature(once_cell)]
#![feature(map_first_last)]

use anyhow::Result;
use axum::AddExtensionLayer;
use axum::Router;
use sqlx::migrate;
use tracing::debug;

use crate::cache::Cache;
use crate::config::Config;
use crate::core::cache::CachePool;
use crate::core::config;
use crate::core::database;
use crate::core::database::DatabasePool;
use crate::core::logging;
use crate::core::{cache, secrets};
use crate::handlers::{healthcheck, profile, register, token};
use crate::repositories::healthcheck::HealthcheckRepository;
use crate::repositories::profile::ProfileRepository;
use crate::secrets::Secrets;
use crate::services::healthcheck::HealthcheckService;
use crate::services::password::PasswordService;
use crate::services::profile::ProfileService;
use crate::services::token::TokenService;

pub mod core;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod validation;

pub async fn build_app() -> Result<Router> {
    logging::init()?;

    let config: Config = config::read()?;
    debug!("Config: {config:#?}");

    let secrets: Secrets = secrets::read(&config).await?;
    debug!("Secrets: {secrets:#?}");

    // Connection Pools
    let database: DatabasePool = database::connect(&config, &secrets.database).await?;

    let cache_pool: CachePool = cache::connect(&config, &secrets.cache)?;
    let cache: Cache = cache::Cache::new(cache_pool);

    // Database Provision
    migrate!("sql/migrations")
        .run(&database)
        .await?;

    // Repositories
    let healthcheck_repository = HealthcheckRepository::new(database.clone(), cache.clone());
    let profile_repository = ProfileRepository::new(database.clone(), cache.clone());

    // Services
    let token_service = TokenService::new(&secrets.jwt);
    let password_service = PasswordService::new();
    let healthcheck_service = HealthcheckService::new(healthcheck_repository);
    let profile_service = ProfileService::new(password_service.clone(), profile_repository);

    // App
    let app = Router::new()
        .merge(healthcheck::routes())
        .merge(register::routes())
        .merge(token::routes())
        .merge(profile::routes())
        .layer(AddExtensionLayer::new(token_service))
        .layer(AddExtensionLayer::new(password_service))
        .layer(AddExtensionLayer::new(healthcheck_service))
        .layer(AddExtensionLayer::new(profile_service));

    Ok(app)
}
