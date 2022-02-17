/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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
use crate::mock_prod_config::{
    constants, events, last_event, mint_rewards, set_author, start_session, Balance,
    Event as MetaEvent, ExtBuilder, NodleStaking, Origin, System, Test,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn join_validator_pool_invulnerable_with_low_bond() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2])
        .with_balances(vec![
            (1, 100 * constants::NODL),
            (2, 50 * constants::NODL),
            (3, 20 * constants::NODL),
            (4, 20 * constants::NODL),
            (5, 20 * constants::NODL),
            (6, 20 * constants::NODL),
            (7, 300 * constants::KILO * constants::NODL),
            (8, 50 * constants::KILO * constants::NODL),
            (9, 300 * constants::KILO * constants::NODL),
        ])
        .with_validators(vec![(1, 10 * constants::NODL), (2, 20 * constants::NODL)])
        .with_nominators(vec![
            (3, 1, 5 * constants::NODL),
            (4, 1, 5 * constants::NODL),
            (5, 2, 5 * constants::NODL),
            (6, 2, 5 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(1), 1 * constants::NODL),
                Error::<Test>::ValidatorExists,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(7),
                    99 * constants::KILO * constants::NODL
                ),
                Error::<Test>::ValidatorBondBelowMin,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(8),
                    150 * constants::KILO * constants::NODL
                ),
                Error::<Test>::InsufficientBalance,
            );
            assert!(System::events().is_empty());
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(9),
                100 * constants::KILO * constants::NODL,
            ),);
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(7),
                150 * constants::KILO * constants::NODL,
            ));
            assert_eq!(
                last_event(),
                MetaEvent::NodleStaking(Event::JoinedValidatorPool(
                    7,
                    150 * constants::KILO * constants::NODL,
                    250050000000000000u128
                ))
            );
        });
}

#[test]
fn validator_bond_more_invulnerable_with_low_bond() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2])
        .with_balances(vec![
            (1, 100 * constants::NODL),
            (2, 50 * constants::NODL),
            (3, 20 * constants::NODL),
            (4, 20 * constants::NODL),
            (5, 20 * constants::NODL),
            (6, 20 * constants::NODL),
            (7, 300 * constants::KILO * constants::NODL),
            (8, 50 * constants::KILO * constants::NODL),
            (9, 300 * constants::KILO * constants::NODL),
        ])
        .with_validators(vec![(1, 10 * constants::NODL), (2, 20 * constants::NODL)])
        .with_nominators(vec![
            (3, 1, 5 * constants::NODL),
            (4, 1, 5 * constants::NODL),
            (5, 2, 5 * constants::NODL),
            (6, 2, 5 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(1), 1 * constants::NODL),
                Error::<Test>::ValidatorExists,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(7),
                    99 * constants::KILO * constants::NODL
                ),
                Error::<Test>::ValidatorBondBelowMin,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(8),
                    150 * constants::KILO * constants::NODL
                ),
                Error::<Test>::InsufficientBalance,
            );
            assert!(System::events().is_empty());
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(9),
                100 * constants::KILO * constants::NODL,
            ),);
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(7),
                150 * constants::KILO * constants::NODL,
            ));
            assert_eq!(
                last_event(),
                MetaEvent::NodleStaking(Event::JoinedValidatorPool(
                    7,
                    150 * constants::KILO * constants::NODL,
                    250050000000000000u128
                ))
            );

            assert_noop!(
                NodleStaking::validator_bond_more(Origin::signed(1), 200 * constants::NODL),
                Error::<Test>::InsufficientBalance,
            );

            assert_ok!(NodleStaking::validator_bond_more(
                Origin::signed(9),
                10 * constants::NODL
            ));

            assert_ok!(NodleStaking::validator_bond_more(
                Origin::signed(9),
                10 * constants::KILO * constants::NODL
            ));

            assert_eq!(
                last_event(),
                MetaEvent::NodleStaking(Event::ValidatorBondedMore(
                    9,
                    100010000000000000u128,
                    110010000000000000u128
                ))
            );
        });
}

