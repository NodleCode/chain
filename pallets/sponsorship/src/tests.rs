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

use crate::{mock::*, Call, ChargeSponsor, Error, Event, Pot, PotDetailsOf, User, UserDetailsOf};
use frame_support::dispatch::DispatchResult;
use frame_support::{
	assert_err, assert_noop, assert_ok,
	dispatch::GetDispatchInfo,
	traits::{Currency, ReservableCurrency},
};
use sp_runtime::transaction_validity::ValidTransaction;
use sp_runtime::{
	traits::SignedExtension,
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	DispatchError, TokenError,
};
use support::LimitedBalance;

type NegativeImbalance =
	<<Test as crate::Config>::Currency as Currency<<Test as frame_system::Config>::AccountId>>::NegativeImbalance;

#[test]
fn creator_of_pot_becomes_sponsor() {
	new_test_ext().execute_with(|| {
		let pot = 0;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		assert_eq!(Pot::<Test>::get(pot), Some(pot_details));
		System::assert_last_event(Event::PotCreated { pot }.into());
	});
}

#[test]
fn pot_creation_fails_if_pot_exists() {
	new_test_ext().execute_with(|| {
		let pot_id = 11;
		System::set_block_number(1);
		let pot = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot.sponsor),
			pot_id,
			pot.sponsorship_type,
			pot.fee_quota.limit(),
			pot.reserve_quota.limit()
		));
		assert_noop!(
			SponsorshipModule::create_pot(
				RuntimeOrigin::signed(pot.sponsor),
				pot_id,
				pot.sponsorship_type,
				pot.fee_quota.limit(),
				pot.reserve_quota.limit()
			),
			Error::<Test>::InUse
		);
	});
}

#[test]
fn sponsors_can_remove_user_free_pots() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		assert_ok!(SponsorshipModule::remove_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot
		));
		assert_eq!(Pot::<Test>::get(pot), None);
		System::assert_last_event(Event::PotRemoved { pot }.into());

		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
	});
}

#[test]
fn only_sponsors_have_permission_to_remove_pots() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		assert_noop!(
			SponsorshipModule::remove_pot(RuntimeOrigin::signed(pot_details.sponsor + 1), pot),
			Error::<Test>::NoPermission
		);
		assert_eq!(Pot::<Test>::get(pot), Some(pot_details));
	});
}

#[test]
fn updating_non_existing_pot_is_error() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_noop!(
			SponsorshipModule::update_pot_limits(RuntimeOrigin::signed(pot_details.sponsor), pot, 6, 8),
			Error::<Test>::PotNotExist
		);
		assert_noop!(
			SponsorshipModule::update_sponsorship_type(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				SponsorshipType::AnySafe
			),
			Error::<Test>::PotNotExist
		);
	});
}

#[test]
fn only_sponsors_have_permission_to_update_pots() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		assert_noop!(
			SponsorshipModule::update_pot_limits(RuntimeOrigin::signed(pot_details.sponsor + 1), pot, 6, 8),
			Error::<Test>::NoPermission
		);
		assert_ok!(SponsorshipModule::update_pot_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			6,
			8
		));
		assert_noop!(
			SponsorshipModule::update_sponsorship_type(
				RuntimeOrigin::signed(pot_details.sponsor + 1),
				pot,
				SponsorshipType::Balances
			),
			Error::<Test>::NoPermission
		);
		assert_ok!(SponsorshipModule::update_sponsorship_type(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			SponsorshipType::Balances
		));
		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(pot_details.sponsorship_type, SponsorshipType::Balances);
	});
}

#[test]
fn sponsors_can_always_increase_pot_limits() {
	new_test_ext().execute_with(|| {
		let unused_pot = 3;
		System::set_block_number(1);
		let unused_pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		Pot::<Test>::insert(unused_pot, &unused_pot_details);
		assert_ok!(SponsorshipModule::update_pot_limits(
			RuntimeOrigin::signed(unused_pot_details.sponsor),
			unused_pot,
			unused_pot_details.fee_quota.limit() + 1,
			unused_pot_details.reserve_quota.limit() + 1
		));
		System::assert_last_event(Event::PotUpdated { pot: unused_pot }.into());
		let updated_pot = Pot::<Test>::get(unused_pot).unwrap();
		assert_eq!(updated_pot.fee_quota.limit(), unused_pot_details.fee_quota.limit() + 1);
		assert_eq!(
			updated_pot.reserve_quota.limit(),
			unused_pot_details.reserve_quota.limit() + 1
		);

		let fully_used_pot = 4;
		let mut fully_used_pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		fully_used_pot_details.fee_quota.add(5).unwrap();
		fully_used_pot_details.reserve_quota.add(7).unwrap();
		Pot::<Test>::insert(fully_used_pot, &fully_used_pot_details);

		assert_ok!(SponsorshipModule::update_pot_limits(
			RuntimeOrigin::signed(fully_used_pot_details.sponsor),
			fully_used_pot,
			fully_used_pot_details.fee_quota.limit() + 1,
			fully_used_pot_details.reserve_quota.limit() + 1
		));
		System::assert_last_event(Event::PotUpdated { pot: fully_used_pot }.into());
		let updated_pot = Pot::<Test>::get(fully_used_pot).unwrap();
		assert_eq!(
			updated_pot.fee_quota.limit(),
			fully_used_pot_details.fee_quota.limit() + 1
		);
		assert_eq!(
			updated_pot.reserve_quota.limit(),
			fully_used_pot_details.reserve_quota.limit() + 1
		);
	});
}

