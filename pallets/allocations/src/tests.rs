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
	traits::{ConstU32, GenesisBuild, Hooks},
	weights::Pays,
	PalletId,
};
use frame_system::EnsureSignedBy;
use lazy_static::lazy_static;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
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
	Perbill::from_parts(0_010_000_000),
	Perbill::from_parts(0_020_000_000),
	Perbill::from_parts(0_005_000_000),
];
const ONE_INFLATION_STEP: &[Perbill] = &[Perbill::from_parts(0_010_000_000)];
const NO_INFLATION_STEPS: &[Perbill] = &[];
const HUNDRED_PERCENT_INFLATION_RATE: &[Perbill] = &[Perbill::from_parts(1_000_000_000)];
const ZERO_PERCENT_INFLATION_RATE: &[Perbill] = &[Perbill::from_parts(0_000_000_000)];
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
fn force_calc_next_session_quota() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.checked_calc_next_session_quota(7, 1000u64, false), None);
	assert_eq!(curve.checked_calc_next_session_quota(7, 1000u64, true), Some(3));
	assert_eq!(curve.checked_calc_next_session_quota(10, 1000u64, false), Some(6));
}

#[test]
fn one_session_period_equals_one_fiscal_period_when_fiscal_is_zero() {
	let curve = <MintCurve<Test>>::new(3u64, 0u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.checked_calc_next_session_quota(2, 1000u64, true), Some(10));
	assert_eq!(curve.checked_calc_next_session_quota(3, 1000u64, true), Some(20));
}

#[test]
fn calc_next_session_quota_for_one_step_inflation() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, ONE_INFLATION_STEP, 1_000_000u64);
	assert_eq!(curve.checked_calc_next_session_quota(1, 1000u64, true), Some(3));
	assert_eq!(curve.checked_calc_next_session_quota(10, 1000u64, true), Some(3));
	assert_eq!(curve.checked_calc_next_session_quota(20, 1000u64, true), Some(3));
	assert_eq!(curve.checked_calc_next_session_quota(32, 1000u64, true), Some(3));
}

#[test]
fn calc_next_session_quota_when_no_inflation_is_configured() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, NO_INFLATION_STEPS, 1_000_000u64);
	assert_eq!(curve.checked_calc_next_session_quota(1, 1000u64, true), Some(0));
	assert_eq!(curve.checked_calc_next_session_quota(10, 1000u64, true), Some(0));
	assert_eq!(curve.checked_calc_next_session_quota(20, 1000u64, true), Some(0));
	assert_eq!(curve.checked_calc_next_session_quota(32, 1000u64, true), Some(0));
}

#[test]
fn calc_next_session_quota_when_inflation_is_zero() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, ZERO_PERCENT_INFLATION_RATE, 1_000_000u64);
	assert_eq!(curve.checked_calc_next_session_quota(1, 1000u64, true), Some(0));
}

#[test]
fn calc_next_session_quota_when_approaching_max_supply() {
	let curve = <MintCurve<Test>>::new(10u64, 10u64, HUNDRED_PERCENT_INFLATION_RATE, 2000u64);
	assert_eq!(curve.checked_calc_next_session_quota(1, 1000u64, true), Some(1000));
	assert_eq!(curve.checked_calc_next_session_quota(1, 1100u64, true), Some(900));
	let curve = <MintCurve<Test>>::new(1u64, 9u64, HUNDRED_PERCENT_INFLATION_RATE, 2000u64);
	assert_eq!(curve.checked_calc_next_session_quota(1, 1100u64, true), Some(100));
}

#[test]
fn should_update_session_quota() {
	let curve = <MintCurve<Test>>::new(3u64, 10u64, THREE_INFLATION_STEPS, 1_000_000u64);
	assert!(!curve.should_update_session_quota(2));
	assert!(curve.should_update_session_quota(3));
	assert!(curve.should_update_session_quota(15));
	assert!(!curve.should_update_session_quota(17));
}

#[test]
fn next_session_quota_is_initially_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::next_session_quota(), None);
	})
}

#[test]
fn both_session_events_are_emitted_on_the_very_first_on_initialize_after_upgrade() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::next_session_quota(), None);
		assert!(System::events().is_empty());
		Allocations::on_initialize(7); // Here any block number is ok
		let events: Vec<_> = System::events()
			.into_iter()
			.map(|event_record| event_record.event)
			.collect();
		assert_eq!(
			events,
			vec![
				Event::Allocations(crate::Event::SessionQuotaCalculated(0)),
				Event::Allocations(crate::Event::SessionQuotaRenewed)
			]
		);
	})
}