#[test]
fn validator_bond_less_invulnerable_with_low_bond() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2])
        .with_balances(vec![
            (1, 100 * constants::NODL),
            (2, 50 * constants::NODL),
            (3, 20 * constants::NODL),
            (4, 20 * constants::NODL),
            (5, 20 * constants::NODL),
            (6, 20 * constants::NODL),
            (7, 300 * constants::KILO * constants::NODL),
            (8, 50 * constants::KILO * constants::NODL),
            (9, 300 * constants::KILO * constants::NODL),
        ])
        .with_validators(vec![(1, 10 * constants::NODL), (2, 20 * constants::NODL)])
        .with_nominators(vec![
            (3, 1, 5 * constants::NODL),
            (4, 1, 5 * constants::NODL),
            (5, 2, 5 * constants::NODL),
            (6, 2, 5 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            assert_noop!(
                NodleStaking::validator_join_pool(Origin::signed(1), 1 * constants::NODL),
                Error::<Test>::ValidatorExists,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(7),
                    99 * constants::KILO * constants::NODL
                ),
                Error::<Test>::ValidatorBondBelowMin,
            );
            assert_noop!(
                NodleStaking::validator_join_pool(
                    Origin::signed(8),
                    150 * constants::KILO * constants::NODL
                ),
                Error::<Test>::InsufficientBalance,
            );
            assert!(System::events().is_empty());
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(9),
                100 * constants::KILO * constants::NODL,
            ),);
            assert_ok!(NodleStaking::validator_join_pool(
                Origin::signed(7),
                150 * constants::KILO * constants::NODL,
            ));
            assert_eq!(
                last_event(),
                MetaEvent::NodleStaking(Event::JoinedValidatorPool(
                    7,
                    150 * constants::KILO * constants::NODL,
                    250050000000000000u128
                ))
            );

            assert_noop!(
                NodleStaking::validator_bond_less(
                    Origin::signed(9),
                    50 * constants::KILO * constants::NODL,
                ),
                Error::<Test>::ValidatorBondBelowMin,
            );

            assert_ok!(NodleStaking::validator_bond_less(
                Origin::signed(1),
                5 * constants::NODL
            ));

            assert_ok!(NodleStaking::validator_bond_more(
                Origin::signed(9),
                100 * constants::KILO * constants::NODL
            ));

            assert_ok!(NodleStaking::validator_bond_less(
                Origin::signed(9),
                50 * constants::NODL
            ));

            assert_eq!(
                last_event(),
                MetaEvent::NodleStaking(Event::ValidatorBondedLess(
                    9,
                    200000000000000000u128,
                    199950000000000000u128
                ))
            );
        });
}

#[test]
fn profile_rewards_check_config1() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2, 3])
        .with_balances(vec![
            (1, 1100 * constants::NODL),
            (2, 1100 * constants::NODL),
            (3, 1100 * constants::NODL),
        ])
        .with_validators(vec![
            (1, 1000 * constants::NODL),
            (2, 1000 * constants::NODL),
            (3, 1000 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            start_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 1000 * constants::NODL),
                Event::ValidatorChosen(2, 2, 1000 * constants::NODL),
                Event::ValidatorChosen(2, 3, 1000 * constants::NODL),
                Event::NewSession(5, 2, 3, 3000 * constants::NODL),
            ];

            assert_eq!(events(), expected);

            let valid_01_reward_points = 6126;
            let valid_02_reward_points = 6132;
            let valid_03_reward_points = 5656;
            let total_minted_rewards: Balance = (0.0530_f64 * constants::NODL as f64) as Balance;

            set_author(1, 1, valid_01_reward_points);
            set_author(1, 2, valid_02_reward_points);
            set_author(1, 3, valid_03_reward_points);
            mint_rewards(total_minted_rewards);
            assert_eq!(NodleStaking::total(), 3000 * constants::NODL);
            assert_eq!(NodleStaking::points(1), 17914);

            start_session(2);

            let valid_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(1)[NodleStaking::stake_rewards(1).len() - 1];

            let valid_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(2)[NodleStaking::stake_rewards(2).len() - 1];

            let valid_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(3)[NodleStaking::stake_rewards(3).len() - 1];

            let total_minted_rewards_sharing = valid_01_alloc_rewards_actual
                .value
                .saturating_add(valid_02_alloc_rewards_actual.value)
                .saturating_add(valid_03_alloc_rewards_actual.value);

            log::trace!(
                "profile_rewards_check_config1:[{:#?}] - Act[{:#?}], Devi[{:#?}], Diff[{:#?}]",
                line!(),
                total_minted_rewards as f64 / constants::NODL as f64,
                total_minted_rewards_sharing as f64 / constants::NODL as f64,
                total_minted_rewards.saturating_sub(total_minted_rewards_sharing) as f64
                    / constants::NODL as f64,
            );

            let mut new1 = vec![
                Event::StakeReward(3, 16733727789),
                Event::StakeReward(1, 18124260328),
                Event::StakeReward(2, 18142011830),
                Event::ValidatorChosen(3, 1, 1000 * constants::NODL),
                Event::ValidatorChosen(3, 2, 1000 * constants::NODL),
                Event::ValidatorChosen(3, 3, 1000 * constants::NODL),
                Event::NewSession(10, 3, 3, 3000 * constants::NODL),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
        });
}

