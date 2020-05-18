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

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;

use nodle_chain_runtime::{opaque::Block, RuntimeApi};
use sc_consensus_babe;
use sc_executor::native_executor_instance;
use sc_finality_grandpa::{
    FinalityProofProvider as GrandpaFinalityProofProvider, StorageAndProofProvider,
};
use sc_service::{
    config::Configuration, error::Error as ServiceError, AbstractService, ServiceBuilder,
};
use sp_inherents::InherentDataProviders;

native_executor_instance!(
    pub Executor,
    nodle_chain_runtime::api::dispatch,
    nodle_chain_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
macro_rules! new_full_start {
    ($config:expr) => {{
        use std::sync::Arc;

        type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

        let mut import_setup = None;
        let mut rpc_setup = None;
        let inherent_data_providers = sp_inherents::InherentDataProviders::new();

        let builder = sc_service::ServiceBuilder::new_full::<
            nodle_chain_runtime::opaque::Block,
            nodle_chain_runtime::RuntimeApi,
            crate::service::Executor,
        >($config)?
        .with_select_chain(|_config, backend| Ok(sc_consensus::LongestChain::new(backend.clone())))?
        .with_transaction_pool(|config, client, _fetcher, prometheus_registry| {
            let pool_api = sc_transaction_pool::FullChainApi::new(client.clone());
            Ok(sc_transaction_pool::BasicPool::new(
                config,
                std::sync::Arc::new(pool_api),
                prometheus_registry,
            ))
        })?
        .with_import_queue(
            |_config,
             client,
             mut select_chain,
             _transaction_pool,
             spawn_task_handle,
             prometheus_registry| {
                let select_chain = select_chain
                    .take()
                    .ok_or_else(|| sc_service::Error::SelectChainRequired)?;
                let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
                    client.clone(),
                    &(client.clone() as Arc<_>),
                    select_chain,
                )?;
                let justification_import = grandpa_block_import.clone();

                let (block_import, babe_link) = sc_consensus_babe::block_import(
                    sc_consensus_babe::Config::get_or_compute(&*client)?,
                    grandpa_block_import,
                    client.clone(),
                )?;

                let import_queue = sc_consensus_babe::import_queue(
                    babe_link.clone(),
                    block_import.clone(),
                    Some(Box::new(justification_import)),
                    None,
                    client,
                    inherent_data_providers.clone(),
                    spawn_task_handle,
                    prometheus_registry,
                )?;

                import_setup = Some((block_import, grandpa_link, babe_link));
                Ok(import_queue)
            },
        )?
        .with_rpc_extensions(|builder| -> std::result::Result<RpcExtension, _> {
            let mut io = jsonrpc_core::IoHandler::default();
            io.extend_with(pallet_root_of_trust_rpc::RootOfTrustApi::to_delegate(
                pallet_root_of_trust_rpc::RootOfTrust::new(builder.client().clone()),
            ));

            let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
            rpc_setup = Some((shared_voter_state));
            Ok(io)
        })?;

        (builder, import_setup, inherent_data_providers, rpc_setup)
    }};
}

