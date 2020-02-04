//! Substrate Node Template CLI library.

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

pub use sc_cli::{error, VersionInfo};

fn main() -> Result<(), error::Error> {
    let version = VersionInfo {
        name: "Nodle Chain Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "node-chain",
        author: "Eliott Teissonniere",
        description: "Nodle Chain Node",
        support_url: "eliott@nodle.co",
        copyright_start_year: 2019,
    };

    command::run(version)
}
