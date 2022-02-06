#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::must_use_candidate,
    clippy::unused_async,
    clippy::struct_excessive_bools
)]
#![feature(const_mut_refs, once_cell, map_first_last, type_alias_impl_trait, core_intrinsics)]

use axum::http::header;
use axum::{AddExtensionLayer, Router};
use axum_extra::middleware::from_fn;
use sqlx::migrate;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing::debug;

use crate::cache::Cache;
use crate::config::Config;
use crate::core::cache::CachePool;
use crate::core::config;
use crate::core::database;
use crate::core::database::DatabasePool;
use crate::core::logging;
use crate::core::{cache, secrets};
use crate::errors::core::ServerError;
use crate::middleware::version_header;
use crate::repositories::profile::ProfileRepository;
use crate::secrets::Secrets;
use crate::services::password::PasswordService;
use crate::services::profile::ProfileService;
use crate::services::token::TokenService;

pub mod attributes;
pub mod core;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod helpers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod validation;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn build_app() -> Result<Router, ServerError> {
    logging::init()?;

    let config: Config = config::read()?;
    debug!("Config: {config:#?}");

    let secrets: Secrets = secrets::read(&config).await?;
    debug!("Secrets: {secrets:#?}");

    // Connection Pools
    let database: DatabasePool = database::connect(&config, &secrets.database).await?;
    let database_deployment: DatabasePool = database::connect(&config, &secrets.database_deployment).await?;

    let cache_pool: CachePool = cache::connect(&config, &secrets.cache)?;
    let cache: Cache = cache::Cache::new(cache_pool);

    // Database Provision
    migrate!("sql/migrations")
        .run(&database_deployment)
        .await
        .map_err(ServerError::SqlxMigrationError)?;

    // Repositories
    let profile_repository = ProfileRepository::new(database.clone(), cache.clone());

    // Services
    let token_service = TokenService::new(&secrets.jwt);
    let password_service = PasswordService::new();
    let profile_service = ProfileService::new(password_service.clone(), profile_repository);

    // Global Middleware
    let version_header = from_fn(version_header::version_header_middleware);
    let sensitive_headers = vec![header::AUTHORIZATION, header::COOKIE];

    let middleware = ServiceBuilder::new()
        .sensitive_headers(sensitive_headers)
        .layer(version_header);

    // App
    let app = Router::new()
        .layer(middleware)
        .merge(handlers::config::routes())
        .merge(handlers::register::routes())
        .merge(handlers::token::routes())
        .merge(handlers::profile::routes())
        .merge(handlers::version::routes())
        .layer(AddExtensionLayer::new(token_service))
        .layer(AddExtensionLayer::new(password_service))
        .layer(AddExtensionLayer::new(profile_service))
        .layer(AddExtensionLayer::new(config))
        .layer(AddExtensionLayer::new(secrets));

    Ok(app)
}
