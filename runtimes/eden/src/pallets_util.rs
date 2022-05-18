/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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
use crate::{
    constants, implementations::RelayChainBlockNumberProvider,
    pallets_governance::MoreThanHalfOfTechComm, Balances, Call, Event, Origin, OriginCaller,
    Preimage, Runtime,
};
use frame_support::{
    parameter_types,
    traits::{AsEnsureOriginWithArg, EqualPrivilegeOnly},
    weights::Weight,
};
use frame_system::{EnsureRoot, EnsureSigned};

use primitives::{AccountId, Balance};
use sp_runtime::Perbill;

impl pallet_grants::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type CancelOrigin = MoreThanHalfOfTechComm;
    type ForceOrigin = MoreThanHalfOfTechComm;
    type WeightInfo = pallet_grants::weights::SubstrateWeight<Runtime>;
    type BlockNumberProvider = RelayChainBlockNumberProvider<Runtime>;
}

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = ();
}

parameter_types! {
    // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
    pub const DepositBase: Balance = constants::deposit(1, 88);
    // Additional storage item size of 32 bytes.
    pub const DepositFactor: Balance = constants::deposit(0, 32);
    pub const MaxSignatories: u16 = 100;
}
impl pallet_multisig::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        constants::RuntimeBlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
    pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
    type PreimageProvider = Preimage;
    type NoPreimagePostponement = NoPreimagePostponement;
}

parameter_types! {
    pub const PreimageMaxSize: u32 = 4096 * 1024;
    pub const PreimageBaseDeposit: Balance = constants::deposit(2, 64);
    pub const PreimageByteDeposit: Balance = constants::deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = pallet_preimage::weights::SubstrateWeight<Runtime>;
    type Event = Event;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type MaxSize = PreimageMaxSize;
    type BaseDeposit = PreimageBaseDeposit;
    type ByteDeposit = PreimageByteDeposit;
}

parameter_types! {
    pub const ClassDeposit: Balance = 10 * constants::NODL;
    pub const InstanceDeposit: Balance = 100 * constants::MILLI_NODL;
    pub const MetadataDepositBase: Balance = 10 * constants::NODL;
    pub const MetadataDepositPerByte: Balance = 1 * constants::NODL;
    pub const KeyLimit: u32 = 32;
    pub const ValueLimit: u32 = 256;
    pub const StringLimit: u32 = 50;
}

impl pallet_uniques::Config for Runtime {
    type Event = Event;
    type ClassId = u32;
    type InstanceId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type ClassDeposit = ClassDeposit;
    type InstanceDeposit = InstanceDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type AttributeDepositBase = MetadataDepositBase;
    type DepositPerByte = MetadataDepositPerByte;
    type StringLimit = StringLimit;
    type KeyLimit = KeyLimit;
    type ValueLimit = ValueLimit;
    type WeightInfo = pallet_uniques::weights::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = ();
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
}
