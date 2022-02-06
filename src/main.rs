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

use std::net::SocketAddr;

use anyhow::Error;
use hyper::Server;
use throwaway::build_app;
use throwaway::errors::core::ServerError;
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let app = build_app().await?;

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Listening on: {address}");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .map_err(ServerError::HyperError)?;

    Ok(())
}