#[test]
fn sponsors_can_set_same_limits_when_updating_pots() {
	new_test_ext().execute_with(|| {
		let unused_pot = 3;
		System::set_block_number(1);
		let unused_pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		Pot::<Test>::insert(unused_pot, &unused_pot_details);

		assert_ok!(SponsorshipModule::update_pot_limits(
			RuntimeOrigin::signed(unused_pot_details.sponsor),
			unused_pot,
			unused_pot_details.fee_quota.limit(),
			unused_pot_details.reserve_quota.limit()
		));

		let updated_pot = Pot::<Test>::get(unused_pot).unwrap();
		assert_eq!(updated_pot.fee_quota, unused_pot_details.fee_quota);
		assert_eq!(updated_pot.reserve_quota, unused_pot_details.reserve_quota);
	});
}

#[test]
fn sponsors_can_decrease_pot_limits_only_when_available_margin_allows() {
	new_test_ext().execute_with(|| {
		let partially_used_pot = 4;
		System::set_block_number(1);
		let mut partially_used_pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		partially_used_pot_details.fee_quota.add(4).unwrap();
		partially_used_pot_details.reserve_quota.add(3).unwrap();
		Pot::<Test>::insert(partially_used_pot, &partially_used_pot_details);

		assert_noop!(
			SponsorshipModule::update_pot_limits(
				RuntimeOrigin::signed(partially_used_pot_details.sponsor),
				partially_used_pot,
				partially_used_pot_details.fee_quota.limit()
					- partially_used_pot_details.fee_quota.available_margin()
					- 1,
				partially_used_pot_details.reserve_quota.limit()
			),
			Error::<Test>::CannotUpdateFeeLimit
		);

		assert_noop!(
			SponsorshipModule::update_pot_limits(
				RuntimeOrigin::signed(partially_used_pot_details.sponsor),
				partially_used_pot,
				partially_used_pot_details.fee_quota.limit(),
				partially_used_pot_details.reserve_quota.limit()
					- partially_used_pot_details.reserve_quota.available_margin()
					- 1
			),
			Error::<Test>::CannotUpdateReserveLimit
		);

		assert_ok!(SponsorshipModule::update_pot_limits(
			RuntimeOrigin::signed(partially_used_pot_details.sponsor),
			partially_used_pot,
			partially_used_pot_details.fee_quota.limit() - partially_used_pot_details.fee_quota.available_margin(),
			partially_used_pot_details.reserve_quota.limit()
				- partially_used_pot_details.reserve_quota.available_margin()
		));

		System::assert_last_event(
			Event::PotUpdated {
				pot: partially_used_pot,
			}
			.into(),
		);

		let updated_pot = Pot::<Test>::get(partially_used_pot).unwrap();
		assert_eq!(updated_pot.fee_quota.available_margin(), 0);
		assert_eq!(updated_pot.reserve_quota.available_margin(), 0);
	});
}

#[test]
fn sponsors_cannot_remove_pots_with_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		let user = 2u64;
		User::<Test>::insert(pot, user, UserDetailsOf::<Test>::default());
		assert_noop!(
			SponsorshipModule::remove_pot(RuntimeOrigin::signed(pot_details.sponsor), pot),
			Error::<Test>::InUse
		);
		User::<Test>::remove(pot, user);
		assert_ok!(SponsorshipModule::remove_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot
		));
	});
}

