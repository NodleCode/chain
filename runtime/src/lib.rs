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

use grandpa::{fg_primitives, AuthorityId as GrandpaId};
use im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::u32_trait::{_1, _2, _4};
use sp_core::OpaqueMetadata;
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        BlakeTwo256, Block as BlockT, ConvertInto, IdentifyAccount, OpaqueKeys,
        SaturatedConversion, StaticLookup, Verify,
    },
    transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use system::offchain::TransactionSubmitter;

// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use frame_support::{
    construct_runtime, debug, parameter_types,
    traits::{Currency, Randomness, SplitTwoWays},
    weights::Weight,
    StorageValue,
};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
pub use timestamp::Call as TimestampCall;

pub mod constants;
mod implementations;

use implementations::{TargetedFeeAdjustment, ToAuthor, WeightToFee};

type NegativeImbalance<T> =
    <balances::Module<T> as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
            pub im_online: ImOnline,
            pub authority_discovery: AuthorityDiscovery,
        }
    }
}

/// This runtime version.
/// This should not be thought of as classic Semver (major/minor/tiny).
/// This triplet have different semantics and mis-interpretation could cause problems.
/// In particular: bug fixes should result in an increment of `spec_version` and possibly `authoring_version`,
/// absolutely not `impl_version` since they change the semantics of the runtime.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("nodle-chain"),
    impl_name: create_runtime_str!("nodle-chain"),

    /// `authoring_version` is the version of the authorship interface. An authoring node
    /// will not attempt to author blocks unless this is equal to its native runtime.
    authoring_version: 1,

    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    spec_version: 8,

    /// Version of the implementation of the specification. Nodes are free to ignore this; it
    /// serves only as an indication that the code is different; as long as the other two versions
    /// are the same then while the actual code may be different, it is nonetheless required to
    /// do the same thing.
    /// Non-consensus-breaking optimizations are about the only changes that could be made which
    /// would result in only the `impl_version` changing.
    impl_version: 0,

    apis: RUNTIME_API_VERSIONS,
};

/// A transaction submitter with the given key type.
pub type TransactionSubmitterOf<KeyType> =
    TransactionSubmitter<KeyType, Runtime, UncheckedExtrinsic>;

/// Submits transaction with the node's public and signature type. Adheres to the signed extension
/// format of the chain.
impl system::offchain::CreateTransaction<Runtime, UncheckedExtrinsic> for Runtime {
    type Public = <Signature as sp_runtime::traits::Verify>::Signer;
    type Signature = Signature;

    fn create_transaction<TSigner: system::offchain::Signer<Self::Public, Self::Signature>>(
        call: Call,
        public: Self::Public,
        account: AccountId,
        index: Index,
    ) -> Option<(
        Call,
        <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
    )> {
        // take the biggest period possible.
        let period = BlockHashCount::get()
            .checked_next_power_of_two()
            .map(|c| c / 2)
            .unwrap_or(2) as u64;
        let current_block = System::block_number()
            .saturated_into::<u64>()
            // The `System::block_number` is initialized with `n+1`,
            // so the actual block number is `n`.
            .saturating_sub(1);
        let tip = 0;
        let extra: SignedExtra = (
            system::CheckVersion::<Runtime>::new(),
            system::CheckGenesis::<Runtime>::new(),
            system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
            system::CheckNonce::<Runtime>::from(index),
            system::CheckWeight::<Runtime>::new(),
            transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
        );
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                debug::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = TSigner::sign(public, &raw_payload)?;
        let address = Indices::unlookup(account);
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (address, signature, extra)))
    }
}

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
    pub const MaximumBlockWeight: Weight = 1_000_000_000;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const Version: RuntimeVersion = VERSION;
}

impl system::Trait for Runtime {
    type AccountId = AccountId;
    type Call = Call;
    type Lookup = Indices;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type Origin = Origin;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = Version;
    type ModuleToIndex = ModuleToIndex;
    type AccountData = balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
}

parameter_types! {
    pub const EpochDuration: u64 = constants::EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: u64 = constants::MILLISECS_PER_BLOCK;
}

impl babe::Trait for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;

    // session module is the trigger
    type EpochChangeTrigger = babe::ExternalTrigger;
}

impl grandpa::Trait for Runtime {
    type Event = Event;
}

impl authority_discovery::Trait for Runtime {}

// TODO: substrate#2986 implement this properly
impl authorship::Trait for Runtime {
    type FindAuthor = session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ImOnline;
}

