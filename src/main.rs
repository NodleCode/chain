mod chain_spec;

#[macro_use]
mod service;
mod cli;
mod command;
mod executor;

use sc_cli::VersionInfo;

fn main() -> Result<(), sc_cli::error::Error> {
    let version = VersionInfo {
        name: "Nodle Chain Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "nodle-chain",
        author: "Eliott Teissonniere <eliott@nodle.co>",
        description: "Nodle Chain Node",
        support_url: "https://github.com/NodleCode/chain/issues",
        copyright_start_year: 2019,
    };

    command::run(version)
}
