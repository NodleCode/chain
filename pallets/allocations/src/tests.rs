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

#![cfg(test)]

use super::*;
use crate::{self as pallet_allocations};
use frame_support::{
	assert_noop, assert_ok, bounded_vec, ord_parameter_types, parameter_types,
	traits::{ConstU32, GenesisBuild},
	weights::Pays,
	PalletId,
};
use frame_system::EnsureSignedBy;
use lazy_static::lazy_static;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BadOrigin, BlakeTwo256, IdentityLookup},
	Perbill,
};

pub(crate) type AccountId = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		Membership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		Allocations: pallet_allocations::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	type Origin = Origin;
	type Call = Call;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type OnSetCode = ();
	type SystemWeightInfo = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 2;
	pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

const THREE_INFLATION_STEPS: &[Perbill] = &[
	Perbill::from_percent(1),
	Perbill::from_percent(2),
	Perbill::from_perthousand(5),
];
const ONE_INFLATION_STEP: &[Perbill] = &[Perbill::from_percent(1)];
const NO_INFLATION_STEPS: &[Perbill] = &[];
const HUNDRED_PERCENT_INFLATION_RATE: &[Perbill] = &[Perbill::from_percent(100)];
const ZERO_PERCENT_INFLATION_RATE: &[Perbill] = &[Perbill::from_percent(0)];
lazy_static! {
	static ref MINT_CURVE: MintCurve<Test> = MintCurve::new(3u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
}
parameter_types! {
	pub const Oracle: u64 = 0;
	pub const Hacker: u64 = 1;
	pub const Grantee: u64 = 2;
	pub const OtherGrantee: u64 = 3;
	pub const Receiver: u64 = 4;
	pub const Fee: Perbill = Perbill::from_percent(10);
	pub const MaxAllocs: u32 = 10;
	pub const AllocPalletId: PalletId = PalletId(*b"py/alloc");
	pub MintCurveParameter: &'static MintCurve<Test> = &MINT_CURVE;
}
ord_parameter_types! {
	pub const Admin: u64 = 4;
}
impl WithAccountId<u64> for Receiver {
	fn account_id() -> u64 {
		Receiver::get()
	}
}

impl pallet_membership::Config for Test {
	type Event = Event;
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

impl Config for Test {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Self>;
	type PalletId = AllocPalletId;
	type ProtocolFee = Fee;
	type ProtocolFeeReceiver = Receiver;
	type ExistentialDeposit = <Test as pallet_balances::Config>::ExistentialDeposit;
	type MintCurve = MintCurveParameter;
	type MaxAllocs = MaxAllocs;
	type OracleMembers = Membership;
	type BlockNumberProvider = frame_system::Pallet<Test>;
	type WeightInfo = ();
}
type Errors = Error<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_tracing::try_init_simple();

	let mut storage = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap_or_else(|err| {
			panic!(
				"new_test_ext:[{:#?}] - FrameSystem GenesisConfig Err:[{:#?}]!!!",
				line!(),
				err
			)
		});

	let _ = pallet_membership::GenesisConfig::<Test> {
		members: bounded_vec![Oracle::get()],
		..Default::default()
	}
	.assimilate_storage(&mut storage)
	.map_err(|err| {
		panic!(
			"new_test_ext:[{:#?}] - Membership GenesisConfig Err [{:#?}]!!!",
			line!(),
			err
		);
	});

	let mut ext = sp_io::TestExternalities::from(storage);

	ext.execute_with(|| {
		System::set_block_number(1);
	});

	ext
}

#[test]
fn mint_curve_direct_build() {
	let curve = MintCurve::<Test> {
		session_period: 0u64,
		fiscal_period: 10u64,
		inflation_steps: THREE_INFLATION_STEPS.to_vec(),
		maximum_supply: 1_000_000u64,
	};
	assert_eq!(curve.session_period(), 0u64);
	assert_eq!(curve.fiscal_period(), 10u64);
	assert_eq!(curve.maximum_supply(), 1_000_000u64);
}

#[test]
fn assume_fiscal_period_0_is_infinitesimal() {
	let curve = MintCurve::<Test> {
		session_period: 3u64,
		fiscal_period: 0u64,
		inflation_steps: THREE_INFLATION_STEPS.to_vec(),
		maximum_supply: 1_000_000u64,
	};
	assert_eq!(curve.calc_session_quota(2, 0, 1000u64), 5);
	assert_eq!(curve.calc_session_quota(3, 0, 1000u64), 5);
}

#[test]
fn no_quota_for_0_session_period() {
	let curve = MintCurve::<Test> {
		session_period: 0u64,
		fiscal_period: 10u64,
		inflation_steps: THREE_INFLATION_STEPS.to_vec(),
		maximum_supply: 1_000_000u64,
	};
	assert_eq!(curve.calc_session_quota(2, 0, 1000u64), 0);
}

#[test]
fn mint_curve_remains_valid_regardless_of_new_params() {
	let curve = <MintCurve<Test>>::new(0u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.session_period(), 1u64);
	let curve = <MintCurve<Test>>::new(3u64, 0u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.fiscal_period(), 3u64);
	let curve = <MintCurve<Test>>::new(0u64, 0u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.session_period(), 1u64);
	assert_eq!(curve.fiscal_period(), 1u64);
}

#[test]
fn mint_curve_maximum_supply() {
	let curve = <MintCurve<Test>>::new(0u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.maximum_supply(), 1_000_000u64);
}

#[test]
fn calc_next_session_quota_for_all_inflation_steps() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(0, 0, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(10, 0, 1000u64), 6);
	assert_eq!(curve.calc_session_quota(13, 3, 1000u64), 6);
	assert_eq!(curve.calc_session_quota(20, 0, 1000u64), 1);
	assert_eq!(curve.calc_session_quota(27, 7, 1000u64), 1);
	assert_eq!(curve.calc_session_quota(30, 0, 1000u64), 1);
	assert_eq!(curve.calc_session_quota(41, 11, 1000u64), 1);
}

