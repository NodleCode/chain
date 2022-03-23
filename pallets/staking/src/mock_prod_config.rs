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
use super::*;
use crate as nodle_staking;
use crate::hooks;
use frame_support::{
    ord_parameter_types, parameter_types,
    traits::{
        Currency, FindAuthor, Imbalance, LockIdentifier, OnFinalize, OnInitialize, OnUnbalanced,
        OneSessionHandler,
    },
    weights::constants::RocksDbWeight,
    PalletId,
};
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::{Header, UintAuthorityId},
    traits::{IdentityLookup, Zero},
    Perbill,
};
use sp_staking::SessionIndex;

use std::{cell::RefCell, collections::HashSet};

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1000;

/// The AccountId alias in this test module.
pub(crate) type AccountId = u64;
pub(crate) type AccountIndex = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;

thread_local! {
    static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
}

/// Another session handler struct to test on_disabled.
pub struct OtherSessionHandler;
impl OneSessionHandler<AccountId> for OtherSessionHandler {
    type Key = UintAuthorityId;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
        AccountId: 'a,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, validators: I, _: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
        AccountId: 'a,
    {
        SESSION.with(|x| {
            *x.borrow_mut() = (validators.map(|x| x.0.clone()).collect(), HashSet::new())
        });
    }

    fn on_disabled(validator_index: u32) {
        SESSION.with(|d| {
            let mut d = d.borrow_mut();
            let value = d.0[validator_index as usize];
            d.1.insert(value);
        })
    }
}

pub(crate) mod constants {
    use super::*;
    pub const NODL: Balance = 1_000_000_000_000;
    pub const MEGA: Balance = 10i32.pow(6) as Balance;
    pub const KILO: Balance = 10i32.pow(3) as Balance;
}

impl sp_runtime::BoundToRuntimeAppPublic for OtherSessionHandler {
    type Public = UintAuthorityId;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        NodleStaking: nodle_staking::{Pallet, Call, Config<T>, Storage, Event<T>},
        Poa: pallet_poa::{Pallet, Storage},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        Historical: pallet_session::historical::{Pallet, Storage},
        CompanyReserve: pallet_reserve::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);

/// Author of block is always 11
pub struct Author11;
impl FindAuthor<AccountId> for Author11 {
    fn find_author<'a, I>(_digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
    {
        Some(11)
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const MaxLocks: u32 = 1024;
    // pub static SessionsPerEra: SessionIndex = 3;
    pub static ExistentialDeposit: Balance = 1;
    pub static SlashDeferDuration: SessionIndex = 0;
    pub static BondedDuration: u32 = 2;
    pub static ElectionLookahead: BlockNumber = 0;
    pub static Period: BlockNumber = 5;
    pub static Offset: BlockNumber = 0;
    pub static MaxIterations: u32 = 0;
}
impl frame_system::Config for Test {
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = RocksDbWeight;
    type Origin = Origin;
    type Index = AccountIndex;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = frame_support::traits::Everything;
    type OnSetCode = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}
parameter_types! {
    pub const UncleGenerations: u64 = 0;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(25);
}

sp_runtime::impl_opaque_keys! {
    pub struct SessionKeys {
        pub other: OtherSessionHandler,
    }
}

impl From<UintAuthorityId> for SessionKeys {
    fn from(other: UintAuthorityId) -> Self {
        Self { other }
    }
}

impl pallet_session::Config for Test {
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, NodleStaking>;
    type Keys = SessionKeys;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionHandler = (OtherSessionHandler,);
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = hooks::StashOf<Test>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type WeightInfo = ();
}
impl pallet_session::historical::Config for Test {
    type FullIdentification = crate::types::ValidatorSnapshot<AccountId, Balance>;
    type FullIdentificationOf = crate::types::ValidatorSnapshotOf<Test>;
}
impl pallet_authorship::Config for Test {
    type FindAuthor = Author11;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = Pallet<Test>;
}

impl pallet_poa::Config for Test {}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const ExternalOrigin: AccountId = 42;
}
parameter_types! {
    // pub const ExternalOrigin: AccountId = 42;
    pub const CompanyReservePalletId: PalletId = PalletId(*b"py/resrv");
}

