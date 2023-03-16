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
use super::*;
use crate as nodle_staking;
use crate::hooks;
use frame_support::{
	assert_ok, ord_parameter_types, parameter_types,
	traits::{
		ConstU32, Currency, FindAuthor, Imbalance, LockIdentifier, OnFinalize, OnInitialize, OnUnbalanced,
		OneSessionHandler,
	},
	weights::constants::RocksDbWeight,
	PalletId,
};
use frame_system::EnsureSignedBy;
use sp_core::H256;

use sp_runtime::{
	testing::{Header, UintAuthorityId},
	traits::{IdentityLookup, Zero},
	Perbill,
};
use sp_staking::{
	offence::{DisableStrategy, OffenceDetails, OnOffenceHandler},
	SessionIndex,
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
		SESSION.with(|x| *x.borrow_mut() = (validators.map(|x| *x.0).collect(), HashSet::new()));
	}

	fn on_disabled(validator_index: u32) {
		SESSION.with(|d| {
			let mut d = d.borrow_mut();
			let value = d.0[validator_index as usize];
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
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		NodleStaking: nodle_staking::{Pallet, Call, Config<T>, Storage, Event<T>},
		ValidatorsSet: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
		Historical: pallet_session::historical::{Pallet, Storage},
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
			frame_support::weights::Weight::from_ref_time(frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2))
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
	type RuntimeOrigin = RuntimeOrigin;
	type Index = AccountIndex;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
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
	pub const Admin: u64 = 4;
}
impl pallet_membership::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureSignedBy<Admin, u64>;
	type RemoveOrigin = EnsureSignedBy<Admin, u64>;
	type SwapOrigin = EnsureSignedBy<Admin, u64>;
	type ResetOrigin = EnsureSignedBy<Admin, u64>;
	type PrimeOrigin = EnsureSignedBy<Admin, u64>;
	type MembershipInitialized = ();
	type MembershipChanged = ();
	type MaxMembers = ConstU32<10>;
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const CancelOrigin: AccountId = 42;
}
parameter_types! {
	pub const MinSelectedValidators: u32 = 5;
	pub const MaxNominatorsPerValidator: u32 = 4;
	pub const MaxValidatorPerNominator: u32 = 4;
	pub const DefaultValidatorFee: Perbill = Perbill::from_percent(20);
	pub const DefaultSlashRewardProportion: Perbill = Perbill::from_percent(10);
	pub const DefaultSlashRewardFraction: Perbill = Perbill::from_percent(50);
	pub const DefaultStakingMaxValidators: u32 = 50;
	pub const DefaultStakingMinStakeSessionSelection: Balance = 10;
	pub const DefaultStakingMinValidatorBond: Balance = 10;
	pub const DefaultStakingMinNominatorTotalBond: Balance = 5;
	pub const DefaultStakingMinNominationChillThreshold: Balance = 3;
	pub const MaxChunkUnlock: usize = 32;
	pub const StakingPalletId: PalletId = PalletId(*b"mockstak");
	pub const StakingLockId: LockIdentifier = *b"staking ";
}
impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
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
	type Slash = ();
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
	validator_pool: bool,
	validator_count: u32,
	fair: bool,
	num_validators: Option<u32>,
	nominate: bool,
	has_stakers: bool,
	initialize_first_session: bool,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			invulnerables: vec![],
			balances: vec![],
			nominators: vec![],
			validators: vec![],
			validator_pool: false,
			validator_count: 2,
			fair: true,
			num_validators: None,
			nominate: true,
			has_stakers: true,
			initialize_first_session: true,
		}
	}
}

