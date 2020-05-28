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

use pallet_grandpa::{fg_primitives, AuthorityId as GrandpaId};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_session::historical as pallet_session_historical;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use parity_scale_codec::Encode;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{
    crypto::KeyTypeId,
    u32_trait::{_1, _2},
    OpaqueMetadata,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        BlakeTwo256, Block as BlockT, ConvertInto, IdentifyAccount, NumberFor, OpaqueKeys,
        SaturatedConversion, Saturating, StaticLookup, Verify,
    },
    transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, ModuleId, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, debug, parameter_types,
    traits::{Currency, Imbalance, KeyOwnerProofSystem, OnUnbalanced, Randomness, SplitTwoWays},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill, Perquintill};

pub mod constants;
mod implementations;

use implementations::{Author, TargetedFeeAdjustment};

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

/// App-specific crypto used for reporting equivocation/misbehavior in BABE and
/// GRANDPA. Any rewards for misbehavior reporting will be paid out to this
/// account.
pub mod report {
    use super::{Signature, Verify};
    use frame_system::offchain::AppCrypto;
    use sp_core::crypto::{key_types, KeyTypeId};

    /// Key type for the reporting module. Used for reporting BABE and GRANDPA
    /// equivocations.
    pub const KEY_TYPE: KeyTypeId = key_types::REPORTING;

    mod app {
        use sp_application_crypto::{app_crypto, sr25519};
        app_crypto!(sr25519, super::KEY_TYPE);
    }

    /// Identity of the equivocation/misbehavior reporter.
    pub type ReporterId = app::Public;

    /// An `AppCrypto` type to allow submitting signed transactions using the reporting
    /// application key as signer.
    pub struct ReporterAppCrypto;

    impl AppCrypto<<Signature as Verify>::Signer, Signature> for ReporterAppCrypto {
        type RuntimeAppPublic = ReporterId;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
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
    spec_version: 24,

    /// Version of the implementation of the specification. Nodes are free to ignore this; it
    /// serves only as an indication that the code is different; as long as the other two versions
    /// are the same then while the actual code may be different, it is nonetheless required to
    /// do the same thing.
    /// Non-consensus-breaking optimizations are about the only changes that could be made which
    /// would result in only the `impl_version` changing.
    impl_version: 0,

    /// Used for hardware wallets. This typically happens when `SignedExtra` changes.
    transaction_version: 2,

    apis: RUNTIME_API_VERSIONS,
};

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
    pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const MaximumExtrinsicWeight: Weight = AvailableBlockRatio::get()
        .saturating_sub(Perbill::from_percent(10)) * MaximumBlockWeight::get();
    pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Trait for Runtime {
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = RocksDbWeight;
    type BlockExecutionWeight = BlockExecutionWeight;
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
}

parameter_types! {
    pub const EpochDuration: u64 = constants::EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: u64 = constants::MILLISECS_PER_BLOCK;
}

impl pallet_babe::Trait for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;

    // session module is the trigger
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
}

impl pallet_grandpa::Trait for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation = pallet_grandpa::EquivocationHandler<
        Self::KeyOwnerIdentification,
        report::ReporterAppCrypto,
        Runtime,
        Offences,
    >;
}

impl pallet_authority_discovery::Trait for Runtime {}

impl pallet_authorship::Trait for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ImOnline;
}

parameter_types! {
    pub const SessionDuration: BlockNumber = constants::EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        public: <Signature as sp_runtime::traits::Verify>::Signer,
        account: AccountId,
        nonce: Index,
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
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
            frame_system::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
            pallet_grandpa::ValidateEquivocationReport::<Runtime>::new(),
        );
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                debug::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        let address = Indices::unlookup(account);
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (address, signature.into(), extra)))
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as sp_runtime::traits::Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = UncheckedExtrinsic;
}

impl pallet_im_online::Trait for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type SessionDuration = SessionDuration;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
}

parameter_types! {
    pub const OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * MaximumBlockWeight::get();
}

impl pallet_offences::Trait for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    // TODO: as of now, we don't execute any slashing, however offences are logged
    // so that we could decide to remove validators later
    type OnOffenceHandler = ();
    type WeightSoftLimit = OffencesWeightSoftLimit;
}

parameter_types! {
    pub const IndexDeposit: Balance = 1 * constants::DOLLARS;
}

impl pallet_indices::Trait for Runtime {
    type AccountIndex = AccountIndex;
    type Currency = Balances;
    type Deposit = IndexDeposit;
    type Event = Event;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 100 * constants::CENTS;
    pub const CreationFee: Balance = 1 * constants::CENTS;
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// Splits fees 20/80 between reserve and block author.
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        if let Some(fees) = fees_then_tips.next() {
            // for fees, 80% to treasury, 20% to author
            let mut split = fees.ration(80, 20);
            if let Some(tips) = fees_then_tips.next() {
                // for tips, if any, 80% to treasury, 20% to author (though this can be anything)
                tips.ration_merge_into(80, 20, &mut split);
            }
            CompanyReserve::on_unbalanced(split.0);
            Author::on_unbalanced(split.1);
        }
    }
}

