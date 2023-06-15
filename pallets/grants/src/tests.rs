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
	context_events, CancelOrigin, ExtBuilder, PalletBalances, RuntimeEvent as TestEvent, RuntimeOrigin, System,
	Test as Runtime, Vesting, ALICE, BOB,
};
use pallet_balances::{BalanceLock, Reasons};
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn check_releases_default_config() {
	ExtBuilder::default().build().execute_with(|| {
		let releases = Releases::default();
		assert_eq!(releases, Releases::V0);
		assert_ne!(releases, Releases::V1);
	})
}
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
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule.clone()
		));
		assert_eq!(Vesting::vesting_schedules(BOB::get()), vec![schedule.clone()]);

		let vested_event = TestEvent::Vesting(Event::VestingScheduleAdded(ALICE::get(), BOB::get(), schedule));
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
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));

		System::set_block_number(12);

		let another_schedule = VestingSchedule {
			start: 10u64,
			period: 13u64,
			period_count: 1u32,
			per_period: 7u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			another_schedule
		));

		assert_eq!(
			PalletBalances::locks(BOB::get()).to_vec().pop(),
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
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));
		assert!(PalletBalances::ensure_can_withdraw(&BOB::get(), 1, WithdrawReasons::TRANSFER, 49).is_err());
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
			Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedule),
			Error::<Runtime>::ZeroVestingPeriod
		);

		let schedule = VestingSchedule {
			start: 1u64,
			period: 1u64,
			period_count: 0u32,
			per_period: 100u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedule),
			Error::<Runtime>::ZeroVestingPeriodCount
		);
	});
}

#[test]
fn add_vesting_schedule_fails_if_transfer_err() {
	use sp_runtime::TokenError::FundsUnavailable;
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 1u64,
			period: 1u64,
			period_count: 1u32,
			per_period: 100u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(RuntimeOrigin::signed(BOB::get()), ALICE::get(), schedule),
			FundsUnavailable,
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
			Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedule),
			Error::<Runtime>::NumOverflow
		);

		let another_schedule = VestingSchedule {
			start: u64::max_value(),
			period: 1u64,
			period_count: 2u32,
			per_period: 1u64,
		};
		assert_err!(
			Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), another_schedule),
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
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));

		System::set_block_number(11);
		// remain locked if not claimed
		assert!(PalletBalances::transfer(RuntimeOrigin::signed(BOB::get()), ALICE::get(), 10).is_err());
		// unlocked after claiming
		assert_ok!(Vesting::claim(RuntimeOrigin::signed(BOB::get())));
		assert_ok!(PalletBalances::transfer(
			RuntimeOrigin::signed(BOB::get()),
			ALICE::get(),
			10
		));
		// more are still locked
		assert!(PalletBalances::transfer(RuntimeOrigin::signed(BOB::get()), ALICE::get(), 1).is_err());
		// does not clear storage
		assert!(<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		System::set_block_number(21);
		// claim more
		assert_ok!(Vesting::claim(RuntimeOrigin::signed(BOB::get())));
		assert_ok!(PalletBalances::transfer(
			RuntimeOrigin::signed(BOB::get()),
			ALICE::get(),
			10
		));
		// all used up
		assert_eq!(PalletBalances::free_balance(BOB::get()), 0);
		// clears the storage
		assert!(!<VestingSchedules<Runtime>>::contains_key(BOB::get()));
		// no locks anymore
		assert_eq!(PalletBalances::locks(BOB::get()), vec![]);
	});
}

