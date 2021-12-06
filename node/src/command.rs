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

use crate::cli_relaychain::RelayChainCli;
use crate::service::MainExecutorDispatch;
use crate::{
    chain_spec,
    cli::{Cli, Subcommand},
    service::{self},
};
use cumulus_client_service::genesis::generate_genesis_block;
use parity_scale_codec::Encode;
use polkadot_parachain::primitives::AccountIdConversion;
use primitives::{Block, ParaId};
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::Block as BlockT;
use std::{io::Write, net::SocketAddr};

// default to the Statemint/Statemine/Westmint id
const DEFAULT_PARA_ID: u32 = 1000;

/// Can be called for a `Configuration` to check what node it belongs to.
pub trait IdentifyChain {
    /// Returns if this is a configuration for the `Staking` node.
    fn is_runtime_staking(&self) -> bool;

    /// Returns if this is a configuration for parachian.
    fn is_runtime_parachain(&self) -> bool;

    /// Returns if this is a configuration for the `Eden` runtime.
    fn is_runtime_eden(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
    fn is_runtime_staking(&self) -> bool {
        self.id().to_lowercase().starts_with("staking")
    }

    fn is_runtime_parachain(&self) -> bool {
        self.id().to_lowercase().starts_with("para")
    }

    fn is_runtime_eden(&self) -> bool {
        self.id().to_lowercase().starts_with("para_eden")
    }
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
    fn is_runtime_staking(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_runtime_staking(self)
    }

    fn is_runtime_parachain(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_runtime_parachain(self)
    }

    fn is_runtime_eden(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_runtime_eden(self)
    }
}

fn load_spec(
    id: &str,
    para_id: ParaId,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    log::info!("KOM=> id-{}, para_id-{}", id, para_id);
    Ok(match id {
        "dev" => Box::new(chain_spec::cs_main::development_config()),
        "local" => Box::new(chain_spec::cs_main::local_testnet_config()),
        "staking-dev" => Box::new(chain_spec::cs_staking::development_config()),
        "staking-local" => Box::new(chain_spec::cs_staking::local_staking_config()),
        "eden-dev" => Box::new(chain_spec::cs_eden::development_config(para_id)),
        "eden-local" => Box::new(chain_spec::cs_eden::local_config(para_id)),
        "" | "main" => Box::new(chain_spec::cs_main::main_config()),
        "arcadia" => Box::new(chain_spec::cs_main::arcadia_config()),
        path => {
            let chain_spec = chain_spec::cs_main::ChainSpec::from_json_file(path.into())?;
            if chain_spec.is_runtime_staking() {
                Box::new(chain_spec::cs_staking::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?)
            } else if chain_spec.is_runtime_eden() {
                Box::new(chain_spec::cs_eden::ChainSpec::from_json_file(
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
        // log::info!("KOM=> run-{:#?}", self.run);
        load_spec(id, self.run.parachain_id.unwrap_or(DEFAULT_PARA_ID).into())
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        if chain_spec.is_runtime_staking() {
            &runtime_staking::VERSION
        } else if chain_spec.is_runtime_eden() {
            &runtime_eden::VERSION
        } else {
            &runtime_main::VERSION
        }
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "nodle collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Nodle collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} [parachain-args] -- [relay_chain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/nodlecode/chain/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name().to_string()].iter())
            .load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        polkadot_cli::Cli::native_runtime_version(chain_spec)
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_ws_listen_port() -> u16 {
        9945
    }

    fn rpc_http_listen_port() -> u16 {
        9934
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_http(default_listen_port)
    }

    fn rpc_ipc(&self) -> Result<Option<String>> {
        self.base.base.rpc_ipc()
    }

    fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_ws(default_listen_port)
    }

    fn prometheus_config(&self, default_listen_port: u16) -> Result<Option<PrometheusConfig>> {
        self.base.base.prometheus_config(default_listen_port)
    }

    fn init<C: SubstrateCli>(&self) -> Result<()> {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool()
    }

    fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
        self.base.base.state_cache_child_ratio()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
        self.base.base.rpc_ws_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }
}

fn extract_genesis_wasm(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Result<Vec<u8>> {
    let mut storage = chain_spec.build_storage()?;

    storage
        .top
        .remove(sp_core::storage::well_known_keys::CODE)
        .ok_or_else(|| "Could not find wasm file in genesis state!".into())
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
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
        Some(Subcommand::ExportGenesisState(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let block: crate::para_service::Block = generate_genesis_block(&load_spec(
                &params.chain.clone().unwrap_or_default(),
                params.parachain_id.unwrap_or(DEFAULT_PARA_ID).into(),
            )?)?;
            let raw_header = block.header().encode();
            let output_buf = if params.raw {
                raw_header
            } else {
                format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::ExportGenesisWasm(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let raw_wasm_blob =
                extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
            let output_buf = if params.raw {
                raw_wasm_blob
            } else {
                format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;

            runner.run_node_until_exit(|config| async move {
                log::info!("Chain Spec :: {:#?}", config.chain_spec);

                log::info!(
                    "Is ParaChain :: {:#?}",
                    config.chain_spec.is_runtime_parachain()
                );

                if !config.chain_spec.is_runtime_parachain() {
                    log::info!("Entering Solo Chain");

                    match config.role {
                        sc_service::Role::Light => service::build_light(config),
                        _ => service::build_full(config).map(|full| full.task_manager),
                    }
                    .map_err(sc_cli::Error::Service)
                } else {
                    log::info!("Entering Para Chain");

                    let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
                        .map(|e| e.para_id)
                        .ok_or_else(|| "Could not find parachain extension in chain-spec.")?;

                    let polkadot_cli = RelayChainCli::new(
                        &config,
                        [RelayChainCli::executable_name().to_string()]
                            .iter()
                            .chain(cli.relaychain_args.iter()),
                    );

                    let id = ParaId::from(para_id);

                    let parachain_account =
                        AccountIdConversion::<polkadot_primitives::v0::AccountId>::into_account(
                            &id,
                        );

                    let block: crate::para_service::Block =
                        generate_genesis_block(&config.chain_spec)
                            .map_err(|e| format!("{:?}", e))?;
                    let genesis_state =
                        format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                    let tokio_handle = config.tokio_handle.clone();
                    let polkadot_config = SubstrateCli::create_configuration(
                        &polkadot_cli,
                        &polkadot_cli,
                        tokio_handle,
                    )
                    .map_err(|err| format!("Relay chain argument error: {}", err))?;

                    log::info!("Parachain id: {:?}", id);
                    log::info!("Parachain Account: {}", parachain_account);
                    log::info!("Parachain genesis state: {}", genesis_state);
                    log::info!(
                        "Is collating: {}",
                        if config.role.is_authority() {
                            "yes"
                        } else {
                            "no"
                        }
                    );

                    crate::para_service::start_eden_parachain_node(config, polkadot_config, id)
                        .await
                        .map(|r| r.0)
                        .map_err(Into::into)
                }
            })
        }
    }
}
