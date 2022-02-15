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

use anyhow::Error;
use throwaway::core::config::Config;
use throwaway::core::database::DatabasePool;
use throwaway::core::migrations::migrate;
use throwaway::core::secrets::Secrets;
use throwaway::core::{config, database, logging, secrets};
use tracing::debug;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logging::init()?;

    let config: Config = config::read()?;
    debug!("Config: {config:#?}");

    let secrets: Secrets = secrets::read(&config).await?;
    debug!("Secrets: {secrets:#?}");

    // Connection Pools
    let database_deployment: DatabasePool = database::connect(&config, &secrets.database_deployment).await?;

    // Database Provision
    migrate(&database_deployment).await?;

    Ok(())
}
