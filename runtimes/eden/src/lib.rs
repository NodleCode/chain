/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is \
        built with `SKIP_WASM_BUILD` flag and it is only usable for \
        production chains. Please rebuild with the flag disabled.",
	)
}

use constants::RuntimeBlockWeights;
use frame_support::{construct_runtime, weights::Weight};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};
pub use primitives::{AuraId, ParaId};
use sp_core::OpaqueMetadata;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

use sp_runtime::{
	generic,
	traits::{BlakeTwo256, Block as BlockT},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub mod constants;
mod implementations;
mod pallets_consensus;
mod pallets_governance;
mod pallets_nodle;
mod pallets_parachain;
mod pallets_system;
mod pallets_util;
mod version;
mod weights;
mod xcm_config;

mod migrations;

pub use pallets_consensus::SessionKeys;
#[cfg(feature = "std")]
pub use version::native_version;
pub use version::VERSION;

construct_runtime! {
	pub enum Runtime {
		// System
		System: frame_system = 0,
		Timestamp: pallet_timestamp = 1,
		Balances: pallet_balances = 2,
		TransactionPayment: pallet_transaction_payment = 3,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 4,

		// Governance
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 10,
		CompanyReserve: pallet_reserve::<Instance1> = 11,
		InternationalReserve: pallet_reserve::<Instance2> = 12,
		UsaReserve: pallet_reserve::<Instance3> = 13,
		Vesting: pallet_grants = 14,
		Mandate: pallet_mandate::{Pallet, Call, Storage, Event<T>} = 15,
		TechnicalCommittee: pallet_collective::<Instance1> = 16,
		TechnicalMembership: pallet_membership::<Instance3> = 17,

		// Consensus
		CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 19,
		Authorship: pallet_authorship = 20,
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 23,
		Aura: pallet_aura::{Pallet, Config<T>, Storage} = 24,
		AuraExt: cumulus_pallet_aura_ext::{Pallet, Config<T>, Storage} = 25,

		// Parachain
		ParachainSystem: cumulus_pallet_parachain_system = 30,
		ParachainInfo: parachain_info = 31,
		CumulusXcm: cumulus_pallet_xcm = 32,
		MessageQueue: pallet_message_queue = 33,
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 34,
		PolkadotXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 35,
		XTokens: orml_xtokens::{Pallet, Call, Storage, Event<T>} = 36,

		// Neat things
		Utility: pallet_utility = 40,
		Multisig: pallet_multisig = 41,
		Uniques: pallet_uniques::{Pallet, Storage, Event<T>} = 42,
		Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>, HoldReason} = 43,
		NodleUniques: pallet_nodle_uniques = 44,
		Sponsorship: pallet_sponsorship = 45,
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 46,
		Proxy: pallet_proxy = 47,

		// Nodle Stack
		// EmergencyShutdown: pallet_emergency_shutdown = 50,
		Allocations: pallet_allocations = 51,
		AllocationsOracles: pallet_membership::<Instance2> = 52,

		// DAO
		DaoReserve: pallet_reserve::<Instance4> = 60,

		// Smart Contracts.
		Contracts: pallet_contracts = 62,
	}
}
#[cfg(feature = "runtime-benchmarks")]
use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	frame_benchmarking::define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[pallet_timestamp, Timestamp]
		[pallet_balances, Balances]
		[pallet_scheduler, Scheduler]
		[pallet_preimage, Preimage]
		[pallet_multisig, Multisig]
		[pallet_reserve, CompanyReserve]
		[pallet_grants, Vesting]
		[pallet_uniques, Uniques]
		[pallet_nodle_uniques, NodleUniques]
		[pallet_message_queue, MessageQueue]
		[pallet_sponsorship, Sponsorship]
		[pallet_proxy, Proxy]
		[pallet_utility, Utility]
		[pallet_allocations, Allocations]
		[pallet_collator_selection, CollatorSelection]
		[pallet_contracts, Contracts]
		[pallet_identity, Identity]
		[pallet_membership, TechnicalMembership]
		[pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
		[pallet_xcm_benchmarks::generic, XcmGenericBenchmarks]
		[pallet_xcm_benchmarks::fungible, XcmFungibleBenchmarks]
		[cumulus_pallet_parachain_system, ParachainSystem]
	);
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	pallet_sponsorship::ChargeSponsor<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
const TEST_ALL_STEPS: bool = cfg!(feature = "try-runtime");

pub type Migrations = (
	pallet_contracts::Migration<Runtime, TEST_ALL_STEPS>,
	cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4<Runtime>,
	pallet_identity::migration::v1::VersionUncheckedMigrateV0ToV1<Runtime, 50>,
	migrations::MultiMigration<Runtime>,
);
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;
#[cfg(feature = "runtime-benchmarks")]
pub type XcmGenericBenchmarks = pallet_xcm_benchmarks::generic::Pallet<Runtime>;
#[cfg(feature = "runtime-benchmarks")]
pub type XcmFungibleBenchmarks = pallet_xcm_benchmarks::fungible::Pallet<Runtime>;

type EventRecord =
	frame_system::EventRecord<<Runtime as frame_system::Config>::RuntimeEvent, <Runtime as frame_system::Config>::Hash>;
