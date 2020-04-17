#[macro_use]
extern crate serde_derive;

pub mod adapters;
mod cli_app;
mod domain;

pub use cli_app::*;
