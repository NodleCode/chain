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

use crate::{
	constants, pallets_governance::EnsureRootOrMoreThanHalfOfTechComm, Aura, Balances, CollatorSelection, Runtime,
	RuntimeEvent, Session,
};
use frame_support::{
	parameter_types,
	traits::{ConstBool, ConstU32, ConstU64},
	PalletId,
};
use primitives::{AccountId, AuraId};
use sp_runtime::impl_opaque_keys;
use sp_std::prelude::*;

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const Period: u32 = 6 * constants::HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type SessionManager = CollatorSelection;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
	type SlotDuration = ConstU64<{ constants::SLOT_DURATION }>;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 100;
	pub const MinEligibleCollators: u32 = 3;
	pub const MaxInvulnerables: u32 = 20;
}

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = EnsureRootOrMoreThanHalfOfTechComm;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	type KickThreshold = Period;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = crate::weights::pallet_collator_selection::WeightInfo<Runtime>;
	type MinEligibleCollators = MinEligibleCollators;
}