sp_api::impl_runtime_apis! {
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			pallets_consensus::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			pallets_consensus::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			version::VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}
		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash, EventRecord>
		for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts::ContractExecResult<Balance,EventRecord> {
			let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
			Contracts::bare_call(
				origin,
				dest,
				value,
				gas_limit,
				storage_deposit_limit,
				input_data,
				constants::CONTRACTS_DEBUG_OUTPUT,
				pallet_contracts::CollectEvents::UnsafeCollect,
				pallet_contracts::Determinism::Enforced,
			)
		}

		fn instantiate(
			origin: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts::ContractInstantiateResult<AccountId, Balance, EventRecord> {
			let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
			Contracts::bare_instantiate(
				origin,
				value,
				gas_limit,
				storage_deposit_limit,
				code,
				data,
				salt,
				constants::CONTRACTS_DEBUG_OUTPUT,
				pallet_contracts::CollectEvents::UnsafeCollect,
			)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
			determinism: pallet_contracts::Determinism,
		) -> pallet_contracts::CodeUploadResult<Hash, Balance> {
			Contracts::bare_upload_code(origin, code, storage_deposit_limit, determinism)
		}

		fn get_storage(
			address: AccountId,
			key: Vec<u8>,
		) -> pallet_contracts::GetStorageResult {
			Contracts::get_storage(address, key)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;

			use frame_system_benchmarking::Pallet as SystemBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmarks!(list, extra);
			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig,
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use cumulus_primitives_core::{Fungibility::Fungible, MultiAsset, MultiLocation, Parent};
			use frame_benchmarking::{Benchmarking, BenchmarkBatch,BenchmarkError};

			use crate::constants::POLKADOT_EXISTENTIAL_DEPOSIT;

			impl pallet_xcm::benchmarking::Config for Runtime {
				fn reachable_dest() -> Option<MultiLocation> {
					Some(Parent.into())
				}

				fn teleportable_asset_and_dest() -> Option<(MultiAsset, MultiLocation)> {
					// Relay/native token can be teleported between People and Relay.
					Some((
						MultiAsset {
							fun: Fungible(POLKADOT_EXISTENTIAL_DEPOSIT),
							id: Parent.into()
						},
						Parent.into(),
					))
				}

				fn reserve_transferable_asset_and_dest() -> Option<(MultiAsset, MultiLocation)> {
					None
				}
			}

			use frame_system_benchmarking::Pallet as SystemBench;

			impl frame_system_benchmarking::Config for Runtime {
				fn setup_set_code_requirements(code: &sp_std::vec::Vec<u8>) -> Result<(), BenchmarkError> {
					ParachainSystem::initialize_for_set_code_benchmark(code.len() as u32);
					Ok(())
				}

				fn verify_set_code() {
					System::assert_last_event(cumulus_pallet_parachain_system::Event::<Runtime>::ValidationFunctionStored.into());
				}
			}


			let whitelist: Vec<sp_storage::TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_support::traits::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			log::debug!("on_runtime_upgrade");
			let weight = Executive::try_runtime_upgrade(checks);
			(weight.unwrap(), constants::RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(block: Block, state_root_check: bool, signature_check: bool, select: frame_support::traits::TryStateSelect) -> Weight {
			log::debug!("Executive::try_execute_block {block:?}-{state_root_check:?}-{select:?}");
			Executive::try_execute_block(block, state_root_check, signature_check,select).expect("execute-block failed")
		}
	}
}

struct CheckInherents;
impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data = cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
			relay_chain_slot,
			sp_std::time::Duration::from_secs(6),
		)
		.create_inherent_data()
		.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block!(
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
);

#[cfg(test)]
mod tests {
	use super::*;
	use frame_system::offchain::CreateSignedTransaction;

	#[test]
	fn validate_transaction_submitter_bounds() {
		fn is_submit_signed_transaction<T>()
		where
			T: CreateSignedTransaction<RuntimeCall>,
		{
		}

		is_submit_signed_transaction::<Runtime>();
	}

	#[test]
	#[ignore = "failing due to preimage depency"]
	fn check_pallet_storage_sizes() {
		use frame_support::traits::StorageInfoTrait;
		let mut storage_info = AllPalletsWithSystem::storage_info();
		println!(
			"| {:^30} | {:^30} | {:^10} | {:^15} |",
			"Pallet", "Storage", "Max Values", "Max Size"
		);
		println!("| {:-<30} | {:-<30} | {:-<10} | {:-<15} |", "", "", "", "");

		storage_info.sort_by_key(|k| k.max_size);
		storage_info.reverse();

		let mut failed = 0;

		for info in storage_info {
			let pallet_name = String::from_utf8(info.pallet_name).unwrap();
			let storage_name = String::from_utf8(info.storage_name).unwrap();
			println!(
				"| {:<30} | {:<30} | {:<10} | {:<15} |",
				pallet_name,
				storage_name,
				format!("{:?}", info.max_values),
				format!("{:?}", info.max_size)
			);

			if let Some(size) = info.max_size {
				// We set the limit for storage size at 4MB
				if size > 4 * 1024 * 1024 {
					failed += 1;
				}
			}
		}

		assert!(failed == 0, "{failed} pallets have too big storage");
	}
}