#[test]
fn cancel_restricted_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Vesting::cancel_all_vesting_schedules(RuntimeOrigin::signed(ALICE::get()), BOB::get(), CancelOrigin::get()),
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
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));

		System::set_block_number(11);

		assert_ok!(Vesting::cancel_all_vesting_schedules(
			RuntimeOrigin::signed(CancelOrigin::get()),
			BOB::get(),
			CancelOrigin::get()
		));

		// Auto claim
		assert_ok!(PalletBalances::transfer(
			RuntimeOrigin::signed(BOB::get()),
			ALICE::get(),
			10
		));

		// Wire the rest
		assert_ok!(PalletBalances::transfer(
			RuntimeOrigin::signed(CancelOrigin::get()),
			ALICE::get(),
			10
		));
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
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));

		System::set_block_number(11);

		assert_ok!(Vesting::cancel_all_vesting_schedules(
			RuntimeOrigin::signed(CancelOrigin::get()),
			BOB::get(),
			CancelOrigin::get()
		));

		assert!(!<VestingSchedules<Runtime>>::contains_key(BOB::get()));
	});
}
#[allow(clippy::assertions_on_constants)]
#[test]
fn cancel_tolerates_corrupted_state() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let allice_vesting_to_bob_schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};

		// Initial Balance Status
		assert_eq!(mock::balances(&ALICE::get()), (100, 0));
		assert_eq!(PalletBalances::total_balance(&ALICE::get()), 100);

		assert_eq!(mock::balances(&BOB::get()), (0, 0));
		assert_eq!(PalletBalances::total_balance(&BOB::get()), 0);

		assert_eq!(mock::balances(&CancelOrigin::get()), (0, 0));
		assert_eq!(PalletBalances::total_balance(&CancelOrigin::get()), 0);

		assert!(!<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		let ans = Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			allice_vesting_to_bob_schedule.clone(),
		);
		assert_ok!(ans);
		assert!(<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		// Event & Balance Status
		let mut expected = vec![Event::VestingScheduleAdded(
			ALICE::get(),
			BOB::get(),
			VestingSchedule {
				start: 0,
				period: 10,
				period_count: 2,
				per_period: 10,
			},
		)];
		assert_eq!(context_events(), expected);

		assert_eq!(mock::balances(&ALICE::get()), (80, 0));
		assert_eq!(PalletBalances::total_balance(&ALICE::get()), 80);

		assert_eq!(mock::balances(&BOB::get()), (20, 20));
		assert_eq!(PalletBalances::total_balance(&BOB::get()), 20);

		assert_eq!(mock::balances(&CancelOrigin::get()), (0, 0));
		assert_eq!(PalletBalances::total_balance(&CancelOrigin::get()), 0);

		// We also add some vesting schedules without any balances to simulate
		// a corrupted / badly canceled state.
		let bob_modified_vesting_schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 1_000u64, // definitely too much money
		};

		let ans = <VestingSchedules<Runtime>>::try_mutate(BOB::get(), |s| -> Result<(), VestingSchedule<u64, u64>> {
			s.try_push(bob_modified_vesting_schedule.clone())
		});

		assert_ok!(ans);

		assert!(<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		if let Ok(schedule_from_chain) = <VestingSchedules<Runtime>>::try_get(BOB::get()) {
			assert_eq!(schedule_from_chain.len(), 2);

			assert_eq!(schedule_from_chain[0], allice_vesting_to_bob_schedule);
			assert_eq!(schedule_from_chain[1], bob_modified_vesting_schedule);
		} else {
			assert!(false, "Expected Bob to have some grants, Got error instead");
		}

		System::set_block_number(11);
		assert!(<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		let res_cancel_all_vesting_schedules = Vesting::cancel_all_vesting_schedules(
			RuntimeOrigin::signed(CancelOrigin::get()),
			BOB::get(),
			CancelOrigin::get(),
		);
		assert_ok!(res_cancel_all_vesting_schedules);

		assert!(!<VestingSchedules<Runtime>>::contains_key(BOB::get()));

		// Event & Balance Status
		expected.append(&mut vec![Event::VestingSchedulesCanceled(2)]);

		assert_eq!(context_events(), expected);

		assert_eq!(mock::balances(&ALICE::get()), (80, 0));
		assert_eq!(PalletBalances::total_balance(&ALICE::get()), 80);

		assert_eq!(mock::balances(&BOB::get()), (0, 0));
		assert_eq!(PalletBalances::total_balance(&BOB::get()), 0);

		assert_eq!(mock::balances(&CancelOrigin::get()), (20, 0));
		assert_eq!(PalletBalances::total_balance(&CancelOrigin::get()), 20);
	});
}