impl ExtBuilder {
	pub fn existential_deposit(self, existential_deposit: Balance) -> Self {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = existential_deposit);
		self
	}
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
	pub(crate) fn with_nominators(mut self, nominators: Vec<(AccountId, AccountId, Balance)>) -> Self {
		self.nominators = nominators;
		self
	}
	pub fn num_validators(mut self, num_validators: u32) -> Self {
		self.num_validators = Some(num_validators);
		self
	}
	pub(crate) fn has_stakers(mut self, has: bool) -> Self {
		self.has_stakers = has;
		self
	}
	pub fn slash_defer_duration(self, session_idx: SessionIndex) -> Self {
		SLASH_DEFER_DURATION.with(|v| *v.borrow_mut() = session_idx);
		self
	}
	pub fn bonded_duration(self, session_idx: SessionIndex) -> Self {
		BONDED_DURATION.with(|v| *v.borrow_mut() = session_idx);
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
			stakers,
			invulnerables: self.invulnerables,
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
							other: UintAuthorityId(x.0),
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
	#[allow(clippy::needless_update)]
	pub fn build(self) -> sp_io::TestExternalities {
		sp_tracing::try_init_simple();
		let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let balance_factor = if ExistentialDeposit::get() > 1 { 256 } else { 1 };

		let num_validators = self.num_validators.unwrap_or(self.validator_count);
		// Check that the number of validators is sensible.
		assert!(num_validators <= 8);
		let validators = (0..num_validators)
			.map(|x| ((x + 1) * 10 + 1) as AccountId)
			.collect::<Vec<_>>();

		let _ = pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(1, 10 * balance_factor),
				(2, 20 * balance_factor),
				(3, 300 * balance_factor),
				(4, 400 * balance_factor),
				(10, balance_factor),
				(11, balance_factor * 2000),
				(20, balance_factor),
				(21, balance_factor * 2000),
				(30, balance_factor),
				(31, balance_factor * 2000),
				(40, balance_factor),
				(41, balance_factor * 2000),
				(50, balance_factor),
				(51, balance_factor * 2000),
				(60, balance_factor),
				(61, balance_factor * 2000),
				(70, balance_factor),
				(71, balance_factor * 2000),
				(80, balance_factor),
				(81, balance_factor * 2000),
				(100, balance_factor),
				(101, balance_factor * 2000),
				// This allows us to have a total_payout different from 0.
				(999, 1_000_000_000_000),
			],
		}
		.assimilate_storage(&mut storage);

		let mut stakers: Vec<(AccountId, Option<AccountId>, Balance)> = vec![];
		if self.has_stakers {
			let stake_21 = if self.fair { 1000 } else { 2000 };
			let stake_31 = if self.validator_pool { balance_factor * 1000 } else { 1 };
			let nominated_101 = if self.nominate { Some(11) } else { None };

			stakers = vec![
				// (controller, staked_amount)
				(11, None, balance_factor * 1000),
				(21, None, stake_21),
				(31, None, stake_31),
				(41, None, balance_factor * 1000),
				// nominator
				(101, nominated_101, balance_factor * 500),
			];
		}

		let _ = nodle_staking::GenesisConfig::<Test> {
			stakers,
			invulnerables: self.invulnerables,
			..Default::default()
		}
		.assimilate_storage(&mut storage);

		let _ = pallet_session::GenesisConfig::<Test> {
			keys: validators
				.iter()
				.map(|&x| (x, x, SessionKeys { other: x.into() }))
				.collect(),
		}
		.assimilate_storage(&mut storage);

		let mut ext = sp_io::TestExternalities::from(storage);
		ext.execute_with(|| {
			let validators = Session::validators();
			SESSION.with(|x| *x.borrow_mut() = (validators.clone(), HashSet::new()));
		});

		if self.initialize_first_session {
			// We consider all test to start after timestamp is initialized This must be ensured by
			// having `timestamp::on_initialize` called before `staking::on_initialize`. Also, if
			// session length is 1, then it is already triggered.
			ext.execute_with(|| {
				System::set_block_number(1);
				Session::on_initialize(1);
				NodleStaking::on_initialize(1);
				Timestamp::set_timestamp(INIT_TIMESTAMP);
			});
		}

		ext
	}

	pub fn build_and_execute(self, test: impl FnOnce()) {
		let mut ext = self.build();
		ext.execute_with(test);
	}
}

pub(crate) fn balances(who: &AccountId) -> (Balance, Balance) {
	(
		Balances::free_balance(who),
		Balances::free_balance(who) - Balances::usable_balance(who),
	)
}

pub fn is_disabled(controller: AccountId) -> bool {
	SESSION.with(|d| d.borrow().1.contains(&controller))
}

pub(crate) fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}

pub(crate) fn events() -> Vec<pallet::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let RuntimeEvent::NodleStaking(inner) = e {
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
	log::trace!("mint_rewards:[{:#?}]=> - {:#?}", line!(), mock::last_event());
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
	start_session(session_index);
	assert_eq!(NodleStaking::active_session(), session_index);
}

pub(crate) fn bond_validator(ctrl: AccountId, val: Balance) {
	let _ = Balances::make_free_balance_be(&ctrl, val);
	assert_ok!(NodleStaking::validator_join_pool(RuntimeOrigin::signed(ctrl), val));
}

pub(crate) fn bond_nominator(ctrl: AccountId, val: Balance, target: AccountId) {
	let _ = Balances::make_free_balance_be(&ctrl, val);
	assert_ok!(NodleStaking::nominator_nominate(
		RuntimeOrigin::signed(ctrl),
		target,
		val,
		false,
	));
}

pub(crate) fn validators_in_pool() -> Vec<AccountId> {
	NodleStaking::validator_pool().0.into_iter().map(|s| s.owner).collect()
}

pub(crate) fn selected_validators() -> Vec<AccountId> {
	NodleStaking::selected_validators()
}

pub(crate) fn on_offence_now(
	offenders: &[OffenceDetails<AccountId, pallet_session::historical::IdentificationTuple<Test>>],
	slash_fraction: &[Perbill],
	disable_strategy: DisableStrategy,
) {
	let now = NodleStaking::active_session();
	on_offence_in_session(offenders, slash_fraction, now, disable_strategy)
}

pub(crate) fn on_offence_in_session(
	offenders: &[OffenceDetails<AccountId, pallet_session::historical::IdentificationTuple<Test>>],
	slash_fraction: &[Perbill],
	session_idx: SessionIndex,
	disable_strategy: DisableStrategy,
) {
	let bonded_session = NodleStaking::bonded_sessions();
	for bond_session in bonded_session.iter() {
		match (*bond_session).cmp(&session_idx) {
			std::cmp::Ordering::Equal => {
				let _ = NodleStaking::on_offence(offenders, slash_fraction, session_idx, disable_strategy);
				return;
			}
			std::cmp::Ordering::Greater => break,
			std::cmp::Ordering::Less => {}
		}
	}

	if NodleStaking::active_session() == session_idx {
		let _ = NodleStaking::on_offence(offenders, slash_fraction, session_idx, disable_strategy);
	} else {
		panic!("cannot slash in session {session_idx}");
	}
}
