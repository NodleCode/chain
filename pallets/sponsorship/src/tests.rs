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
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_pot_works() {
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
fn create_pot_fails_when_pot_already_created() {
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
fn sponsor_can_remove_pot_with_no_users() {
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
fn no_permission_for_non_sponsor_to_remove_pot() {
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
fn sponsor_cannot_remove_pot_with_users() {
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
