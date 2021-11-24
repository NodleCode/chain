/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::service::MainExecutorDispatch;
use crate::{
    chain_spec,
    cli::{Cli, Subcommand},
    service::{self},
};
use primitives::Block;
use sc_cli::{ChainSpec, Result, Role, RuntimeVersion, SubstrateCli};

/// Can be called for a `Configuration` to check what node it belongs to.
pub trait IdentifyChain {
    /// Returns if this is a configuration for the `Staking` node.
    fn is_runtime_staking(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
    fn is_runtime_staking(&self) -> bool {
        self.id().to_lowercase().starts_with("staking")
    }
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
    fn is_runtime_staking(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_runtime_staking(self)
    }
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(chain_spec::cs_main::development_config()),
        "local" => Box::new(chain_spec::cs_main::local_testnet_config()),
        "staking-dev" => Box::new(chain_spec::cs_staking::development_config()),
        "staking-local" => Box::new(chain_spec::cs_staking::local_staking_config()),
        "" | "main" => Box::new(chain_spec::cs_main::main_config()),
        "arcadia" => Box::new(chain_spec::cs_main::arcadia_config()),
        path => {
            let chain_spec = chain_spec::cs_main::ChainSpec::from_json_file(path.into())?;
            if chain_spec.is_runtime_staking() {
                Box::new(chain_spec::cs_staking::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?)
            } else {
                Box::new(chain_spec::cs_main::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?)
            }
        }
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Nodle Chain Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/NodleCode/chain/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2019
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &runtime_main::VERSION
    }
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    Role::Light => service::build_light(config),
                    _ => service::build_full(config).map(|full| full.task_manager),
                }
                .map_err(sc_cli::Error::Service)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;

                runner.sync_run(|config| cmd.run::<Block, MainExecutorDispatch>(config))
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                    .into())
            }
        }
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, backend, _, task_manager) = service::new_chain_ops(&mut config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
    }
}
