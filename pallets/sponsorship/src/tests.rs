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

use crate::{mock::*, Error, Event, Pot, PotDetailsOf};
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
