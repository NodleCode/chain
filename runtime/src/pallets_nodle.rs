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

use crate::{
    constants,
    pallets_governance::{RootCollective, TechnicalCollective},
    Allocations, Balances, CompanyReserve, Event, PkiRootOfTrust, Runtime,
};

use frame_support::parameter_types;
use nodle_chain_primitives::{AccountId, Balance, BlockNumber, CertificateId};
use sp_core::u32_trait::{_1, _2};
use sp_runtime::Perbill;

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

impl pallet_tcr::Config<pallet_tcr::Instance1> for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MinimumApplicationAmount = MinimumApplicationAmount;
    type MinimumCounterAmount = MinimumCounterAmount;
    type MinimumChallengeAmount = MinimumChallengeAmount;
    type LoosersSlash = LoosersSlash;
    type FinalizeApplicationPeriod = FinalizeApplicationPeriod;
    type FinalizeChallengePeriod = FinalizeChallengePeriod;
    type ChangeMembers = PkiRootOfTrust;
    type WeightInfo = pallet_tcr::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    // Total onboarding cost: 10 NODL + fees (with TCR application)
    pub const SlotBookingCost: Balance = 10 * constants::NODL;
    // Doesn't need to be as expensive
    pub const SlotRenewingCost: Balance = 1 * constants::NODL;
    // One year validity, unless revoked or renewed
    pub const SlotValidity: BlockNumber = 365 * constants::DAYS;
}

impl pallet_root_of_trust::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type CertificateId = CertificateId;
    type SlotBookingCost = SlotBookingCost;
    type SlotRenewingCost = SlotRenewingCost;
    type SlotValidity = SlotValidity;
    type FundsCollector = CompanyReserve;
    type WeightInfo = pallet_root_of_trust::weights::SubstrateWeight<Runtime>;
}

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
}