#[test]
fn cannot_vest_to_self() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		assert_noop!(
			Vesting::add_vesting_schedule(
				RuntimeOrigin::signed(ALICE::get()),
				ALICE::get(),
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

#[test]
fn add_vesting_schedule_overflow_check() {
	ExtBuilder::default()
		.balances(vec![(ALICE::get(), 1000)])
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			let schedules = vec![
				VestingSchedule {
					start: 0u64,
					period: 10u64,
					period_count: 1u32,
					per_period: 100u64,
				},
				VestingSchedule {
					start: 0u64,
					period: 10u64,
					period_count: 1u32,
					per_period: 100u64,
				},
				VestingSchedule {
					start: 0u64,
					period: 10u64,
					period_count: 1u32,
					per_period: 100u64,
				},
			];

			assert_ok!(Vesting::add_vesting_schedule(
				RuntimeOrigin::signed(ALICE::get()),
				BOB::get(),
				schedules[0].clone(),
			));
			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 1);

			let expected = vec![Event::VestingScheduleAdded(1, 2, schedules[0].clone())];

			assert_eq!(context_events(), expected);

			assert_ok!(Vesting::add_vesting_schedule(
				RuntimeOrigin::signed(ALICE::get()),
				BOB::get(),
				schedules[1].clone(),
			));
			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 2);

			let expected = vec![
				Event::VestingScheduleAdded(1, 2, schedules[0].clone()),
				Event::VestingScheduleAdded(1, 2, schedules[1].clone()),
			];

			assert_eq!(context_events(), expected);

			assert_err!(
				Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedules[2].clone(),),
				<Error<Runtime>>::MaxScheduleOverflow,
			);

			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 2);
		});
}

#[test]
fn add_vesting_schedule_overflow_cfg_min_check() {
	ExtBuilder::default()
		.balances(vec![(ALICE::get(), 1000)])
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			let schedules = vec![VestingSchedule {
				start: 0u64,
				period: 10u64,
				period_count: 1u32,
				per_period: 100u64,
			}];

			mock::MAX_SCHEDULE.with(|v| *v.borrow_mut() = 0);

			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 0);

			assert_err!(
				Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedules[0].clone(),),
				<Error<Runtime>>::MaxScheduleOverflow,
			);

			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 0);

			let expected = vec![];
			assert_eq!(context_events(), expected);

			mock::MAX_SCHEDULE.with(|v| *v.borrow_mut() = 1);

			assert_ok!(Vesting::add_vesting_schedule(
				RuntimeOrigin::signed(ALICE::get()),
				BOB::get(),
				schedules[0].clone(),
			));
			assert_eq!(Vesting::vesting_schedules(BOB::get()).to_vec().len(), 1);

			let expected = vec![Event::VestingScheduleAdded(1, 2, schedules[0].clone())];

			assert_eq!(context_events(), expected);
		});
}

#[test]
fn add_vesting_schedule_overflow_cfg_max_check() {
	ExtBuilder::default()
		.balances(vec![(ALICE::get(), 10_000_000)])
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			let schedules = vec![VestingSchedule {
				start: 0u64,
				period: 10u64,
				period_count: 1u32,
				per_period: 100u64,
			}];

			let schedule_max = 500;

			mock::MAX_SCHEDULE.with(|v| *v.borrow_mut() = schedule_max);

			(0..schedule_max).for_each(|iter_index| {
				assert_ok!(Vesting::add_vesting_schedule(
					RuntimeOrigin::signed(ALICE::get()),
					BOB::get(),
					schedules[0].clone(),
				));
				assert_eq!(
					Vesting::vesting_schedules(BOB::get()).to_vec().len() as u32,
					iter_index + 1
				);
				assert_eq!(context_events().len() as u32, iter_index + 1);
			});

			assert_err!(
				Vesting::add_vesting_schedule(RuntimeOrigin::signed(ALICE::get()), BOB::get(), schedules[0].clone(),),
				<Error<Runtime>>::MaxScheduleOverflow,
			);

			assert_eq!(
				Vesting::vesting_schedules(BOB::get()).to_vec().len(),
				schedule_max as usize
			);
			assert_eq!(context_events().len(), schedule_max as usize);
		});
}

#[test]
fn renounce_only_works_for_cancel_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Vesting::renounce(RuntimeOrigin::signed(ALICE::get()), BOB::get()),
			BadOrigin
		);
	})
}

#[test]
fn renounce_privileges() {
	ExtBuilder::default().one_hundred_for_alice().build().execute_with(|| {
		let schedule = VestingSchedule {
			start: 0u64,
			period: 10u64,
			period_count: 2u32,
			per_period: 10u64,
		};
		assert_ok!(Vesting::add_vesting_schedule(
			RuntimeOrigin::signed(ALICE::get()),
			BOB::get(),
			schedule
		));

		assert!(!Vesting::renounced(BOB::get()));
		assert_ok!(Vesting::renounce(
			RuntimeOrigin::signed(CancelOrigin::get()),
			BOB::get()
		));
		assert!(Vesting::renounced(BOB::get()));
		assert_noop!(
			Vesting::cancel_all_vesting_schedules(
				RuntimeOrigin::signed(CancelOrigin::get()),
				BOB::get(),
				CancelOrigin::get()
			),
			Error::<Runtime>::Renounced
		);
	});
}