#[test]
fn sponsors_can_register_new_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_details_1 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_1, &pot).unwrap(),
			fee_quota: LimitedBalance::with_limit(common_fee_quota),
			reserve_quota: LimitedBalance::with_limit(common_reserve_quota),
		};

		let user_2 = 17u64;
		let user_details_2 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_2, &pot).unwrap(),
			fee_quota: LimitedBalance::with_limit(common_fee_quota),
			reserve_quota: LimitedBalance::with_limit(common_reserve_quota),
		};

		assert_eq!(User::<Test>::get(pot, user_1), None);
		assert_eq!(User::<Test>::get(pot, user_2), None);
		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2],
			common_fee_quota,
			common_reserve_quota
		));
		assert_eq!(User::<Test>::get(pot, user_1), Some(user_details_1));
		assert_eq!(User::<Test>::get(pot, user_2), Some(user_details_2));
		System::assert_last_event(
			Event::UsersRegistered {
				pot,
				users: vec![user_1, user_2],
			}
			.into(),
		);

		let user_3 = 18u64;
		let user_3_fee_quota = common_fee_quota + 1;
		let user_3_reserve_quota = common_reserve_quota + 5;
		let user_details_3 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_3, &pot).unwrap(),
			fee_quota: LimitedBalance::with_limit(user_3_fee_quota),
			reserve_quota: LimitedBalance::with_limit(user_3_reserve_quota),
		};
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_details_3.proxy), 0);
		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_3],
			user_3_fee_quota,
			user_3_reserve_quota
		));
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3), 1);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_details_3.proxy), 1);
		assert_eq!(User::<Test>::get(pot, user_3), Some(user_details_3));
		System::assert_last_event(
			Event::UsersRegistered {
				pot,
				users: vec![user_3],
			}
			.into(),
		);
	});
}

#[test]
fn sponsors_cannot_register_users_more_than_once() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;
		let user_4 = 24u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2],
			common_fee_quota,
			common_reserve_quota
		));

		assert_noop!(
			SponsorshipModule::register_users(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				vec![user_2, user_3],
				common_fee_quota,
				common_reserve_quota
			),
			Error::<Test>::UserAlreadyRegistered
		);

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_3, user_4],
			common_fee_quota,
			common_reserve_quota
		));
	});
}

#[test]
fn only_sponsors_have_permission_to_register_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));
		assert_noop!(
			SponsorshipModule::register_users(RuntimeOrigin::signed(2), pot, vec![3, 4], 7, 12),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn only_sponsors_have_permission_to_remove_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_noop!(
			SponsorshipModule::remove_users(
				RuntimeOrigin::signed(pot_details.sponsor + 1),
				pot,
				vec![user_1, user_2]
			),
			Error::<Test>::NoPermission
		);

		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_1), 1);
		let user_1_proxy = <User<Test>>::get(pot, user_1).unwrap().proxy;
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_1_proxy), 1);
		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2]
		));
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_1), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_1_proxy), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3), 1);
		let user_3_proxy = <User<Test>>::get(pot, user_3).unwrap().proxy;
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3_proxy), 1);

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2],
			common_fee_quota,
			common_reserve_quota
		));
	});
}

#[test]
fn only_sponsors_have_permission_to_update_users_limits() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_noop!(
			SponsorshipModule::update_users_limits(
				RuntimeOrigin::signed(pot_details.sponsor + 1),
				pot,
				common_fee_quota + 1,
				common_reserve_quota + 1,
				vec![user_1, user_2]
			),
			Error::<Test>::NoPermission
		);

		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			common_fee_quota + 1,
			common_reserve_quota + 1,
			vec![user_1, user_2]
		));
		System::assert_last_event(
			Event::UsersLimitsUpdated {
				pot,
				users: vec![user_1, user_2],
			}
			.into(),
		);
	});
}

#[test]
fn updating_user_limits_for_non_existing_pots_is_error() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;

		assert_noop!(
			SponsorshipModule::register_users(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				vec![user_1],
				common_fee_quota,
				common_reserve_quota
			),
			Error::<Test>::PotNotExist
		);
	});
}

#[test]
fn updating_users_limits_only_impacts_the_given_list() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			common_fee_quota + 1,
			common_reserve_quota + 1,
			vec![user_1, user_3]
		),);

		let user_1_details = <User<Test>>::get(pot, user_1).unwrap();
		let user_2_details = <User<Test>>::get(pot, user_2).unwrap();
		let user_3_details = <User<Test>>::get(pot, user_3).unwrap();

		assert_eq!(user_1_details.fee_quota.limit(), common_fee_quota + 1);
		assert_eq!(user_1_details.reserve_quota.limit(), common_reserve_quota + 1);
		assert_eq!(user_2_details.fee_quota.limit(), common_fee_quota);
		assert_eq!(user_2_details.reserve_quota.limit(), common_reserve_quota);
		assert_eq!(user_3_details.fee_quota.limit(), common_fee_quota + 1);
		assert_eq!(user_3_details.reserve_quota.limit(), common_reserve_quota + 1);
	});
}

