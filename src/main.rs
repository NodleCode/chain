mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
    let version = sc_cli::VersionInfo {
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
