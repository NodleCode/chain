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
use frame_support::{parameter_types, PalletId, traits::ConstBool};
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

parameter_types! {
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
	#[doc = " Whether to allow block authors to create multiple blocks per slot."]
	#[doc = ""]
	#[doc = " If this is `true`, the pallet will allow slots to stay the same across sequential"]
	#[doc = " blocks. If this is `false`, the pallet will require that subsequent blocks always have"]
	#[doc = " higher slots than previous ones."]
	#[doc = ""]
	#[doc = " Regardless of the setting of this storage value, the pallet will always enforce the"]
	#[doc = " invariant that slots don\\'t move backwards as the chain progresses."]
	#[doc = ""]
	#[doc = " The typical value for this should be \\'false\\' unless this pallet is being augmented by"]
	#[doc = " another pallet which enforces some limitation on the number of blocks authors can create"]
	#[doc = " using the same slot."]
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const MinEligibleCollators: u32 = 3;
	pub const MaxInvulnerables: u32 = 50;
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
	#[doc = " Minimum number eligible collators. Should always be greater than zero. This includes"]
	#[doc = " Invulnerable collators. This ensures that there will always be one collator who can"]
	#[doc = " produce a block."]
	type MinEligibleCollators = MinEligibleCollators;
}
