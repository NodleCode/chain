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
#![cfg(test)]

use super::*;
use crate::mock::{
    balances, bond_nominator, bond_validator, events, is_disabled, last_event,
    on_offence_in_session, on_offence_now, set_author, start_session, Balance, Balances,
    CancelOrigin, Event as MetaEvent, ExtBuilder, NodleStaking, Origin, Session, System, Test,
};
use crate::set::OrderedSet;
use crate::types::{Bond, ValidatorSnapshot, ValidatorStatus};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::{
    testing::UintAuthorityId,
    traits::{BadOrigin, Zero},
    Perbill,
};
use sp_staking::offence::OffenceDetails;

#[test]
fn join_validator_pool_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 300),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 9),
            (9, 4),
        ])
        .with_validators(vec![(1, 500), (2, 200)])
        .with_nominators(vec![(3, 1, 100), (4, 1, 100), (5, 2, 100), (6, 2, 100)])
        .tst_staking_build()
        .execute_with(|| {
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(1), 11u128,),
                Error::<Test>::ValidatorExists,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(7), 9u128,),
                Error::<Test>::ValidatorBondBelowMin,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(8), 10u128,),
                Error::<Test>::InsufficientBalance,
            );
            assert!(System::events().is_empty());
            assert_ok!(NodleStaking::validator_join_pool(Origin::signed(3), 11u128,),);
            assert_ok!(NodleStaking::validator_join_pool(Origin::signed(7), 10u128,));
            assert_eq!(
                last_event(),
                MetaEvent::nodle_staking(Event::JoinedValidatorPool(7, 10u128, 1121u128))
            );
        });
}

#[test]
fn validator_activate_works() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert!(Session::validators().contains(&11));
            assert!(<ValidatorState<Test>>::contains_key(11));

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(Balances::total_balance(&11), 2000);

            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(Balances::total_balance(&101), 2000);

            assert_eq!(NodleStaking::total(), 3500);

            let slash_percent = Perbill::from_percent(0);
            let initial_exposure = NodleStaking::at_stake(NodleStaking::active_session(), 11);

            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Active
            );

            mock::on_offence_now(
                &[OffenceDetails {
                    offender: (11, initial_exposure.clone()),
                    reporters: vec![],
                }],
                &[slash_percent],
            );

            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Idle
            );

            mock::start_active_session(1);

            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Idle
            );

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(11), 2000),
                Error::<Test>::InsufficientBalance,
            );

            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(11), 10));

            let mut new1 = vec![Event::ValidatorBondedMore(11, 1000, 1010)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1010));
            assert_eq!(Balances::total_balance(&11), 2000);

            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(Balances::total_balance(&101), 2000);

            assert_eq!(NodleStaking::total(), 3510);

            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Active
            );

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("S-1");
        });
}

#[test]
fn validator_exit_executes_after_delay() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 300),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 9),
            (9, 4),
        ])
        .with_validators(vec![(1, 500), (2, 200)])
        .with_nominators(vec![(3, 1, 100), (4, 1, 100), (5, 2, 100), (6, 2, 100)])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 700),
                Event::ValidatorChosen(2, 2, 400),
                Event::NewSession(5, 2, 2, 1100),
                Event::ValidatorChosen(3, 1, 700),
                Event::ValidatorChosen(3, 2, 400),
                Event::NewSession(10, 3, 2, 1100),
                Event::ValidatorChosen(4, 1, 700),
                Event::ValidatorChosen(4, 2, 400),
                Event::NewSession(15, 4, 2, 1100),
                Event::ValidatorChosen(5, 1, 700),
                Event::ValidatorChosen(5, 2, 400),
                Event::NewSession(20, 5, 2, 1100),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (1000, 500));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&3), (100, 100));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 100));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&2), (300, 200));
            assert_eq!(Balances::total_balance(&2), 300);

            assert_eq!(mock::balances(&5), (100, 100));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 100));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(NodleStaking::total(), 1100);

            assert_noop!(
                NodleStaking::validator_exit_pool(Origin::signed(3)),
                Error::<Test>::ValidatorDNE,
            );
            mock::start_active_session(6);
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(2)));
            assert_eq!(
                last_event(),
                MetaEvent::nodle_staking(Event::ValidatorScheduledExit(6, 2, 8)),
            );

            let mut new1 = vec![
                Event::ValidatorChosen(6, 1, 700),
                Event::ValidatorChosen(6, 2, 400),
                Event::NewSession(25, 6, 2, 1100),
                Event::ValidatorChosen(7, 1, 700),
                Event::ValidatorChosen(7, 2, 400),
                Event::NewSession(30, 7, 2, 1100),
                Event::ValidatorScheduledExit(6, 2, 8),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            let info = NodleStaking::validator_state(&2).unwrap();
            assert_eq!(info.state, ValidatorStatus::Leaving(8));
            mock::start_active_session(9);
            // we must exclude leaving collators from rewards while
            // holding them retroactively accountable for previous faults
            // (within the last T::SlashingWindow blocks)
            let mut new2 = vec![
                Event::ValidatorChosen(8, 1, 700),
                Event::NewSession(35, 8, 1, 700),
                Event::NominatorLeftValidator(5, 2, 100, 0),
                Event::NominatorLeftValidator(6, 2, 100, 0),
                Event::ValidatorLeft(2, 400, 700),
                Event::ValidatorChosen(9, 1, 700),
                Event::NewSession(40, 9, 1, 700),
                Event::ValidatorChosen(10, 1, 700),
                Event::NewSession(45, 10, 1, 700),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (1000, 500));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&3), (100, 100));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 100));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&2), (300, 0));
            assert_eq!(Balances::total_balance(&2), 300);

            assert_eq!(mock::balances(&5), (100, 100));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 100));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(NodleStaking::total(), 700);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(5)));

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));

            let mut new3 = vec![
                Event::Withdrawn(5, 100),
                Event::NominatorLeft(5, 100),
                Event::Withdrawn(6, 100),
                Event::NominatorLeft(6, 100),
            ];
            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&5), (100, 0));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 0));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(NodleStaking::total(), 700);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("S-1");
        });
}

#[test]
fn validator_selection_chooses_top_candidates() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1000),
            (6, 1000),
            (7, 33),
            (8, 33),
            (9, 33),
        ])
        .with_validators(vec![(1, 100), (2, 90), (3, 80), (4, 70), (5, 60), (6, 50)])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 100),
                Event::ValidatorChosen(2, 2, 90),
                Event::ValidatorChosen(2, 3, 80),
                Event::ValidatorChosen(2, 4, 70),
                Event::ValidatorChosen(2, 5, 60),
                Event::NewSession(5, 2, 5, 400),
                Event::ValidatorChosen(3, 1, 100),
                Event::ValidatorChosen(3, 2, 90),
                Event::ValidatorChosen(3, 3, 80),
                Event::ValidatorChosen(3, 4, 70),
                Event::ValidatorChosen(3, 5, 60),
                Event::NewSession(10, 3, 5, 400),
                Event::ValidatorChosen(4, 1, 100),
                Event::ValidatorChosen(4, 2, 90),
                Event::ValidatorChosen(4, 3, 80),
                Event::ValidatorChosen(4, 4, 70),
                Event::ValidatorChosen(4, 5, 60),
                Event::NewSession(15, 4, 5, 400),
                Event::ValidatorChosen(5, 1, 100),
                Event::ValidatorChosen(5, 2, 90),
                Event::ValidatorChosen(5, 3, 80),
                Event::ValidatorChosen(5, 4, 70),
                Event::ValidatorChosen(5, 5, 60),
                Event::NewSession(20, 5, 5, 400),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (1000, 100));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&2), (1000, 90));
            assert_eq!(Balances::total_balance(&2), 1000);

            assert_eq!(mock::balances(&3), (1000, 80));
            assert_eq!(Balances::total_balance(&3), 1000);

            assert_eq!(mock::balances(&4), (1000, 70));
            assert_eq!(Balances::total_balance(&4), 1000);

            assert_eq!(mock::balances(&5), (1000, 60));
            assert_eq!(Balances::total_balance(&5), 1000);

            assert_eq!(mock::balances(&6), (1000, 50));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 450);

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(6)));
            assert_eq!(
                last_event(),
                MetaEvent::nodle_staking(Event::ValidatorScheduledExit(4, 6, 6)),
            );
            let info = NodleStaking::validator_state(&6).unwrap();
            assert_eq!(info.state, ValidatorStatus::Leaving(6));

            mock::start_active_session(6);

            let mut new1 = vec![
                Event::ValidatorScheduledExit(4, 6, 6),
                Event::ValidatorChosen(6, 1, 100),
                Event::ValidatorChosen(6, 2, 90),
                Event::ValidatorChosen(6, 3, 80),
                Event::ValidatorChosen(6, 4, 70),
                Event::ValidatorChosen(6, 5, 60),
                Event::NewSession(25, 6, 5, 400),
                Event::ValidatorLeft(6, 50, 400),
                Event::ValidatorChosen(7, 1, 100),
                Event::ValidatorChosen(7, 2, 90),
                Event::ValidatorChosen(7, 3, 80),
                Event::ValidatorChosen(7, 4, 70),
                Event::ValidatorChosen(7, 5, 60),
                Event::NewSession(30, 7, 5, 400),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (1000, 0));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 400);

            assert_ok!(NodleStaking::validator_join_pool(Origin::signed(6), 69u128));

            assert_eq!(
                mock::last_event(),
                MetaEvent::nodle_staking(Event::JoinedValidatorPool(6, 69, 469u128))
            );

            mock::start_active_session(8);

            let mut new2 = vec![
                Event::JoinedValidatorPool(6, 69, 469),
                Event::ValidatorChosen(8, 1, 100),
                Event::ValidatorChosen(8, 2, 90),
                Event::ValidatorChosen(8, 3, 80),
                Event::ValidatorChosen(8, 4, 70),
                Event::ValidatorChosen(8, 6, 69),
                Event::NewSession(35, 8, 5, 409),
                Event::ValidatorChosen(9, 1, 100),
                Event::ValidatorChosen(9, 2, 90),
                Event::ValidatorChosen(9, 3, 80),
                Event::ValidatorChosen(9, 4, 70),
                Event::ValidatorChosen(9, 6, 69),
                Event::NewSession(40, 9, 5, 409),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (1000, 69));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 469);

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(1)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(2)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(3)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(4)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(5)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(6)));

            let mut new3 = vec![
                Event::ValidatorScheduledExit(8, 1, 10),
                Event::ValidatorScheduledExit(8, 2, 10),
                Event::ValidatorScheduledExit(8, 3, 10),
                Event::ValidatorScheduledExit(8, 4, 10),
                Event::ValidatorScheduledExit(8, 5, 10),
                Event::ValidatorScheduledExit(8, 6, 10),
            ];
            expected.append(&mut new3);
            assert_eq!(events(), expected);

            mock::start_active_session(10);

            let mut new4 = vec![
                Event::NewSession(45, 10, 0, 0),
                Event::ValidatorLeft(1, 100, 369),
                Event::ValidatorLeft(2, 90, 279),
                Event::ValidatorLeft(3, 80, 199),
                Event::ValidatorLeft(4, 70, 129),
                Event::ValidatorLeft(5, 60, 69),
                Event::ValidatorLeft(6, 69, 0),
                Event::NewSession(50, 11, 0, 0),
            ];
            expected.append(&mut new4);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (1000, 0));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&2), (1000, 0));
            assert_eq!(Balances::total_balance(&2), 1000);

            assert_eq!(mock::balances(&3), (1000, 0));
            assert_eq!(Balances::total_balance(&3), 1000);

            assert_eq!(mock::balances(&4), (1000, 0));
            assert_eq!(Balances::total_balance(&4), 1000);

            assert_eq!(mock::balances(&5), (1000, 0));
            assert_eq!(Balances::total_balance(&5), 1000);

            assert_eq!(mock::balances(&6), (1000, 0));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 0);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("S-1");
        });
}