impl pallet_balances::Trait for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = CompanyReserve;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10 * constants::MILLICENTS;
    // For a sane configuration, this should always be less than `AvailableBlockRatio`.
    // Fees raises after a fullness of 25%
    pub const TargetBlockFullness: Perquintill = constants::TARGET_BLOCK_FULLNESS;
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = DealWithFees;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = TargetedFeeAdjustment<TargetBlockFullness>;
}

parameter_types! {
    pub const MinVestedTransfer: Balance = 1 * constants::DOLLARS;
}

impl pallet_vesting::Trait for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
}

parameter_types! {
    pub const MinimumPeriod: u64 = constants::SLOT_DURATION / 2;
}

impl pallet_timestamp::Trait for Runtime {
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

impl pallet_session::Trait for Runtime {
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

impl pallet_session::historical::Trait for Runtime {
    type FullIdentification = pallet_poa::FullIdentification;
    type FullIdentificationOf = pallet_poa::FullIdentificationOf<Runtime>;
}

impl pallet_membership::Trait<pallet_membership::Instance2> for Runtime {
    type Event = Event;
    type AddOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type MembershipInitialized = PoaSessions;
    type MembershipChanged = PoaSessions;
}

impl pallet_poa::Trait for Runtime {}

// Shared parameters with all collectives / committees
parameter_types! {
    pub const MotionDuration: BlockNumber = 2 * constants::DAYS;
    pub const MaxProposals: u32 = 100;
}

// --- Technical committee

impl pallet_membership::Trait<pallet_membership::Instance1> for Runtime {
    type Event = Event;
    type AddOrigin = pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
}

type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Trait<TechnicalCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
}

// --- Financial committee

impl pallet_membership::Trait<pallet_membership::Instance3> for Runtime {
    type Event = Event;
    type AddOrigin = pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type MembershipInitialized = FinancialCommittee;
    type MembershipChanged = FinancialCommittee;
}

type FinancialCollective = pallet_collective::Instance3;
impl pallet_collective::Trait<FinancialCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
}

// --- Root committee

impl pallet_membership::Trait<pallet_membership::Instance4> for Runtime {
    type Event = Event;
    type AddOrigin = pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type MembershipInitialized = RootCommittee;
    type MembershipChanged = RootCommittee;
}

type RootCollective = pallet_collective::Instance4;
impl pallet_collective::Trait<RootCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
}

impl pallet_mandate::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type ExternalOrigin =
        pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, RootCollective>;
}

parameter_types! {
    pub const MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * MaximumBlockWeight::get();
}

impl pallet_scheduler::Trait for Runtime {
    type Event = Event;
    type Origin = Origin;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
}

parameter_types! {
    pub const AmendmentDelay: BlockNumber = 2 * constants::DAYS;
}

impl pallet_amendments::Trait for Runtime {
    type Event = Event;
    type Amendment = Call;
    type Scheduler = Scheduler;
    type SubmissionOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type VetoOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type Delay = AmendmentDelay;
}

parameter_types! {
    pub const CompanyReserveModuleId: ModuleId = ModuleId(*b"py/resrv");
}

impl pallet_reserve::Trait<pallet_reserve::Instance1> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = CompanyReserveModuleId;
}

parameter_types! {
    pub const InternationalReserveModuleId: ModuleId = ModuleId(*b"py/rvint");
}

impl pallet_reserve::Trait<pallet_reserve::Instance2> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = InternationalReserveModuleId;
}

parameter_types! {
    pub const UsaReserveModuleId: ModuleId = ModuleId(*b"py/rvusa");
}

impl pallet_reserve::Trait<pallet_reserve::Instance3> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = UsaReserveModuleId;
}

parameter_types! {
    pub const BasicDeposit: Balance = 10 * constants::DOLLARS;       // 258 bytes on-chain
    pub const FieldDeposit: Balance = 250 * constants::CENTS;        // 66 bytes on-chain
    pub const SubAccountDeposit: Balance = 2 * constants::DOLLARS;   // 53 bytes on-chain
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Trait for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxAdditionalFields = MaxAdditionalFields;
    type Slashed = CompanyReserve;
    type ForceOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type RegistrarOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type MaxRegistrars = MaxRegistrars;
}

parameter_types! {
    pub const ConfigDepositBase: Balance = 5 * constants::DOLLARS;
    pub const FriendDepositFactor: Balance = 50 * constants::CENTS;
    pub const MaxFriends: u16 = 9;
    pub const RecoveryDeposit: Balance = 5 * constants::DOLLARS;
}