#[test]
fn no_events_if_not_at_the_beginning_of_a_session_or_a_fiscal_period() {
	new_test_ext().execute_with(|| {
		assert_eq!(Allocations::next_session_quota(), None);
		Allocations::on_initialize(7); // Here any block number is ok
		System::reset_events();
		Allocations::on_initialize(8);
		assert!(System::events().is_empty());
	})
}

#[test]
fn emit_session_quota_renewed_at_the_beginning_of_a_session() {
	new_test_ext().execute_with(|| {
		Allocations::on_initialize(7);
		System::reset_events();
		Allocations::on_initialize(9);
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
		let total_issuance = 1000u64;
		let _issuance = Balances::issue(total_issuance);
		Allocations::on_initialize(7);
		System::reset_events();
		Allocations::on_initialize(10);
		let events: Vec<_> = System::events()
			.into_iter()
			.map(|event_record| event_record.event)
			.collect();
		assert_eq!(
			events,
			vec![Event::Allocations(crate::Event::SessionQuotaCalculated(6))]
		);
	})
}

#[test]
fn next_session_quota() {
	new_test_ext().execute_with(|| {
		let total_issuance = 1000u64;
		let _issuance = Balances::issue(total_issuance);
		let session_share = total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		Allocations::on_initialize(7);
		assert_eq!(
			Allocations::next_session_quota(),
			Some(THREE_INFLATION_STEPS[0] * session_share)
		);
		Allocations::on_initialize(10);
		assert_eq!(
			Allocations::next_session_quota(),
			Some(THREE_INFLATION_STEPS[1] * session_share)
		);
		Allocations::on_initialize(20);
		assert_eq!(
			Allocations::next_session_quota(),
			Some(THREE_INFLATION_STEPS[2] * session_share)
		);
		Allocations::on_initialize(80);
		assert_eq!(
			Allocations::next_session_quota(),
			Some(THREE_INFLATION_STEPS[2] * session_share)
		);
	})
}

#[test]
fn next_session_quota_stays_the_same_during_one_fiscal_period() {
	new_test_ext().execute_with(|| {
		let total_issuance = 1000u64;
		let _issuance = Balances::issue(total_issuance);
		let quota =
			THREE_INFLATION_STEPS[1] * total_issuance * MINT_CURVE.session_period() / MINT_CURVE.fiscal_period();
		Allocations::on_initialize(10);
		Allocations::on_initialize(11);
		assert_eq!(Allocations::next_session_quota(), Some(quota));
		Allocations::on_initialize(19);
		assert_eq!(Allocations::next_session_quota(), Some(quota));
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
		Allocations::on_initialize(2);

		assert_eq!(Allocations::next_session_quota(), Some(quota0));
		assert_eq!(Allocations::session_quota(), quota0);

		Allocations::on_initialize(3);
		assert_eq!(Allocations::session_quota(), quota0);

		// Consume session quota and check it will be renewed on a new session
		<SessionQuota<Test>>::put(0);

		Allocations::on_initialize(5);
		assert_eq!(Allocations::session_quota(), 0);

		Allocations::on_initialize(6);
		assert_eq!(Allocations::session_quota(), quota0);

		Allocations::on_initialize(10);
		assert_eq!(Allocations::session_quota(), quota0);

		Allocations::on_initialize(12);
		assert_eq!(Allocations::session_quota(), quota1);
	})
}

#[test]
fn non_oracle_is_rejected() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Hacker::get()), bounded_vec![(Grantee::get(), 50)]),
			Errors::OracleAccessDenied
		);
	})
}

#[test]
fn oracle_does_not_pay_fees() {
	new_test_ext().execute_with(|| {
		<SessionQuota<Test>>::put(50);
		assert_eq!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![(Grantee::get(), 50)]),
			Ok(Pays::No.into())
		);
	})
}

#[test]
fn simple_allocation_works() {
	new_test_ext().execute_with(|| {
		<SessionQuota<Test>>::put(50);
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
		<SessionQuota<Test>>::put(120);
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50), (OtherGrantee::get(), 50)]
		));
		assert_eq!(Allocations::session_quota(), 20);
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
		<SessionQuota<Test>>::put(150);
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
