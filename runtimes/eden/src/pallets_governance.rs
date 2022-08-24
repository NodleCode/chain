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

use crate::{constants, Call, Event, Origin, Runtime, TechnicalCommittee};
use frame_support::{parameter_types, traits::EitherOfDiverse, PalletId};
use frame_system::{EnsureNever, EnsureRoot};
use primitives::{AccountId, BlockNumber};
pub use sp_runtime::{Perbill, Perquintill};

parameter_types! {
	pub const CompanyReservePalletId: PalletId = PalletId(*b"py/resrv"); // 5EYCAe5ijiYfha9GzQDgPVtUCYDY9B8ZgcyiANL2L34crMoR
}

impl pallet_reserve::Config<pallet_reserve::Instance1> for Runtime {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Runtime>;
	type ExternalOrigin = MoreThanHalfOfTechComm;
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
	type ExternalOrigin = MoreThanHalfOfTechComm;
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
	type ExternalOrigin = MoreThanHalfOfTechComm;
	type Call = Call;
	type PalletId = UsaReservePalletId;
	type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const DaoReservePalletId: PalletId = PalletId(*b"py/nddao"); // 5EYCAe5ijiYfABcws2T5dgN35iWYaWwvh8wPgbZaBKRRpMzV
}

impl pallet_reserve::Config<pallet_reserve::Instance4> for Runtime {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Runtime>;
	// as of now nobody can spend this, later, we need to map this to the
	// correct governance origin.
	type ExternalOrigin = EnsureNever<AccountId>;
	type Call = Call;
	type PalletId = DaoReservePalletId;
	type WeightInfo = pallet_reserve::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MotionDuration: BlockNumber = 2 * constants::DAYS;
	pub const MaxProposals: u32 = 100;
	pub const MaxMembers: u32 = 50;
}

pub type MoreThanHalfOfTechComm =
	pallet_collective::EnsureProportionMoreThan<AccountId, pallet_collective::Instance1, 1, 2>;
pub type EnsureRootOrMoreThanHalfOfTechComm = EitherOfDiverse<EnsureRoot<AccountId>, MoreThanHalfOfTechComm>;
impl pallet_collective::Config<pallet_collective::Instance1> for Runtime {
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
	type AddOrigin = MoreThanHalfOfTechComm;
	type RemoveOrigin = MoreThanHalfOfTechComm;
	type SwapOrigin = MoreThanHalfOfTechComm;
	type ResetOrigin = MoreThanHalfOfTechComm;
	type PrimeOrigin = MoreThanHalfOfTechComm;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

impl pallet_mandate::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type ExternalOrigin = MoreThanHalfOfTechComm;
}