impl pallet_recovery::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Balances;
    type ConfigDepositBase = ConfigDepositBase;
    type FriendDepositFactor = FriendDepositFactor;
    type MaxFriends = MaxFriends;
    type RecoveryDeposit = RecoveryDeposit;
}

parameter_types! {
    // One storage item; value is size 4+4+16+32 bytes = 56 bytes.
    pub const MultisigDepositBase: Balance = 30 * constants::CENTS;
    // Additional storage item size of 32 bytes.
    pub const MultisigDepositFactor: Balance = 5 * constants::CENTS;
    pub const MaxSignatories: u16 = 100;
}

impl pallet_utility::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Balances;
    type MultisigDepositBase = MultisigDepositBase;
    type MultisigDepositFactor = MultisigDepositFactor;
    type MaxSignatories = MaxSignatories;
    type IsCallable = ();
}

parameter_types! {
    // TCR economics
    pub const MinimumApplicationAmount: Balance = 5 * constants::NODL;
    pub const MinimumCounterAmount: Balance = 10 * constants::NODL;
    // Challenging is considerably more expensive as it would lead to the removal of the member
    pub const MinimumChallengeAmount: Balance = 100 * constants::NODL;
    // If you lose you loose 1/3 of your bid
    pub const LoosersSlash: Perbill = Perbill::from_percent(33);

    // TCR ops
    // We use 3 days to account for different time zones and weekends
    pub const FinalizeApplicationPeriod: BlockNumber = 3 * constants::DAYS;
    // 7 days was chosen to provide enough for a complete review but still manageable
    pub const FinalizeChallengePeriod: BlockNumber = 7 * constants::DAYS;
}

impl pallet_tcr::Trait<pallet_tcr::Instance1> for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MinimumApplicationAmount = MinimumApplicationAmount;
    type MinimumCounterAmount = MinimumCounterAmount;
    type MinimumChallengeAmount = MinimumChallengeAmount;
    type LoosersSlash = LoosersSlash;
    type FinalizeApplicationPeriod = FinalizeApplicationPeriod;
    type FinalizeChallengePeriod = FinalizeChallengePeriod;
    type ChangeMembers = ();
}

parameter_types! {
    // Total onboarding cost: 15 NODL + fees (with TCR application)
    pub const SlotBookingCost: Balance = 10 * constants::NODL;
    // Doesn't need to be as expensive
    pub const SlotRenewingCost: Balance = 1 * constants::NODL;
    // One year validity, unless revoked or renewed
    pub const SlotValidity: BlockNumber = 365 * constants::DAYS;
}

impl pallet_root_of_trust::Trait for Runtime {
    type Event = Event;
    type Currency = Balances;
    type CertificateId = AccountId;
    type SlotBookingCost = SlotBookingCost;
    type SlotRenewingCost = SlotRenewingCost;
    type SlotValidity = SlotValidity;
    type FundsCollector = CompanyReserve;
}

impl pallet_emergency_shutdown::Trait for Runtime {
    type Event = Event;
    type ShutdownOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
}

parameter_types! {
    pub const ProtocolFee: Perbill = Perbill::from_percent(20);
    pub const MaximumCoinsEverAllocated: Balance = 1_000_000_000 * constants::NODL;
}

