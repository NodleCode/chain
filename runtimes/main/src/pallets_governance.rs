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
    constants, Call, Event, FinancialCommittee, Origin, OriginCaller, RootCommittee, Runtime,
    Scheduler, TechnicalCommittee,
};
use frame_support::parameter_types;
use primitives::{AccountId, BlockNumber};
use sp_core::u32_trait::{_1, _2};
use sp_runtime::ModuleId;
pub use sp_runtime::{Perbill, Perquintill};

// Shared parameters with all collectives / committees
parameter_types! {
    pub const MotionDuration: BlockNumber = 2 * constants::DAYS;
    pub const MaxProposals: u32 = 100;
    pub const MaxMembers: u32 = 50;
}

// --- Technical committee
pub type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
    type WeightInfo = ();
    type MaxMembers = MaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
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

// --- Financial committee
pub type FinancialCollective = pallet_collective::Instance3;
impl pallet_collective::Config<FinancialCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
    type WeightInfo = ();
    type MaxMembers = MaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
}

impl pallet_membership::Config<pallet_membership::Instance3> for Runtime {
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

// --- Root committee
pub type RootCollective = pallet_collective::Instance4;
impl pallet_collective::Config<RootCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
    type WeightInfo = ();
    type MaxMembers = MaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
}

impl pallet_membership::Config<pallet_membership::Instance4> for Runtime {
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

parameter_types! {
    pub const CompanyReserveModuleId: ModuleId = ModuleId(*b"py/resrv"); // 5EYCAe5ijiYfha9GzQDgPVtUCYDY9B8ZgcyiANL2L34crMoR
}

impl pallet_reserve::Config<pallet_reserve::Instance1> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = CompanyReserveModuleId;
    type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const InternationalReserveModuleId: ModuleId = ModuleId(*b"py/rvint"); // 5EYCAe5ijiYfi6GQAEPSHYDwvw4CkyGtPTS52BjLh42GygSv
}

impl pallet_reserve::Config<pallet_reserve::Instance2> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = InternationalReserveModuleId;
    type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const UsaReserveModuleId: ModuleId = ModuleId(*b"py/rvusa"); // 5EYCAe5ijiYfi6MEfWpZC3nJ38KFZ9EQSFpsj9mgYgTtVNri
}

impl pallet_reserve::Config<pallet_reserve::Instance3> for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Module<Runtime>;
    type ExternalOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type Call = Call;
    type ModuleId = UsaReserveModuleId;
    type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

impl pallet_mandate::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type ExternalOrigin =
        pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, RootCollective>;
}

parameter_types! {
    pub const AmendmentDelay: BlockNumber = 2 * constants::DAYS;
}

impl pallet_amendments::Config for Runtime {
    type Event = Event;
    type Amendment = Call;
    type Scheduler = Scheduler;
    type SubmissionOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type VetoOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type Delay = AmendmentDelay;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_amendments::weights::SubstrateWeight<Runtime>;
}
