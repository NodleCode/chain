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

use crate::{
	constants, Call, Event, FinancialCommittee, Origin, OriginCaller, RootCommittee, Runtime, Scheduler,
	TechnicalCommittee,
};
use frame_support::{parameter_types, PalletId};
use primitives::{AccountId, BlockNumber};
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
	type AddOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type RemoveOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type SwapOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type ResetOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type PrimeOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
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
	type AddOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type RemoveOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type SwapOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type ResetOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type PrimeOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type MembershipInitialized = FinancialCommittee;
	type MembershipChanged = FinancialCommittee;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
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
	type AddOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type RemoveOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type SwapOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type ResetOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type PrimeOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type MembershipInitialized = RootCommittee;
	type MembershipChanged = RootCommittee;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const CompanyReservePalletId: PalletId = PalletId(*b"py/resrv"); // 5EYCAe5ijiYfha9GzQDgPVtUCYDY9B8ZgcyiANL2L34crMoR
}

impl pallet_reserve::Config<pallet_reserve::Instance1> for Runtime {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Runtime>;
	type ExternalOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, FinancialCollective, 1, 2>;
	type Call = Call;
	type PalletId = CompanyReservePalletId;
	type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const InternationalReservePalletId: PalletId = PalletId(*b"py/rvint"); // 5EYCAe5ijiYfi6GQAEPSHYDwvw4CkyGtPTS52BjLh42GygSv
}

impl pallet_reserve::Config<pallet_reserve::Instance2> for Runtime {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Runtime>;
	type ExternalOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, FinancialCollective, 1, 2>;
	type Call = Call;
	type PalletId = InternationalReservePalletId;
	type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const UsaReservePalletId: PalletId = PalletId(*b"py/rvusa"); // 5EYCAe5ijiYfi6MEfWpZC3nJ38KFZ9EQSFpsj9mgYgTtVNri
}

impl pallet_reserve::Config<pallet_reserve::Instance3> for Runtime {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Runtime>;
	type ExternalOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, FinancialCollective, 1, 2>;
	type Call = Call;
	type PalletId = UsaReservePalletId;
	type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

impl pallet_mandate::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type ExternalOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, RootCollective, 1, 2>;
}

parameter_types! {
	pub const AmendmentDelay: BlockNumber = 2 * constants::DAYS;
}

impl pallet_amendments::Config for Runtime {
	type Event = Event;
	type Amendment = Call;
	type Scheduler = Scheduler;
	type SubmissionOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type VetoOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, RootCollective, 1, 2>;
	type Delay = AmendmentDelay;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = pallet_amendments::weights::SubstrateWeight<Runtime>;
}
