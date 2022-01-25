#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_async)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::must_use_candidate)]
#![feature(once_cell)]

use anyhow::Error;
use throwaway::start_server;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    start_server().await
}