#[test]
fn exit_queue() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1000),
            (6, 1000),
            (7, 33),
            (8, 33),
            (9, 33),
        ])
        .with_validators(vec![(1, 100), (2, 90), (3, 80), (4, 70), (5, 60), (6, 50)])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 100),
                Event::ValidatorChosen(2, 2, 90),
                Event::ValidatorChosen(2, 3, 80),
                Event::ValidatorChosen(2, 4, 70),
                Event::ValidatorChosen(2, 5, 60),
                Event::NewSession(5, 2, 5, 400),
                Event::ValidatorChosen(3, 1, 100),
                Event::ValidatorChosen(3, 2, 90),
                Event::ValidatorChosen(3, 3, 80),
                Event::ValidatorChosen(3, 4, 70),
                Event::ValidatorChosen(3, 5, 60),
                Event::NewSession(10, 3, 5, 400),
                Event::ValidatorChosen(4, 1, 100),
                Event::ValidatorChosen(4, 2, 90),
                Event::ValidatorChosen(4, 3, 80),
                Event::ValidatorChosen(4, 4, 70),
                Event::ValidatorChosen(4, 5, 60),
                Event::NewSession(15, 4, 5, 400),
                Event::ValidatorChosen(5, 1, 100),
                Event::ValidatorChosen(5, 2, 90),
                Event::ValidatorChosen(5, 3, 80),
                Event::ValidatorChosen(5, 4, 70),
                Event::ValidatorChosen(5, 5, 60),
                Event::NewSession(20, 5, 5, 400),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (1000, 100));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&2), (1000, 90));
            assert_eq!(Balances::total_balance(&2), 1000);

            assert_eq!(mock::balances(&3), (1000, 80));
            assert_eq!(Balances::total_balance(&3), 1000);

            assert_eq!(mock::balances(&4), (1000, 70));
            assert_eq!(Balances::total_balance(&4), 1000);

            assert_eq!(mock::balances(&5), (1000, 60));
            assert_eq!(Balances::total_balance(&5), 1000);

            assert_eq!(mock::balances(&6), (1000, 50));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 450);

            mock::start_active_session(5);

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(6)));

            let mut new1 = vec![
                Event::ValidatorChosen(6, 1, 100),
                Event::ValidatorChosen(6, 2, 90),
                Event::ValidatorChosen(6, 3, 80),
                Event::ValidatorChosen(6, 4, 70),
                Event::ValidatorChosen(6, 5, 60),
                Event::NewSession(25, 6, 5, 400),
                Event::ValidatorScheduledExit(5, 6, 7),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            mock::start_active_session(6);

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(5)));

            let mut new2 = vec![
                Event::ValidatorChosen(7, 1, 100),
                Event::ValidatorChosen(7, 2, 90),
                Event::ValidatorChosen(7, 3, 80),
                Event::ValidatorChosen(7, 4, 70),
                Event::ValidatorChosen(7, 5, 60),
                Event::NewSession(30, 7, 5, 400),
                Event::ValidatorScheduledExit(6, 5, 8),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            mock::start_active_session(7);
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(4)));

            let mut new3 = vec![
                Event::ValidatorLeft(6, 50, 400),
                Event::ValidatorChosen(8, 1, 100),
                Event::ValidatorChosen(8, 2, 90),
                Event::ValidatorChosen(8, 3, 80),
                Event::ValidatorChosen(8, 4, 70),
                Event::NewSession(35, 8, 4, 340),
                Event::ValidatorScheduledExit(7, 4, 9),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            mock::start_active_session(8);

            assert_noop!(
                NodleStaking::validator_exit_pool(Origin::signed(4)),
                Error::<Test>::AlreadyLeaving,
            );

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(3)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(2)));
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(1)));

            let mut new4 = vec![
                Event::ValidatorLeft(5, 60, 340),
                Event::ValidatorChosen(9, 1, 100),
                Event::ValidatorChosen(9, 2, 90),
                Event::ValidatorChosen(9, 3, 80),
                Event::NewSession(40, 9, 3, 270),
                Event::ValidatorScheduledExit(8, 3, 10),
                Event::ValidatorScheduledExit(8, 2, 10),
                Event::ValidatorScheduledExit(8, 1, 10),
            ];
            expected.append(&mut new4);
            assert_eq!(events(), expected);

            mock::start_active_session(10);

            let mut new5 = vec![
                Event::ValidatorLeft(4, 70, 270),
                Event::NewSession(45, 10, 0, 0),
                Event::ValidatorLeft(1, 100, 170),
                Event::ValidatorLeft(2, 90, 80),
                Event::ValidatorLeft(3, 80, 0),
                Event::NewSession(50, 11, 0, 0),
            ];
            expected.append(&mut new5);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (1000, 0));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&2), (1000, 0));
            assert_eq!(Balances::total_balance(&2), 1000);

            assert_eq!(mock::balances(&3), (1000, 0));
            assert_eq!(Balances::total_balance(&3), 1000);

            assert_eq!(mock::balances(&4), (1000, 0));
            assert_eq!(Balances::total_balance(&4), 1000);

            assert_eq!(mock::balances(&5), (1000, 0));
            assert_eq!(Balances::total_balance(&5), 1000);

            assert_eq!(mock::balances(&6), (1000, 0));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 0);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("S-1");
        });
}

#[test]
fn payout_distribution_to_solo_validators() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 1000),
            (3, 1000),
            (4, 1000),
            (5, 1000),
            (6, 1000),
            (7, 33),
            (8, 33),
            (9, 33),
        ])
        .with_validators(vec![(1, 100), (2, 90), (3, 80), (4, 70), (5, 60), (6, 50)])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 100),
                Event::ValidatorChosen(2, 2, 90),
                Event::ValidatorChosen(2, 3, 80),
                Event::ValidatorChosen(2, 4, 70),
                Event::ValidatorChosen(2, 5, 60),
                Event::NewSession(5, 2, 5, 400),
                Event::ValidatorChosen(3, 1, 100),
                Event::ValidatorChosen(3, 2, 90),
                Event::ValidatorChosen(3, 3, 80),
                Event::ValidatorChosen(3, 4, 70),
                Event::ValidatorChosen(3, 5, 60),
                Event::NewSession(10, 3, 5, 400),
                Event::ValidatorChosen(4, 1, 100),
                Event::ValidatorChosen(4, 2, 90),
                Event::ValidatorChosen(4, 3, 80),
                Event::ValidatorChosen(4, 4, 70),
                Event::ValidatorChosen(4, 5, 60),
                Event::NewSession(15, 4, 5, 400),
                Event::ValidatorChosen(5, 1, 100),
                Event::ValidatorChosen(5, 2, 90),
                Event::ValidatorChosen(5, 3, 80),
                Event::ValidatorChosen(5, 4, 70),
                Event::ValidatorChosen(5, 5, 60),
                Event::NewSession(20, 5, 5, 400),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (1000, 100));
            assert_eq!(Balances::total_balance(&1), 1000);

            assert_eq!(mock::balances(&2), (1000, 90));
            assert_eq!(Balances::total_balance(&2), 1000);

            assert_eq!(mock::balances(&3), (1000, 80));
            assert_eq!(Balances::total_balance(&3), 1000);

            assert_eq!(mock::balances(&4), (1000, 70));
            assert_eq!(Balances::total_balance(&4), 1000);

            assert_eq!(mock::balances(&5), (1000, 60));
            assert_eq!(Balances::total_balance(&5), 1000);

            assert_eq!(mock::balances(&6), (1000, 50));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 450);

            // Session-5 - set block author as 1 for all blocks
            mock::start_active_session(5);

            mock::set_author(5, 1, 100);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(6);

            let mut new1 = vec![
                Event::ValidatorChosen(6, 1, 100),
                Event::ValidatorChosen(6, 2, 90),
                Event::ValidatorChosen(6, 3, 80),
                Event::ValidatorChosen(6, 4, 70),
                Event::ValidatorChosen(6, 5, 60),
                Event::NewSession(25, 6, 5, 400),
                Event::StakeReward(1, 1000000),
                Event::ValidatorChosen(7, 1, 100),
                Event::ValidatorChosen(7, 2, 90),
                Event::ValidatorChosen(7, 3, 80),
                Event::ValidatorChosen(7, 4, 70),
                Event::ValidatorChosen(7, 5, 60),
                Event::NewSession(30, 7, 5, 400),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            // Session-7 - set block author as 1 & 2 for all blocks
            set_author(6, 1, 60);
            set_author(6, 2, 40);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(7);

            let mut new2 = vec![
                Event::StakeReward(1, 600000),
                Event::StakeReward(2, 400000),
                Event::ValidatorChosen(8, 1, 100),
                Event::ValidatorChosen(8, 2, 90),
                Event::ValidatorChosen(8, 3, 80),
                Event::ValidatorChosen(8, 4, 70),
                Event::ValidatorChosen(8, 5, 60),
                Event::NewSession(35, 8, 5, 400),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            // Session-8 - each validator produces 1 block this round
            set_author(7, 1, 20);
            set_author(7, 2, 20);
            set_author(7, 3, 20);
            set_author(7, 4, 20);
            set_author(7, 5, 20);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(8);

            let mut new3 = vec![
                Event::StakeReward(5, 200000),
                Event::StakeReward(3, 200000),
                Event::StakeReward(4, 200000),
                Event::StakeReward(1, 200000),
                Event::StakeReward(2, 200000),
                Event::ValidatorChosen(9, 1, 100),
                Event::ValidatorChosen(9, 2, 90),
                Event::ValidatorChosen(9, 3, 80),
                Event::ValidatorChosen(9, 4, 70),
                Event::ValidatorChosen(9, 5, 60),
                Event::NewSession(40, 9, 5, 400),
            ];
            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert!(NodleStaking::awarded_pts(2, 1).is_zero());
            assert!(NodleStaking::awarded_pts(3, 1).is_zero());
            assert!(NodleStaking::awarded_pts(3, 2).is_zero());
            assert!(NodleStaking::awarded_pts(4, 1).is_zero());
            assert!(NodleStaking::awarded_pts(4, 2).is_zero());
            assert!(NodleStaking::awarded_pts(4, 3).is_zero());
            assert!(NodleStaking::awarded_pts(4, 4).is_zero());
            assert!(NodleStaking::awarded_pts(4, 5).is_zero());

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(1)));

            let mut new4 = vec![Event::Rewarded(1, 1800000)];
            expected.append(&mut new4);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (1801000, 100));
            assert_eq!(Balances::total_balance(&1), 1801000);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(2)));

            let mut new5 = vec![Event::Rewarded(2, 600000)];
            expected.append(&mut new5);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&2), (601000, 90));
            assert_eq!(Balances::total_balance(&2), 601000);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(3)));

            let mut new6 = vec![Event::Rewarded(3, 200000)];
            expected.append(&mut new6);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&3), (201000, 80));
            assert_eq!(Balances::total_balance(&3), 201000);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(4)));

            let mut new7 = vec![Event::Rewarded(4, 200000)];
            expected.append(&mut new7);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&4), (201000, 70));
            assert_eq!(Balances::total_balance(&4), 201000);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(5)));

            let mut new8 = vec![Event::Rewarded(5, 200000)];
            expected.append(&mut new8);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&5), (201000, 60));
            assert_eq!(Balances::total_balance(&5), 201000);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(6)));

            let mut new9 = vec![Event::Rewarded(6, 0)];
            expected.append(&mut new9);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (1000, 50));
            assert_eq!(Balances::total_balance(&6), 1000);

            assert_eq!(NodleStaking::total(), 450);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        });
}

#[test]
fn validator_commission() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
        ])
        .with_validators(vec![(1, 20)])
        .with_nominators(vec![(2, 1, 10), (3, 1, 10)])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 40),
                Event::NewSession(5, 2, 1, 40),
                Event::ValidatorChosen(3, 1, 40),
                Event::NewSession(10, 3, 1, 40),
                Event::ValidatorChosen(4, 1, 40),
                Event::NewSession(15, 4, 1, 40),
                Event::ValidatorChosen(5, 1, 40),
                Event::NewSession(20, 5, 1, 40),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 10));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 10));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 0));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 0));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 0));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(NodleStaking::total(), 40);

            assert_ok!(NodleStaking::validator_join_pool(Origin::signed(4), 20u128));
            assert_eq!(
                last_event(),
                MetaEvent::nodle_staking(Event::JoinedValidatorPool(4, 20u128, 60u128))
            );

            assert_ok!(Session::set_keys(
                Origin::signed(4),
                UintAuthorityId(4).into(),
                vec![]
            ));

            mock::start_active_session(5);

            let mut new1 = vec![
                Event::JoinedValidatorPool(4, 20, 60),
                Event::ValidatorChosen(6, 1, 40),
                Event::ValidatorChosen(6, 4, 20),
                Event::NewSession(25, 6, 2, 60),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(5),
                4,
                10,
                false
            ));
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                4,
                10,
                false
            ));

            let mut new2 = vec![
                Event::Nomination(5, 10, 4, 30),
                Event::Nomination(6, 10, 4, 40),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            mock::start_active_session(6);

            let mut new3 = vec![
                Event::ValidatorChosen(7, 1, 40),
                Event::ValidatorChosen(7, 4, 40),
                Event::NewSession(30, 7, 2, 80),
            ];
            expected.append(&mut new3);
            assert_eq!(events(), expected);

            mock::start_active_session(7);

            set_author(7, 1, 50);
            set_author(7, 4, 50);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(8);

            let mut new4 = vec![
                Event::ValidatorChosen(8, 1, 40),
                Event::ValidatorChosen(8, 4, 40),
                Event::NewSession(35, 8, 2, 80),
                Event::StakeReward(4, 300000),
                Event::StakeReward(5, 100000),
                Event::StakeReward(6, 100000),
                Event::StakeReward(1, 300000),
                Event::StakeReward(2, 100000),
                Event::StakeReward(3, 100000),
                Event::ValidatorChosen(9, 1, 40),
                Event::ValidatorChosen(9, 4, 40),
                Event::NewSession(40, 9, 2, 80),
            ];
            expected.append(&mut new4);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(1)));

            let mut new5 = vec![Event::Rewarded(1, 300000)];
            expected.append(&mut new5);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (300100, 20));
            assert_eq!(Balances::total_balance(&1), 300100);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(2)));

            let mut new6 = vec![Event::Rewarded(2, 100000)];
            expected.append(&mut new6);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&2), (100100, 10));
            assert_eq!(Balances::total_balance(&2), 100100);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(3)));

            let mut new7 = vec![Event::Rewarded(3, 100000)];
            expected.append(&mut new7);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&3), (100100, 10));
            assert_eq!(Balances::total_balance(&3), 100100);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(4)));

            let mut new8 = vec![Event::Rewarded(4, 300000)];
            expected.append(&mut new8);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&4), (300100, 20));
            assert_eq!(Balances::total_balance(&4), 300100);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(5)));

            let mut new9 = vec![Event::Rewarded(5, 100000)];
            expected.append(&mut new9);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&5), (100100, 10));
            assert_eq!(Balances::total_balance(&5), 100100);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(6)));

            let mut new10 = vec![Event::Rewarded(6, 100000)];
            expected.append(&mut new10);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (100100, 10));
            assert_eq!(Balances::total_balance(&6), 100100);

            assert_eq!(NodleStaking::total(), 80);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events(),);
        });
}

