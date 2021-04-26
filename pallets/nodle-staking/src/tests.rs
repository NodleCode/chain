/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

use super::*;
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency, OnFinalize, OnInitialize, ReservableCurrency},
    StorageMap,
};
use mock::*;
use pallet_balances::Error as BalancesError;
use sp_runtime::{
    assert_eq_error_rate,
    traits::{AccountIdConversion, BadOrigin},
};

#[test]
fn set_invulnerables_works() {
    ExtBuilder::default().build_and_execute(|| {
        let new_set = vec![1, 2, 3, 4];
        assert_ok!(NodleStaking::set_invulnerables(
            Origin::root(),
            new_set.clone()
        ));
        assert_eq!(NodleStaking::invulnerables(), new_set);

        // cannot set with non-root.
        assert_noop!(
            NodleStaking::set_invulnerables(Origin::signed(1), new_set.clone()),
            BadOrigin
        );
    });
}
