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

use crate::constants::deposit;
use crate::{
	constants, implementations::RelayChainBlockNumberProvider, pallets_governance::MoreThanHalfOfTechComm, Balances,
	OriginCaller, Preimage, RandomnessCollectiveFlip, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, Timestamp,
};
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstBool, ConstU32, EqualPrivilegeOnly, Nothing},
	weights::Weight,
};
use frame_system::{EnsureRoot, EnsureSigned};
use pallet_contracts::{Frame, Schedule};

use primitives::{AccountId, Balance};
use sp_runtime::Perbill;

parameter_types! {
	pub const MaxSchedule: u32 = 100;
}

impl pallet_grants::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CancelOrigin = MoreThanHalfOfTechComm;
	type MaxSchedule = MaxSchedule;
	type WeightInfo = pallet_grants::weights::SubstrateWeight<Runtime>;
	type BlockNumberProvider = RelayChainBlockNumberProvider<Runtime>;
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
}

#[allow(clippy::identity_op)]
impl pallet_preimage::Config for Runtime {
	type WeightInfo = crate::weights::pallet_preimage::WeightInfo<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
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

impl pallet_nodle_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_nodle_uniques::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const DepositPerItem: Balance = constants::deposit(1, 0);
	pub const DepositPerByte: Balance = constants::deposit(0, 1);
	pub const DefaultDepositLimit: Balance = deposit(1024, 1024 * 1024);
	pub MySchedule: Schedule<Runtime> = Default::default();
}

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
	// TODO check 	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type WeightInfo = crate::weights::pallet_contracts::WeightInfo<Runtime>;
	type ChainExtension = ();

	type Schedule = MySchedule;

	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = ConstBool<false>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
}