#[test]
fn sponsors_can_always_set_user_limits_to_an_amount_equal_or_greater_than_before() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1],
			common_fee_quota,
			common_reserve_quota
		));

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_2],
			common_fee_quota - 1,
			common_reserve_quota - 1
		));

		let mut user_1_details = <User<Test>>::get(pot, user_1).unwrap();
		let mut user_2_details = <User<Test>>::get(pot, user_2).unwrap();
		user_1_details.fee_quota.add(common_fee_quota).unwrap();
		user_1_details.reserve_quota.add(common_reserve_quota).unwrap();
		user_2_details.fee_quota.add(common_fee_quota - 1).unwrap();
		user_2_details.reserve_quota.add(common_reserve_quota - 1).unwrap();
		<User<Test>>::insert(pot, user_1, user_1_details);
		<User<Test>>::insert(pot, user_2, user_2_details);

		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			common_fee_quota + 1,
			common_reserve_quota + 1,
			vec![user_1, user_2]
		),);
		System::assert_last_event(
			Event::UsersLimitsUpdated {
				pot,
				users: vec![user_1, user_2],
			}
			.into(),
		);

		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			common_fee_quota + 1,
			common_reserve_quota + 1,
			vec![user_1, user_2]
		),);

		let user_1_details = <User<Test>>::get(pot, user_1).unwrap();
		let user_2_details = <User<Test>>::get(pot, user_2).unwrap();

		assert_eq!(user_1_details.fee_quota.limit(), common_fee_quota + 1);
		assert_eq!(user_1_details.reserve_quota.limit(), common_reserve_quota + 1);
		assert_eq!(user_2_details.fee_quota.limit(), common_fee_quota + 1);
		assert_eq!(user_2_details.reserve_quota.limit(), common_reserve_quota + 1);
	});
}

#[test]
fn sponsors_can_reduce_user_limits_when_available_margin_allows() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2],
			common_fee_quota,
			common_reserve_quota
		));

		let mut user_1_details = <User<Test>>::get(pot, user_1).unwrap();
		user_1_details.fee_quota.add(common_fee_quota / 2).unwrap();
		user_1_details.reserve_quota.add(common_reserve_quota / 2).unwrap();
		<User<Test>>::insert(pot, user_1, user_1_details.clone());

		let lowest_fee_limit = user_1_details.fee_quota.limit() - user_1_details.fee_quota.available_margin();
		let lowest_reserve_limit =
			user_1_details.reserve_quota.limit() - user_1_details.reserve_quota.available_margin();

		assert_noop!(
			SponsorshipModule::update_users_limits(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				lowest_fee_limit - 1,
				lowest_reserve_limit,
				vec![user_1, user_2]
			),
			Error::<Test>::CannotUpdateFeeLimit
		);

		assert_noop!(
			SponsorshipModule::update_users_limits(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				lowest_fee_limit,
				lowest_reserve_limit - 1,
				vec![user_1, user_2]
			),
			Error::<Test>::CannotUpdateReserveLimit
		);

		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			lowest_fee_limit,
			lowest_reserve_limit,
			vec![user_1]
		),);
		System::assert_last_event(
			Event::UsersLimitsUpdated {
				pot,
				users: vec![user_1],
			}
			.into(),
		);

		// User 2 has bigger available margin, so it can be updated to lower limits separately
		assert_ok!(SponsorshipModule::update_users_limits(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			lowest_fee_limit - 1,
			lowest_reserve_limit - 1,
			vec![user_2]
		),);
		System::assert_last_event(
			Event::UsersLimitsUpdated {
				pot,
				users: vec![user_2],
			}
			.into(),
		);

		let user_1_details = <User<Test>>::get(pot, user_1).unwrap();
		assert_eq!(user_1_details.fee_quota.available_margin(), 0);
		assert_eq!(user_1_details.reserve_quota.available_margin(), 0);
	});
}

#[test]
fn sponsors_can_update_user_limits_for_registered_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;
		let non_registered_user = 42u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_noop!(
			SponsorshipModule::update_users_limits(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				common_fee_quota + 1,
				common_reserve_quota + 1,
				vec![user_1, non_registered_user]
			),
			Error::<Test>::UserNotRegistered
		);
	});
}

