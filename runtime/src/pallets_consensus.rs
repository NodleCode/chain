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
    constants, pallets_governance::TechnicalCollective, AuthorityDiscovery, Babe, Call, Event,
    Grandpa, Historical, ImOnline, Offences, PoaSessions, Runtime,
};
use frame_support::{parameter_types, traits::KeyOwnerProofSystem, weights::Weight};
use nodle_chain_primitives::{AccountId, BlockNumber, Moment};
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::{
    crypto::KeyTypeId,
    u32_trait::{_1, _2},
};
use sp_runtime::{
    impl_opaque_keys,
    traits::{ConvertInto, OpaqueKeys},
    transaction_validity::TransactionPriority,
    Perbill,
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
    pub const SessionsPerEra: sp_staking::SessionIndex = 6;
    pub const BondingDuration: pallet_staking::EraIndex = 24 * 28;
    //pub const SlashDeferDuration: pallet_staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
    //pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
    //pub const MaxNominatorRewardedPerValidator: u32 = 256;
}

parameter_types! {
    pub const EpochDuration: u64 = constants::EPOCH_DURATION_IN_SLOTS;
    pub const ExpectedBlockTime: Moment = constants::MILLISECS_PER_BLOCK;
    pub const ReportLongevity: u64 =
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type KeyOwnerProofSystem = Historical;
    type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::IdentificationTuple;
    type HandleEquivocation =
        pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
    type WeightInfo = ();
}

impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProofSystem = Historical;
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type HandleEquivocation = pallet_grandpa::EquivocationHandler<
        Self::KeyOwnerIdentification,
        Offences,
        ReportLongevity,
    >;
    type WeightInfo = ();
}

impl pallet_authority_discovery::Config for Runtime {}

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
    pub const SessionDuration: BlockNumber = constants::EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl pallet_im_online::Config for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type ValidatorSet = Historical;
    type SessionDuration = SessionDuration;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
    type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    // When this percentage is reached the module will force a new era, we never
    // call `session.disable()` so this should never be used.
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl pallet_session::Config for Runtime {
    type SessionManager = PoaSessions;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type ShouldEndSession = Babe;
    type Event = Event;
    type Keys = SessionKeys;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ConvertInto;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
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
    type MembershipInitialized = PoaSessions;
    type MembershipChanged = PoaSessions;
}

parameter_types! {
    pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) *
        constants::RuntimeBlockWeights::get().max_block;
}

impl pallet_offences::Config for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = ();
    type WeightSoftLimit = OffencesWeightSoftLimit;
}
