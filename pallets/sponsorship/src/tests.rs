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
use std::num::NonZeroU32;
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
		System::assert_last_event(Event::PotRemoved { pot }.into());

		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			reserve: LimitedBalance::with_limit(common_reserve_quota),
		};

		let user_2 = 17u64;
		let user_details_2 = UserDetailsOf::<Test> {
			proxy: SponsorshipModule::pure_account(&user_2, &pot).unwrap(),
			remained_fee_quota: common_fee_quota,
			reserve: LimitedBalance::with_limit(common_reserve_quota),
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
			remained_fee_quota: user_3_fee_quota,
			reserve: LimitedBalance::with_limit(user_3_reserve_quota),
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

		assert_noop!(
			SponsorshipModule::remove_inactive_users(
				RuntimeOrigin::signed(pot_details.sponsor + 1),
				pot,
				NonZeroU32::new(10).unwrap()
			),
			Error::<Test>::NoPermission
		);

		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3), 1);
		let user_3_proxy = <User<Test>>::get(pot, user_3).unwrap().proxy;
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3_proxy), 1);
		assert_ok!(SponsorshipModule::remove_inactive_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			NonZeroU32::new(10).unwrap()
		));
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3), 0);
		assert_eq!(frame_system::Pallet::<Test>::reference_count(&user_3_proxy), 0);

		assert_ok!(SponsorshipModule::register_users(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			vec![user_1, user_2, user_3],
			common_fee_quota,
			common_reserve_quota
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
		System::assert_last_event(
			Event::UsersRemoved {
				pot,
				users: vec![user_3, user_1],
			}
			.into(),
		);
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
		System::assert_last_event(
			Event::UsersRemoved {
				pot,
				users: vec![user_3],
			}
			.into(),
		);
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			collection: 0u32.into(),
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
				top_up: user_reserve_quota,
				refund: user_reserve_quota - user_details.reserve.balance(),
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
		assert_eq!(Balances::free_balance(&user), 0);
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_eq!(Balances::free_balance(&user), 0);
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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

		assert_eq!(Balances::free_balance(&user), 0);
	});
}

#[test]
fn sponsor_for_calls_will_refund_unused_reserve() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 100_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			collection: 0u32.into(),
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call
		));
		let user_details = User::<Test>::get(pot, user).unwrap();
		let uniques_call_reserve = user_details.reserve.balance() - Balances::minimum_balance();
		assert_eq!(Balances::free_balance(&user_details.proxy), Balances::minimum_balance());
		assert_eq!(Balances::reserved_balance(&user_details.proxy), uniques_call_reserve);
		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(
			pot_details.remained_reserve_quota,
			pot_reserve_quota - user_details.reserve.balance()
		);

		let uniques_destroy_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::destroy {
			collection: 0u32.into(),
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
				top_up: user_reserve_quota - user_details.reserve.balance(),
				refund: user_reserve_quota - Balances::minimum_balance(),
			}
			.into(),
		);
		assert_eq!(Balances::free_balance(&user_details.proxy), Balances::minimum_balance());
		assert_eq!(Balances::reserved_balance(&user_details.proxy), 0);
		assert_eq!(
			Balances::free_balance(&pot_details.sponsor),
			pot_reserve_quota - Balances::minimum_balance()
		);
		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(
			pot_details.remained_reserve_quota,
			pot_reserve_quota - Balances::minimum_balance()
		);
		let user_1_details = User::<Test>::get(pot, user).unwrap();
		assert_eq!(
			user_1_details.reserve.available_margin(),
			user_1_details.reserve.limit() - Balances::minimum_balance()
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
			collection: 0u32.into(),
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call_1
		));

		let user_details = User::<Test>::get(pot, user).unwrap();
		let user_owing = user_details.reserve.balance();
		assert_eq!(user_owing, TestCollectionDeposit::get() + Balances::minimum_balance());

		// Assume user's proxy account somehow earns enough funds through an interaction or from an external source.
		let user_free_balance_after_earning = 2 * user_details.reserve.balance();
		Balances::make_free_balance_be(&user_details.proxy, user_free_balance_after_earning);

		let uniques_create_call_2 = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 1u32.into(),
			admin: user,
		}));
		assert_ok!(SponsorshipModule::sponsor_for(
			RuntimeOrigin::signed(user),
			pot,
			uniques_create_call_2
		));

		System::assert_last_event(
			Event::Sponsored {
				top_up: user_reserve_quota - user_free_balance_after_earning,
				refund: user_reserve_quota - user_free_balance_after_earning + user_owing,
			}
			.into(),
		);

		let pot_details = Pot::<Test>::get(pot).unwrap();
		assert_eq!(Balances::free_balance(&pot_details.sponsor), pot_reserve_quota);
		assert_eq!(pot_details.remained_reserve_quota, pot_reserve_quota);

		let user_details = User::<Test>::get(pot, user).unwrap();
		assert_eq!(user_details.reserve.balance(), 0);
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
			remained_fee_quota: 5,
			remained_reserve_quota: 7,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details_1.sponsor),
			pot_1,
			pot_details_1.sponsorship_type.clone(),
			pot_details_1.remained_fee_quota,
			pot_details_1.remained_reserve_quota
		));

		let pot_details_2 = PotDetailsOf::<Test> {
			sponsor: 2,
			sponsorship_type: SponsorshipType::AnySafe,
			remained_fee_quota: 10,
			remained_reserve_quota: 14,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details_2.sponsor),
			pot_2,
			pot_details_2.sponsorship_type.clone(),
			pot_details_2.remained_fee_quota,
			pot_details_2.remained_reserve_quota
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
fn sponsor_call_for_existing_pot_from_registered_user_with_enough_fee_limit_is_valid() {
	new_test_ext().execute_with(|| {
		let pot = 3;
		System::set_block_number(1);
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
		let pot_fee_quota = 10_000_000_000;
		let pot_reserve_quota = 100_000_000_000;
		let pot_details = PotDetailsOf::<Test> {
			sponsor: 1,
			sponsorship_type: SponsorshipType::Uniques,
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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

		let fee = pot_details.remained_fee_quota - pot_details_post_dispatch.remained_fee_quota;
		assert_ne!(fee, 0);
		assert_eq!(
			user_details.remained_fee_quota - user_details_post_dispatch.remained_fee_quota,
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
fn non_sponsor_calls_is_not_the_sponsorship_module_business() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let user = 2u64;

		let uniques_create_call = Box::new(RuntimeCall::Uniques(pallet_uniques::Call::create {
			collection: 0u32.into(),
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
			remained_fee_quota: pot_fee_quota,
			remained_reserve_quota: pot_reserve_quota,
		};
		assert_ok!(SponsorshipModule::create_pot(
			RuntimeOrigin::signed(pot_details.sponsor),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota
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
				collection: 0u32.into(),
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
