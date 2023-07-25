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

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(SponsorshipModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(SponsorshipModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			SponsorshipModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