impl pallet_reserve::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type ExternalOrigin = EnsureSignedBy<ExternalOrigin, AccountId>;
    type Call = Call;
    type PalletId = CompanyReservePalletId;
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const CancelOrigin: AccountId = 42;
}
parameter_types! {
    pub const MinSelectedValidators: u32 = 10;
    pub const MaxNominatorsPerValidator: u32 = 100;
    pub const MaxValidatorPerNominator: u32 = 10;
    pub const DefaultValidatorFee: Perbill = Perbill::from_percent(5);
    pub const DefaultSlashRewardProportion: Perbill = Perbill::from_percent(10);
    pub const DefaultSlashRewardFraction: Perbill = Perbill::from_percent(50);
    pub const DefaultStakingMaxValidators: u32 = 20;
    pub const DefaultStakingMinStakeSessionSelection: Balance = 3 * constants::MEGA * constants::NODL;
    pub const DefaultStakingMinValidatorBond: Balance = 100 * constants::KILO * constants::NODL;
    pub const DefaultStakingMinNominatorTotalBond: Balance = 1 * constants::NODL;
    pub const DefaultStakingMinNominationChillThreshold: Balance = 1 * constants::NODL;
    pub const MaxChunkUnlock: usize = 256;
    pub const StakingPalletId: PalletId = PalletId(*b"mockstak");
    pub const StakingLockId: LockIdentifier = *b"staking ";
}
impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BondedDuration = BondedDuration;
    type MinSelectedValidators = MinSelectedValidators;
    type MaxNominatorsPerValidator = MaxNominatorsPerValidator;
    type MaxValidatorPerNominator = MaxValidatorPerNominator;
    type DefaultValidatorFee = DefaultValidatorFee;
    type DefaultSlashRewardProportion = DefaultSlashRewardProportion;
    type DefaultSlashRewardFraction = DefaultSlashRewardFraction;
    type DefaultStakingMaxValidators = DefaultStakingMaxValidators;
    type DefaultStakingMinStakeSessionSelection = DefaultStakingMinStakeSessionSelection;
    type DefaultStakingMinValidatorBond = DefaultStakingMinValidatorBond;
    type DefaultStakingMinNominatorTotalBond = DefaultStakingMinNominatorTotalBond;
    type DefaultStakingMinNominationChillThreshold = DefaultStakingMinNominationChillThreshold;
    type RewardRemainder = RewardRemainderMock;
    type MaxChunkUnlock = MaxChunkUnlock;
    type PalletId = StakingPalletId;
    type StakingLockId = StakingLockId;
    type Slash = CompanyReserve;
    type SlashDeferDuration = SlashDeferDuration;
    type SessionInterface = Self;
    type ValidatorRegistration = Session;
    type CancelOrigin = EnsureSignedBy<CancelOrigin, AccountId>;
    type WeightInfo = ();
}

thread_local! {
    pub static REWARD_REMAINDER_UNBALANCED: RefCell<u128> = RefCell::new(0);
}

pub struct RewardRemainderMock;

impl OnUnbalanced<NegativeImbalanceOf<Test>> for RewardRemainderMock {
    fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<Test>) {
        REWARD_REMAINDER_UNBALANCED.with(|v| {
            *v.borrow_mut() += amount.peek();
        });
        drop(amount);
    }
}

pub struct ExtBuilder {
    invulnerables: Vec<AccountId>,
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [validator, amount]
    validators: Vec<(AccountId, Balance)>,
    // [nominator, validator, nomination_amount]
    nominators: Vec<(AccountId, AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            invulnerables: vec![],
            balances: vec![],
            nominators: vec![],
            validators: vec![],
        }
    }
}

