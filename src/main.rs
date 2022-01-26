#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_async)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::must_use_candidate)]
#![feature(once_cell)]

use std::net::SocketAddr;

use anyhow::Error;
use hyper::Server;
use throwaway::build_app;
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let app = build_app().await?;

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Listening on: {address}");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
