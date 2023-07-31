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

use crate::pallet::User;
use crate::{mock::*, Error, Event, Pot, PotDetailsOf, UserDetailsOf};
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, ReservableCurrency},
};
use std::num::NonZeroU32;

#[test]
fn creator_of_pot_becomes_sponsor() {
	new_test_ext().execute_with(|| {
		let pot = 0;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
		));
		assert_eq!(Pot::<Test>::get(pot), Some(pot_details));
		System::assert_last_event(Event::PotCreated(pot).into());
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot.sponsor),
			pot_id,
			pot.sponsorship_type,
			pot.remained_fee_quota,
			pot.remained_reserve_quota
		));
		assert_noop!(
			SponsorshipModule::create_pot(
				RuntimeOrigin::signed(pot.sponsor),
				pot_id,
				pot.sponsorship_type,
				pot.remained_fee_quota,
				pot.remained_reserve_quota
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
		));
		assert_ok!(SponsorshipModule::remove_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot
		));
		assert_eq!(Pot::<Test>::get(pot), None);
		System::assert_last_event(Event::PotRemoved(pot).into());
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
		));
		assert_noop!(
			SponsorshipModule::remove_pot(RuntimeOrigin::signed(pot_details.sponsor + 1), pot),
			Error::<Test>::NoPermission
		);
		assert_eq!(Pot::<Test>::get(pot), Some(pot_details));
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
		));

		let common_fee_quota = 7;
		let common_reserve_quota = 12;

		let user_1 = 2u64;
		let user_details_1 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_1, &pot).unwrap(),
			remained_fee_quota: common_fee_quota,
			remained_reserve_quota: common_reserve_quota,
		};

		let user_2 = 17u64;
		let user_details_2 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_2, &pot).unwrap(),
			remained_fee_quota: common_fee_quota,
			remained_reserve_quota: common_reserve_quota,
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
		System::assert_last_event(Event::UsersRegistered(pot, vec![user_1, user_2]).into());

		let user_3 = 18u64;
		let user_3_fee_quota = common_fee_quota + 1;
		let user_3_reserve_quota = common_reserve_quota + 5;
		let user_details_3 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_3, &pot).unwrap(),
			remained_fee_quota: user_3_fee_quota,
			remained_reserve_quota: user_3_reserve_quota,
		};
		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_3],
			user_3_fee_quota,
			user_3_reserve_quota
		));
		assert_eq!(User::<Test>::get(pot, user_3), Some(user_details_3));
		System::assert_last_event(Event::UsersRegistered(pot, vec![user_3]).into());
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2]
		));

		assert_noop!(
			SponsorshipModule::remove_inactive_users(
				RuntimeOrigin::signed(pot_details.sponsor + 1),
				pot,
				NonZeroU32::new(10).unwrap()
			),
			Error::<Test>::NoPermission
		);

		assert_ok!(SponsorshipModule::remove_inactive_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			NonZeroU32::new(10).unwrap()
		));
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			Error::<Test>::ContainsUserWithNonZeroReserve
		);

		assert_ok!(SponsorshipModule::remove_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_3],
		));
		System::assert_last_event(Event::UsersRemoved(pot, vec![user_1, user_3]).into());
		assert_eq!(User::<Test>::iter_prefix_values(pot).count(), 1);
	});
}

#[test]
fn sponsors_can_remove_inactive_users() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_ok!(SponsorshipModule::remove_inactive_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			NonZeroU32::new(10).unwrap()
		));
		System::assert_last_event(Event::UsersRemoved(pot, vec![user_3, user_1]).into());
		assert_eq!(User::<Test>::iter_prefix_values(pot).count(), 1);
	});
}

#[test]
fn removing_inactive_users_respects_limit() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_ok!(SponsorshipModule::remove_inactive_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			NonZeroU32::new(1).unwrap()
		));
		System::assert_last_event(Event::UsersRemoved(pot, vec![user_3]).into());
		assert_eq!(User::<Test>::iter_prefix_values(pot).count(), 2);
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_eq!(Balances::free_balance(&user_2), user_2_free);
		assert_eq!(Balances::free_balance(&user_3), user_3_free);

		let user_4 = 29u64;
		let user_5 = 37u64;

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_4, user_5],
			common_fee_quota,
			common_reserve_quota
		));

		let user_4_free = 19;
		let user_4_details = User::<Test>::get(pot, user_4).unwrap();
		Balances::make_free_balance_be(&user_4_details.proxy, user_4_free);

		let user_5_free = 29;
		let user_5_details = User::<Test>::get(pot, user_5).unwrap();
		Balances::make_free_balance_be(&user_5_details.proxy, user_5_free);

		assert_ok!(SponsorshipModule::remove_inactive_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			NonZeroU32::new(10).unwrap()
		));

		assert_eq!(Balances::free_balance(&user_4), user_4_free);
		assert_eq!(Balances::free_balance(&user_5), user_5_free);
	});
}
