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

//! Unit tests for the vesting module.

#![cfg(test)]

use super::*;
use frame_support::{assert_err, assert_noop, assert_ok, traits::WithdrawReasons};
use mock::{
	CancelOrigin, Event as TestEvent, ExtBuilder, Origin, PalletBalances, System, Test as Runtime, Vesting, ALICE, BOB,
};
use pallet_balances::{BalanceLock, Reasons};
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn add_vesting_schedule_works() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		System::set_block_number(1);

		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 1u32,
			per_period: 100u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(
			Origin::signed(ALICE),
			BOB,
			schedule.clone()
		));
		assert_eq!(Vesting::vesting_schedules(&BOB), vec![schedule.clone()]);

		let vested_event = TestEvent::Vesting(Event::VestingScheduleAdded(ALICE, BOB, schedule));
		assert!(System::events().iter().any(|record| record.event == vested_event));
	});
}

#[test]
fn add_new_vesting_schedule_merges_with_current_locked_balance_and_until() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));

		System::set_block_number(12);

		let another_schedule = VestingSchedule {
			start: 10u64,
			period: 13u64,
			period_count: 1u32,
			per_period: 7u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(
			Origin::signed(ALICE),
			BOB,
			another_schedule
		));

		assert_eq!(
			PalletBalances::locks(&BOB).to_vec().pop(),
			Some(BalanceLock {
				id: VESTING_LOCK_ID,
				amount: 17u64,
				reasons: Reasons::All,
			})
		);
	});
}

#[test]
fn cannot_use_fund_if_not_claimed() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 10u64,
			period: 10u64,
			period_count: 1u32,
			per_period: 50u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));
		assert!(PalletBalances::ensure_can_withdraw(&BOB, 1, WithdrawReasons::TRANSFER, 49).is_err());
	});
}

#[test]
fn add_vesting_schedule_fails_if_zero_period_or_count() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 1u64,
			period: 0u64,
			period_count: 1u32,
			per_period: 100u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule),
			Error::<Runtime>::ZeroVestingPeriod
		);

		let schedule = VestingSchedule {
			start: 1u64,
			period: 1u64,
			period_count: 0u32,
			per_period: 100u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule),
			Error::<Runtime>::ZeroVestingPeriodCount
		);
	});
}

#[test]
fn add_vesting_schedule_fails_if_transfer_err() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 1u64,
			period: 1u64,
			period_count: 1u32,
			per_period: 100u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(Origin::signed(BOB), ALICE, schedule),
			pallet_balances::Error::<Runtime, _>::InsufficientBalance,
		);
	});
}

#[test]
fn add_vesting_schedule_fails_if_overflow() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 1u64,
			period: 1u64,
			period_count: 2u32,
			per_period: u64::max_value(),
		};
		assert_err!(
			Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule),
			Error::<Runtime>::NumOverflow
		);

		let another_schedule = VestingSchedule {
			start: u64::max_value(),
			period: 1u64,
			period_count: 2u32,
			per_period: 1u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, another_schedule),
			Error::<Runtime>::NumOverflow
		);
	});
}

#[test]
fn claim_works() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));

		System::set_block_number(11);
		// remain locked if not claimed
		assert!(PalletBalances::transfer(Origin::signed(BOB), ALICE, 10).is_err());
		// unlocked after claiming
		assert_ok!(Vesting::claim(Origin::signed(BOB)));
		assert_ok!(PalletBalances::transfer(Origin::signed(BOB), ALICE, 10));
		// more are still locked
		assert!(PalletBalances::transfer(Origin::signed(BOB), ALICE, 1).is_err());
		// does not clear storage
		assert!(VestingSchedules::<Runtime>::contains_key(BOB));

		System::set_block_number(21);
		// claim more
		assert_ok!(Vesting::claim(Origin::signed(BOB)));
		assert_ok!(PalletBalances::transfer(Origin::signed(BOB), ALICE, 10));
		// all used up
		assert_eq!(PalletBalances::free_balance(BOB), 0);
		// clears the storage
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB));
		// no locks anymore
		assert_eq!(PalletBalances::locks(&BOB), vec![]);
	});
}

#[test]
fn cancel_restricted_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Vesting::cancel_all_vesting_schedules(Origin::signed(ALICE), BOB, CancelOrigin::get()),
			BadOrigin
		);
	})
}

#[test]
fn cancel_auto_claim_recipient_funds_and_wire_the_rest() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));

		System::set_block_number(11);

		assert_ok!(Vesting::cancel_all_vesting_schedules(
			Origin::signed(CancelOrigin::get()),
			BOB,
			CancelOrigin::get()
		));

		// Auto claim
		assert_ok!(PalletBalances::transfer(Origin::signed(BOB), ALICE, 10));

		// Wire the rest
		assert_ok!(PalletBalances::transfer(Origin::signed(CancelOrigin::get()), ALICE, 10));
	});
}

#[test]
fn cancel_clears_storage() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));

		System::set_block_number(11);

		assert_ok!(Vesting::cancel_all_vesting_schedules(
			Origin::signed(CancelOrigin::get()),
			BOB,
			CancelOrigin::get()
		));

		assert!(!VestingSchedules::<Runtime>::contains_key(BOB));
	});
}

#[test]
fn cancel_tolerates_corrupted_state() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(Origin::signed(ALICE), BOB, schedule));

		// We also add some vesting schedules without any balances to simulate
		// a corrupted / badly canceled state.
		VestingSchedules::<Runtime>::mutate(BOB, |s| {
			s.push(VestingSchedule {
				start: 0u64,
				period: 10u64,
				period_count: 2u32,
				per_period: 1_000u64, // definitely too much money
			})
		});

		System::set_block_number(11);

		assert_ok!(Vesting::cancel_all_vesting_schedules(
			Origin::signed(CancelOrigin::get()),
			BOB,
			CancelOrigin::get(),
		));

		assert!(!VestingSchedules::<Runtime>::contains_key(BOB));
	});
}

#[test]
fn cannot_vest_to_self() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		assert_noop!(
			Vesting::add_vesting_schedule(
				Origin::signed(ALICE),
				ALICE,
				VestingSchedule {
					start: 0u64,
					period: 10u64,
					period_count: 1u32,
					per_period: 100u64,
				}
			),
			Error::<Runtime>::VestingToSelf
		);
	});
}