/// Creates a full service from the configuration.
///
/// We need to use a macro because the test suit doesn't work with an opaque service. It expects
/// concrete types instead.
macro_rules! new_full {
	($config:expr, $with_startup_data: expr) => {{
		use futures::prelude::*;
		use sc_network::Event;
		use sc_client_api::ExecutorProvider;

		let (
			role,
			force_authoring,
			name,
			disable_grandpa,
		) = (
			$config.role.clone(),
			$config.force_authoring,
			$config.network.node_name.clone(),
			$config.disable_grandpa,
		);

		let (builder, mut import_setup, inherent_data_providers, mut rpc_setup) = new_full_start!($config);

		let service = builder
			.with_finality_proof_provider(|client, backend| {
				// GenesisAuthoritySetProvider is implemented for StorageAndProofProvider
				let provider = client as Arc<dyn sc_finality_grandpa::StorageAndProofProvider<_, _>>;
				Ok(Arc::new(sc_finality_grandpa::FinalityProofProvider::new(backend, provider)) as _)
			})?
			.build()?;

		let (block_import, grandpa_link, babe_link) = import_setup.take()
			.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

		let shared_voter_state = rpc_setup.take()
			.expect("The SharedVoterState is present for Full Services or setup failed before. qed");

		($with_startup_data)(&block_import, &babe_link);

		if let sc_service::config::Role::Authority { .. } = &role {
			let proposer = sc_basic_authorship::ProposerFactory::new(
				service.client(),
				service.transaction_pool()
			);

			let client = service.client();
			let select_chain = service.select_chain()
				.ok_or(sc_service::Error::SelectChainRequired)?;

			let can_author_with =
				sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

			let babe_config = sc_consensus_babe::BabeParams {
				keystore: service.keystore(),
				client,
				select_chain,
				env: proposer,
				block_import,
				sync_oracle: service.network(),
				inherent_data_providers: inherent_data_providers.clone(),
				force_authoring,
				babe_link,
				can_author_with,
			};

			let babe = sc_consensus_babe::start_babe(babe_config)?;
			service.spawn_essential_task("babe-proposer", babe);
		}

		// Spawn authority discovery module.
		if matches!(role, sc_service::config::Role::Authority{..} | sc_service::config::Role::Sentry {..}) {
			let (sentries, authority_discovery_role) = match role {
				sc_service::config::Role::Authority { ref sentry_nodes } => (
					sentry_nodes.clone(),
					sc_authority_discovery::Role::Authority (
						service.keystore(),
					),
				),
				sc_service::config::Role::Sentry {..} => (
					vec![],
					sc_authority_discovery::Role::Sentry,
				),
				_ => unreachable!("Due to outer matches! constraint; qed.")
			};

			let network = service.network();
			let dht_event_stream = network.event_stream("authority-discovery").filter_map(|e| async move { match e {
				Event::Dht(e) => Some(e),
				_ => None,
			}}).boxed();
			let authority_discovery = sc_authority_discovery::AuthorityDiscovery::new(
				service.client(),
				network,
				sentries,
				dht_event_stream,
				authority_discovery_role,
				service.prometheus_registry(),
			);

			service.spawn_task("authority-discovery", authority_discovery);
		}

		// if the node isn't actively participating in consensus then it doesn't
		// need a keystore, regardless of which protocol we use below.
		let keystore = if role.is_authority() {
			Some(service.keystore())
		} else {
			None
		};

		let config = sc_finality_grandpa::Config {
			// FIXME #1578 make this available through chainspec
			gossip_duration: std::time::Duration::from_millis(333),
			justification_period: 512,
			name: Some(name),
			observer_enabled: false,
			keystore,
			is_authority: role.is_network_authority(),
		};

		let enable_grandpa = !disable_grandpa;
		if enable_grandpa {
			// start the full GRANDPA voter
			// NOTE: non-authorities could run the GRANDPA observer protocol, but at
			// this point the full voter should provide better guarantees of block
			// and vote data availability than the observer. The observer has not
			// been tested extensively yet and having most nodes in a network run it
			// could lead to finality stalls.
			let grandpa_config = sc_finality_grandpa::GrandpaParams {
				config,
				link: grandpa_link,
				network: service.network(),
				inherent_data_providers: inherent_data_providers.clone(),
				telemetry_on_connect: Some(service.telemetry_on_connect_stream()),
				voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
				prometheus_registry: service.prometheus_registry(),
				shared_voter_state,
			};

			// the GRANDPA voter task is considered infallible, i.e.
			// if it fails we take down the service with it.
			service.spawn_essential_task(
				"grandpa-voter",
				sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
			);
		} else {
			sc_finality_grandpa::setup_disabled_grandpa(
				service.client(),
				&inherent_data_providers,
				service.network(),
			)?;
		}

		Ok((service, inherent_data_providers))
	}};
	($config:expr) => {{
		new_full!($config, |_, _| {})
	}}
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<impl AbstractService, ServiceError> {
    new_full!(config).map(|(service, _)| service)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<impl AbstractService, ServiceError> {
    type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

    let inherent_data_providers = InherentDataProviders::new();

    let service = ServiceBuilder::new_light::<Block, RuntimeApi, Executor>(config)?
        .with_select_chain(|_config, backend| Ok(sc_consensus::LongestChain::new(backend.clone())))?
        .with_transaction_pool(|config, client, fetcher, prometheus_registry| {
            let fetcher = fetcher
                .ok_or_else(|| "Trying to start light transaction pool without active fetcher")?;
            let pool_api = sc_transaction_pool::LightChainApi::new(client.clone(), fetcher.clone());
            let pool = sc_transaction_pool::BasicPool::with_revalidation_type(
                config,
                Arc::new(pool_api),
                prometheus_registry,
                sc_transaction_pool::RevalidationType::Light,
            );
            Ok(pool)
        })?
        .with_import_queue_and_fprb(
            |_config,
             client,
             backend,
             fetcher,
             _select_chain,
             _tx_pool,
             spawn_task_handle,
             prometheus_registry| {
                let fetch_checker = fetcher
                    .map(|fetcher| fetcher.checker().clone())
                    .ok_or_else(|| {
                        "Trying to start light import queue without active fetch checker"
                    })?;
                let grandpa_block_import = sc_finality_grandpa::light_block_import(
                    client.clone(),
                    backend,
                    &(client.clone() as Arc<_>),
                    Arc::new(fetch_checker),
                )?;

                let finality_proof_import = grandpa_block_import.clone();
                let finality_proof_request_builder =
                    finality_proof_import.create_finality_proof_request_builder();

                let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
                    sc_consensus_babe::Config::get_or_compute(&*client)?,
                    grandpa_block_import,
                    client.clone(),
                )?;

                let import_queue = sc_consensus_babe::import_queue(
                    babe_link,
                    babe_block_import,
                    None,
                    Some(Box::new(finality_proof_import)),
                    client.clone(),
                    inherent_data_providers.clone(),
                    spawn_task_handle,
                    prometheus_registry,
                )?;

                Ok((import_queue, finality_proof_request_builder))
            },
        )?
        .with_finality_proof_provider(|client, backend| {
            // GenesisAuthoritySetProvider is implemented for StorageAndProofProvider
            let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
            Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
        })?
        .with_rpc_extensions(|builder| -> std::result::Result<RpcExtension, _> {
            let mut io = jsonrpc_core::IoHandler::default();
            io.extend_with(pallet_root_of_trust_rpc::RootOfTrustApi::to_delegate(
                pallet_root_of_trust_rpc::RootOfTrust::new(builder.client().clone()),
            ));

            Ok(io)
        })?
        .build()?;

    Ok(service)
}
