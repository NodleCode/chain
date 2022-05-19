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
	constants, pallets_governance::MaxMembers, pallets_governance::TechnicalCollective, AuthorityDiscovery, Babe, Call,
	Event, Grandpa, Historical, ImOnline, Offences, Poa, Runtime, Session,
};
use frame_support::{parameter_types, traits::KeyOwnerProofSystem};
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use primitives::{AccountId, Moment};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	impl_opaque_keys,
	traits::{ConvertInto, OpaqueKeys},
	transaction_validity::TransactionPriority,
};
use sp_std::prelude::*;

impl_opaque_keys! {
	pub struct SessionKeys {
		pub babe: Babe,
		pub grandpa: Grandpa,
		pub im_online: ImOnline,
		pub authority_discovery: AuthorityDiscovery,
	}
}

// Normally used with the staking pallet
parameter_types! {
	// 28 Days for unbonding
	pub const BondingDuration: sp_staking::SessionIndex = 28 * 6;
}

parameter_types! {
	pub const EpochDuration: u64 = constants::EPOCH_DURATION_IN_SLOTS;
	pub const ExpectedBlockTime: Moment = constants::MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	type DisabledValidators = Session;

	type KeyOwnerProofSystem = Historical;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::Proof;
	type KeyOwnerIdentification =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::IdentificationTuple;
	type HandleEquivocation = pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type KeyOwnerProofSystem = Historical;
	type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type KeyOwnerIdentification =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;
	type HandleEquivocation =
		pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const UncleGenerations: u32 = 5;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ImOnline;
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	pub const MaxAuthorities: u32 = 100;
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type Event = Event;
	type ValidatorSet = Historical;
	type NextSessionRotation = Babe;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

impl pallet_session::Config for Runtime {
	type SessionManager = Poa;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type ShouldEndSession = Babe;
	type Event = Event;
	type Keys = SessionKeys;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ConvertInto;
	type NextSessionRotation = Babe;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_poa::FullIdentification;
	type FullIdentificationOf = pallet_poa::FullIdentificationOf<Runtime>;
}

impl pallet_poa::Config for Runtime {}

impl pallet_membership::Config<pallet_membership::Instance2> for Runtime {
	type Event = Event;
	type AddOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type RemoveOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type SwapOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type ResetOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type PrimeOrigin = pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>;
	type MembershipInitialized = Poa;
	type MembershipChanged = Poa;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

impl pallet_offences::Config for Runtime {
	type Event = Event;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = ();
}