parameter_types! {
    pub const SessionDuration: BlockNumber = constants::EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl im_online::Trait for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type Call = Call;
    type SubmitTransaction = TransactionSubmitterOf<ImOnlineId>;
    type SessionDuration = SessionDuration;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
}

impl offences::Trait for Runtime {
    type Event = Event;
    type IdentificationTuple = session::historical::IdentificationTuple<Self>;
    // TODO: as of now, we don't execute any slashing, however offences are logged
    // so that we could decide to remove validators later
    type OnOffenceHandler = ();
}

parameter_types! {
    pub const IndexDeposit: Balance = 1 * constants::DOLLARS;
}

impl indices::Trait for Runtime {
    type AccountIndex = AccountIndex;
    type Currency = Balances;
    type Deposit = IndexDeposit;
    type Event = Event;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 100 * constants::CENTS;
    pub const CreationFee: Balance = 1 * constants::CENTS;
}

/// Splits fees 20/80 between reserve and block author.
pub type DealWithFees = SplitTwoWays<
    Balance,
    NegativeImbalance<Runtime>,
    _1,
    CompanyReserve, // 1/5 to the company reserve
    _4,
    ToAuthor<Runtime>, // 4/5 to the block author
>;

impl balances::Trait for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = CompanyReserve;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
}

parameter_types! {
    pub const TransactionBaseFee: Balance = 1 * constants::CENTS;
    pub const TransactionByteFee: Balance = 10 * constants::MILLICENTS;
    // For a sane configuration, this should always be less than `AvailableBlockRatio`.
    // Fees raises after a fullness of 25%
    pub const TargetBlockFullness: Perbill = constants::TARGET_BLOCK_FULLNESS;
}

impl transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = DealWithFees;
    type TransactionBaseFee = TransactionBaseFee;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = WeightToFee;
    type FeeMultiplierUpdate = TargetedFeeAdjustment<TargetBlockFullness>;
}

parameter_types! {
    pub const MinVestedTransfer: Balance = 1 * constants::DOLLARS;
}

impl vesting::Trait for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
}

parameter_types! {
    pub const MinimumPeriod: u64 = constants::SLOT_DURATION / 2;
}

impl timestamp::Trait for Runtime {
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
}

parameter_types! {
    pub const UncleGenerations: u32 = 0;
}

parameter_types! {
    // When this percentage is reached the module will force a new era, we never
    // call `session.disable()` so this should never be used.
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}

impl session::Trait for Runtime {
    type SessionManager = PoaSessions;
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type ShouldEndSession = Babe;
    type Event = Event;
    type Keys = opaque::SessionKeys;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ConvertInto;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type NextSessionRotation = Babe;
}

impl session::historical::Trait for Runtime {
    type FullIdentification = poa::FullIdentification;
    type FullIdentificationOf = poa::FullIdentificationOf<Runtime>;
}

impl membership::Trait<membership::Instance2> for Runtime {
    type Event = Event;
    type AddOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type RemoveOrigin =
        collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type SwapOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type ResetOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type PrimeOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type MembershipInitialized = PoaSessions;
    type MembershipChanged = PoaSessions;
}

impl poa::Trait for Runtime {}

impl membership::Trait<membership::Instance1> for Runtime {
    type Event = Event;
    type AddOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type RemoveOrigin =
        collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type SwapOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type ResetOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type PrimeOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
}

parameter_types! {
    pub const MotionDuration: BlockNumber = 2 * constants::DAYS;
}

type TechnicalCollective = collective::Instance2;
impl collective::Trait<TechnicalCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
}

impl mandate::Trait for Runtime {
    type Event = Event;
    type Call = Call;

    // A majority of the committee can dispatch root calls
    type ExternalOrigin =
        collective::EnsureProportionAtLeast<_1, _2, AccountId, TechnicalCollective>;
}

impl reserve::Trait for Runtime {
    type Event = Event;
    type Currency = balances::Module<Runtime>;
    type ExternalOrigin =
        collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
}

parameter_types! {
    // One storage item; value is size 4+4+16+32 bytes = 56 bytes.
    pub const MultisigDepositBase: Balance = 30 * constants::CENTS;
    // Additional storage item size of 32 bytes.
    pub const MultisigDepositFactor: Balance = 5 * constants::CENTS;
    pub const MaxSignatories: u16 = 100;
}