#[test]
fn sponsors_can_remove_users_with_no_reserve_in_their_proxies() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		let user_2_reserve = 19;
		let user_2_details = User::<Test>::get(pot, user_2).unwrap();
		Balances::make_free_balance_be(&user_2_details.proxy, user_2_reserve);
		assert_ok!(Balances::reserve(&user_2_details.proxy, user_2_reserve - 1));

		assert_noop!(
			SponsorshipModule::remove_users(RuntimeOrigin::signed(pot_details.sponsor), pot, vec![user_1, user_2],),
			Error::<Test>::CannotRemoveProxy
		);

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_3],
		));
		System::assert_last_event(
			Event::UsersRemoved {
				pot,
				users: vec![user_1, user_3],
			}
			.into(),
		);
		assert_eq!(User::<Test>::iter_prefix_values(pot).count(), 1);
	});
}

#[test]
fn sponsors_cannot_remove_unregistered_user() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_noop!(
			SponsorshipModule::remove_users(
				RuntimeOrigin::signed(pot_details.sponsor),
				pot,
				vec![user_1, user_2, user_3],
			),
			Error::<Test>::UserNotRegistered
		);
	});
}

#[test]
fn users_get_their_free_balance_back_in_their_original_account_after_being_removed() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64;
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		let user_2_free = 19;
		let user_2_details = User::<Test>::get(pot, user_2).unwrap();
		Balances::make_free_balance_be(&user_2_details.proxy, user_2_free);

		let user_3_free = 29;
		let user_3_details = User::<Test>::get(pot, user_3).unwrap();
		Balances::make_free_balance_be(&user_3_details.proxy, user_3_free);

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
		));

		assert_eq!(Balances::free_balance(user_1), 0);
		assert_eq!(Balances::free_balance(user_2), user_2_free);
		assert_eq!(Balances::free_balance(user_3), user_3_free);
	});
}

#[test]
fn sponsorship_filter_will_block_undesired_sponsor_for_calls() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let uniques_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32,
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_call
		));
		let user_details = User::<Test>::get(pot, user).unwrap();
		System::assert_last_event(
			Event::Sponsored {
				paid: user_reserve_quota,
				repaid: user_reserve_quota - user_details.reserve_quota.balance(),
			}
			.into(),
		);

		let balances_call = Box::new(RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
			dest: user,
			value: 1,
		}));
		assert_noop!(
			SponsorshipModule::sponsor_for(RuntimeOrigin::signed(user), pot, balances_call),
			frame_system::Error::<Test>::CallFiltered
		);
		assert_eq!(Balances::free_balance(user), 0);
	});
}

#[test]
fn sponsor_for_calls_will_fail_if_call_itself_should_fail() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Balances,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let balances_call = Box::new(RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
			dest: user,
			value: user_reserve_quota + 1,
		}));
		assert_noop!(
			SponsorshipModule::sponsor_for(RuntimeOrigin::signed(user), pot, balances_call),
			DispatchError::Token(TokenError::FundsUnavailable)
		);

		assert_eq!(Balances::free_balance(user), 0);
	});
}

#[test]
fn sponsor_for_calls_will_fail_if_call_leaks_balance_out_of_proxy_account() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Balances,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let balances_call = Box::new(RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
			dest: user,
			value: 1,
		}));
		assert_noop!(
			SponsorshipModule::sponsor_for(RuntimeOrigin::signed(user), pot, balances_call),
			Error::<Test>::BalanceLeak
		);

		assert_eq!(Balances::free_balance(user), 0);
	});
}

#[test]
fn sponsor_for_calls_will_repay_unused_reserve() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let uniques_create_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32,
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call
		));
		let user_details = User::<Test>::get(pot, user).unwrap();
		let uniques_call_reserve = user_details.reserve_quota.balance() - Balances::minimum_balance();
		assert_eq!(Balances::free_balance(user_details.proxy), Balances::minimum_balance());
		assert_eq!(Balances::reserved_balance(user_details.proxy), uniques_call_reserve);
		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(
			pot_details.reserve_quota.available_margin(),
			pot_reserve_quota - user_details.reserve_quota.balance()
		);

		let uniques_destroy_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::destroy {
			collection: 0u32,
			witness: pallet_uniques::DestroyWitness {
				items: 0,
				item_metadatas: 0,
				attributes: 0,
			},
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_destroy_call
		));
		System::assert_last_event(
			Event::Sponsored {
				paid: user_reserve_quota - user_details.reserve_quota.balance(),
				repaid: user_reserve_quota - Balances::minimum_balance(),
			}
			.into(),
		);
		assert_eq!(Balances::free_balance(user_details.proxy), Balances::minimum_balance());
		assert_eq!(Balances::reserved_balance(user_details.proxy), 0);
		assert_eq!(
			Balances::free_balance(pot_details.sponsor),
			pot_reserve_quota - Balances::minimum_balance()
		);
		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(
			pot_details.reserve_quota.available_margin(),
			pot_reserve_quota - Balances::minimum_balance()
		);
		let user_1_details = User::<Test>::get(pot, user).unwrap();
		assert_eq!(
			user_1_details.reserve_quota.available_margin(),
			user_1_details.reserve_quota.limit() - Balances::minimum_balance()
		);
	});
}

