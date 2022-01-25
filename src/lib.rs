#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_async)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::must_use_candidate)]
#![feature(once_cell)]

use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use axum::{AddExtensionLayer, Server};
use tracing::{debug, info};

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

pub async fn start_server() -> Result<()> {
    logging::init()?;

    let config: Config = config::read().await?;
    debug!("Config: {config:#?}");

    let secrets: Secrets = secrets::read(&config).await?;
    debug!("Secrets: {secrets:#?}");

    // Connection Pools
    let database: DatabasePool = database::connect(&config, &secrets.database).await?;

    let cache_pool: CachePool = cache::connect(&config, &secrets.cache).await?;
    let cache: Cache = cache::Cache::new(cache_pool);

    // Repositories
    let healthcheck_repository = HealthcheckRepository::new(database.clone(), cache.clone());
    let profile_repository = ProfileRepository::new(database.clone(), cache.clone());

    // Services
    let token_service = TokenService::new(&secrets.jwt);
    let password_service = PasswordService::new();
    let healthcheck_service = HealthcheckService::new(healthcheck_repository.clone());
    let profile_service = ProfileService::new(password_service.clone(), profile_repository.clone());

    let app = Router::new()
        .merge(healthcheck::routes())
        .merge(register::routes())
        .merge(token::routes())
        .merge(profile::routes())
        .layer(AddExtensionLayer::new(token_service))
        .layer(AddExtensionLayer::new(password_service))
        .layer(AddExtensionLayer::new(healthcheck_service))
        .layer(AddExtensionLayer::new(profile_service));

    let address = SocketAddr::from(([0, 0, 0, 0], config.app_port));
    info!("Listening on: {address}");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
