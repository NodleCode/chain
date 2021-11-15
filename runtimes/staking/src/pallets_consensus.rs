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
    constants, pallets_governance::RootCollective, AuthorityDiscovery, Babe, Balances, Call, Event,
    Grandpa, Historical, ImOnline, Offences, Runtime, Session, Staking,
};
use frame_support::{
    parameter_types,
    traits::{KeyOwnerProofSystem, LockIdentifier},
    PalletId,
};
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use primitives::{AccountId, Balance, Moment};
use sp_core::{
    crypto::KeyTypeId,
    u32_trait::{_1, _2},
};
use sp_runtime::{
    impl_opaque_keys, traits::OpaqueKeys, transaction_validity::TransactionPriority, Perbill,
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
    type MaxAuthorities = MaxAuthorities;
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
    type EventHandler = (Staking, ImOnline);
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
    type SessionManager = Staking;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type ShouldEndSession = Babe;
    type Event = Event;
    type Keys = SessionKeys;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Runtime>;
    type NextSessionRotation = Babe;
    type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = pallet_staking::ValidatorSnapshot<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ValidatorSnapshotOf<Runtime>;
}

// TODO::Have to fine tune parameters for practical use-case
parameter_types! {
    // 27 Days
    pub const SlashDeferDuration: sp_staking::SessionIndex = 27 * 6;
    pub const MinSelectedValidators: u32 = 5;
    pub const MaxNominatorsPerValidator: u32 = 25;
    pub const MaxValidatorPerNominator: u32 = 25;
    pub const DefaultValidatorFee: Perbill = Perbill::from_percent(20);
    pub const DefaultSlashRewardProportion: Perbill = Perbill::from_percent(10);
    pub const DefaultSlashRewardFraction: Perbill = Perbill::from_percent(50);
    pub const DefaultStakingMaxValidators: u32 = 50;
    pub const DefaultStakingMinStakeSessionSelection: Balance = 10 * constants::MILLICENTS;
    pub const DefaultStakingMinValidatorBond: Balance = 10 * constants::MILLICENTS;
    pub const DefaultStakingMinNominatorTotalBond: Balance = 10 * constants::MILLICENTS;
    pub const DefaultStakingMinNominationChillThreshold: Balance = 3 * constants::MILLICENTS;
    pub const MaxChunkUnlock: usize = 32;
    pub const StakingPalletId: PalletId = PalletId(*b"mockstak");
    pub const StakingLockId: LockIdentifier = *b"staking ";
}
impl pallet_staking::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BondedDuration = BondingDuration;
    type MinSelectedValidators = MinSelectedValidators;
    type MaxNominatorsPerValidator = MaxNominatorsPerValidator;
    type MaxValidatorPerNominator = MaxValidatorPerNominator;
    type DefaultValidatorFee = DefaultValidatorFee;
    type DefaultSlashRewardProportion = DefaultSlashRewardProportion;
    type DefaultSlashRewardFraction = DefaultSlashRewardFraction;
    type DefaultStakingMaxValidators = DefaultStakingMaxValidators;
    type DefaultStakingMinValidatorBond = DefaultStakingMinValidatorBond;
    type DefaultStakingMinStakeSessionSelection = DefaultStakingMinStakeSessionSelection;
    type DefaultStakingMinNominatorTotalBond = DefaultStakingMinNominatorTotalBond;
    type DefaultStakingMinNominationChillThreshold = DefaultStakingMinNominationChillThreshold;
    type RewardRemainder = ();
    type MaxChunkUnlock = MaxChunkUnlock;
    type PalletId = StakingPalletId;
    type StakingLockId = StakingLockId;
    type Slash = ();
    type SlashDeferDuration = SlashDeferDuration;
    type SessionInterface = Self;
    type ValidatorRegistration = Session;
    type CancelOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, RootCollective>;
    type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}

impl pallet_offences::Config for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
}