#[test]
fn profile_rewards_check_config1_1() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2, 3])
        .with_balances(vec![
            (1, 100 * constants::NODL),
            (2, 100 * constants::NODL),
            (3, 100 * constants::NODL),
            (6, 1100 * constants::NODL),
            (7, 1100 * constants::NODL),
            (8, 1100 * constants::NODL),
        ])
        .with_validators(vec![
            (1, 11 * constants::NODL),
            (2, 11 * constants::NODL),
            (3, 11 * constants::NODL),
        ])
        .with_nominators(vec![
            (6, 1, 1000 * constants::NODL),
            (7, 2, 1000 * constants::NODL),
            (8, 3, 1000 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            start_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 1011 * constants::NODL),
                Event::ValidatorChosen(2, 2, 1011 * constants::NODL),
                Event::ValidatorChosen(2, 3, 1011 * constants::NODL),
                Event::NewSession(5, 2, 3, 3033 * constants::NODL),
            ];

            assert_eq!(events(), expected);

            let valid_01_reward_points = 4624;
            let valid_02_reward_points = 4506;
            let valid_03_reward_points = 4718;
            let total_minted_rewards: Balance = (0.0383f64 * constants::NODL as f64) as Balance;

            set_author(1, 1, valid_01_reward_points);
            set_author(1, 2, valid_02_reward_points);
            set_author(1, 3, valid_03_reward_points);
            mint_rewards(total_minted_rewards);
            assert_eq!(NodleStaking::total(), 3033 * constants::NODL);
            assert_eq!(NodleStaking::points(1), 13848);

            start_session(2);

            let valid_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(1)[NodleStaking::stake_rewards(1).len() - 1];

            let valid_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(2)[NodleStaking::stake_rewards(2).len() - 1];

            let valid_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(3)[NodleStaking::stake_rewards(3).len() - 1];

            let nomi_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(6)[NodleStaking::stake_rewards(6).len() - 1];

            let nomi_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(7)[NodleStaking::stake_rewards(7).len() - 1];

            let nomi_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(8)[NodleStaking::stake_rewards(8).len() - 1];

            let total_minted_rewards_sharing = valid_01_alloc_rewards_actual
                .value
                .saturating_add(valid_02_alloc_rewards_actual.value)
                .saturating_add(valid_03_alloc_rewards_actual.value)
                .saturating_add(nomi_01_alloc_rewards_actual.value)
                .saturating_add(nomi_02_alloc_rewards_actual.value)
                .saturating_add(nomi_03_alloc_rewards_actual.value);

            log::trace!(
                "profile_rewards_check_config1:[{:#?}] - Act[{:#?}], Devi[{:#?}], Diff[{:#?}]",
                line!(),
                total_minted_rewards as f64 / constants::NODL as f64,
                total_minted_rewards_sharing as f64 / constants::NODL as f64,
                total_minted_rewards.saturating_sub(total_minted_rewards_sharing) as f64
                    / constants::NODL as f64,
            );

            let mut new1 = vec![
                Event::StakeReward(3, 787314646),
                Event::StakeReward(8, 12261457692),
                Event::StakeReward(1, 771628430),
                Event::StakeReward(6, 12017164160),
                Event::StakeReward(2, 751937219),
                Event::StakeReward(7, 11710497777),
                Event::ValidatorChosen(3, 1, 1011 * constants::NODL),
                Event::ValidatorChosen(3, 2, 1011 * constants::NODL),
                Event::ValidatorChosen(3, 3, 1011 * constants::NODL),
                Event::NewSession(10, 3, 3, 3033 * constants::NODL),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
        });
}

