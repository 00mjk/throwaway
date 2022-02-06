#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    dead_code,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::must_use_candidate,
    clippy::unused_async,
    clippy::struct_excessive_bools
)]

pub mod framework;
pub mod models;
pub mod server;