#[test]
fn sessions_before_curve_start_use_same_quota_as_in_first_inflation_step() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(0, 37, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(10, 37, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(36, 37, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(37, 37, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(47, 37, 1000u64), 6);
}

#[test]
fn next_schedule_before_curve_start() {
	assert_eq!(<MintCurve<Test>>::next_schedule(0, 7, 3), 1);
	assert_eq!(<MintCurve<Test>>::next_schedule(1, 7, 3), 4);
	assert_eq!(<MintCurve<Test>>::next_schedule(2, 7, 3), 4);
	assert_eq!(<MintCurve<Test>>::next_schedule(3, 7, 3), 4);
	assert_eq!(<MintCurve<Test>>::next_schedule(4, 7, 3), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(5, 7, 3), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(6, 7, 3), 7);
}

#[test]
fn next_schedule_after_curve_start() {
	assert_eq!(<MintCurve<Test>>::next_schedule(7, 7, 3), 10);
	assert_eq!(<MintCurve<Test>>::next_schedule(8, 7, 3), 10);
	assert_eq!(<MintCurve<Test>>::next_schedule(9, 7, 3), 10);
	assert_eq!(<MintCurve<Test>>::next_schedule(10, 7, 3), 13);
	assert_eq!(<MintCurve<Test>>::next_schedule(11, 7, 3), 13);
	assert_eq!(<MintCurve<Test>>::next_schedule(12, 7, 3), 13);
	assert_eq!(<MintCurve<Test>>::next_schedule(36, 7, 3), 37);
}

#[test]
fn next_schedule_when_period_is_0() {
	assert_eq!(<MintCurve<Test>>::next_schedule(0, 7, 0), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(6, 7, 0), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(7, 7, 0), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(8, 7, 0), 7);
	assert_eq!(<MintCurve<Test>>::next_schedule(36, 7, 0), 7);
}

#[test]
fn one_session_period_equals_one_fiscal_period_when_fiscal_is_zero() {
	let curve = <MintCurve<Test>>::new(3u64, 0u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(2, 0, 1000u64), 10);
	assert_eq!(curve.calc_session_quota(3, 0, 1000u64), 20);
}

#[test]
fn calc_next_session_quota_for_one_step_inflation() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, ONE_INFLATION_STEP, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(1, 0, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(10, 0, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(20, 0, 1000u64), 3);
	assert_eq!(curve.calc_session_quota(32, 0, 1000u64), 3);
}

#[test]
fn calc_next_session_quota_when_no_inflation_is_configured() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, NO_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(1, 0, 1000u64), 0);
	assert_eq!(curve.calc_session_quota(10, 0, 1000u64), 0);
	assert_eq!(curve.calc_session_quota(20, 0, 1000u64), 0);
	assert_eq!(curve.calc_session_quota(32, 0, 1000u64), 0);
}