impl utility::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Balances;
    type MultisigDepositBase = MultisigDepositBase;
    type MultisigDepositFactor = MultisigDepositFactor;
    type MaxSignatories = MaxSignatories;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // System
        System: system::{Module, Call, Storage, Config, Event<T>},
        Timestamp: timestamp::{Module, Call, Storage, Inherent},
        Indices: indices::{Module, Call, Storage, Config<T>, Event<T>},
        Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: transaction_payment::{Module, Storage},
        RandomnessCollectiveFlip: randomness_collective_flip::{Module, Call, Storage},
        Vesting: vesting::{Module, Call, Storage, Event<T>, Config<T>},

        // Consensus
        Babe: babe::{Module, Call, Storage, Config, Inherent(Timestamp)},
        Grandpa: grandpa::{Module, Call, Storage, Config, Event},
        Authorship: authorship::{Module, Call, Storage},
        ImOnline: im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
        Offences: offences::{Module, Call, Storage, Event},
        PoaSessions: poa::{Module, Storage},
        ValidatorsSet: membership::<Instance2>::{Module, Call, Storage, Event<T>, Config<T>},
        Session: session::{Module, Call, Storage, Event, Config<T>},
        AuthorityDiscovery: authority_discovery::{Module, Call, Config},

        // Governance
        TechnicalCommittee: collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
        TechnicalMembership: membership::<Instance1>::{Module, Call, Storage, Event<T>, Config<T>},
        Mandate: mandate::{Module, Call, Event},
        CompanyReserve: reserve::{Module, Call, Storage, Config, Event<T>},

        // Neat things
        Utility: utility::{Module, Call, Storage, Event<T>},
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
    system::CheckVersion<Runtime>,
    system::CheckGenesis<Runtime>,
    system::CheckEra<Runtime>,
    system::CheckNonce<Runtime>,
    system::CheckWeight<Runtime>,
    transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive =
    frame_executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllModules>;

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
            RandomnessCollectiveFlip::random_seed()
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

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> Vec<(GrandpaId, u64)> {
            Grandpa::grandpa_authorities()
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeConfiguration {
            // The choice of `c` parameter (where `1 - c` represents the
            // probability of a slot being empty), is done in accordance to the
            // slot duration and expected target block time, for safely
            // resisting network delays of maximum two seconds.
            // <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
            sp_consensus_babe::BabeConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDuration::get(),
                c: constants::PRIMARY_PROBABILITY,
                genesis_authorities: Babe::authorities(),
                randomness: Babe::randomness(),
                secondary_slots: true,
            }
        }

        fn current_epoch_start() -> sp_consensus_babe::SlotNumber {
            Babe::current_epoch_start()
        }
    }

    impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
        fn authorities() -> Vec<AuthorityDiscoveryId> {
            AuthorityDiscovery::authorities()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        Balance,
        UncheckedExtrinsic,
    > for Runtime {
        fn query_info(uxt: UncheckedExtrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            pallet: Vec<u8>,
            benchmark: Vec<u8>,
            lowest_range_values: Vec<u32>,
            highest_range_values: Vec<u32>,
            steps: Vec<u32>,
            repeat: u32,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            // We did not include the offences and sessions benchmarks as they are parity
            // specific and were causing some issues at compile time.

            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark};

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&pallet, &benchmark, &lowest_range_values, &highest_range_values, &steps, repeat);

            add_benchmark!(params, batches, b"balances", Balances);
            add_benchmark!(params, batches, b"collective", TechnicalCommittee);
            add_benchmark!(params, batches, b"im-online", ImOnline);
            add_benchmark!(params, batches, b"timestamp", Timestamp);
            add_benchmark!(params, batches, b"utility", Utility);
            add_benchmark!(params, batches, b"vesting", Vesting);

            add_benchmark!(params, batches, b"reserve", CompanyReserve);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use system::offchain::{SignAndSubmitTransaction, SubmitSignedTransaction};

    #[test]
    fn validate_transaction_submitter_bounds() {
        fn is_submit_signed_transaction<T>()
        where
            T: SubmitSignedTransaction<Runtime, Call>,
        {
        }

        fn is_sign_and_submit_transaction<T>()
        where
            T: SignAndSubmitTransaction<
                Runtime,
                Call,
                Extrinsic = UncheckedExtrinsic,
                CreateTransaction = Runtime,
                Signer = ImOnlineId,
            >,
        {
        }

        is_submit_signed_transaction::<TransactionSubmitterOf<ImOnlineId>>();
        is_sign_and_submit_transaction::<TransactionSubmitterOf<ImOnlineId>>();
    }
}