#[test]
fn multiple_nominations() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            // chooses top TotalSelectedCandidates (5), in order
            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);

            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(6), 1, 10, false),
                Error::<Test>::AlreadyNominatedValidator,
            );

            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(6), 2, 2, false),
                Error::<Test>::NominationBelowMin,
            );

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                2,
                10,
                false
            ));
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                3,
                10,
                false
            ));
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                4,
                10,
                false
            ));

            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(6), 5, 10, false),
                Error::<Test>::ExceedMaxValidatorPerNom,
            );

            let mut new1 = vec![
                Event::Nomination(6, 10, 2, 50),
                Event::Nomination(6, 10, 3, 30),
                Event::Nomination(6, 10, 4, 30),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            mock::start_active_session(5);

            let mut new2 = vec![
                Event::ValidatorChosen(6, 1, 50),
                Event::ValidatorChosen(6, 2, 50),
                Event::ValidatorChosen(6, 3, 30),
                Event::ValidatorChosen(6, 4, 30),
                Event::ValidatorChosen(6, 5, 10),
                Event::NewSession(25, 6, 5, 170),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(7),
                2,
                80,
                false
            ));
            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(7), 3, 11, false),
                Error::<Test>::InsufficientBalance
            );

            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(10), 2, 10, false),
                Error::<Test>::TooManyNominators
            );

            mock::start_active_session(6);

            let mut new3 = vec![
                Event::Nomination(7, 80, 2, 130),
                Event::ValidatorChosen(7, 1, 50),
                Event::ValidatorChosen(7, 2, 130),
                Event::ValidatorChosen(7, 3, 30),
                Event::ValidatorChosen(7, 4, 30),
                Event::ValidatorChosen(7, 5, 10),
                Event::NewSession(30, 7, 5, 250),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(2)));

            assert_eq!(
                last_event(),
                MetaEvent::nodle_staking(Event::ValidatorScheduledExit(6, 2, 8))
            );

            mock::start_active_session(7);

            let mut new4 = vec![
                Event::ValidatorScheduledExit(6, 2, 8),
                Event::ValidatorChosen(8, 1, 50),
                Event::ValidatorChosen(8, 3, 30),
                Event::ValidatorChosen(8, 4, 30),
                Event::ValidatorChosen(8, 5, 10),
                Event::NewSession(35, 8, 4, 120),
            ];

            expected.append(&mut new4);
            assert_eq!(events(), expected);

            // verify that nominations are removed after validator leaves, not before
            assert_eq!(NodleStaking::nominator_state(7).unwrap().total, 90);
            assert_eq!(
                NodleStaking::nominator_state(7)
                    .unwrap()
                    .nominations
                    .0
                    .len(),
                2usize
            );

            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 40);
            assert_eq!(
                NodleStaking::nominator_state(6)
                    .unwrap()
                    .nominations
                    .0
                    .len(),
                4usize
            );

            assert_eq!(
                NodleStaking::validator_state(&2).unwrap().nominators.0,
                vec![
                    Bond {
                        owner: 6,
                        amount: 10,
                    },
                    Bond {
                        owner: 7,
                        amount: 80,
                    },
                    Bond {
                        owner: 8,
                        amount: 10,
                    },
                    Bond {
                        owner: 9,
                        amount: 10,
                    },
                ],
            );

            assert_eq!(mock::balances(&6), (100, 40));
            assert_eq!(mock::balances(&7), (100, 90));
            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(mock::balances(&9), (100, 10));

            assert_eq!(NodleStaking::total(), 250);

            mock::start_active_session(8);

            let mut new5 = vec![
                Event::NominatorLeftValidator(6, 2, 10, 0),
                Event::NominatorLeftValidator(7, 2, 80, 0),
                Event::NominatorLeftValidator(8, 2, 10, 0),
                Event::NominatorLeftValidator(9, 2, 10, 0),
                Event::ValidatorLeft(2, 130, 120),
                Event::ValidatorChosen(9, 1, 50),
                Event::ValidatorChosen(9, 3, 30),
                Event::ValidatorChosen(9, 4, 30),
                Event::ValidatorChosen(9, 5, 10),
                Event::NewSession(40, 9, 4, 120),
            ];

            expected.append(&mut new5);
            assert_eq!(events(), expected);

            assert_eq!(System::consumers(&2), 1);

            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(7)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(8)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(9)));

            let mut new6 = vec![
                Event::Withdrawn(6, 10),
                Event::Withdrawn(7, 80),
                Event::Withdrawn(8, 10),
                Event::NominatorLeft(8, 10),
                Event::Withdrawn(9, 10),
                Event::NominatorLeft(9, 10),
            ];

            expected.append(&mut new6);
            assert_eq!(events(), expected);

            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 0);
            assert_eq!(System::consumers(&9), 0);
            assert_eq!(System::consumers(&10), 1);

            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().total, 10);

            assert_eq!(
                NodleStaking::nominator_state(6)
                    .unwrap()
                    .nominations
                    .0
                    .len(),
                3usize
            );
            assert_eq!(
                NodleStaking::nominator_state(7)
                    .unwrap()
                    .nominations
                    .0
                    .len(),
                1usize
            );

            // Group-2
            assert_eq!(mock::balances(&2), (100, 0));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&6), (100, 30));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 0));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 0));
            assert_eq!(Balances::total_balance(&9), 100);

            // Group-1
            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 120);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 1);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 0);
            assert_eq!(System::consumers(&9), 0);
            assert_eq!(System::consumers(&10), 1);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        });
}

#[test]
fn switch_nomination_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            // chooses top TotalSelectedCandidates (5), in order
            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                2,
                10,
                false
            ));

            let mut new1 = vec![Event::Nomination(6, 10, 2, 50)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 10,
                        },
                        Bond {
                            owner: 2,
                            amount: 10,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 20);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 20);
            assert_eq!(NodleStaking::total(), 150);

            // Check with invalid arguments
            assert_noop!(
                NodleStaking::nominator_move_nomination(Origin::signed(6), 2, 2, 5, false),
                Error::<Test>::ValidatorDNE,
            );

            assert_noop!(
                NodleStaking::nominator_move_nomination(Origin::signed(6), 2, 7, 5, false),
                Error::<Test>::ValidatorDNE,
            );

            assert_noop!(
                NodleStaking::nominator_move_nomination(Origin::signed(6), 7, 2, 5, false),
                Error::<Test>::ValidatorDNE,
            );

            assert_noop!(
                NodleStaking::nominator_move_nomination(Origin::signed(1), 2, 1, 5, false),
                Error::<Test>::NominatorDNE,
            );

            assert_ok!(NodleStaking::nominator_move_nomination(
                Origin::signed(6),
                2,
                1,
                0,
                false,
            ));

            let mut new2 = vec![
                Event::NominatorLeftValidator(6, 2, 10, 40),
                Event::NominationMoved(6, 20, 2, 40, 1, 60),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [Bond {
                        owner: 1,
                        amount: 20,
                    }]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 20);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 20);
            assert_eq!(NodleStaking::total(), 150);

            assert_noop!(
                NodleStaking::nominator_move_nomination(Origin::signed(1), 2, 1, 5, false),
                Error::<Test>::NominatorDNE,
            );

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                2,
                10,
                false
            ));
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                3,
                10,
                false
            ));

            let mut new3 = vec![
                Event::Nomination(6, 10, 2, 50),
                Event::Nomination(6, 10, 3, 30),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 20,
                        },
                        Bond {
                            owner: 2,
                            amount: 10,
                        },
                        Bond {
                            owner: 3,
                            amount: 10,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 40);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 40);
            assert_eq!(NodleStaking::total(), 170);

            assert_ok!(NodleStaking::nominator_move_nomination(
                Origin::signed(6),
                3,
                4,
                5,
                false,
            ));

            assert_eq!(NodleStaking::total(), 175);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                5,
                10,
                false
            ));

            assert_ok!(NodleStaking::nominator_move_nomination(
                Origin::signed(6),
                2,
                5,
                0,
                false,
            ));

            let mut new4 = vec![
                Event::NominatorLeftValidator(6, 3, 10, 20),
                Event::NominationMoved(6, 45, 3, 20, 4, 35),
                Event::Nomination(6, 10, 5, 20),
                Event::NominatorLeftValidator(6, 2, 10, 40),
                Event::NominationMoved(6, 55, 2, 40, 5, 30),
            ];

            expected.append(&mut new4);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 20,
                        },
                        Bond {
                            owner: 4,
                            amount: 15,
                        },
                        Bond {
                            owner: 5,
                            amount: 20,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 55);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 55);
            assert_eq!(NodleStaking::total(), 185);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 55));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);
        });
}

#[test]
fn reconciliation_basics_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            // chooses top TotalSelectedCandidates (5), in order
            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                2,
                40,
                false
            ));

            assert_eq!(mock::balances(&6), (100, 50));
            assert_eq!(Balances::total_balance(&6), 100);

            let mut new1 = vec![Event::Nomination(6, 40, 2, 80)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 10,
                        },
                        Bond {
                            owner: 2,
                            amount: 40,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 50);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 50);
            assert_eq!(NodleStaking::total(), 180);

            let s1_max_validators = NodleStaking::staking_max_validators();
            let s1_min_stake = NodleStaking::staking_min_stake_session_selection();
            let s1_min_validator_bond = NodleStaking::staking_min_validator_bond();
            let s1_min_nomination_total_bond = NodleStaking::staking_min_nominator_total_bond();
            let s1_min_nomination_chill_threshold =
                NodleStaking::staking_min_nomination_chill_threshold();

            let s1_new_min_nomination_total_bond = 25;
            let s1_new_min_nomination_chill_threshold = 15;

            assert_ok!(NodleStaking::set_staking_limits(
                Origin::signed(CancelOrigin::get()),
                s1_max_validators,
                s1_min_stake,
                s1_min_validator_bond,
                s1_new_min_nomination_total_bond,
                s1_new_min_nomination_chill_threshold,
            ));

            let mut new1 = vec![
                Event::NewStakingLimits(
                    s1_max_validators,
                    s1_max_validators,
                    s1_min_validator_bond,
                    s1_min_validator_bond,
                    s1_min_stake,
                    s1_min_stake,
                    s1_min_nomination_total_bond,
                    s1_new_min_nomination_total_bond,
                    s1_min_nomination_chill_threshold,
                    s1_new_min_nomination_chill_threshold,
                ),
                Event::NominationBelowThreashold(6, 1, 10, 10, 40),
                Event::NominationBelowThreashold(7, 1, 10, 10, 0),
                Event::NominationBelowThreashold(10, 1, 10, 10, 0),
                Event::NominationBelowThreashold(8, 2, 10, 10, 0),
                Event::NominationBelowThreashold(9, 2, 10, 10, 0),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(NodleStaking::total(), 180);

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [Bond {
                        owner: 2,
                        amount: 40,
                    },]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 50);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 40);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(7).unwrap().nominations,
                OrderedSet::from([].to_vec()),
            );
            assert_eq!(NodleStaking::nominator_state(7).unwrap().total, 10);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().active_bond, 0);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(10).unwrap().nominations,
                OrderedSet::from([].to_vec()),
            );
            assert_eq!(NodleStaking::nominator_state(10).unwrap().total, 10);
            assert_eq!(NodleStaking::nominator_state(10).unwrap().active_bond, 0);
            assert_eq!(NodleStaking::nominator_state(10).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(8).unwrap().nominations,
                OrderedSet::from([].to_vec()),
            );
            assert_eq!(NodleStaking::nominator_state(8).unwrap().total, 10);
            assert_eq!(NodleStaking::nominator_state(8).unwrap().active_bond, 0);
            assert_eq!(NodleStaking::nominator_state(8).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(9).unwrap().nominations,
                OrderedSet::from([].to_vec()),
            );
            assert_eq!(NodleStaking::nominator_state(9).unwrap().total, 10);
            assert_eq!(NodleStaking::nominator_state(9).unwrap().active_bond, 0);
            assert_eq!(NodleStaking::nominator_state(9).unwrap().frozen_bond, 10);

            assert_ok!(NodleStaking::unbond_frozen(Origin::signed(6)));

            let mut new1 = vec![Event::NominationUnbondFrozen(6, 10, 40, 40)];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (100, 40));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(System::consumers(&6), 1);

            assert_ok!(NodleStaking::unbond_frozen(Origin::signed(7)));

            let mut new1 = vec![
                Event::NominationUnbondFrozen(7, 10, 0, 0),
                Event::NominatorLeft(7, 10),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&7), (100, 0));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(System::consumers(&7), 0);

            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(7), 2, 50, false),
                Error::<Test>::NominatorDNE,
            );

            assert_ok!(NodleStaking::unbond_frozen(Origin::signed(10)));

            let mut new1 = vec![
                Event::NominationUnbondFrozen(10, 10, 0, 0),
                Event::NominatorLeft(10, 10),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&10), (100, 0));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(System::consumers(&10), 0);

            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(10), 2, 50, false),
                Error::<Test>::NominatorDNE,
            );

            assert_ok!(NodleStaking::unbond_frozen(Origin::signed(8)));

            let mut new1 = vec![
                Event::NominationUnbondFrozen(8, 10, 0, 0),
                Event::NominatorLeft(8, 10),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&8), (100, 0));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(System::consumers(&8), 0);

            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(8), 2, 50, false),
                Error::<Test>::NominatorDNE,
            );

            assert_ok!(NodleStaking::unbond_frozen(Origin::signed(9)));

            let mut new1 = vec![
                Event::NominationUnbondFrozen(9, 10, 0, 0),
                Event::NominatorLeft(9, 10),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&9), (100, 0));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(System::consumers(&9), 0);

            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(9), 2, 50, false),
                Error::<Test>::NominatorDNE,
            );
        });
}