impl ExtBuilder {
    pub fn with_invulnerables(mut self, invulnerables: Vec<AccountId>) -> Self {
        self.invulnerables = invulnerables;
        self
    }
    pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }
    pub(crate) fn with_validators(mut self, validators: Vec<(AccountId, Balance)>) -> Self {
        self.validators = validators;
        self
    }
    pub(crate) fn with_nominators(
        mut self,
        nominators: Vec<(AccountId, AccountId, Balance)>,
    ) -> Self {
        self.nominators = nominators;
        self
    }
    pub(crate) fn tst_staking_build(self) -> sp_io::TestExternalities {
        sp_tracing::try_init_simple();
        // frame_support::debug::RuntimeLogger::init();

        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut storage)
        .expect("Pallet balances storage can be assimilated");

        let mut stakers: Vec<(AccountId, Option<AccountId>, Balance)> = Vec::new();
        for validator in &self.validators {
            stakers.push((validator.0, None, validator.1));
        }
        for nominator in self.nominators {
            stakers.push((nominator.0, Some(nominator.1), nominator.2));
        }
        let _ = nodle_staking::GenesisConfig::<Test> {
            stakers: stakers,
            invulnerables: self.invulnerables,
            ..Default::default()
        }
        .assimilate_storage(&mut storage);

        let _ = pallet_session::GenesisConfig::<Test> {
            keys: self
                .validators
                .iter()
                .map(|x| {
                    (
                        x.0,
                        x.0,
                        SessionKeys {
                            other: UintAuthorityId(x.0 as u64),
                        },
                    )
                })
                .collect(),
        }
        .assimilate_storage(&mut storage);

        let mut ext = sp_io::TestExternalities::new(storage);
        ext.execute_with(|| {
            let validators = Session::validators();
            SESSION.with(|x| *x.borrow_mut() = (validators.clone(), HashSet::new()));
        });
        ext.execute_with(|| {
            System::set_block_number(1);
            Session::on_initialize(1);
            NodleStaking::on_initialize(1);
            Timestamp::set_timestamp(INIT_TIMESTAMP);
        });
        ext
    }
}

pub(crate) fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

pub(crate) fn events() -> Vec<pallet::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::NodleStaking(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

// Same storage changes as EventHandler::note_author impl
pub(crate) fn set_author(session: u32, acc: u64, pts: u32) {
    <Points<Test>>::mutate(session, |p| *p += pts);
    <AwardedPts<Test>>::mutate(session, acc, |p| *p += pts);
}

pub(crate) fn mint_rewards(amount: Balance) {
    let imbalance = <pallet_balances::Pallet<Test> as Currency<AccountId>>::issue(amount);
    NodleStaking::on_unbalanced(imbalance);
    log::trace!(
        "mint_rewards:[{:#?}]=> - {:#?}",
        line!(),
        mock::last_event()
    );
}

/// Progress to the given block, triggering session and era changes as we progress.
///
/// This will finalize the previous block, initialize up to the given block, essentially simulating
/// a block import/propose process where we first initialize the block, then execute some stuff (not
/// in the function), and then finalize the block.
pub(crate) fn run_to_block(n: BlockNumber) {
    NodleStaking::on_finalize(System::block_number());
    for b in (System::block_number() + 1)..=n {
        System::set_block_number(b);
        Session::on_initialize(b);
        NodleStaking::on_initialize(b);
        Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        if b != n {
            NodleStaking::on_finalize(System::block_number());
        }
    }
}

/// Progresses from the current block number (whatever that may be) to the `P * session_index + 1`.
pub(crate) fn start_session(session_index: SessionIndex) {
    let end: u64 = if Offset::get().is_zero() {
        (session_index as u64) * Period::get()
    } else {
        Offset::get() + (session_index.saturating_sub(1) as u64) * Period::get()
    };
    run_to_block(end);
    // session must have progressed properly.
    assert_eq!(
        Session::current_index(),
        session_index,
        "current session index = {}, expected = {}",
        Session::current_index(),
        session_index,
    );
}