#[test]
fn calc_next_session_quota_when_inflation_is_zero() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, ZERO_PERCENT_INFLATION_RATE, 1_000_000u64);
	assert_eq!(curve.calc_session_quota(1, 0, 1000u64), 0);
}

#[test]
fn calc_next_session_quota_when_approaching_max_supply() {
	let curve = <MintCurve<Test>>::new(10u64, 10u64, HUNDRED_PERCENT_INFLATION_RATE, 2000u64);
	assert_eq!(curve.calc_session_quota(1, 0, 1000u64), 1000);
	assert_eq!(curve.calc_session_quota(1, 0, 1100u64), 900);
	let curve = <MintCurve<Test>>::new(1u64, 9u64, HUNDRED_PERCENT_INFLATION_RATE, 2000u64);
	assert_eq!(curve.calc_session_quota(1, 0, 1100u64), 100);
}

#[test]
fn next_session_quota_is_initially_0() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::next_session_quota(), 0);
	})
}

#[test]
fn both_session_events_are_emitted_on_the_very_first_on_initialize_after_upgrade() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::next_session_quota(), 0);
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		let session_share = total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		let session_quota = THREE_INFLATION_STEPS[0] * session_share;
		assert!(System::events().is_empty());
		Allocations::checked_update_session_quota();
		let events: Vec<_> = System::events()
			.into_iter()
			.map(|event_record| event_record.event)
			.collect();
		assert_eq!(
			events,
			vec![
				Event::Allocations(crate::Event::SessionQuotaCalculated(session_quota)),
				Event::Allocations(crate::Event::SessionQuotaRenewed)
			]
		);
	})
}

#[test]
fn no_events_if_not_at_the_beginning_of_a_session_or_a_fiscal_period() {
	new_test_ext().execute_with(|| {
		System::set_block_number(7);
		Allocations::checked_update_session_quota();
		System::reset_events();
		System::set_block_number(8);
		Allocations::checked_update_session_quota();
		assert!(System::events().is_empty());
	})
}

#[test]
fn emit_session_quota_renewed_at_the_beginning_of_a_session() {
	new_test_ext().execute_with(|| {
		System::set_block_number(7);
		Allocations::checked_update_session_quota();
		System::reset_events();
		System::set_block_number(10);
		Allocations::checked_update_session_quota();
		let events: Vec<_> = System::events()
			.into_iter()
			.map(|event_record| event_record.event)
			.collect();
		assert_eq!(events, vec![Event::Allocations(crate::Event::SessionQuotaRenewed)]);
	})
}

#[test]
fn emit_session_quota_calculated_at_the_beginning_of_a_fiscal_period() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		let session_share = total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		System::set_block_number(7);
		Allocations::checked_update_session_quota();
		System::reset_events();
		System::set_block_number(16);
		Allocations::checked_update_session_quota();
		System::reset_events();
		System::set_block_number(17);
		let session_quota = THREE_INFLATION_STEPS[1] * session_share;
		Allocations::checked_update_session_quota();
		let events: Vec<_> = System::events()
			.into_iter()
			.map(|event_record| event_record.event)
			.collect();
		assert_eq!(
			events,
			vec![Event::Allocations(crate::Event::SessionQuotaCalculated(session_quota))]
		);
	})
}

