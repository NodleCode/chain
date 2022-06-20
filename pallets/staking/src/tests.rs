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
#![cfg(test)]

use super::*;
use crate::mock::{
	balances, bond_nominator, bond_validator, events, is_disabled, last_event, set_author, start_session, Balance,
	Balances, CancelOrigin, Event as MetaEvent, ExtBuilder, NodleStaking, Origin, Session, System, Test,
}; //on_offence_in_session, on_offence_now,
use crate::set::OrderedSet;
use crate::types::{Bond, StakeReward, ValidatorSnapshot, ValidatorStatus};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::{
	testing::UintAuthorityId,
	traits::{BadOrigin, Zero},
	Perbill,
};
use sp_staking::offence::{DisableStrategy, OffenceDetails};

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
				MetaEvent::NodleStaking(Event::JoinedValidatorPool(7, 10u128, 1121u128))
			);
		});
}

#[test]
fn validator_activate_works() {
	ExtBuilder::default().num_validators(4).build_and_execute(|| {
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
				offender: (11, initial_exposure),
				reporters: vec![],
			}],
			&[slash_percent],
			DisableStrategy::Always,
		);

		assert_eq!(NodleStaking::validator_state(11).unwrap().state, ValidatorStatus::Idle);

		mock::start_active_session(1);

		assert_eq!(NodleStaking::validator_state(11).unwrap().state, ValidatorStatus::Idle);

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