#[test]
fn users_pay_back_more_debts_on_sponsor_for_calls_if_their_free_balance_allows() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let uniques_create_call_1 = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32,
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call_1
		));

		let user_details = User::<Test>::get(pot, user).unwrap();
		let user_owing = user_details.reserve_quota.balance();
		assert_eq!(user_owing, TestCollectionDeposit::get() + Balances::minimum_balance());

		// Assume user's proxy account somehow earns enough funds through an interaction or from an external source.
		let user_free_balance_after_earning = 2 * user_details.reserve_quota.balance();
		Balances::make_free_balance_be(&user_details.proxy, user_free_balance_after_earning);

		let uniques_create_call_2 = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 1u32,
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call_2
		));

		System::assert_last_event(
			Event::Sponsored {
				paid: user_reserve_quota - user_free_balance_after_earning,
				repaid: user_reserve_quota - user_free_balance_after_earning + user_owing,
			}
			.into(),
		);

		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(Balances::free_balance(pot_details.sponsor), pot_reserve_quota);
		assert_eq!(pot_details.reserve_quota.available_margin(), pot_reserve_quota);

		let user_details = User::<Test>::get(pot, user).unwrap();
		assert_eq!(user_details.reserve_quota.balance(), 0);
	});
}

#[test]
fn users_pay_back_debts_when_removed_if_their_free_balance_allows() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 100;
		let user_reserve_quota = pot_reserve_quota / 100;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user_1 = 2u64;
		let user_2 = 21u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2],
			user_fee_quota,
			user_reserve_quota
		));

		let uniques_create_call_1 = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32,
			admin: user_1,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user_1),
			pot,
			uniques_create_call_1
		));
		let uniques_create_call_2 = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 1u32,
			admin: user_2,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user_2),
			pot,
			uniques_create_call_2
		));

		let pot_details = Pot::<Test>::get(pot).unwrap();
		let user_1_details = User::<Test>::get(pot, user_1).unwrap();
		let user_2_details = User::<Test>::get(pot, user_2).unwrap();

		let user_1_owing = user_1_details.reserve_quota.balance();
		assert_eq!(user_1_owing, TestCollectionDeposit::get() + Balances::minimum_balance());
		let user_2_owing = user_2_details.reserve_quota.balance();
		assert_eq!(
			pot_details.reserve_quota.available_margin(),
			pot_reserve_quota - user_1_owing - user_2_owing
		);

		// Now for the sake of this test let's manipulate user's proxy account so we can remove this
		// user from the pot.
		Balances::unreserve(&user_1_details.proxy, user_1_owing - Balances::minimum_balance());
		Balances::unreserve(&user_2_details.proxy, user_1_owing - Balances::minimum_balance());

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1]
		));

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_2]
		));

		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(Balances::free_balance(pot_details.sponsor), pot_reserve_quota);
		assert_eq!(pot_details.reserve_quota.available_margin(), pot_reserve_quota);
	});
}

#[test]
fn pallet_continues_to_provide_user_when_removed_from_one_pot_but_still_exists_in_other_pots() {
	new_test_ext().execute_with(|| {
		let pot_1 = 3;
		let pot_2 = 4;

		System::set_block_number(1);

		let pot_details_1 = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(5),
			reserve_quota: LimitedBalance::with_limit(7),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details_1.sponsor),
			pot_1,
			pot_details_1.sponsorship_type,
			pot_details_1.fee_quota.limit(),
			pot_details_1.reserve_quota.limit()
		));

		let pot_details_2 = PotDetailsOf::<Test> {
			sponsor: 2,
			sponsorship_type: SponsorshipType::AnySafe,
			fee_quota: LimitedBalance::with_limit(10),
			reserve_quota: LimitedBalance::with_limit(14),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details_2.sponsor),
			pot_2,
			pot_details_2.sponsorship_type,
			pot_details_2.fee_quota.limit(),
			pot_details_2.reserve_quota.limit()
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_2 = 17u64; // Will be in both pots.
		let user_3 = 23u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details_1.sponsor),
			pot_1,
			vec![user_1, user_2],
			common_fee_quota,
			common_reserve_quota
		));
		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details_2.sponsor),
			pot_2,
			vec![user_2, user_3],
			common_fee_quota,
			common_reserve_quota
		));

		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2), 1);
		let user_2_proxy_1 = <User<Test>>::get(pot_1, user_2).unwrap().proxy;
		let user_2_proxy_2 = <User<Test>>::get(pot_2, user_2).unwrap().proxy;
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2_proxy_1), 1);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2_proxy_2), 1);
		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details_1.sponsor),
			pot_1,
			vec![user_2]
		));
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2), 1);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2_proxy_1), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2_proxy_2), 1);

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details_2.sponsor),
			pot_2,
			vec![user_2]
		));
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_2_proxy_2), 0);
	});
}