#[test]
fn next_session_quota() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		let session_share = total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		System::set_block_number(7);
		Allocations::checked_update_session_quota();
		assert_eq!(
			Allocations::next_session_quota(),
			THREE_INFLATION_STEPS[0] * session_share
		);
		System::set_block_number(17);
		Allocations::checked_update_session_quota();
		assert_eq!(
			Allocations::next_session_quota(),
			THREE_INFLATION_STEPS[1] * session_share
		);
		System::set_block_number(27);
		Allocations::checked_update_session_quota();
		assert_eq!(
			Allocations::next_session_quota(),
			THREE_INFLATION_STEPS[2] * session_share
		);
		System::set_block_number(87);
		Allocations::checked_update_session_quota();
		assert_eq!(
			Allocations::next_session_quota(),
			THREE_INFLATION_STEPS[2] * session_share
		);
	})
}

#[test]
fn next_session_quota_stays_the_same_during_one_fiscal_period() {
	new_test_ext().execute_with(|| {
		let total_issuance = 1000u64;
		let _issuance = Balances::issue(total_issuance);
		let quota =
			THREE_INFLATION_STEPS[0] * total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		System::set_block_number(10);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::next_session_quota(), quota);
		System::set_block_number(11);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::next_session_quota(), quota);
		System::set_block_number(19);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::next_session_quota(), quota);
	})
}

#[test]
fn session_quota_is_initially_zero() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::session_quota(), 0);
	})
}

#[test]
fn session_quota_is_renewed_every_session() {
	new_test_ext().execute_with(|| {
		let total_issuance = 1000u64;
		let _issuance = Balances::issue(total_issuance);
		let session_share = total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();

		let quota0 = THREE_INFLATION_STEPS[0] * session_share;
		let quota1 = THREE_INFLATION_STEPS[1] * session_share;
		System::set_block_number(2);
		Allocations::checked_update_session_quota();

		assert_eq!(Allocations::next_session_quota(), quota0);
		assert_eq!(Allocations::session_quota(), quota0);

		System::set_block_number(5);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::session_quota(), quota0);

		// Consume session quota and check it will be renewed on a new session
		<SessionQuota<Test>>::put(0);

		System::set_block_number(7);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::session_quota(), 0);

		System::set_block_number(8);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::session_quota(), quota0);

		System::set_block_number(12);
		Allocations::checked_update_session_quota();
		assert_eq!(Allocations::session_quota(), quota1);
	})
}

#[test]
fn non_oracle_is_rejected() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		assert_noop!(
			Allocations::batch(Origin::signed(Hacker::get()), bounded_vec![(Grantee::get(), 50)]),
			Errors::OracleAccessDenied
		);
	})
}

#[test]
fn oracle_does_not_pay_fees() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		let result = Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![(Grantee::get(), 50)])
			.expect("batch call failed");
		assert_eq!(result.pays_fee, Pays::No);
	})
}

#[test]
fn only_root_can_set_curve_starting_block() {
	new_test_ext().execute_with(|| {
		assert_eq!(<MintCurveStartingBlock<Test>>::get(), None);
		assert_eq!(
			Allocations::set_curve_starting_block(Origin::root(), 13),
			Ok(Pays::No.into())
		);
		assert_eq!(<MintCurveStartingBlock<Test>>::get(), Some(13));
		assert_noop!(
			Allocations::set_curve_starting_block(Origin::signed(Oracle::get()), 17),
			BadOrigin
		);
	})
}