#[test]
fn unfreeze_bond_arg_flag_works() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 25),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            // chooses top TotalSelectedCandidates (5), in order
            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 65),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 155),
                Event::ValidatorChosen(3, 1, 65),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 155),
                Event::ValidatorChosen(4, 1, 65),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 155),
                Event::ValidatorChosen(5, 1, 65),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 155),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 25));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 155);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);

            assert_eq!(
                NodleStaking::validator_state(1).unwrap().nominators,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 6,
                            amount: 10,
                        },
                        Bond {
                            owner: 7,
                            amount: 10,
                        },
                        Bond {
                            owner: 10,
                            amount: 25,
                        },
                    ]
                    .to_vec()
                ),
            );

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                3,
                20,
                false
            ));

            assert_eq!(mock::balances(&6), (100, 30));
            assert_eq!(Balances::total_balance(&6), 100);

            let mut new1 = vec![Event::Nomination(6, 20, 3, 40)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::validator_state(3).unwrap().nominators,
                OrderedSet::from(
                    [Bond {
                        owner: 6,
                        amount: 20,
                    },]
                    .to_vec()
                ),
            );

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 10,
                        },
                        Bond {
                            owner: 3,
                            amount: 20,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 30);
            assert_eq!(NodleStaking::total(), 175);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(7),
                3,
                20,
                true,
            ));

            assert_eq!(mock::balances(&7), (100, 30));
            assert_eq!(Balances::total_balance(&7), 100);

            let mut new1 = vec![Event::Nomination(7, 20, 3, 60)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(7).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 1,
                            amount: 10,
                        },
                        Bond {
                            owner: 3,
                            amount: 20,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(7).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().active_bond, 30);
            assert_eq!(NodleStaking::total(), 195);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(8),
                4,
                20,
                true,
            ));

            assert_eq!(mock::balances(&8), (100, 30));
            assert_eq!(Balances::total_balance(&8), 100);

            let mut new1 = vec![Event::Nomination(8, 20, 4, 40)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::nominator_state(8).unwrap().nominations,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 2,
                            amount: 10,
                        },
                        Bond {
                            owner: 4,
                            amount: 20,
                        },
                    ]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(8).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(8).unwrap().active_bond, 30);
            assert_eq!(NodleStaking::total(), 215);

            let s1_max_validators = NodleStaking::staking_max_validators();
            let s1_min_stake = NodleStaking::staking_min_stake_session_selection();
            let s1_min_validator_bond = NodleStaking::staking_min_validator_bond();
            let s1_min_nomination_total_bond = NodleStaking::staking_min_nominator_total_bond();
            let s1_min_nomination_chill_threshold =
                NodleStaking::staking_min_nomination_chill_threshold();

            let s1_new_min_nomination_total_bond = 25;
            let s1_new_min_nomination_chill_threshold = 15;

            assert_ok!(NodleStaking::set_staking_limits(
                Origin::signed(CancelOrigin::get()),
                s1_max_validators,
                s1_min_stake,
                s1_min_validator_bond,
                s1_new_min_nomination_total_bond,
                s1_new_min_nomination_chill_threshold,
            ));

            let mut new1 = vec![
                Event::NewStakingLimits(
                    s1_max_validators,
                    s1_max_validators,
                    s1_min_validator_bond,
                    s1_min_validator_bond,
                    s1_min_stake,
                    s1_min_stake,
                    s1_min_nomination_total_bond,
                    s1_new_min_nomination_total_bond,
                    s1_min_nomination_chill_threshold,
                    s1_new_min_nomination_chill_threshold,
                ),
                Event::NominationBelowThreashold(6, 1, 10, 10, 20),
                Event::NominationBelowThreashold(7, 1, 10, 10, 20),
                Event::NominationBelowThreashold(8, 2, 10, 10, 20),
                Event::NominationBelowThreashold(9, 2, 10, 10, 0),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::validator_state(1).unwrap().nominators,
                OrderedSet::from(
                    [Bond {
                        owner: 10,
                        amount: 25,
                    },]
                    .to_vec()
                ),
            );

            assert_eq!(
                NodleStaking::nominator_state(6).unwrap().nominations,
                OrderedSet::from(
                    [Bond {
                        owner: 3,
                        amount: 20,
                    },]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(6).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().active_bond, 20);
            assert_eq!(NodleStaking::nominator_state(6).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(7).unwrap().nominations,
                OrderedSet::from(
                    [Bond {
                        owner: 3,
                        amount: 20,
                    },]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(7).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().active_bond, 20);
            assert_eq!(NodleStaking::nominator_state(7).unwrap().frozen_bond, 10);

            assert_eq!(
                NodleStaking::nominator_state(8).unwrap().nominations,
                OrderedSet::from(
                    [Bond {
                        owner: 4,
                        amount: 20,
                    },]
                    .to_vec()
                ),
            );
            assert_eq!(NodleStaking::nominator_state(8).unwrap().total, 30);
            assert_eq!(NodleStaking::nominator_state(8).unwrap().active_bond, 20);
            assert_eq!(NodleStaking::nominator_state(8).unwrap().frozen_bond, 10);

            assert_noop!(
                NodleStaking::nominator_nominate(Origin::signed(6), 1, 14, false),
                Error::<Test>::NominationBelowMin,
            );

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                1,
                14,
                true
            ));

            let mut new1 = vec![Event::Nomination(6, 24, 1, 69)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::nominator_bond_more(
                Origin::signed(6),
                1,
                6,
                true
            ),);

            let mut new1 = vec![Event::NominationIncreased(6, 30, 1, 69, 75)];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::validator_state(1).unwrap().nominators,
                OrderedSet::from(
                    [
                        Bond {
                            owner: 6,
                            amount: 30
                        },
                        Bond {
                            owner: 10,
                            amount: 25
                        }
                    ]
                    .to_vec()
                ),
            );

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 60));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 30));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 30));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 25));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 245);

            assert_eq!(System::consumers(&1), 2);
            assert_eq!(System::consumers(&2), 2);
            assert_eq!(System::consumers(&3), 2);
            assert_eq!(System::consumers(&4), 2);
            assert_eq!(System::consumers(&5), 2);
            assert_eq!(System::consumers(&6), 1);
            assert_eq!(System::consumers(&7), 1);
            assert_eq!(System::consumers(&8), 1);
            assert_eq!(System::consumers(&9), 1);
            assert_eq!(System::consumers(&10), 1);
        });
}

#[test]
fn validators_bond() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
            (20, 1),
            (30, 1),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(6), 50),
                Error::<Test>::ValidatorDNE
            );

            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(1), 50));

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(1), 40),
                Error::<Test>::InsufficientBalance
            );

            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(1)));

            let mut new1 = vec![
                Event::ValidatorBondedMore(1, 20, 70),
                Event::ValidatorScheduledExit(4, 1, 6),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            mock::start_active_session(5);

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(1), 30),
                Error::<Test>::CannotActivateIfLeaving,
            );

            mock::start_active_session(6);

            let mut new2 = vec![
                Event::ValidatorChosen(6, 2, 40),
                Event::ValidatorChosen(6, 3, 20),
                Event::ValidatorChosen(6, 4, 20),
                Event::ValidatorChosen(6, 5, 10),
                Event::NewSession(25, 6, 4, 90),
                Event::NominatorLeftValidator(6, 1, 10, 0),
                Event::NominatorLeftValidator(7, 1, 10, 0),
                Event::NominatorLeftValidator(10, 1, 10, 0),
                Event::ValidatorLeft(1, 100, 90),
                Event::ValidatorChosen(7, 2, 40),
                Event::ValidatorChosen(7, 3, 20),
                Event::ValidatorChosen(7, 4, 20),
                Event::ValidatorChosen(7, 5, 10),
                Event::NewSession(30, 7, 4, 90),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(1), 40),
                Error::<Test>::ValidatorDNE,
            );

            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(2), 80));

            assert_ok!(NodleStaking::validator_bond_less(Origin::signed(2), 90));

            assert_ok!(NodleStaking::validator_bond_less(Origin::signed(3), 10));

            let mut new3 = vec![
                Event::ValidatorBondedMore(2, 20, 100),
                Event::ValidatorBondedLess(2, 100, 10),
                Event::ValidatorBondedLess(3, 20, 10),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert_noop!(
                NodleStaking::validator_bond_less(Origin::signed(2), 11),
                Error::<Test>::Underflow
            );
            assert_noop!(
                NodleStaking::validator_bond_less(Origin::signed(2), 1),
                Error::<Test>::ValidatorBondBelowMin
            );
            assert_noop!(
                NodleStaking::validator_bond_less(Origin::signed(3), 1),
                Error::<Test>::ValidatorBondBelowMin
            );
            assert_noop!(
                NodleStaking::validator_bond_less(Origin::signed(4), 11),
                Error::<Test>::ValidatorBondBelowMin
            );

            assert_ok!(NodleStaking::validator_bond_less(Origin::signed(4), 10));

            mock::start_active_session(9);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(2)));
            assert_eq!(mock::balances(&2), (100, 10));

            let mut new4 = vec![
                Event::ValidatorBondedLess(4, 20, 10),
                Event::ValidatorChosen(8, 2, 30),
                Event::ValidatorChosen(8, 3, 10),
                Event::ValidatorChosen(8, 4, 10),
                Event::ValidatorChosen(8, 5, 10),
                Event::NewSession(35, 8, 4, 60),
                Event::ValidatorChosen(9, 2, 30),
                Event::ValidatorChosen(9, 3, 10),
                Event::ValidatorChosen(9, 4, 10),
                Event::ValidatorChosen(9, 5, 10),
                Event::NewSession(40, 9, 4, 60),
                Event::ValidatorChosen(10, 2, 30),
                Event::ValidatorChosen(10, 3, 10),
                Event::ValidatorChosen(10, 4, 10),
                Event::ValidatorChosen(10, 5, 10),
                Event::NewSession(45, 10, 4, 60),
                Event::Withdrawn(2, 90),
            ];
            expected.append(&mut new4);
            assert_eq!(events(), expected);
        });
}

#[test]
fn nominators_bond() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(1), 2, 50, false),
                Error::<Test>::NominatorDNE,
            );
            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(6), 2, 50, false),
                Error::<Test>::NominationDNE,
            );
            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(7), 6, 50, false),
                Error::<Test>::ValidatorDNE,
            );
            assert_noop!(
                NodleStaking::nominator_bond_less(Origin::signed(6), 1, 11),
                Error::<Test>::Underflow,
            );
            assert_noop!(
                NodleStaking::nominator_bond_less(Origin::signed(6), 1, 8),
                Error::<Test>::NominationBelowMin,
            );
            assert_noop!(
                NodleStaking::nominator_bond_less(Origin::signed(6), 1, 6),
                Error::<Test>::NominatorBondBelowMin,
            );

            assert_ok!(NodleStaking::nominator_bond_more(
                Origin::signed(6),
                1,
                10,
                false
            ));

            assert_noop!(
                NodleStaking::nominator_bond_less(Origin::signed(6), 2, 5),
                Error::<Test>::NominationDNE,
            );
            assert_noop!(
                NodleStaking::nominator_bond_more(Origin::signed(6), 1, 81, false),
                Error::<Test>::InsufficientBalance,
            );

            mock::start_active_session(5);

            assert_eq!(mock::balances(&6), (100, 20));
            assert_eq!(NodleStaking::total(), 150);
            assert_ok!(NodleStaking::validator_exit_pool(Origin::signed(1)));

            let mut new1 = vec![
                Event::NominationIncreased(6, 20, 1, 50, 60),
                Event::ValidatorChosen(6, 1, 60),
                Event::ValidatorChosen(6, 2, 40),
                Event::ValidatorChosen(6, 3, 20),
                Event::ValidatorChosen(6, 4, 20),
                Event::ValidatorChosen(6, 5, 10),
                Event::NewSession(25, 6, 5, 150),
                Event::ValidatorScheduledExit(5, 1, 7),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            mock::start_active_session(7);

            let mut new2 = vec![
                Event::ValidatorChosen(7, 2, 40),
                Event::ValidatorChosen(7, 3, 20),
                Event::ValidatorChosen(7, 4, 20),
                Event::ValidatorChosen(7, 5, 10),
                Event::NewSession(30, 7, 4, 90),
                Event::NominatorLeftValidator(6, 1, 20, 0),
                Event::NominatorLeftValidator(7, 1, 10, 0),
                Event::NominatorLeftValidator(10, 1, 10, 0),
                Event::ValidatorLeft(1, 60, 90),
                Event::ValidatorChosen(8, 2, 40),
                Event::ValidatorChosen(8, 3, 20),
                Event::ValidatorChosen(8, 4, 20),
                Event::ValidatorChosen(8, 5, 10),
                Event::NewSession(35, 8, 4, 90),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            mock::start_active_session(8);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(7)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(10)));

            let mut new3 = vec![
                Event::ValidatorChosen(9, 2, 40),
                Event::ValidatorChosen(9, 3, 20),
                Event::ValidatorChosen(9, 4, 20),
                Event::ValidatorChosen(9, 5, 10),
                Event::NewSession(40, 9, 4, 90),
                Event::Withdrawn(6, 20),
                Event::NominatorLeft(6, 20),
                Event::Withdrawn(7, 10),
                Event::NominatorLeft(7, 10),
                Event::Withdrawn(10, 10),
                Event::NominatorLeft(10, 10),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert!(!NodleStaking::is_nominator(&6));
            assert!(!NodleStaking::is_nominator(&7));
            assert!(!NodleStaking::is_nominator(&10));

            assert_eq!(mock::balances(&1), (100, 0));
            assert_eq!(mock::balances(&6), (100, 0));
            assert_eq!(mock::balances(&7), (100, 0));
            assert_eq!(mock::balances(&10), (100, 0));

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(mock::balances(&9), (100, 10));

            assert_eq!(NodleStaking::total(), 90);
        });
}

#[test]
fn revoke_nomination_or_leave_nominators() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            assert_noop!(
                NodleStaking::nominator_denominate(Origin::signed(1), 2),
                Error::<Test>::NominatorDNE,
            );
            assert_noop!(
                NodleStaking::nominator_denominate(Origin::signed(6), 2),
                Error::<Test>::NominationDNE,
            );
            assert_noop!(
                NodleStaking::nominator_denominate_all(Origin::signed(1)),
                Error::<Test>::NominatorDNE,
            );
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                2,
                3,
                false
            ));
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(6),
                3,
                3,
                false
            ));
            assert_ok!(NodleStaking::nominator_denominate(Origin::signed(6), 1));

            let mut new1 = vec![
                Event::Nomination(6, 3, 2, 43),
                Event::Nomination(6, 3, 3, 23),
                Event::NominatorLeftValidator(6, 1, 10, 40),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            mock::start_active_session(6);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));

            let mut new2 = vec![
                Event::ValidatorChosen(6, 1, 40),
                Event::ValidatorChosen(6, 2, 43),
                Event::ValidatorChosen(6, 3, 23),
                Event::ValidatorChosen(6, 4, 20),
                Event::ValidatorChosen(6, 5, 10),
                Event::NewSession(25, 6, 5, 136),
                Event::ValidatorChosen(7, 1, 40),
                Event::ValidatorChosen(7, 2, 43),
                Event::ValidatorChosen(7, 3, 23),
                Event::ValidatorChosen(7, 4, 20),
                Event::ValidatorChosen(7, 5, 10),
                Event::NewSession(30, 7, 5, 136),
                Event::Withdrawn(6, 10),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_noop!(
                NodleStaking::nominator_denominate(Origin::signed(6), 2),
                Error::<Test>::NominatorBondBelowMin,
            );
            assert_noop!(
                NodleStaking::nominator_denominate(Origin::signed(6), 3),
                Error::<Test>::NominatorBondBelowMin,
            );
            // can revoke both remaining by calling leave nominators
            assert_ok!(NodleStaking::nominator_denominate_all(Origin::signed(6)));
            // this leads to 8 leaving set of nominators
            assert_noop!(
                NodleStaking::nominator_denominate(Origin::signed(8), 2),
                Error::<Test>::NominatorBondBelowMin,
            );
            assert_ok!(NodleStaking::nominator_denominate_all(Origin::signed(8)));

            let mut new3 = vec![
                Event::NominatorLeftValidator(6, 2, 3, 40),
                Event::NominatorLeftValidator(6, 3, 3, 20),
                Event::NominatorLeftValidator(8, 2, 10, 30),
            ];

            expected.append(&mut new3);
            assert_eq!(events(), expected);

            mock::start_active_session(8);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));
            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(8)));

            let mut new4 = vec![
                Event::ValidatorChosen(8, 1, 40),
                Event::ValidatorChosen(8, 2, 30),
                Event::ValidatorChosen(8, 3, 20),
                Event::ValidatorChosen(8, 4, 20),
                Event::ValidatorChosen(8, 5, 10),
                Event::NewSession(35, 8, 5, 120),
                Event::ValidatorChosen(9, 1, 40),
                Event::ValidatorChosen(9, 2, 30),
                Event::ValidatorChosen(9, 3, 20),
                Event::ValidatorChosen(9, 4, 20),
                Event::ValidatorChosen(9, 5, 10),
                Event::NewSession(40, 9, 5, 120),
                Event::Withdrawn(6, 6),
                Event::NominatorLeft(6, 6),
                Event::Withdrawn(8, 10),
                Event::NominatorLeft(8, 10),
            ];

            expected.append(&mut new4);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (100, 0));
            assert_eq!(mock::balances(&8), (100, 0));

            assert_eq!(NodleStaking::total(), 120);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("S-1");
        });
}

