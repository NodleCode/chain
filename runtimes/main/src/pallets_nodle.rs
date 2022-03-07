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

#![cfg_attr(not(feature = "std"), no_std)]

use crate::{
    pallets_governance::{MaxMembers, RootCollective, TechnicalCollective},
    Allocations, Balances, CompanyReserve, Event, Runtime,
};

use frame_support::parameter_types;
use primitives::{AccountId, Balance};
use sp_core::u32_trait::{_1, _2};
use sp_runtime::Perbill;

impl pallet_emergency_shutdown::Config for Runtime {
    type Event = Event;
    type ShutdownOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type WeightInfo = pallet_emergency_shutdown::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const ProtocolFee: Perbill = Perbill::from_percent(20);
    pub const MaximumCoinsEverAllocated: Balance = 1_259_995_654_473_120_000_000;
}

impl pallet_allocations::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type ProtocolFee = ProtocolFee;
    type ProtocolFeeReceiver = CompanyReserve;
    type MaximumCoinsEverAllocated = MaximumCoinsEverAllocated;
    type ExistentialDeposit = <Runtime as pallet_balances::Config>::ExistentialDeposit;
    type WeightInfo = pallet_allocations::weights::SubstrateWeight<Runtime>;
}

impl pallet_membership::Config<pallet_membership::Instance5> for Runtime {
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
    type MembershipInitialized = Allocations;
    type MembershipChanged = Allocations;
    type MaxMembers = MaxMembers;
    type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}