#[test]
fn very_first_batch_call_sets_curve_starting_block() {
	new_test_ext().execute_with(|| {
		assert_eq!(<MintCurveStartingBlock<Test>>::get(), None);
		System::set_block_number(5);
		let _issuance = Balances::issue(100000u64);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50)]
		));
		assert_eq!(<MintCurveStartingBlock<Test>>::get(), Some(5));
		assert_eq!(<SessionQuotaCalculationSchedule<Test>>::get(), 15);
		assert_eq!(<SessionQuotaRenewSchedule<Test>>::get(), 8);
		System::set_block_number(6);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50)]
		));
		assert_eq!(<MintCurveStartingBlock<Test>>::get(), Some(5));
		assert_eq!(<SessionQuotaCalculationSchedule<Test>>::get(), 15);
		assert_eq!(<SessionQuotaRenewSchedule<Test>>::get(), 8);
	})
}

#[test]
fn simple_allocation_works() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50)]
		));
		assert_eq!(Balances::free_balance(Grantee::get()), 45);
		assert_eq!(Balances::free_balance(Receiver::get()), 5);

		let alloc_account_id: u64 = AllocPalletId::get().into_account_truncating();
		assert_eq!(Balances::free_balance(alloc_account_id), 0);
	})
}

#[test]
fn batched_allocation_works() {
	new_test_ext().execute_with(|| {
		let total_issuance = 100000u64;
		let _issuance = Balances::issue(total_issuance);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50), (OtherGrantee::get(), 50)]
		));
		assert_eq!(Allocations::session_quota(), 200);
		assert_eq!(Balances::free_balance(Grantee::get()), 45);
		assert_eq!(Balances::free_balance(OtherGrantee::get()), 45);
		assert_eq!(Balances::free_balance(Receiver::get()), 10);

		let alloc_account_id: u64 = AllocPalletId::get().into_account_truncating();
		assert_eq!(Balances::free_balance(alloc_account_id), 0);
	})
}

#[test]
fn use_session_quota_in_multiple_calls() {
	new_test_ext().execute_with(|| {
		let total_issuance = 50000u64;
		let _issuance = Balances::issue(total_issuance);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 30), (OtherGrantee::get(), 50)]
		));
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 70)]
		));
		assert_eq!(Allocations::session_quota(), 0);
		assert_eq!(Balances::free_balance(Grantee::get()), 90);
		assert_eq!(Balances::free_balance(OtherGrantee::get()), 45);
		assert_eq!(Balances::free_balance(Receiver::get()), 15);
	})
}

#[test]
fn exceeding_session_quota_fails() {
	new_test_ext().execute_with(|| {
		<SessionQuota<Test>>::put(79);
		assert_noop!(
			Allocations::batch(
				Origin::signed(Oracle::get()),
				bounded_vec![(Grantee::get(), 30), (OtherGrantee::get(), 50)]
			),
			Errors::AllocationExceedsSessionQuota
		);
	})
}

#[test]
fn ensure_issuance_checks() {
	new_test_ext().execute_with(|| {
		<SessionQuota<Test>>::put(MINT_CURVE.maximum_supply());
		let inputs: Vec<BoundedVec<(u64, u64), MaxAllocs>> = vec![
			// overflow checks
			bounded_vec![(Grantee::get(), u64::MAX), (OtherGrantee::get(), 10)],
			// actual issuance checks
			bounded_vec![(Grantee::get(), MINT_CURVE.maximum_supply() + 10)],
			bounded_vec![(Grantee::get(), MINT_CURVE.maximum_supply()), (OtherGrantee::get(), 10)],
		];
		for input in inputs.iter().cloned() {
			assert_noop!(
				Allocations::batch(Origin::signed(Oracle::get()), input),
				Errors::AllocationExceedsSessionQuota
			);
		}
	})
}

#[test]
fn ensure_existential_deposit_checks() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![(Grantee::get(), 1)]),
			Errors::DoesNotSatisfyExistentialDeposit
		);
	})
}

#[test]
fn no_issuance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![]),
			Errors::BatchEmpty
		);
	})
}