#[test]
fn profile_rewards_check_config2() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2, 3])
        .with_balances(vec![
            (1, 1100 * constants::NODL),
            (2, 1100 * constants::NODL),
            (3, 1100 * constants::NODL),
            (6, 1100 * constants::NODL),
            (7, 1100 * constants::NODL),
            (8, 1100 * constants::NODL),
        ])
        .with_validators(vec![
            (1, 1000 * constants::NODL),
            (2, 1000 * constants::NODL),
            (3, 1000 * constants::NODL),
        ])
        .with_nominators(vec![
            (6, 1, 1000 * constants::NODL),
            (7, 2, 1000 * constants::NODL),
            (8, 3, 1000 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            start_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 2000 * constants::NODL),
                Event::ValidatorChosen(2, 2, 2000 * constants::NODL),
                Event::ValidatorChosen(2, 3, 2000 * constants::NODL),
                Event::NewSession(5, 2, 3, 6000 * constants::NODL),
            ];

            assert_eq!(events(), expected);

            let valid_01_reward_points = 4832;
            let valid_02_reward_points = 4442;
            let valid_03_reward_points = 4586;
            let total_minted_rewards: Balance = (0.0383f64 * constants::NODL as f64) as Balance;

            set_author(1, 1, valid_01_reward_points);
            set_author(1, 2, valid_02_reward_points);
            set_author(1, 3, valid_03_reward_points);
            mint_rewards(total_minted_rewards);
            assert_eq!(NodleStaking::total(), 6000 * constants::NODL);
            assert_eq!(NodleStaking::points(1), 13860);

            start_session(2);

            let valid_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(1)[NodleStaking::stake_rewards(1).len() - 1];

            let valid_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(2)[NodleStaking::stake_rewards(2).len() - 1];

            let valid_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(3)[NodleStaking::stake_rewards(3).len() - 1];

            let nomi_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(6)[NodleStaking::stake_rewards(6).len() - 1];

            let nomi_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(7)[NodleStaking::stake_rewards(7).len() - 1];

            let nomi_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(8)[NodleStaking::stake_rewards(8).len() - 1];

            let total_minted_rewards_sharing = valid_01_alloc_rewards_actual
                .value
                .saturating_add(valid_02_alloc_rewards_actual.value)
                .saturating_add(valid_03_alloc_rewards_actual.value)
                .saturating_add(nomi_01_alloc_rewards_actual.value)
                .saturating_add(nomi_02_alloc_rewards_actual.value)
                .saturating_add(nomi_03_alloc_rewards_actual.value);

            log::trace!(
                "profile_rewards_check_config1:[{:#?}] - Act[{:#?}], Devi[{:#?}], Diff[{:#?}]",
                line!(),
                total_minted_rewards as f64 / constants::NODL as f64,
                total_minted_rewards_sharing as f64 / constants::NODL as f64,
                total_minted_rewards.saturating_sub(total_minted_rewards_sharing) as f64
                    / constants::NODL as f64,
            );

            let mut new1 = vec![
                Event::StakeReward(3, 6653174224),
                Event::StakeReward(8, 6019538584),
                Event::StakeReward(1, 7010060593),
                Event::StakeReward(6, 6342435775),
                Event::StakeReward(2, 6444265141),
                Event::StakeReward(7, 5830525604),
                Event::ValidatorChosen(3, 1, 2000 * constants::NODL),
                Event::ValidatorChosen(3, 2, 2000 * constants::NODL),
                Event::ValidatorChosen(3, 3, 2000 * constants::NODL),
                Event::NewSession(10, 3, 3, 6000 * constants::NODL),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
        });
}

