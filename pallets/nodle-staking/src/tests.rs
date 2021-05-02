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
    bond_nominator, bond_validator, events, last_event, mint_rewards, roll_to, run_to_block,
    set_author, start_active_session, start_session, Balances, Event as MetaEvent, ExtBuilder,
    NodleStaking, Origin, Session, System, Test, Timestamp,
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
use substrate_test_utils::assert_eq_uvec;

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
    ExtBuilder::default().build_and_execute(|| {
        let balance = 1000;
        // Create a validator:
        bond_validator(10, balance);

        assert_eq!(
            last_event(),
            MetaEvent::nodle_staking(Event::JoinedValidatorPool(10, balance, 4500)),
        );

        // Create a nominator
        bond_nominator(1337, 100, 10);

        assert_eq!(
            last_event(),
            MetaEvent::nodle_staking(Event::Nomination(1337, 100, 10, 1100,),),
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

#[test]
fn staking_should_work() {
    ExtBuilder::default().nominate(true).build_and_execute(|| {
        // put some money in account that we'll use.
        for i in 1..5 {
            let _ = Balances::make_free_balance_be(&i, 2000);
        }

        assert_eq!(mock::validators_in_pool(), vec![11, 21, 41],);

        assert_eq!(mock::selected_validators(), vec![11, 21, 41],);

        // --- Block 2:
        start_session(2);

        println!("Blk-2::{:#?}", last_event());

        // add a new validator [account 3] to the pool.
        bond_validator(3, 1500);
        assert_eq!(
            last_event(),
            MetaEvent::nodle_staking(Event::JoinedValidatorPool(3, 1500, 5000)),
        );

        assert_eq!(mock::validators_in_pool(), vec![3, 11, 21, 41],);

        assert_eq!(mock::selected_validators(), vec![11, 21, 41],);

        // --- Block 3:
        start_session(3);

        println!("Blk-2::{:#?}", last_event());

        assert_eq!(mock::selected_validators(), vec![3, 11, 21, 41],);

        println!("{:#?}", NodleStaking::at_stake(3, 3),);

        println!("{:#?}", NodleStaking::at_stake(3, 11),);

        println!("{:#?}", NodleStaking::at_stake(3, 21),);

        println!("{:#?}", NodleStaking::at_stake(3, 41),);

        // --- Block 4:
        start_session(4);

        println!("{:#?}", NodleStaking::at_stake(4, 3),);

        // println!("{:#?}", last_event());
        // println!("{:#?}", NodleStaking::total_selected());
        // println!("{:#?}", NodleStaking::selected_validators());
        // println!("{:#?}", NodleStaking::validator_pool());
    })
}

#[test]
fn reward_from_authorship_event_handler_works() {
    ExtBuilder::default().build_and_execute(|| {
        use pallet_authorship::EventHandler;
        assert_eq!(<pallet_authorship::Module<Test>>::author(), 11);
        NodleStaking::note_author(11);
        NodleStaking::note_uncle(21, 1);
        // Rewarding the same two times works.
        NodleStaking::note_uncle(11, 1);
        // Not mandatory but must be coherent with rewards
        assert_eq!(Session::validators(), vec![11, 21, 41]);

        // 11 is rewarded as a block producer and uncle reference and uncle producer
        assert_eq!(
            NodleStaking::awarded_pts(NodleStaking::active_session(), 11),
            25
        );

        // 21 is rewarded as an uncle producer
        assert_eq!(
            NodleStaking::awarded_pts(NodleStaking::active_session(), 21),
            1
        );

        // Total rewarded points
        assert_eq!(NodleStaking::points(NodleStaking::active_session()), 26);
    })
}

#[test]
fn add_reward_points_fns_works() {
    ExtBuilder::default().build_and_execute(|| {
        // Not mandatory but must be coherent with rewards
        assert_eq!(Session::validators(), vec![11, 21, 41]);
        NodleStaking::reward_by_ids(vec![(21, 1), (11, 1), (11, 1)]);
        NodleStaking::reward_by_ids(vec![(21, 1), (11, 1), (11, 1), (41, 2)]);

        assert_eq!(
            NodleStaking::awarded_pts(NodleStaking::active_session(), 11),
            4
        );

        assert_eq!(
            NodleStaking::awarded_pts(NodleStaking::active_session(), 21),
            2
        );

        assert_eq!(
            NodleStaking::awarded_pts(NodleStaking::active_session(), 41),
            2
        );

        assert_eq!(NodleStaking::points(NodleStaking::active_session()), 8);
    })
}