#[test]
fn debug_format_of_charge_sponsor() {
	let charge_sponsor = ChargeSponsor::<Test>::default();
	assert_eq!(
		format!("{charge_sponsor:?}"),
		"ChargeTransactionPayment<PhantomData<u64>>"
	);
}

#[test]
fn charging_sponsors_does_not_need_additional_signed_data() {
	let charge_sponsor = ChargeSponsor::<Test>::default();
	assert_eq!(charge_sponsor.additional_signed(), Ok(()));
}

#[test]
fn sponsor_call_for_existing_pot_from_registered_user_with_enough_fee_limit_is_valid() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_eq!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			Ok(ValidTransaction::default())
		);
	});
}

#[test]
fn valid_sponsor_call_for_yields_correct_pre_dispatch_details() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));
		let user_details = User::<Test>::get(pot, user).unwrap();

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		let pre_dispatch_details = ChargeSponsor::<Test>::default()
			.pre_dispatch(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0,
			)
			.unwrap()
			.unwrap();
		assert_eq!(pre_dispatch_details.pot, pot);
		assert_eq!(pre_dispatch_details.pot_details, pot_details);
		assert_eq!(pre_dispatch_details.user, user);
		assert_eq!(pre_dispatch_details.user_details, user_details);

		assert!(matches!(
			pre_dispatch_details.fee_imbalance,
			Some(NegativeImbalance { .. })
		));
	});
}

#[test]
fn valid_sponsor_call_settle_paid_fee_post_dispatch() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));
		let user_details = User::<Test>::get(pot, user).unwrap();

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		let pre_dispatch_details = ChargeSponsor::<Test>::default()
			.pre_dispatch(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0,
			)
			.ok();

		assert_ok!(ChargeSponsor::<Test>::post_dispatch(
			pre_dispatch_details,
			&sponsor_for_uniques_create_call.get_dispatch_info(),
			&().into(),
			0,
			&DispatchResult::Ok(())
		));

		let user_details_post_dispatch = User::<Test>::get(pot, user).unwrap();
		let pot_details_post_dispatch = Pot::<Test>::get(pot).unwrap();

		let fee = pot_details_post_dispatch.fee_quota.balance() - pot_details.fee_quota.balance();
		assert_ne!(fee, 0);
		assert_eq!(
			user_details_post_dispatch.fee_quota.balance() - user_details.fee_quota.balance(),
			fee
		);

		System::assert_last_event(
			Event::TransactionFeePaid {
				sponsor: pot_details.sponsor,
				fee,
			}
			.into(),
		);
		assert_eq!(Balances::free_balance(pot_details.sponsor), pot_reserve_quota - fee);
	});
}

#[test]
fn post_dispatch_for_non_valid_sponsor_calls_is_noop() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));
		let user_details = User::<Test>::get(pot, user).unwrap();

		let right_call_type_but_wrong_pot = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 2,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));
		let pre_dispatch_details = ChargeSponsor::<Test>::default()
			.pre_dispatch(
				&user,
				&right_call_type_but_wrong_pot,
				&right_call_type_but_wrong_pot.get_dispatch_info(),
				0,
			)
			.ok();
		assert!(pre_dispatch_details.is_none());
		assert_ok!(ChargeSponsor::<Test>::post_dispatch(
			pre_dispatch_details,
			&right_call_type_but_wrong_pot.get_dispatch_info(),
			&().into(),
			0,
			&DispatchResult::Ok(())
		));

		let disallowed_call_type = Box::new(RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
			dest: user,
			value: 1,
		}));
		let pre_dispatch_details = ChargeSponsor::<Test>::default()
			.pre_dispatch(
				&user,
				&disallowed_call_type,
				&disallowed_call_type.get_dispatch_info(),
				0,
			)
			.ok();
		assert!(pre_dispatch_details.is_some());
		assert_ok!(ChargeSponsor::<Test>::post_dispatch(
			pre_dispatch_details,
			&disallowed_call_type.get_dispatch_info(),
			&().into(),
			0,
			&DispatchResult::Ok(())
		));

		let user_details_post_dispatch = User::<Test>::get(pot, user).unwrap();
		let pot_details_post_dispatch = Pot::<Test>::get(pot).unwrap();

		let fee = pot_details_post_dispatch.fee_quota.balance() - pot_details.fee_quota.balance();
		assert_eq!(fee, 0);
		assert_eq!(
			user_details_post_dispatch.fee_quota.balance() - user_details.fee_quota.balance(),
			fee
		);
		assert_eq!(Balances::free_balance(pot_details.sponsor), pot_reserve_quota - fee);
	});
}