impl pallet_allocations::Trait for Runtime {
    type Event = Event;
    type Currency = Balances;
    type ProtocolFee = ProtocolFee;
    type ProtocolFeeReceiver = CompanyReserve;
    type SourceOfTheCoins = ();
    type MaximumCoinsEverAllocated = MaximumCoinsEverAllocated;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // System
        System: frame_system::{Module, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Indices: pallet_indices::{Module, Call, Storage, Config<T>, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Vesting: pallet_vesting::{Module, Call, Storage, Event<T>, Config<T>},

        // Consensus
        Babe: pallet_babe::{Module, Call, Storage, Config, Inherent(Timestamp)},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        Authorship: pallet_authorship::{Module, Call, Storage},
        ImOnline: pallet_im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
        Offences: pallet_offences::{Module, Call, Storage, Event},
        PoaSessions: pallet_poa::{Module, Storage},
        ValidatorsSet: pallet_membership::<Instance2>::{Module, Call, Storage, Event<T>, Config<T>},
        Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
        Historical: pallet_session_historical::{Module},
        AuthorityDiscovery: pallet_authority_discovery::{Module, Call, Config},

        // Governance
        TechnicalCommittee: pallet_collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
        TechnicalMembership: pallet_membership::<Instance1>::{Module, Call, Storage, Event<T>, Config<T>},
        FinancialCommittee: pallet_collective::<Instance3>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
        FinancialMembership: pallet_membership::<Instance3>::{Module, Call, Storage, Event<T>, Config<T>},
        RootCommittee: pallet_collective::<Instance4>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
        RootMembership: pallet_membership::<Instance4>::{Module, Call, Storage, Event<T>, Config<T>},
        Scheduler: pallet_scheduler::{Module, Call, Storage, Event<T>},
        Amendments: pallet_amendments::{Module, Call, Storage, Event<T>},
        Mandate: pallet_mandate::{Module, Call, Event},
        CompanyReserve: pallet_reserve::<Instance1>::{Module, Call, Storage, Config, Event<T>},
        InternationalReserve: pallet_reserve::<Instance2>::{Module, Call, Storage, Config, Event<T>},
        UsaReserve: pallet_reserve::<Instance3>::{Module, Call, Storage, Config, Event<T>},

        // Neat things
        Identity: pallet_identity::{Module, Call, Storage, Event<T>},
        Recovery: pallet_recovery::{Module, Call, Storage, Event<T>},
        Utility: pallet_utility::{Module, Call, Storage, Event<T>},

        // Nodle Stack
        PkiTcr: pallet_tcr::<Instance1>::{Module, Call, Storage, Event<T>},
        PkiRootOfTrust: pallet_root_of_trust::{Module, Call, Storage, Event<T>},
        EmergencyShutdown: pallet_emergency_shutdown::{Module, Call, Event, Storage},
        Allocations: pallet_allocations::{Module, Call, Event<T>, Storage},
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
    pallet_grandpa::ValidateEquivocationReport<Runtime>,
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
    AllModules,
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

        fn submit_report_equivocation_extrinsic(
            equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Grandpa::submit_report_equivocation_extrinsic(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            use parity_scale_codec::Encode;

            Historical::prove((fg_primitives::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(fg_primitives::OpaqueKeyOwnershipProof::new)
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeGenesisConfiguration {
            // The choice of `c` parameter (where `1 - c` represents the
            // probability of a slot being empty), is done in accordance to the
            // slot duration and expected target block time, for safely
            // resisting network delays of maximum two seconds.
            // <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
            sp_consensus_babe::BabeGenesisConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDuration::get(),
                c: constants::PRIMARY_PROBABILITY,
                genesis_authorities: Babe::authorities(),
                randomness: Babe::randomness(),
                allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
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

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
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

    impl pallet_root_of_trust_runtime_api::RootOfTrustApi<Block, AccountId> for Runtime {
        fn is_root_certificate_valid(cert: &AccountId) -> bool {
            PkiRootOfTrust::is_root_certificate_valid(cert)
        }

        fn is_child_certificate_valid(root: &AccountId, child: &AccountId) -> bool {
            PkiRootOfTrust::is_child_certificate_valid(root, child)
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

            //use frame_system_benchmarking::Module as SystemBench;
            //use pallet_offences_benchmarking::Module as OffencesBench;
            //use pallet_session_benchmarking::Module as SessionBench;

            //impl frame_system_benchmarking::Trait for Runtime {}
            //impl pallet_offences_benchmarking::Trait for Runtime{}
            //impl pallet_session_benchmarking::Trait for Runtime {}

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&pallet, &benchmark, &lowest_range_values, &highest_range_values, &steps, repeat);

            add_benchmark!(params, batches, b"allocations", Allocations);
            add_benchmark!(params, batches, b"amendments", Amendments);
            add_benchmark!(params, batches, b"balances", Balances);
            add_benchmark!(params, batches, b"collective", TechnicalCommittee);
            add_benchmark!(params, batches, b"emergency-shutdown", EmergencyShutdown);
            add_benchmark!(params, batches, b"identity", Identity);
            add_benchmark!(params, batches, b"im-online", ImOnline);
            //add_benchmark!(params, batches, b"offences", OffencesBench);
            add_benchmark!(params, batches, b"reserve", CompanyReserve);
            //add_benchmark!(params, batches, b"session", SessionBench::<Runtime>);
            //add_benchmark!(params, batches, b"system", SystemBench::<Runtime>);
            add_benchmark!(params, batches, b"root-of-trust", PkiRootOfTrust);
            add_benchmark!(params, batches, b"scheduler", Scheduler);
            add_benchmark!(params, batches, b"tcr", PkiTcr);
            add_benchmark!(params, batches, b"timestamp", Timestamp);
            add_benchmark!(params, batches, b"utility", Utility);
            add_benchmark!(params, batches, b"vesting", Vesting);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_system::offchain::CreateSignedTransaction;

    #[test]
    fn validate_transaction_submitter_bounds() {
        fn is_submit_signed_transaction<T>()
        where
            T: CreateSignedTransaction<Call>,
        {
        }

        is_submit_signed_transaction::<Runtime>();
    }
}