#[test]
fn payouts_follow_nomination_changes() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_validators(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_nominators(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .tst_staking_build()
        .execute_with(|| {
            mock::start_active_session(4);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 50),
                Event::ValidatorChosen(2, 2, 40),
                Event::ValidatorChosen(2, 3, 20),
                Event::ValidatorChosen(2, 4, 20),
                Event::ValidatorChosen(2, 5, 10),
                Event::NewSession(5, 2, 5, 140),
                Event::ValidatorChosen(3, 1, 50),
                Event::ValidatorChosen(3, 2, 40),
                Event::ValidatorChosen(3, 3, 20),
                Event::ValidatorChosen(3, 4, 20),
                Event::ValidatorChosen(3, 5, 10),
                Event::NewSession(10, 3, 5, 140),
                Event::ValidatorChosen(4, 1, 50),
                Event::ValidatorChosen(4, 2, 40),
                Event::ValidatorChosen(4, 3, 20),
                Event::ValidatorChosen(4, 4, 20),
                Event::ValidatorChosen(4, 5, 10),
                Event::NewSession(15, 4, 5, 140),
                Event::ValidatorChosen(5, 1, 50),
                Event::ValidatorChosen(5, 2, 40),
                Event::ValidatorChosen(5, 3, 20),
                Event::ValidatorChosen(5, 4, 20),
                Event::ValidatorChosen(5, 5, 10),
                Event::NewSession(20, 5, 5, 140),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&1), (100, 20));
            assert_eq!(Balances::total_balance(&1), 100);

            assert_eq!(mock::balances(&2), (100, 20));
            assert_eq!(Balances::total_balance(&2), 100);

            assert_eq!(mock::balances(&3), (100, 20));
            assert_eq!(Balances::total_balance(&3), 100);

            assert_eq!(mock::balances(&4), (100, 20));
            assert_eq!(Balances::total_balance(&4), 100);

            assert_eq!(mock::balances(&5), (100, 10));
            assert_eq!(Balances::total_balance(&5), 100);

            assert_eq!(mock::balances(&6), (100, 10));
            assert_eq!(Balances::total_balance(&6), 100);

            assert_eq!(mock::balances(&7), (100, 10));
            assert_eq!(Balances::total_balance(&7), 100);

            assert_eq!(mock::balances(&8), (100, 10));
            assert_eq!(Balances::total_balance(&8), 100);

            assert_eq!(mock::balances(&9), (100, 10));
            assert_eq!(Balances::total_balance(&9), 100);

            assert_eq!(mock::balances(&10), (100, 10));
            assert_eq!(Balances::total_balance(&10), 100);

            assert_eq!(NodleStaking::total(), 140);

            set_author(4, 1, 100);
            mock::mint_rewards(1_000_000);
            mock::start_active_session(5);

            let mut new1 = vec![
                Event::StakeReward(1, 520000),
                Event::StakeReward(6, 160000),
                Event::StakeReward(7, 160000),
                Event::StakeReward(10, 160000),
                Event::ValidatorChosen(6, 1, 50),
                Event::ValidatorChosen(6, 2, 40),
                Event::ValidatorChosen(6, 3, 20),
                Event::ValidatorChosen(6, 4, 20),
                Event::ValidatorChosen(6, 5, 10),
                Event::NewSession(25, 6, 5, 140),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            // ~ set block author as 1 for all blocks this round
            set_author(5, 1, 100);
            mock::mint_rewards(1_000_000);

            // 1. ensure nominators are paid for 2 rounds after they leave
            assert_noop!(
                NodleStaking::nominator_denominate_all(Origin::signed(66)),
                Error::<Test>::NominatorDNE
            );

            assert_ok!(NodleStaking::nominator_denominate_all(Origin::signed(6)));

            mock::start_active_session(6);

            let mut new2 = vec![
                Event::NominatorLeftValidator(6, 1, 10, 40),
                Event::StakeReward(1, 520000),
                Event::StakeReward(6, 160000),
                Event::StakeReward(7, 160000),
                Event::StakeReward(10, 160000),
                Event::ValidatorChosen(7, 1, 40),
                Event::ValidatorChosen(7, 2, 40),
                Event::ValidatorChosen(7, 3, 20),
                Event::ValidatorChosen(7, 4, 20),
                Event::ValidatorChosen(7, 5, 10),
                Event::NewSession(30, 7, 5, 130),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            set_author(6, 1, 100);
            mock::mint_rewards(1_000_000);
            mock::start_active_session(7);

            let mut new3 = vec![
                Event::StakeReward(1, 520000),
                Event::StakeReward(6, 160000),
                Event::StakeReward(7, 160000),
                Event::StakeReward(10, 160000),
                Event::ValidatorChosen(8, 1, 40),
                Event::ValidatorChosen(8, 2, 40),
                Event::ValidatorChosen(8, 3, 20),
                Event::ValidatorChosen(8, 4, 20),
                Event::ValidatorChosen(8, 5, 10),
                Event::NewSession(35, 8, 5, 130),
            ];
            expected.append(&mut new3);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(8),
                1,
                10,
                false
            ));

            mock::start_active_session(8);

            let mut new4 = vec![
                Event::Nomination(8, 10, 1, 50),
                Event::ValidatorChosen(9, 1, 50),
                Event::ValidatorChosen(9, 2, 40),
                Event::ValidatorChosen(9, 3, 20),
                Event::ValidatorChosen(9, 4, 20),
                Event::ValidatorChosen(9, 5, 10),
                Event::NewSession(40, 9, 5, 140),
            ];
            expected.append(&mut new4);
            assert_eq!(events(), expected);

            set_author(8, 1, 100);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(10);

            let mut new5 = vec![
                Event::StakeReward(1, 600000),
                Event::StakeReward(7, 200000),
                Event::StakeReward(10, 200000),
                Event::ValidatorChosen(10, 1, 50),
                Event::ValidatorChosen(10, 2, 40),
                Event::ValidatorChosen(10, 3, 20),
                Event::ValidatorChosen(10, 4, 20),
                Event::ValidatorChosen(10, 5, 10),
                Event::NewSession(45, 10, 5, 140),
                Event::ValidatorChosen(11, 1, 50),
                Event::ValidatorChosen(11, 2, 40),
                Event::ValidatorChosen(11, 3, 20),
                Event::ValidatorChosen(11, 4, 20),
                Event::ValidatorChosen(11, 5, 10),
                Event::NewSession(50, 11, 5, 140),
            ];
            expected.append(&mut new5);
            assert_eq!(events(), expected);

            set_author(10, 1, 100);
            mock::mint_rewards(1_000_000);

            mock::start_active_session(11);

            let mut new6 = vec![
                Event::StakeReward(1, 520000),
                Event::StakeReward(7, 160000),
                Event::StakeReward(8, 160000),
                Event::StakeReward(10, 160000),
                Event::ValidatorChosen(12, 1, 50),
                Event::ValidatorChosen(12, 2, 40),
                Event::ValidatorChosen(12, 3, 20),
                Event::ValidatorChosen(12, 4, 20),
                Event::ValidatorChosen(12, 5, 10),
                Event::NewSession(55, 12, 5, 140),
            ];
            expected.append(&mut new6);
            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::withdraw_unbonded(Origin::signed(6)));

            let mut new7 = vec![Event::Withdrawn(6, 10), Event::NominatorLeft(6, 10)];
            expected.append(&mut new7);

            assert_eq!(events(), expected);

            assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(6)));

            let mut new8 = vec![Event::Rewarded(6, 480000)];
            expected.append(&mut new8);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&6), (480100, 0));
            assert_eq!(NodleStaking::total(), 140);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        });
}

#[test]
fn set_invulnerables_works() {
    ExtBuilder::default().build_and_execute(|| {
        let new_set1 = vec![1, 2];

        assert_ok!(NodleStaking::set_invulnerables(
            Origin::root(),
            new_set1.clone()
        ));
        assert_eq!(NodleStaking::invulnerables(), new_set1);

        let mut expected = vec![Event::NewInvulnerables([1, 2].to_vec())];
        assert_eq!(events(), expected);

        let new_set2 = vec![3, 4];

        assert_ok!(NodleStaking::set_invulnerables(
            Origin::signed(CancelOrigin::get()),
            new_set2.clone()
        ));
        assert_eq!(NodleStaking::invulnerables(), new_set2);

        let mut new1 = vec![Event::NewInvulnerables([3, 4].to_vec())];
        expected.append(&mut new1);
        assert_eq!(events(), expected);

        // cannot set with non-root.
        assert_noop!(
            NodleStaking::set_invulnerables(Origin::signed(1), new_set2.clone()),
            BadOrigin
        );
    });
}

#[test]
fn set_total_validator_per_round_works() {
    ExtBuilder::default().build_and_execute(|| {
        let old_total_selected = NodleStaking::total_selected();
        let new_total_selected = old_total_selected * 4u32;

        assert_noop!(
            NodleStaking::set_total_validator_per_round(Origin::signed(1), new_total_selected),
            BadOrigin
        );

        assert_ok!(NodleStaking::set_total_validator_per_round(
            Origin::signed(CancelOrigin::get()),
            new_total_selected
        ));

        let mut expected = vec![Event::TotalSelectedSet(
            old_total_selected,
            new_total_selected,
        )];
        assert_eq!(events(), expected);

        assert_eq!(NodleStaking::total_selected(), new_total_selected);

        let new_total_selected = old_total_selected * 2u32;

        assert_ok!(NodleStaking::set_total_validator_per_round(
            Origin::root(),
            new_total_selected
        ));

        let mut new1 = vec![Event::TotalSelectedSet(
            old_total_selected * 4u32,
            new_total_selected,
        )];
        expected.append(&mut new1);
        assert_eq!(events(), expected);

        assert_eq!(NodleStaking::total_selected(), new_total_selected);
    });
}