#[test]
fn profile_rewards_check_config3() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2, 3])
        .with_balances(vec![
            (1, 1100 * constants::NODL),
            (2, 1100 * constants::NODL),
            (3, 1100 * constants::NODL),
            (6, 1100 * constants::NODL),
            (7, 1100 * constants::NODL),
            (8, 1100 * constants::NODL),
        ])
        .with_validators(vec![
            (1, 500 * constants::NODL),
            (2, 500 * constants::NODL),
            (3, 500 * constants::NODL),
        ])
        .with_nominators(vec![
            (6, 1, 1000 * constants::NODL),
            (7, 2, 1000 * constants::NODL),
            (8, 3, 1000 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            start_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 1500 * constants::NODL),
                Event::ValidatorChosen(2, 2, 1500 * constants::NODL),
                Event::ValidatorChosen(2, 3, 1500 * constants::NODL),
                Event::NewSession(5, 2, 3, 4500 * constants::NODL),
            ];

            assert_eq!(events(), expected);

            let valid_01_reward_points = 6620;
            let valid_02_reward_points = 6500;
            let valid_03_reward_points = 6420;
            let total_minted_rewards: Balance = (0.0537f64 * constants::NODL as f64) as Balance;

            set_author(1, 1, valid_01_reward_points);
            set_author(1, 2, valid_02_reward_points);
            set_author(1, 3, valid_03_reward_points);
            mint_rewards(total_minted_rewards);
            assert_eq!(NodleStaking::total(), 4500 * constants::NODL);
            assert_eq!(NodleStaking::points(1), 19540);

            start_session(2);

            let valid_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(1)[NodleStaking::stake_rewards(1).len() - 1];

            let valid_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(2)[NodleStaking::stake_rewards(2).len() - 1];

            let valid_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(3)[NodleStaking::stake_rewards(3).len() - 1];

            let nomi_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(6)[NodleStaking::stake_rewards(6).len() - 1];

            let nomi_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(7)[NodleStaking::stake_rewards(7).len() - 1];

            let nomi_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(8)[NodleStaking::stake_rewards(8).len() - 1];

            let total_minted_rewards_sharing = valid_01_alloc_rewards_actual
                .value
                .saturating_add(valid_02_alloc_rewards_actual.value)
                .saturating_add(valid_03_alloc_rewards_actual.value)
                .saturating_add(nomi_01_alloc_rewards_actual.value)
                .saturating_add(nomi_02_alloc_rewards_actual.value)
                .saturating_add(nomi_03_alloc_rewards_actual.value);

            log::trace!(
                "profile_rewards_check_config1:[{:#?}] - Act[{:#?}], Devi[{:#?}], Diff[{:#?}]",
                line!(),
                total_minted_rewards as f64 / constants::NODL as f64,
                total_minted_rewards_sharing as f64 / constants::NODL as f64,
                total_minted_rewards.saturating_sub(total_minted_rewards_sharing) as f64
                    / constants::NODL as f64,
            );

            let mut new1 = vec![
                Event::StakeReward(3, 6469283504),
                Event::StakeReward(8, 11174216961),
                Event::StakeReward(1, 6670818826),
                Event::StakeReward(6, 11522323425),
                Event::StakeReward(2, 6549897633),
                Event::StakeReward(7, 11313459546),
                Event::ValidatorChosen(3, 1, 1500 * constants::NODL),
                Event::ValidatorChosen(3, 2, 1500 * constants::NODL),
                Event::ValidatorChosen(3, 3, 1500 * constants::NODL),
                Event::NewSession(10, 3, 3, 4500 * constants::NODL),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
        });
}

