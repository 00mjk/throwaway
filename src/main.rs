#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::unused_async)]
#![feature(once_cell)]
#![feature(map_first_last)]

use std::error::Error;
use std::net::SocketAddr;

use hyper::Server;
use throwaway::build_app;
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let app = build_app().await?;

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Listening on: {address}");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
