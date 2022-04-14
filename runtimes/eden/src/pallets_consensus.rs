/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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
    constants, pallets_governance::MoreThanHalfOfTechComm, Aura, Balances, Event, Historical,
    ImOnline, Offences, Runtime, Session, Staking,
};
use frame_support::{parameter_types, traits::LockIdentifier, PalletId};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use primitives::{AccountId, AuraId, Balance};
use sp_runtime::{
    impl_opaque_keys, traits::OpaqueKeys, transaction_validity::TransactionPriority, Perbill,
};
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
    type EventHandler = (Staking, ImOnline);
}

parameter_types! {
    pub const Period: u32 = constants::EPOCH_DURATION_IN_BLOCKS;
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type SessionManager = Staking;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type Event = Event;
    type Keys = SessionKeys;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Runtime>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = pallet_staking::ValidatorSnapshot<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ValidatorSnapshotOf<Runtime>;
}

// Normally used with the staking pallet
parameter_types! {
    // 7 Days for unbonding
    pub const BondingDuration: sp_staking::SessionIndex = 7 * 6;
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
    type CancelOrigin = MoreThanHalfOfTechComm;
    type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}

impl pallet_offences::Config for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
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

parameter_types! {
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

impl pallet_im_online::Config for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type ValidatorSet = Historical;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
    type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
    type MaxKeys = MaxKeys;
    type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
    type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}