#[test]
fn non_sponsor_calls_is_not_the_sponsorship_module_business() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let user = 2u64;

		let uniques_create_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32,
			admin: user,
		}));

		assert_eq!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&uniques_create_call,
				&uniques_create_call.get_dispatch_info(),
				0
			),
			Ok(ValidTransaction::default())
		);

		let pre_dispatch_details = ChargeSponsor::<Test>::default()
			.pre_dispatch(&user, &uniques_create_call, &uniques_create_call.get_dispatch_info(), 0)
			.unwrap();
		assert!(pre_dispatch_details.is_none());
	});
}

#[test]
fn sponsor_call_for_non_existing_pot_is_invalid() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 4, // non existing pot
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_err!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			TransactionValidityError::Invalid(InvalidTransaction::Call)
		);

		assert_eq!(
			ChargeSponsor::<Test>::default()
				.pre_dispatch(
					&user,
					&sponsor_for_uniques_create_call,
					&sponsor_for_uniques_create_call.get_dispatch_info(),
					0,
				)
				.err(),
			Some(TransactionValidityError::Invalid(InvalidTransaction::Call))
		);
	});
}

#[test]
fn sponsor_call_for_not_registered_user_is_invalid() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;
		let non_registered_user = 3u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_err!(
			ChargeSponsor::<Test>::default().validate(
				&non_registered_user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			TransactionValidityError::Invalid(InvalidTransaction::BadSigner)
		);

		assert_eq!(
			ChargeSponsor::<Test>::default()
				.pre_dispatch(
					&non_registered_user,
					&sponsor_for_uniques_create_call,
					&sponsor_for_uniques_create_call.get_dispatch_info(),
					0,
				)
				.err(),
			Some(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))
		);
	});
}

#[test]
fn sponsor_call_is_invalid_if_pot_is_running_low() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_err!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			TransactionValidityError::Invalid(InvalidTransaction::Payment)
		);

		assert_eq!(
			ChargeSponsor::<Test>::default()
				.pre_dispatch(
					&user,
					&sponsor_for_uniques_create_call,
					&sponsor_for_uniques_create_call.get_dispatch_info(),
					0,
				)
				.err(),
			Some(TransactionValidityError::Invalid(InvalidTransaction::Payment))
		);
	});
}

#[test]
fn sponsor_call_is_invalid_if_user_limit_is_not_enough() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 1000000; // low user limit
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, pot_reserve_quota);

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_err!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			TransactionValidityError::Invalid(InvalidTransaction::Payment)
		);

		assert_eq!(
			ChargeSponsor::<Test>::default()
				.pre_dispatch(
					&user,
					&sponsor_for_uniques_create_call,
					&sponsor_for_uniques_create_call.get_dispatch_info(),
					0,
				)
				.err(),
			Some(TransactionValidityError::Invalid(InvalidTransaction::Payment))
		);
	});
}

#[test]
fn sponsor_call_is_invalid_if_sponsor_account_is_running_low() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			fee_quota: LimitedBalance::with_limit(pot_fee_quota),
			reserve_quota: LimitedBalance::with_limit(pot_reserve_quota),
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type,
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit()
		));

		let user_fee_quota = pot_fee_quota / 10;
		let user_reserve_quota = pot_reserve_quota / 10;

		Balances::make_free_balance_be(&pot_details.sponsor, 1); // low balance for sponsor

		let user = 2u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user],
			user_fee_quota,
			user_reserve_quota
		));

		let sponsor_for_uniques_create_call = Box::new(RuntimeCall::SponsorshipModule(Call::sponsor_for {
			pot: 3,
			call: Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
				collection: 0u32,
				admin: user,
			})),
		}));

		assert_err!(
			ChargeSponsor::<Test>::default().validate(
				&user,
				&sponsor_for_uniques_create_call,
				&sponsor_for_uniques_create_call.get_dispatch_info(),
				0
			),
			TransactionValidityError::Invalid(InvalidTransaction::Payment)
		);

		assert_eq!(
			ChargeSponsor::<Test>::default()
				.pre_dispatch(
					&user,
					&sponsor_for_uniques_create_call,
					&sponsor_for_uniques_create_call.get_dispatch_info(),
					0,
				)
				.err(),
			Some(TransactionValidityError::Invalid(InvalidTransaction::Payment))
		);
	});
}
