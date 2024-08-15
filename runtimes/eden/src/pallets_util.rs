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
#![allow(clippy::identity_op)]

use crate::{
	constants, constants::deposit, constants::DAYS, pallets_governance::MoreThanHalfOfTechComm, Balances, DaoReserve,
	OriginCaller, Preimage, RandomnessCollectiveFlip, Runtime, RuntimeCall, RuntimeEvent, RuntimeHoldReason,
	RuntimeOrigin, Signature, Timestamp,
};
use cumulus_pallet_parachain_system::RelaychainDataProvider;
use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, RuntimeDebug},
	parameter_types,
	traits::{fungible::HoldConsideration, LinearStoragePrice},
	traits::{AsEnsureOriginWithArg, ConstBool, ConstU32, EqualPrivilegeOnly, InstanceFilter, Nothing},
	weights::Weight,
};
use frame_system::{EnsureRoot, EnsureSigned};
use pallet_contracts::{Frame, Schedule};
use pallet_identity::legacy::IdentityInfo;
use polkadot_primitives::BlakeTwo256;
use primitives::{AccountId, Balance};
use sp_runtime::{traits::Verify, Perbill};

parameter_types! {
	pub const MaxSchedule: u32 = 100;
}

impl pallet_grants::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CancelOrigin = MoreThanHalfOfTechComm;
	type MaxSchedule = MaxSchedule;
	type WeightInfo = crate::weights::pallet_grants::WeightInfo<Runtime>;
	type BlockNumberProvider = RelaychainDataProvider<Runtime>;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = crate::weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = constants::deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = constants::deposit(0, 32);
	pub const MaxSignatories: u16 = 100;
}
impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = crate::weights::pallet_multisig::WeightInfo<Runtime>;
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		constants::RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type WeightInfo = crate::weights::pallet_scheduler::WeightInfo<Runtime>;
	type Preimages = Preimage;
}

parameter_types! {
	pub const PreimageBaseDeposit: Balance = constants::deposit(2, 64);
	pub const PreimageByteDeposit: Balance = constants::deposit(0, 1);
	pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

#[allow(clippy::identity_op)]
impl pallet_preimage::Config for Runtime {
	type WeightInfo = crate::weights::pallet_preimage::WeightInfo<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
	>;
}

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * constants::NODL;
	pub const ItemDeposit: Balance = 1 * constants::NODL;
	pub const MetadataDepositBase: Balance = 100 * constants::MILLI_NODL;
	pub const MetadataDepositPerByte: Balance = 10 * constants::MILLI_NODL;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const StringLimit: u32 = 128;
}

impl pallet_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = crate::weights::pallet_uniques::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}

#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum SponsorshipType {
	AnySafe,
	Uniques,
}
impl InstanceFilter<RuntimeCall> for SponsorshipType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			SponsorshipType::AnySafe => !matches!(c, RuntimeCall::Utility { .. }),
			SponsorshipType::Uniques => matches!(c, RuntimeCall::NodleUniques { .. }),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		(self == &SponsorshipType::AnySafe) || (self == o)
	}
}
impl Default for SponsorshipType {
	fn default() -> Self {
		Self::AnySafe
	}
}

parameter_types! {
	pub const PotDeposit: Balance = 1000 * constants::NODL;
	pub const UserDeposit: Balance = constants::NODL / 3;
}
impl pallet_sponsorship::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type PotId = u32;
	type SponsorshipType = SponsorshipType;
	type PotDeposit = PotDeposit;
	type UserDeposit = UserDeposit;
	type WeightInfo = crate::weights::pallet_sponsorship::WeightInfo<Runtime>;
}

impl pallet_nodle_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = crate::weights::pallet_nodle_uniques::WeightInfo<Runtime>;
}

parameter_types! {
	pub const DepositPerItem: Balance = constants::deposit(1, 0);
	pub const DepositPerByte: Balance = constants::deposit(0, 1);
	pub const DefaultDepositLimit: Balance = constants::deposit(1024, 1024 * 1024);
	pub MySchedule: Schedule<Runtime> = Default::default();
	pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(30);
}

type ContractsMigrations<T> = (
	pallet_contracts::migration::v13::Migration<T>,
	pallet_contracts::migration::v14::Migration<T, Balances>,
	pallet_contracts::migration::v15::Migration<T>,
	pallet_contracts::migration::v16::Migration<T>,
);

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type DefaultDepositLimit = DefaultDepositLimit;
	type CallStack = [Frame<Self>; 5];
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = crate::weights::pallet_contracts::WeightInfo<Runtime>;
	type ChainExtension = ();

	type Schedule = MySchedule;

	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = ConstBool<false>;
	type UploadOrigin = EnsureSigned<Self::AccountId>;
	type InstantiateOrigin = EnsureSigned<Self::AccountId>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type Migrations = ContractsMigrations<Runtime>;
	type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
	type MaxDelegateDependencies = ConstU32<32>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Debug = ();

	type Environment = ();
	type Xcm = pallet_xcm::Pallet<Self>;
	type ApiVersion = ();
}

parameter_types! {
	pub const BasicDeposit: Balance = 1000 * constants::NODL;       // 258 bytes on-chain
	pub const FieldDeposit: Balance = 200 * constants::NODL;        // 66 bytes on-chain
	pub const SubAccountDeposit: Balance = 200 * constants::NODL;   // 53 bytes on-chain
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
	pub const ByteDeposit: Balance = constants::deposit(0, 1);
}
impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = DaoReserve;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type RegistrarOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = crate::weights::pallet_identity::WeightInfo<Runtime>;
	type IdentityInformation = IdentityInfo<MaxAdditionalFields>;
	type ByteDeposit = ByteDeposit;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
	type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
	type MaxSuffixLength = ConstU32<7>;
	type MaxUsernameLength = ConstU32<32>;
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances(..) | RuntimeCall::NodleUniques(..) // Do we need this??? RuntimeCall::Vesting(pallet_grants::Call::vested_transfer { .. }) |
			),
			ProxyType::Governance => matches!(c, RuntimeCall::TechnicalCommittee(..)),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = crate::weights::pallet_proxy::WeightInfo<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}