#[test]
fn set_staking_limits_works() {
    ExtBuilder::default().build_and_execute(|| {
        let old_max_validators = NodleStaking::staking_max_validators();
        let old_min_stake = NodleStaking::staking_min_stake_session_selection();
        let old_min_validator_bond = NodleStaking::staking_min_validator_bond();
        let old_min_nomination_total_bond = NodleStaking::staking_min_nominator_total_bond();
        let old_min_nomination_chill_threshold =
            NodleStaking::staking_min_nomination_chill_threshold();

        let new_max_validators = 100;
        let new_min_stake = old_min_stake.saturating_mul(4u32.into());
        let new_min_validator_bond = old_min_validator_bond.saturating_mul(4u32.into());
        let new_min_nomination_total_bond =
            old_min_nomination_total_bond.saturating_mul(4u32.into());
        let new_min_nomination_chill_threshold =
            old_min_nomination_chill_threshold.saturating_mul(4u32.into());

        assert_noop!(
            NodleStaking::set_staking_limits(
                Origin::signed(1),
                new_max_validators,
                new_min_stake,
                new_min_validator_bond,
                new_min_nomination_total_bond,
                new_min_nomination_chill_threshold,
            ),
            BadOrigin
        );

        assert_ok!(NodleStaking::set_staking_limits(
            Origin::signed(CancelOrigin::get()),
            new_max_validators,
            new_min_stake,
            new_min_validator_bond,
            new_min_nomination_total_bond,
            new_min_nomination_chill_threshold,
        ));

        let mut expected = vec![Event::NewStakingLimits(
            old_max_validators,
            new_max_validators,
            old_min_stake,
            new_min_stake,
            old_min_validator_bond,
            new_min_validator_bond,
            old_min_nomination_total_bond,
            new_min_nomination_total_bond,
            old_min_nomination_chill_threshold,
            new_min_nomination_chill_threshold,
        )];
        assert_eq!(events(), expected);

        assert_eq!(NodleStaking::staking_max_validators(), new_max_validators);
        assert_eq!(
            NodleStaking::staking_min_stake_session_selection(),
            new_min_stake
        );
        assert_eq!(
            NodleStaking::staking_min_validator_bond(),
            new_min_validator_bond
        );
        assert_eq!(
            NodleStaking::staking_min_nominator_total_bond(),
            new_min_nomination_total_bond
        );
        assert_eq!(
            NodleStaking::staking_min_nomination_chill_threshold(),
            new_min_nomination_chill_threshold
        );

        let new2_max_validators = 75;
        let new2_min_stake = old_min_stake.saturating_mul(2u32.into());
        let new2_min_validator_bond = old_min_validator_bond.saturating_mul(2u32.into());
        let new2_min_nomination_total_bond =
            old_min_nomination_total_bond.saturating_mul(2u32.into());
        let new2_min_nomination_chill_threshold =
            old_min_nomination_chill_threshold.saturating_mul(2u32.into());

        assert_ok!(NodleStaking::set_staking_limits(
            Origin::root(),
            new2_max_validators,
            new2_min_stake,
            new2_min_validator_bond,
            new2_min_nomination_total_bond,
            new2_min_nomination_chill_threshold,
        ));

        let mut new1 = vec![Event::NewStakingLimits(
            new_max_validators,
            new2_max_validators,
            new_min_stake,
            new2_min_stake,
            new_min_validator_bond,
            new2_min_validator_bond,
            new_min_nomination_total_bond,
            new2_min_nomination_total_bond,
            new_min_nomination_chill_threshold,
            new2_min_nomination_chill_threshold,
        )];
        expected.append(&mut new1);
        assert_eq!(events(), expected);

        assert_eq!(NodleStaking::staking_max_validators(), new2_max_validators);
        assert_eq!(
            NodleStaking::staking_min_stake_session_selection(),
            new2_min_stake
        );
        assert_eq!(
            NodleStaking::staking_min_validator_bond(),
            new2_min_validator_bond
        );
        assert_eq!(
            NodleStaking::staking_min_nominator_total_bond(),
            new2_min_nomination_total_bond
        );
        assert_eq!(
            NodleStaking::staking_min_nomination_chill_threshold(),
            new2_min_nomination_chill_threshold
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

        mock::mint_rewards(1_000_000);
        // NodleStaking::reward_by_ids(vec![(11, 1)]);

        mock::start_active_session(2);

        // tst_tst_log!(trace,
        // 	"Actv Sess 2 event {:#?}",
        // 	mock::events()
        // );

        // assert_ok!(NodleStaking::payout_stakers(Origin::signed(1337), 11, 1));

        // // Controller is created
        // assert!(Balances::free_balance(1337) > 0);
    })
}

#[test]
fn reward_from_authorship_event_handler_works() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
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
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
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

#[test]
fn reward_validator_slashing_validator_does_not_overflow() {
    ExtBuilder::default()
        .num_validators(4)
        .has_stakers(true)
        .build_and_execute(|| {
            let stake = u64::max_value() as Balance * 2;
            let reward_slash = u64::max_value() as Balance * 2;

            // --- Session 2:
            start_session(2);

            // Assert multiplication overflows in balance arithmetic.
            assert!(stake.checked_mul(reward_slash).is_none());

            // Set staker
            let _ = Balances::make_free_balance_be(&81, stake);

            bond_validator(81, stake);

			assert_ok!(Session::set_keys(Origin::signed(81), UintAuthorityId(81).into(), vec![]));

            // --- Session 3:
            start_session(3);

            assert_eq!(mock::validators_in_pool(), vec![11, 21, 41, 81]);
            assert_eq!(mock::selected_validators(), vec![11, 21, 41, 81]);

            // Inject reward
            Points::<Test>::insert(3, 1);
            AwardedPts::<Test>::insert(3, 81, 1);
            SessionAccumulatedBalance::<Test>::insert(3, stake);
            assert_eq!(NodleStaking::total(), 36893488147419106730);

            // --- Session 4:
            // Trigger payout for Session 3
            start_session(4);

			let mut expected = vec![
				Event::ValidatorChosen(
					2,
					11,
					1500,
				),
				Event::ValidatorChosen(
					2,
					21,
					1000,
				),
				Event::ValidatorChosen(
					2,
					41,
					1000,
				),
				Event::NewSession(
					5,
					2,
					3,
					3500,
				),
				Event::ValidatorChosen(
					3,
					11,
					1500,
				),
				Event::ValidatorChosen(
					3,
					21,
					1000,
				),
				Event::ValidatorChosen(
					3,
					41,
					1000,
				),
				Event::NewSession(
					10,
					3,
					3,
					3500,
				),
				Event::JoinedValidatorPool(
					81,
					36893488147419103230,
					36893488147419106730,
				),
				Event::ValidatorChosen(
					4,
					11,
					1500,
				),
				Event::ValidatorChosen(
					4,
					21,
					1000,
				),
				Event::ValidatorChosen(
					4,
					41,
					1000,
				),
				Event::ValidatorChosen(
					4,
					81,
					36893488147419103230,
				),
				Event::NewSession(
					15,
					4,
					4,
					36893488147419106730,
				),
				Event::StakeReward(
					81,
					36893488147419103230,
				),
				Event::ValidatorChosen(
					5,
					11,
					1500,
				),
				Event::ValidatorChosen(
					5,
					21,
					1000,
				),
				Event::ValidatorChosen(
					5,
					41,
					1000,
				),
				Event::ValidatorChosen(
					5,
					81,
					36893488147419103230,
				),
				Event::NewSession(
					20,
					5,
					4,
					36893488147419106730,
				),
			];
            assert_eq!(events(), expected);

			assert_ok!(NodleStaking::withdraw_staking_rewards(Origin::signed(81)));

			let mut new1 = vec![
				Event::Rewarded(
					81,
					36893488147419103230,
				),
			];
			expected.append(&mut new1);
            assert_eq!(events(), expected);

            // Payout made for Acc-81
            assert_eq!(Balances::total_balance(&81), (stake * 2));

            // Inject reward
            Points::<Test>::insert(4, 1);
            AwardedPts::<Test>::insert(4, 81, 1);
            SessionAccumulatedBalance::<Test>::insert(4, stake);

            // Add Nominators to Validator 81
            let _ = Balances::make_free_balance_be(&201, stake);
            let _ = Balances::make_free_balance_be(&301, stake);

            bond_nominator(201, stake - 1, 81);
            bond_nominator(301, stake - 1, 81);

            // --- Session 5:
            start_session(5);

            // Ensure nominators are updated to
            // AtStake Validator snapshot
            assert_eq!(
                NodleStaking::at_stake(6, 81),
                ValidatorSnapshot {
                    bond: 36893488147419103230,
                    nominators: vec![
                        Bond {
                            owner: 201,
                            amount: 36893488147419103229
                        },
                        Bond {
                            owner: 301,
                            amount: 36893488147419103229
                        },
                    ],
                    total: 110680464442257309688,
                }
            );

            log::trace!(
                "reward_validator_slashing_validator_does_not_overflow:[{:#?}] - Bonded Sess - {:#?}",
                line!(),
                NodleStaking::bonded_sessions()
            );

			log::trace!(
                "reward_validator_slashing_validator_does_not_overflow:[{:#?}] - AtStake - {:#?}",
                line!(),
                NodleStaking::at_stake(4, 81)
            );

            // Check slashing
            on_offence_in_session(
                &[OffenceDetails {
                    offender: (81, NodleStaking::at_stake(4, 81)),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(50)],
                4,
            );

            // assert_eq!(Balances::total_balance(&11), stake - 1);
            // assert_eq!(Balances::total_balance(&2), 1);
        });
}

#[test]
fn nominators_also_get_slashed_pro_rata() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            let slash_percent = Perbill::from_percent(5);
            let initial_exposure = NodleStaking::at_stake(NodleStaking::active_session(), 11);

            // 101 is a nominator for 11
            assert_eq!(initial_exposure.nominators.first().unwrap().owner, 101);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            // staked values;
            let nominator_stake = NodleStaking::nominator_state(101).unwrap().total;
            let validator_stake = NodleStaking::validator_state(11).unwrap().bond;
            let exposed_stake = initial_exposure.total;
            let exposed_validator = initial_exposure.bond;
            let exposed_nominator = initial_exposure.nominators.first().unwrap().amount;

            mock::on_offence_now(
                &[OffenceDetails {
                    offender: (11, initial_exposure.clone()),
                    reporters: vec![],
                }],
                &[slash_percent],
            );

            // Ensure both Validator & Nominator are slashed.
            let mut new1 = vec![Event::Slash(11, 50), Event::Slash(101, 25)];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            // both stakes must have been decreased.
            assert!(NodleStaking::nominator_state(101).unwrap().total < nominator_stake);
            assert!(NodleStaking::validator_state(11).unwrap().bond < validator_stake);
            assert_eq!(NodleStaking::total(), 3425);

            let slash_amount = slash_percent * exposed_stake;
            let validator_share =
                Perbill::from_rational_approximation(exposed_validator, exposed_stake)
                    * slash_amount;
            let nominator_share =
                Perbill::from_rational_approximation(exposed_nominator, exposed_stake)
                    * slash_amount;

            // both slash amounts need to be positive for the test to make sense.
            assert!(validator_share > 0);
            assert!(nominator_share > 0);

            // both stakes must have been decreased pro-rata.
            assert_eq!(
                NodleStaking::nominator_state(101).unwrap().total,
                nominator_stake - nominator_share,
            );
            assert_eq!(
                NodleStaking::validator_state(11).unwrap().bond,
                validator_stake - validator_share,
            );
            assert_eq!(
                mock::balances(&101),
                (1975, nominator_stake - nominator_share)
            );
            assert_eq!(
                mock::balances(&11),
                (1950, validator_stake - validator_share),
            );

            // Because slashing happened.
            assert!(is_disabled(11));
        });
}

#[test]
fn slashing_performed_according_exposure() {
    // This test checks that slashing is performed according the exposure (or more precisely,
    // historical exposure), not the current balance.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            let validator_bond_value = mock::balances(&11).1;

            assert_eq!(
                NodleStaking::at_stake(NodleStaking::active_session(), 11).bond,
                1000
            );

            // Handle an offence with a historical exposure.
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        ValidatorSnapshot {
                            total: 500,
                            bond: 500,
                            nominators: vec![],
                        },
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(50)],
            );

            let mut new2 = vec![Event::Slash(11, 250)];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            // The validator controller account should be slashed for 250 (50% of 500).
            assert_eq!(mock::balances(&11), (1750, validator_bond_value - 250));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3250);
        });
}

#[test]
fn slash_in_old_span_does_not_deselect() {
    ExtBuilder::default()
        .bonded_duration(10)
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(5);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
                Event::ValidatorChosen(3, 11, 1500),
                Event::ValidatorChosen(3, 21, 1000),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 3, 3500),
                Event::ValidatorChosen(4, 11, 1500),
                Event::ValidatorChosen(4, 21, 1000),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 3, 3500),
                Event::ValidatorChosen(5, 11, 1500),
                Event::ValidatorChosen(5, 21, 1000),
                Event::ValidatorChosen(5, 41, 1000),
                Event::NewSession(20, 5, 3, 3500),
                Event::ValidatorChosen(6, 11, 1500),
                Event::ValidatorChosen(6, 21, 1000),
                Event::ValidatorChosen(6, 41, 1000),
                Event::NewSession(25, 6, 3, 3500),
            ];
            assert_eq!(events(), expected);

            mock::set_author(5, 11, 20);
            mock::set_author(5, 21, 40);
            mock::set_author(5, 41, 40);

            assert!(<AwardedPts<Test>>::contains_key(5, 11));
            assert!(<AwardedPts<Test>>::contains_key(5, 21));
            assert!(<AwardedPts<Test>>::contains_key(5, 41));

            assert!(<ValidatorState<Test>>::contains_key(11));
            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Active
            );

            assert!(Session::validators().contains(&11));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(20)],
            );

            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Idle
            );

            let mut new1 = vec![Event::Slash(11, 200), Event::Slash(101, 100)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);
            assert_eq!(NodleStaking::total(), 3200);

            mock::start_active_session(8);

            let mut new2 = vec![
                Event::ValidatorChosen(7, 21, 1000),
                Event::ValidatorChosen(7, 41, 1000),
                Event::NewSession(30, 7, 2, 2000),
                Event::ValidatorChosen(8, 21, 1000),
                Event::ValidatorChosen(8, 41, 1000),
                Event::NewSession(35, 8, 2, 2000),
                Event::ValidatorChosen(9, 21, 1000),
                Event::ValidatorChosen(9, 41, 1000),
                Event::NewSession(40, 9, 2, 2000),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert!(<AwardedPts<Test>>::contains_key(5, 11));
            assert!(<AwardedPts<Test>>::contains_key(5, 21));
            assert!(<AwardedPts<Test>>::contains_key(5, 41));

            assert_eq!(NodleStaking::total(), 3200);

            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(11), 10));
            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Active
            );
            assert_eq!(
                mock::last_event(),
                MetaEvent::nodle_staking(Event::ValidatorBondedMore(11, 800, 810))
            );

            mock::start_active_session(11);

            let mut new3 = vec![
                Event::ValidatorBondedMore(11, 800, 810),
                Event::ValidatorChosen(10, 11, 1210),
                Event::ValidatorChosen(10, 21, 1000),
                Event::ValidatorChosen(10, 41, 1000),
                Event::NewSession(45, 10, 3, 3210),
                Event::ValidatorChosen(11, 11, 1210),
                Event::ValidatorChosen(11, 21, 1000),
                Event::ValidatorChosen(11, 41, 1000),
                Event::NewSession(50, 11, 3, 3210),
                Event::ValidatorChosen(12, 11, 1210),
                Event::ValidatorChosen(12, 21, 1000),
                Event::ValidatorChosen(12, 41, 1000),
                Event::NewSession(55, 12, 3, 3210),
            ];
            expected.append(&mut new3);
            assert_eq!(mock::events(), expected);
            assert_eq!(NodleStaking::total(), 3210);

            mock::start_active_session(14);

            let bonded_sess_state1 = vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
            assert_eq!(NodleStaking::bonded_sessions(), bonded_sess_state1);

            let mut new4 = vec![
                Event::ValidatorChosen(13, 11, 1210),
                Event::ValidatorChosen(13, 21, 1000),
                Event::ValidatorChosen(13, 41, 1000),
                Event::NewSession(60, 13, 3, 3210),
                Event::ValidatorChosen(14, 11, 1210),
                Event::ValidatorChosen(14, 21, 1000),
                Event::ValidatorChosen(14, 41, 1000),
                Event::NewSession(65, 14, 3, 3210),
                Event::ValidatorChosen(15, 11, 1210),
                Event::ValidatorChosen(15, 21, 1000),
                Event::ValidatorChosen(15, 41, 1000),
                Event::NewSession(70, 15, 3, 3210),
            ];
            expected.append(&mut new4);
            assert_eq!(mock::events(), expected);
            assert_eq!(NodleStaking::total(), 3210);

            on_offence_in_session(
                &[OffenceDetails {
                    offender: (11, NodleStaking::at_stake(5, 11)),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
                5,
            );

            // Ensure no new events
            // Since slash 10% is less-than prev-slash 20%.
            assert_eq!(events(), expected);

            on_offence_in_session(
                &[OffenceDetails {
                    offender: (11, NodleStaking::at_stake(5, 11)),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(30)],
                5,
            );

            // Ensure Validator 11 & nominator are slashed
            let mut new5 = vec![Event::Slash(11, 100), Event::Slash(101, 50)];
            expected.append(&mut new5);
            assert_eq!(events(), expected);
            assert_eq!(NodleStaking::total(), 3060);

            // Ensure Validator 11 is not deactivated
            // Since offence incident is reported on old slash span.
            assert_eq!(
                NodleStaking::validator_state(11).unwrap().state,
                ValidatorStatus::Active,
            );

            mock::start_active_session(16);

            assert!(!<AwardedPts<Test>>::contains_key(5, 11));
            assert!(!<AwardedPts<Test>>::contains_key(5, 21));
            assert!(!<AwardedPts<Test>>::contains_key(5, 41));
        });
}

