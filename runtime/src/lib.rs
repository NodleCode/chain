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

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use frame_support::{construct_runtime, traits::Randomness};
use nodle_chain_primitives::{AccountId, Balance, BlockNumber, CertificateId, Index, Signature};
use pallet_transaction_payment::FeeDetails;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use sp_core::OpaqueMetadata;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
    generic, impl_opaque_keys,
    traits::{BlakeTwo256, Block as BlockT, StaticLookup},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub mod constants;
mod implementations;
mod pallets_cumulus;
mod pallets_governance;
mod pallets_nodle;
mod pallets_system;
mod pallets_util;
mod pallets_xtoken;
mod version;

#[cfg(feature = "std")]
pub use version::native_version;
pub use version::VERSION;

impl_opaque_keys! {
    pub struct SessionKeys {}
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = nodle_chain_primitives::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // System
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Indices: pallet_indices::{Pallet, Call, Storage, Config<T>, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Call, Storage},

        // Governance
        TechnicalCommittee: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
        TechnicalMembership: pallet_membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>},
        FinancialCommittee: pallet_collective::<Instance3>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
        FinancialMembership: pallet_membership::<Instance3>::{Pallet, Call, Storage, Event<T>, Config<T>},
        RootCommittee: pallet_collective::<Instance4>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
        RootMembership: pallet_membership::<Instance4>::{Pallet, Call, Storage, Event<T>, Config<T>},
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>},
        Amendments: pallet_amendments::{Pallet, Call, Storage, Event<T>},
        Mandate: pallet_mandate::{Pallet, Call, Event},
        CompanyReserve: pallet_reserve::<Instance1>::{Pallet, Call, Storage, Config<T>, Event<T>},
        InternationalReserve: pallet_reserve::<Instance2>::{Pallet, Call, Storage, Config<T>, Event<T>},
        UsaReserve: pallet_reserve::<Instance3>::{Pallet, Call, Storage, Config<T>, Event<T>},
        Grants: pallet_grants::{Pallet, Call, Storage, Config<T>, Event<T>},

        // Neat things
        Identity: pallet_identity::{Pallet, Call, Storage, Event<T>},
        Recovery: pallet_recovery::{Pallet, Call, Storage, Event<T>},
        Utility: pallet_utility::{Pallet, Call, Event},
        Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>},
        Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>},
        // Contracts: pallet_contracts::{Pallet, Call, Config<T>, Storage, Event<T>},

        // Cumulus parachain
        ParachainInfo: parachain_info::{Pallet, Storage, Config},
        ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Storage, Inherent, Event},
        XcmHandler: cumulus_pallet_xcm_handler::{Pallet, Call, Event<T>, Origin},

        // Cross tokens support
        Currencies: orml_currencies::{Pallet, Call, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Call, Event<T>, Config<T>},
        UnknownTokens: orml_unknown_tokens::{Pallet, Storage, Event},
        XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>},

        // Nodle Stack
        PkiTcr: pallet_tcr::<Instance1>::{Pallet, Call, Storage, Event<T>},
        PkiRootOfTrust: pallet_root_of_trust::{Pallet, Call, Storage, Event<T>},
        EmergencyShutdown: pallet_emergency_shutdown::{Pallet, Call, Event<T>, Storage},
        Allocations: pallet_allocations::{Pallet, Call, Event<T>, Storage},
        AllocationsOracles: pallet_membership::<Instance5>::{Pallet, Call, Storage, Event<T>, Config<T>},
    }
);

/// The address format for describing accounts.
pub type Address = <Indices as StaticLookup>::Source;
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
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
>;

sp_api::impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
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
            Runtime::metadata().into()
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

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed().0
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(source: TransactionSource, tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    #[cfg(feature = "enable-contracts")]
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
    for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult {
            Contracts::bare_call(origin, dest, value, gas_limit, input_data)
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            address: AccountId,
        ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
            Contracts::rent_projection(address)
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
    }

    impl pallet_root_of_trust_runtime_api::RootOfTrustApi<Block, CertificateId> for Runtime {
        fn is_root_certificate_valid(cert: &CertificateId) -> bool {
            PkiRootOfTrust::is_root_certificate_valid(cert)
        }

        fn is_child_certificate_valid(root: &CertificateId, child: &CertificateId) -> bool {
            PkiRootOfTrust::is_child_certificate_valid(root, child)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            // We did not include the offences and sessions benchmarks as they are parity
            // specific and were causing some issues at compile time as they depend on the
            // presence of the staking and elections pallets.

            use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey, add_benchmark};
            use frame_system_benchmarking::Module as SystemBench;

            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![];
            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_allocations, Allocations);
            add_benchmark!(params, batches, pallet_amendments, Amendments);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_collective, TechnicalCommittee);
            add_benchmark!(params, batches, pallet_contracts, Contracts);
            add_benchmark!(params, batches, pallet_emergency_shutdown, EmergencyShutdown);
            add_benchmark!(params, batches, pallet_grants, Grants);
            add_benchmark!(params, batches, pallet_identity, Identity);
            add_benchmark!(params, batches, pallet_indices, Indices);
            add_benchmark!(params, batches, pallet_multisig, Multisig);
            add_benchmark!(params, batches, pallet_proxy, Proxy);
            add_benchmark!(params, batches, pallet_reserve, CompanyReserve);
            add_benchmark!(params, batches, pallet_root_of_trust, PkiRootOfTrust);
            add_benchmark!(params, batches, pallet_scheduler, Scheduler);
            add_benchmark!(params, batches, pallet_tcr, PkiTcr);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_utility, Utility);

            // TODO: add benchs for ORML

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

cumulus_pallet_parachain_system::register_validate_block!(Runtime, Executive);