#[test]
fn profile_rewards_check_config4() {
    ExtBuilder::default()
        .with_invulnerables(vec![1, 2, 3])
        .with_balances(vec![
            (1, 1100 * constants::NODL),
            (2, 1100 * constants::NODL),
            (3, 1100 * constants::NODL),
            (6, 1100 * constants::NODL),
            (7, 1100 * constants::NODL),
            (8, 1100 * constants::NODL),
        ])
        .with_validators(vec![
            (1, 1000 * constants::NODL),
            (2, 1000 * constants::NODL),
            (3, 1000 * constants::NODL),
        ])
        .with_nominators(vec![
            (6, 1, 500 * constants::NODL),
            (7, 2, 500 * constants::NODL),
            (8, 3, 500 * constants::NODL),
        ])
        .tst_staking_build()
        .execute_with(|| {
            start_session(1);

            let mut expected = vec![
                Event::ValidatorChosen(2, 1, 1500 * constants::NODL),
                Event::ValidatorChosen(2, 2, 1500 * constants::NODL),
                Event::ValidatorChosen(2, 3, 1500 * constants::NODL),
                Event::NewSession(5, 2, 3, 4500 * constants::NODL),
            ];

            assert_eq!(events(), expected);

            let valid_01_reward_points = 3442;
            let valid_02_reward_points = 3880;
            let valid_03_reward_points = 4054;
            let total_minted_rewards: Balance = (0.0332f64 * constants::NODL as f64) as Balance;

            set_author(1, 1, valid_01_reward_points);
            set_author(1, 2, valid_02_reward_points);
            set_author(1, 3, valid_03_reward_points);
            mint_rewards(total_minted_rewards);
            assert_eq!(NodleStaking::total(), 4500 * constants::NODL);
            assert_eq!(NodleStaking::points(1), 11376);

            start_session(2);

            let valid_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(1)[NodleStaking::stake_rewards(1).len() - 1];

            let valid_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(2)[NodleStaking::stake_rewards(2).len() - 1];

            let valid_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(3)[NodleStaking::stake_rewards(3).len() - 1];

            let nomi_01_alloc_rewards_actual =
                NodleStaking::stake_rewards(6)[NodleStaking::stake_rewards(6).len() - 1];

            let nomi_02_alloc_rewards_actual =
                NodleStaking::stake_rewards(7)[NodleStaking::stake_rewards(7).len() - 1];

            let nomi_03_alloc_rewards_actual =
                NodleStaking::stake_rewards(8)[NodleStaking::stake_rewards(8).len() - 1];

            let total_minted_rewards_sharing = valid_01_alloc_rewards_actual
                .value
                .saturating_add(valid_02_alloc_rewards_actual.value)
                .saturating_add(valid_03_alloc_rewards_actual.value)
                .saturating_add(nomi_01_alloc_rewards_actual.value)
                .saturating_add(nomi_02_alloc_rewards_actual.value)
                .saturating_add(nomi_03_alloc_rewards_actual.value);

            log::trace!(
                "profile_rewards_check_config1:[{:#?}] - Act[{:#?}], Devi[{:#?}], Diff[{:#?}]",
                line!(),
                total_minted_rewards as f64 / constants::NODL as f64,
                total_minted_rewards_sharing as f64 / constants::NODL as f64,
                total_minted_rewards.saturating_sub(total_minted_rewards_sharing) as f64
                    / constants::NODL as f64,
            );

            let mut new1 = vec![
                Event::StakeReward(3, 8084717511),
                Event::StakeReward(8, 3746576408),
                Event::StakeReward(1, 6864232288),
                Event::StakeReward(6, 3180985694),
                Event::StakeReward(2, 7737716823),
                Event::StakeReward(7, 3585771210),
                Event::ValidatorChosen(3, 1, 1500 * constants::NODL),
                Event::ValidatorChosen(3, 2, 1500 * constants::NODL),
                Event::ValidatorChosen(3, 3, 1500 * constants::NODL),
                Event::NewSession(10, 3, 3, 4500 * constants::NODL),
            ];
            expected.append(&mut new1);
            assert_eq!(events(), expected);
        });
}