#[test]
fn reporters_receive_their_slice() {
    // This test verifies that the reporters of the offence receive their slice from the slashed
    // amount.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            // The reporters' reward is calculated from the total exposure.
            let initial_balance = 1500;

            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert_eq!(
                NodleStaking::at_stake(NodleStaking::active_session(), 11).total,
                initial_balance
            );

            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![1, 2],
                }],
                &[Perbill::from_percent(50)],
            );

            let mut new2 = vec![
                Event::Slash(11, 500),
                Event::Slash(101, 250),
                Event::PayReporterReward(1, 18),
                Event::PayReporterReward(2, 18),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);
            assert_eq!(NodleStaking::total(), 2750);

            // F1 * (reward_proportion * slash - 0)
            // 50% * (10% * initial_balance / 2)
            let reward = (initial_balance / 20) / 2;
            let reward_each = reward / 2; // split into two pieces.
            assert_eq!(Balances::free_balance(1), 10 + reward_each);
            assert_eq!(Balances::free_balance(2), 20 + reward_each);
            assert_eq!(NodleStaking::total(), 2750);
        });
}

#[test]
fn subsequent_reports_in_same_span_pay_out_less() {
    // This test verifies that the reporters of the offence receive their slice from the slashed
    // amount, but less and less if they submit multiple reports in one span.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            let initial_balance = NodleStaking::validator_state(11).unwrap().total;

            assert_eq!(
                NodleStaking::at_stake(NodleStaking::active_session(), 11).total,
                initial_balance
            );
            assert_eq!(NodleStaking::total(), 3500);

            log::trace!(
                "subsequent_reports_in_same_span_pay_out_less:[{:#?}]=> Acc-1[{:#?}]",
                line!(),
                Balances::free_balance(1)
            );

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![1],
                }],
                &[Perbill::from_percent(20)],
            );

            let mut new1 = vec![
                Event::Slash(11, 200),
                Event::Slash(101, 100),
                Event::PayReporterReward(1, 15),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
            assert_eq!(NodleStaking::total(), 3200);

            // F1 * (reward_proportion * slash - 0)
            // 50% * (10% * initial_balance * 20%)
            let prior_slash = initial_balance / 5;
            let reward = prior_slash / 20;
            assert_eq!(Balances::free_balance(1), 10 + reward);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![1],
                }],
                &[Perbill::from_percent(25)],
            );

            let prior_payout = reward;

            // F1 * (reward_proportion * slash - prior_payout)
            // 50% * (10% * (initial_balance / 4) - prior_payout)
            // TODO :: Have to recheck the rounding error.
            let curr_slash = initial_balance / 4;
            let reward = (((curr_slash / 10) - prior_payout) / 2) - 1u128;

            let mut new2 = vec![
                Event::Slash(11, 50),
                Event::Slash(101, 25),
                Event::PayReporterReward(1, 10),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(Balances::free_balance(1), 10 + prior_payout + reward);
            assert_eq!(NodleStaking::total(), 3125);
        });
}

#[test]
fn invulnerables_are_not_slashed() {
    // For invulnerable validators no slashing is performed.
    ExtBuilder::default()
        .num_validators(4)
        .invulnerables(vec![11])
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            Balances::make_free_balance_be(&201, 1000);

            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(201),
                21,
                500,
                false,
            ));

            mock::start_active_session(3);

            let mut new1 = vec![
                Event::Nomination(201, 500, 21, 1500),
                Event::ValidatorChosen(3, 11, 1500),
                Event::ValidatorChosen(3, 21, 1500),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 3, 4000),
                Event::ValidatorChosen(4, 11, 1500),
                Event::ValidatorChosen(4, 21, 1500),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 3, 4000),
            ];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&21), (2000, 1000));
            assert_eq!(mock::balances(&201), (1000, 500));
            assert_eq!(NodleStaking::total(), 4000);

            let valid21_exposure = NodleStaking::at_stake(NodleStaking::active_session(), 21);
            let validator21_initial_bond = valid21_exposure.bond;
            let validator21_nominator_initial_bond: Vec<_> = valid21_exposure
                .nominators
                .iter()
                .map(|nom| nom.amount)
                .collect();

            on_offence_now(
                &[
                    OffenceDetails {
                        offender: (
                            11,
                            NodleStaking::at_stake(NodleStaking::active_session(), 11),
                        ),
                        reporters: vec![],
                    },
                    OffenceDetails {
                        offender: (
                            21,
                            NodleStaking::at_stake(NodleStaking::active_session(), 21),
                        ),
                        reporters: vec![],
                    },
                ],
                &[Perbill::from_percent(50), Perbill::from_percent(20)],
            );

            // Ensure Validator-11 is not slashed
            // Ensure Validator-21 & nominator-201 are slashed
            let mut new2 = vec![Event::Slash(21, 200), Event::Slash(201, 100)];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            // The validator 11 hasn't been slashed, but 21 has been.
            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(NodleStaking::total(), 3700);
            // 2000 - (0.2 * initial_balance)
            assert_eq!(
                mock::balances(&21),
                (
                    1800,
                    validator21_initial_bond - (2 * validator21_initial_bond / 10)
                )
            );

            // ensure that nominators were slashed as well.
            for (initial_bond, nominator) in validator21_nominator_initial_bond
                .into_iter()
                .zip(valid21_exposure.nominators)
            {
                assert_eq!(
                    mock::balances(&nominator.owner),
                    (900, initial_bond - (2 * initial_bond / 10)),
                );
            }
            assert_eq!(NodleStaking::total(), 3700);
        });
}

#[test]
fn dont_slash_if_fraction_is_zero() {
    // Don't slash if the fraction is zero.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(0)],
            );

            // Ensure no slash or new events
            assert_eq!(events(), expected);

            // The validator hasn't been slashed. The new era is not forced.
            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(NodleStaking::total(), 3500);
        });
}

#[test]
fn only_slash_for_max_in_session() {
    // multiple slashes within one session are only applied if it is
    // more than any previous slash in the same session.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&21), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(50)],
            );

            let mut new1 = vec![Event::Slash(11, 500), Event::Slash(101, 250)];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (1500, 500));
            assert_eq!(mock::balances(&101), (1750, 250));
            assert_eq!(NodleStaking::total(), 2750);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(25)],
            );

            // slash fraction 25% is less than last value of 50%
            // Ensure no events are fired
            assert_eq!(events(), expected);

            // The validator has not been slashed additionally.
            assert_eq!(mock::balances(&11), (1500, 500));
            assert_eq!(mock::balances(&101), (1750, 250));

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(60)],
            );

            // slash fraction 60% is more than last slash fraction of 50%
            // Ensure Validator 11 & nominator 101 are slashed with diff.
            let mut new2 = vec![Event::Slash(11, 100), Event::Slash(101, 50)];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            // The validator got slashed 10% more.
            assert_eq!(mock::balances(&11), (1400, 400));
            assert_eq!(mock::balances(&101), (1700, 200));
            assert_eq!(NodleStaking::total(), 2600);
        })
}

#[test]
fn garbage_collection_after_slashing() {
    // ensures that `SlashingSpans` and `SpanSlash` of an account is removed after reaping.
    ExtBuilder::default()
        .num_validators(4)
        .existential_deposit(2)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 384000),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 256000),
                Event::NewSession(5, 2, 3, 641000),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (512000, 256000));
            assert_eq!(mock::balances(&101), (512000, 128000));

            let slash_percent = Perbill::from_percent(10);
            let initial_exposure = NodleStaking::at_stake(NodleStaking::active_session(), 11);

            // 101 is a nominator for 11
            assert_eq!(initial_exposure.nominators.first().unwrap().owner, 101);

            assert_eq!(NodleStaking::total(), 641000);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[slash_percent],
            );

            let mut new1 = vec![Event::Slash(11, 25600), Event::Slash(101, 12800)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(balances(&11), (512000 - 25600, 256000 - (256000 / 10)));
            assert_eq!(balances(&101), (512000 - 12800, 128000 - (128000 / 10)));

            assert_eq!(NodleStaking::total(), 602600);

            assert!(<SlashingSpans<Test>>::get(&11).is_some());

            assert_eq!(
                <SpanSlash<Test>>::get(&(11, 0)).amount_slashed(),
                &(256000 / 10)
            );
            assert_eq!(
                <SpanSlash<Test>>::get(&(101, 0)).amount_slashed(),
                &(128000 / 10)
            );

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(100)],
            );

            let mut new2 = vec![
                Event::Slash(11, 230398),
                Event::Slash(101, 115198),
                Event::NominationBelowThreashold(101, 11, 2, 2, 0),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (256002, 2));
            assert_eq!(mock::balances(&101), (384002, 2));
            assert_eq!(NodleStaking::total(), 257004);

            let slashing_spans = <SlashingSpans<Test>>::get(&11).unwrap();
            assert_eq!(slashing_spans.iter().count(), 2);

            // TODO :: Validation of DB instance Clean-off pending
            // // reap_stash respects num_slashing_spans so that weight is accurate
            // assert_noop!(Staking::reap_stash(Origin::none(), 11, 0),
            // Error::<Test>::IncorrectSlashingSpans); assert_ok!(Staking::reap_stash(Origin::none(),
            // 11, 2));

            // assert!(<Staking as crate::Store>::SlashingSpans::get(&11).is_none());
            // assert_eq!(<Staking as crate::Store>::SpanSlash::get(&(11, 0)).amount_slashed(), &0);
        })
}

#[test]
fn garbage_collection_after_slashing_ed_1() {
    // ensures that `SlashingSpans` and `SpanSlash` of an account is removed after reaping.
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            let mut new1 = vec![Event::Slash(11, 100), Event::Slash(101, 50)];

            expected.append(&mut new1);
            assert_eq!(events(), expected);

            assert_eq!(balances(&11), (1900, 900));
            assert_eq!(balances(&101), (1950, 450));

            assert_eq!(NodleStaking::total(), 3350);

            assert!(<SlashingSpans<Test>>::get(&11).is_some());

            assert_eq!(<SpanSlash<Test>>::get(&(11, 0)).amount_slashed(), &(100));
            assert_eq!(<SpanSlash<Test>>::get(&(101, 0)).amount_slashed(), &(50));

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(100)],
            );

            let mut new2 = vec![
                Event::Slash(11, 899),
                Event::Slash(101, 449),
                Event::NominationBelowThreashold(101, 11, 1, 1, 0),
            ];

            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(mock::balances(&11), (1001, 1));
            assert_eq!(mock::balances(&101), (1501, 1));
            assert_eq!(NodleStaking::total(), 2002);

            let slashing_spans = <SlashingSpans<Test>>::get(&11).unwrap();
            assert_eq!(slashing_spans.iter().count(), 2);

            // TODO :: Validation of DB instance Clean-off pending
            // // reap_stash respects num_slashing_spans so that weight is accurate
            // assert_noop!(Staking::reap_stash(Origin::none(), 11, 0),
            // Error::<Test>::IncorrectSlashingSpans); assert_ok!(Staking::reap_stash(Origin::none(),
            // 11, 2));

            // assert!(<Staking as crate::Store>::SlashingSpans::get(&11).is_none());
            // assert_eq!(<Staking as crate::Store>::SpanSlash::get(&(11, 0)).amount_slashed(), &0);
        })
}

#[test]
fn slash_kicks_validators_not_nominators_and_activate_validator_to_rejoin_pool() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(events(), expected);

            // pre-slash balance
            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            assert_eq!(Session::validators(), vec![11, 21, 41]);

            let exposure_11 = NodleStaking::at_stake(NodleStaking::active_session(), &11);
            let exposure_21 = NodleStaking::at_stake(NodleStaking::active_session(), &21);

            assert_eq!(exposure_11.total, 1500);
            assert_eq!(exposure_21.total, 1000);

            on_offence_now(
                &[OffenceDetails {
                    offender: (11, exposure_11.clone()),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            let mut new1 = vec![Event::Slash(11, 100), Event::Slash(101, 50)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            // post-slash balance
            assert_eq!(mock::balances(&11), (1900, 900));
            assert_eq!(mock::balances(&101), (1950, 450));
            assert_eq!(NodleStaking::total(), 3350);

            // Validator-11 is deactivated
            assert_eq!(
                NodleStaking::validator_state(&11).unwrap().state,
                ValidatorStatus::Idle
            );

            mock::start_active_session(3);

            // ensure validator 11 is not selected for the session 3 & 4

            let mut new1 = vec![
                Event::ValidatorChosen(3, 21, 1000),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 2, 2000),
                Event::ValidatorChosen(4, 21, 1000),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 2, 2000),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);

            // activate validator 11 in session 3
            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(11), 10));

            mock::start_active_session(4);

            // ensure validator 11 is part of session 5

            // log::trace!("[{:#?}]=> - {:#?}", line!(), mock::events());
            // panic!("Stop-1");

            // let mut new2 = vec![
            //     Event::ValidatorBondedMore(11, 900, 910),
            //     Event::ValidatorChosen(5, 11, 1410),
            //     Event::ValidatorChosen(5, 21, 1000),
            //     Event::ValidatorChosen(5, 41, 1000),
            //     Event::NewSession(20, 5, 3, 3410),
            // ];
            // expected.append(&mut new2);
            // assert_eq!(events(), expected);

            let mut new2 = vec![
                Event::ValidatorBondedMore(11, 900, 910),
                Event::ValidatorChosen(5, 11, 1360),
                Event::ValidatorChosen(5, 21, 1000),
                Event::ValidatorChosen(5, 41, 1000),
                Event::NewSession(20, 5, 3, 3360),
            ];
            expected.append(&mut new2);
            assert_eq!(events(), expected);

            assert_eq!(NodleStaking::total(), 3360);
        });
}

