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
// use mock::*;
use crate::mock::{
	Origin,
	System, Timestamp, Balances,
	NodleStaking, Session, Test,
	ExtBuilder,	Event as MetaEvent,
	roll_to, last_event, events,
	set_author, mint_rewards, active_round,
	run_to_block, start_session,
	start_active_session, bond_validator,
	bond_nominator,
};
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency, OnFinalize, OnInitialize, ReservableCurrency},
    StorageMap,
};
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

#[test]
fn payout_creates_controller() {
    ExtBuilder::default()
        .build_and_execute(|| {
            let balance = 1000;
            // Create a validator:
            bond_validator(10, balance);

			assert_eq!(
				last_event(),
				MetaEvent::nodle_staking(
					Event::JoinedValidatorPool(
						10,
						balance,
						1000,
					),
				),
			);

            // Create a nominator
            bond_nominator(1337, 100, 10);

			assert_eq!(
				last_event(),
				MetaEvent::nodle_staking(
					Event::Nomination(
						1337,
						100,
						10,
						1100,
					),
				),
			);

            mock::mint_rewards(1_000_000);

            mock::start_active_session(1);

			// println!(
			// 	"Actv Sess 1 event {:#?}",
			// 	mock::events()
			// );

			mock::mint_rewards(1_000_000);
            // NodleStaking::reward_by_ids(vec![(11, 1)]);

            mock::start_active_session(2);

			// println!(
			// 	"Actv Sess 2 event {:#?}",
			// 	mock::events()
			// );

            // assert_ok!(NodleStaking::payout_stakers(Origin::signed(1337), 11, 1));

            // // Controller is created
            // assert!(Balances::free_balance(1337) > 0);
        })
}
