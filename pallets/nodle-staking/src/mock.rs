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

#![cfg(test)]

use crate as nodle_staking;
use crate::*;
use frame_support::{
    assert_ok, parameter_types,
    traits::{
        Currency, ExistenceRequirement, FindAuthor, Get, OnFinalize, OnInitialize,
        OneSessionHandler, OnUnbalanced, Imbalance,
    },
    weights::{constants::RocksDbWeight, Weight},
    IterableStorageMap, StorageDoubleMap, StorageMap, StorageValue,
};
use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::{Header, TestXt, UintAuthorityId},
    traits::{IdentityLookup, Zero},
    ModuleId, Perbill,
};
use sp_staking::{
	SessionIndex,
	offence::{OffenceDetails, OnOffenceHandler}
};
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

    fn on_disabled(validator_index: usize) {
        SESSION.with(|d| {
            let mut d = d.borrow_mut();
            let value = d.0[validator_index];
            d.1.insert(value);
        })
    }
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
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        NodleStaking: nodle_staking::{Module, Call, Config<T>, Storage, Event<T>},
        Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
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
    // pub static SlashDeferDuration: EraIndex = 0;
    pub static ElectionLookahead: BlockNumber = 0;
    pub static Period: BlockNumber = 5;
    pub static Offset: BlockNumber = 0;
    pub static MaxIterations: u32 = 0;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
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
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
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
impl pallet_session::Config for Test {
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, NodleStaking>;
    type Keys = SessionKeys;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionHandler = (OtherSessionHandler,);
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = nodle_staking::StashOf<Test>;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type WeightInfo = ();
}
impl pallet_session::historical::Config for Test {
    type FullIdentification = nodle_staking::ValidatorSnapshot<AccountId, Balance>;
    type FullIdentificationOf = nodle_staking::ValidatorSnapshotOf<Test>;
}
impl pallet_authorship::Config for Test {
    type FindAuthor = Author11;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = Module<Test>;
}
parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}
parameter_types! {
    pub const MinBlocksPerRound: u32 = 3;
    pub const DefaultBlocksPerRound: u32 = 5;
    pub const BondDuration: u32 = 2;
    pub const MinSelectedValidators: u32 = 5;
    pub const MaxNominatorsPerValidator: u32 = 4;
    pub const MaxValidatorPerNominator: u32 = 4;
    pub const DefaultValidatorCommission: Perbill = Perbill::from_percent(20);
    pub const MinValidatorStake: u128 = 10;
    pub const MinNominatorStake: u128 = 5;
    pub const MinNomination: u128 = 3;
	pub const StakingPalletId: ModuleId = ModuleId(*b"mockstak");
}
impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type MinBlocksPerRound = MinBlocksPerRound;
    type DefaultBlocksPerRound = DefaultBlocksPerRound;
    type BondDuration = BondDuration;
    type MinSelectedValidators = MinSelectedValidators;
    type MaxNominatorsPerValidator = MaxNominatorsPerValidator;
    type MaxValidatorPerNominator = MaxValidatorPerNominator;
    type DefaultValidatorCommission = DefaultValidatorCommission;
    type MinValidatorStake = MinValidatorStake;
    type MinValidatorPoolStake = MinValidatorStake;
    type MinNominatorStake = MinNominatorStake;
    type MinNomination = MinNomination;
	type RewardRemainder = RewardRemainderMock;
	type PalletId = StakingPalletId;
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
    pub fn invulnerables(mut self, invulnerables: Vec<AccountId>) -> Self {
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

    pub fn build(self) -> sp_io::TestExternalities {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut storage)
        .expect("Pallet balances storage can be assimilated");

        let mut stakers: Vec<(AccountId, Option<AccountId>, Balance)> = Vec::new();
        for validator in self.validators {
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

        let mut ext = sp_io::TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
        let mut ext = self.build();
        ext.execute_with(test);
    }
}

pub(crate) fn roll_to(n: u64) {
    while System::block_number() < n {
        NodleStaking::on_finalize(System::block_number());
        Balances::on_finalize(System::block_number());
        Timestamp::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Timestamp::on_initialize(System::block_number());
        Balances::on_initialize(System::block_number());
        NodleStaking::on_initialize(System::block_number());
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
            if let Event::nodle_staking(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

// Same storage changes as EventHandler::note_author impl
pub(crate) fn set_author(round: u32, acc: u64, pts: u32) {
    <Points<Test>>::mutate(round, |p| *p += pts);
    <AwardedPts<Test>>::mutate(round, acc, |p| *p += pts);
}

pub(crate) fn mint_rewards(amount: Balance) {
    let imbalance = <pallet_balances::Module<Test> as Currency<AccountId>>::issue(amount);
    NodleStaking::on_unbalanced(imbalance);

	println!(
		"last event {:#?}",
		mock::last_event()
	);
}

pub(crate) fn active_round() -> SessionIndex {
    NodleStaking::round().current
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

/// Progress until the given Session.
pub(crate) fn start_active_session(session_index: SessionIndex) {
    assert_eq!(active_round(), session_index);
	start_session(session_index);
	println!(
		"last event {:#?}",
		mock::last_event()
	);
    assert_eq!(active_round(), session_index + 1);
}


pub(crate) fn bond_validator(ctrl: AccountId, val: Balance) {
    let _ = Balances::make_free_balance_be(&ctrl, val);
    assert_ok!(NodleStaking::join_validator_pool(
        Origin::signed(ctrl),
        val,
    ));

	// println!(
	// 	"last event {:#?}",
	// 	mock::last_event()
	// );
}

pub(crate) fn bond_nominator(
    ctrl: AccountId,
    val: Balance,
    target: AccountId,
) {
    let _ = Balances::make_free_balance_be(&ctrl, val);
    assert_ok!(NodleStaking::nominate(
        Origin::signed(ctrl),
		target,
        val,
    ));
}