#[test]
fn slashing_nominators_by_span_max() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(101),
                21,
                500,
                false,
            ));

            mock::start_active_session(1);
            mock::start_active_session(2);
            mock::start_active_session(3);

            let mut expected = vec![
                Event::Nomination(101, 500, 21, 1500),
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1500),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 4000),
                Event::ValidatorChosen(3, 11, 1500),
                Event::ValidatorChosen(3, 21, 1500),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 3, 4000),
                Event::ValidatorChosen(4, 11, 1500),
                Event::ValidatorChosen(4, 21, 1500),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 3, 4000),
            ];

            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&21), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 1000));
            assert_eq!(NodleStaking::total(), 4000);

            let get_span = |account| <SlashingSpans<Test>>::get(&account).unwrap();

            let exposure_11 = NodleStaking::at_stake(NodleStaking::active_session(), 11);
            let exposure_21 = NodleStaking::at_stake(NodleStaking::active_session(), 21);
            let nominated_value_11 = exposure_11
                .nominators
                .iter()
                .find(|o| o.owner == 101)
                .unwrap()
                .amount;
            let nominated_value_21 = exposure_21
                .nominators
                .iter()
                .find(|o| o.owner == 101)
                .unwrap()
                .amount;

            // Check slashing
            on_offence_in_session(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
                2,
            );

            let mut new1 = vec![Event::Slash(11, 100), Event::Slash(101, 50)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            let nominator101_prev_slashed_val = Perbill::from_percent(10) * nominated_value_11;

            assert_eq!(mock::balances(&11), (1900, 900));

            assert_eq!(
                mock::balances(&101),
                (1950, 1000 - nominator101_prev_slashed_val,)
            );
            assert_eq!(NodleStaking::total(), 3850);

            let expected_spans = vec![
                slashing::SlashingSpan {
                    index: 1,
                    start: 4,
                    length: None,
                },
                slashing::SlashingSpan {
                    index: 0,
                    start: 1,
                    length: Some(3),
                },
            ];

            assert_eq!(get_span(11).iter().collect::<Vec<_>>(), expected_spans);
            assert_eq!(get_span(101).iter().collect::<Vec<_>>(), expected_spans);

            // second slash: higher era, higher value, same span.
            on_offence_in_session(
                &[OffenceDetails {
                    offender: (
                        21,
                        NodleStaking::at_stake(NodleStaking::active_session(), 21),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(30)],
                3,
            );

            // Since on same span for 101, slash_value = 150 - 50 = 100
            let mut new2 = vec![Event::Slash(21, 300), Event::Slash(101, 100)];

            expected.append(&mut new2);
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&21), (1700, 700));
            assert_eq!(mock::balances(&101), (1850, 850));
            assert_eq!(NodleStaking::total(), 3450);

            let nominator101_prev_slashed_val =
                (Perbill::from_percent(30) * nominated_value_21) - nominator101_prev_slashed_val;

            assert_eq!(
                mock::balances(&101),
                (1850, 950 - nominator101_prev_slashed_val)
            );

            let expected_spans = vec![
                slashing::SlashingSpan {
                    index: 1,
                    start: 4,
                    length: None,
                },
                slashing::SlashingSpan {
                    index: 0,
                    start: 1,
                    length: Some(3),
                },
            ];

            assert_eq!(get_span(21).iter().collect::<Vec<_>>(), expected_spans);
            assert_eq!(get_span(101).iter().collect::<Vec<_>>(), expected_spans);

            on_offence_in_session(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(20)],
                2,
            );

            // Only Validator-11 is slashed, and Nominator-101 is not slashed since
            // Here slash value is less than the Span Max.
            let mut new3 = vec![Event::Slash(11, 100)];

            expected.append(&mut new3);
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (1800, 800));
            assert_eq!(NodleStaking::total(), 3350);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        });
}

#[test]
fn slashes_are_summed_across_spans() {
    ExtBuilder::default()
        .num_validators(4)
        .build_and_execute(|| {
            assert_ok!(NodleStaking::nominator_nominate(
                Origin::signed(101),
                21,
                500,
                false,
            ));
            mock::start_active_session(1);
            mock::start_active_session(2);
            mock::start_active_session(3);

            let mut expected = vec![
                Event::Nomination(101, 500, 21, 1500),
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1500),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 4000),
                Event::ValidatorChosen(3, 11, 1500),
                Event::ValidatorChosen(3, 21, 1500),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 3, 4000),
                Event::ValidatorChosen(4, 11, 1500),
                Event::ValidatorChosen(4, 21, 1500),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 3, 4000),
            ];

            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&21), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 1000));
            assert_eq!(NodleStaking::total(), 4000);

            let get_span = |account| <SlashingSpans<Test>>::get(&account).unwrap();

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        21,
                        NodleStaking::at_stake(NodleStaking::active_session(), 21),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            let mut new1 = vec![Event::Slash(21, 100), Event::Slash(101, 50)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&21), (1900, 900));
            assert_eq!(mock::balances(&101), (1950, 950));
            assert_eq!(NodleStaking::total(), 3850);

            let expected_spans = vec![
                slashing::SlashingSpan {
                    index: 1,
                    start: 4,
                    length: None,
                },
                slashing::SlashingSpan {
                    index: 0,
                    start: 1,
                    length: Some(3),
                },
            ];

            assert_eq!(get_span(21).iter().collect::<Vec<_>>(), expected_spans,);

            assert_ok!(NodleStaking::validator_bond_more(Origin::signed(21), 10));

            mock::start_active_session(5);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        21,
                        NodleStaking::at_stake(NodleStaking::active_session(), 21),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            let mut new2 = vec![
                Event::ValidatorBondedMore(21, 900, 910),
                Event::ValidatorChosen(5, 11, 1500),
                Event::ValidatorChosen(5, 21, 1360),
                Event::ValidatorChosen(5, 41, 1000),
                Event::NewSession(20, 5, 3, 3860),
                Event::ValidatorChosen(6, 11, 1500),
                Event::ValidatorChosen(6, 21, 1360),
                Event::ValidatorChosen(6, 41, 1000),
                Event::NewSession(25, 6, 3, 3860),
                Event::Slash(21, 91),
                Event::Slash(101, 45),
            ];
            expected.append(&mut new2);
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&21), (1809, 819));
            assert_eq!(mock::balances(&101), (1905, 905));
            assert_eq!(NodleStaking::total(), 3724);

            let expected_spans = vec![
                slashing::SlashingSpan {
                    index: 2,
                    start: 6,
                    length: None,
                },
                slashing::SlashingSpan {
                    index: 1,
                    start: 4,
                    length: Some(2),
                },
                slashing::SlashingSpan {
                    index: 0,
                    start: 1,
                    length: Some(3),
                },
            ];
            assert_eq!(get_span(21).iter().collect::<Vec<_>>(), expected_spans);

            // tst_log!(
            //     debug,
            //     "[{:#?}]=> - {:#?}",
            //     line!(),
            //     get_span(21).iter().collect::<Vec<_>>()
            // );

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        });
}

#[test]
fn deferred_slashes_are_deferred() {
    ExtBuilder::default()
        .num_validators(4)
        .slash_defer_duration(2)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new1 = vec![Event::DeferredUnappliedSlash(1, 11)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            mock::start_active_session(2);

            let mut new2 = vec![
                Event::ValidatorChosen(3, 21, 1000),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 2, 2000),
            ];

            expected.append(&mut new2);
            assert_eq!(mock::events(), expected);
            assert_eq!(NodleStaking::total(), 3500);

            // Ensure slash occur at start of 3 session ( 1 + 2 [deferred duration] )
            mock::start_active_session(3);

            let mut new3 = vec![
                Event::Slash(11, 100),
                Event::Slash(101, 50),
                Event::ValidatorChosen(4, 21, 1000),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 2, 2000),
            ];
            expected.append(&mut new3);
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (1900, 900));
            assert_eq!(mock::balances(&101), (1950, 450));
            assert_eq!(NodleStaking::total(), 3350);
        })
}

#[test]
fn remove_deferred() {
    ExtBuilder::default()
        .slash_defer_duration(2)
        .num_validators(4)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new1 = vec![Event::DeferredUnappliedSlash(1, 11)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            mock::start_active_session(2);

            let mut new2 = vec![
                Event::ValidatorChosen(3, 21, 1000),
                Event::ValidatorChosen(3, 41, 1000),
                Event::NewSession(10, 3, 2, 2000),
            ];

            expected.append(&mut new2);
            assert_eq!(mock::events(), expected);
            assert_eq!(NodleStaking::total(), 3500);

            on_offence_in_session(
                &[OffenceDetails {
                    offender: (11, NodleStaking::at_stake(1, 11)),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(15)],
                1,
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new3 = vec![Event::DeferredUnappliedSlash(2, 11)];

            expected.append(&mut new3);
            assert_eq!(mock::events(), expected);

            // fails if empty
            assert_noop!(
                NodleStaking::slash_cancel_deferred(Origin::root(), 1, vec![]),
                Error::<Test>::EmptyTargets
            );

            assert_ok!(NodleStaking::slash_cancel_deferred(
                Origin::root(),
                1,
                vec![11],
            ));

            mock::start_active_session(3);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            let mut new4 = vec![
                Event::ValidatorChosen(4, 21, 1000),
                Event::ValidatorChosen(4, 41, 1000),
                Event::NewSession(15, 4, 2, 2000),
            ];

            expected.append(&mut new4);
            assert_eq!(mock::events(), expected);

            mock::start_active_session(4);

            // Ensure deffered slash event have fired.
            assert_eq!(mock::balances(&11), (1950, 950));
            assert_eq!(mock::balances(&101), (1975, 475));

            let mut new5 = vec![
                Event::Slash(11, 50),
                Event::Slash(101, 25),
                Event::ValidatorChosen(5, 21, 1000),
                Event::ValidatorChosen(5, 41, 1000),
                Event::NewSession(20, 5, 2, 2000),
            ];

            expected.append(&mut new5);
            assert_eq!(mock::events(), expected);
            assert_eq!(NodleStaking::total(), 3425);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());
        })
}

#[test]
fn remove_multi_deferred() {
    ExtBuilder::default()
        .num_validators(4)
        .slash_defer_duration(2)
        .build_and_execute(|| {
            mock::start_active_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 11, 1500),
                Event::ValidatorChosen(2, 21, 1000),
                Event::ValidatorChosen(2, 41, 1000),
                Event::NewSession(5, 2, 3, 3500),
            ];
            assert_eq!(mock::events(), expected);

            assert_eq!(mock::balances(&11), (2000, 1000));
            assert_eq!(mock::balances(&101), (2000, 500));
            assert_eq!(NodleStaking::total(), 3500);

            // Add 11 to Unapplied Slash Q
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new1 = vec![Event::DeferredUnappliedSlash(1, 11)];

            expected.append(&mut new1);
            assert_eq!(mock::events(), expected);

            // Add 21 to Unapplied Slash Q
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        21,
                        NodleStaking::at_stake(NodleStaking::active_session(), 21),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(10)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new2 = vec![Event::DeferredUnappliedSlash(1, 21)];

            expected.append(&mut new2);
            assert_eq!(mock::events(), expected);

            // Add 11 to Unapplied Slash Q [25%]
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        11,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(25)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new3 = vec![Event::DeferredUnappliedSlash(1, 11)];

            expected.append(&mut new3);
            assert_eq!(mock::events(), expected);

            // Add 42 with exposure of 11 to Unapplied Slash Q [25%]
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        41,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(25)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new4 = vec![Event::DeferredUnappliedSlash(1, 41)];

            expected.append(&mut new4);
            assert_eq!(mock::events(), expected);

            // Add 69 with exposure of 11 to Unapplied Slash Q [25%]
            on_offence_now(
                &[OffenceDetails {
                    offender: (
                        69,
                        NodleStaking::at_stake(NodleStaking::active_session(), 11),
                    ),
                    reporters: vec![],
                }],
                &[Perbill::from_percent(25)],
            );

            // Since slash is deffered,
            // Ensure deffered unapplied slashing events
            let mut new5 = vec![Event::DeferredUnappliedSlash(1, 69)];

            expected.append(&mut new5);
            assert_eq!(mock::events(), expected);

            // mock::SLASH_DEFER_DURATION.saturating_add(NodleStaking::active_session());
            // let apply_at = mock::SLASH_DEFER_DURATION
            //     .with(|v| *v.get() + NodleStaking::active_session());

            let apply_at =
                NodleStaking::active_session() + mock::SLASH_DEFER_DURATION.with(|l| *l.borrow());

            assert_eq!(<UnappliedSlashes<Test>>::get(&apply_at).len(), 5);

            assert_noop!(
                NodleStaking::slash_cancel_deferred(Origin::root(), 1, vec![]),
                Error::<Test>::EmptyTargets
            );

            assert_noop!(
                NodleStaking::slash_cancel_deferred(Origin::root(), apply_at, vec![11]),
                Error::<Test>::InvalidSessionIndex
            );

            assert_ok!(NodleStaking::slash_cancel_deferred(
                Origin::root(),
                1,
                vec![11]
            ),);

            assert_eq!(<UnappliedSlashes<Test>>::get(&apply_at).len(), 3);

            assert_ok!(NodleStaking::slash_cancel_deferred(
                Origin::root(),
                1,
                vec![69]
            ),);

            assert_eq!(<UnappliedSlashes<Test>>::get(&apply_at).len(), 2);

            assert_eq!(<UnappliedSlashes<Test>>::get(&apply_at)[0].validator, 21);
            assert_eq!(<UnappliedSlashes<Test>>::get(&apply_at)[1].validator, 41);

            mock::start_active_session(4);

            let mut new6 = vec![
                Event::NewSession(10, 3, 0, 0),
                Event::Slash(21, 100),
                Event::Slash(41, 250),
                Event::NewSession(15, 4, 0, 0),
                Event::NewSession(20, 5, 0, 0),
            ];

            expected.append(&mut new6);
            assert_eq!(mock::events(), expected);

            assert_eq!(NodleStaking::total(), 3150);

            // tst_log!(debug, "[{:#?}]=> - {:#?}", line!(), mock::events());

            // tst_log!(
            //     debug,
            //     "[{:#?}]=> - [{:#?}] | [{:#?}]",
            //     line!(),
            //     apply_at,
            //     <UnappliedSlashes<Test>>::get(&apply_at)
            // );
        })
}
