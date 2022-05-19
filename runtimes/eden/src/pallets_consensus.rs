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

use crate::{constants, pallets_governance::MoreThanHalfOfTechComm, Aura, Event, Poa, Runtime};
use frame_support::parameter_types;
use primitives::{AccountId, AuraId};
use sp_runtime::{impl_opaque_keys, traits::ConvertInto};
use sp_std::prelude::*;

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ();
}

parameter_types! {
	pub const Period: u32 = 6 * constants::HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type SessionManager = Poa;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type Event = Event;
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ConvertInto;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_poa::Config for Runtime {}

parameter_types! {
	pub const MaxMembers: u32 = 50;
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type Event = Event;
	type AddOrigin = MoreThanHalfOfTechComm;
	type RemoveOrigin = MoreThanHalfOfTechComm;
	type SwapOrigin = MoreThanHalfOfTechComm;
	type ResetOrigin = MoreThanHalfOfTechComm;
	type PrimeOrigin = MoreThanHalfOfTechComm;
	type MembershipInitialized = Poa;
	type MembershipChanged = Poa;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}
