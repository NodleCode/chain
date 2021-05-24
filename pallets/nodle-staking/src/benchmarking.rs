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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{
    account, benchmarks, impl_benchmark_test_suite, whitelist_account, whitelisted_caller,
};
use frame_support::traits::{Currency, Get};
use frame_system::{EventRecord, RawOrigin};
use sp_std::prelude::*;

use crate::Pallet as NodleStaking;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn register_validator<T: Config>(perfix: &'static str, count: u32) -> Vec<T::AccountId> {
    let validators: Vec<T::AccountId> = (0..count)
        .map(|c| account(perfix, c, SEED))
        .collect::<Vec<_>>();
    log::trace!(
        "[register_validator > {:#?}]=> - validators-len-{:#?}",
        line!(),
        validators.len()
    );
    assert!(
        T::MinValidatorPoolStake::get() > 0u32.into(),
        "Bond cannot be zero!"
    );
    for who in validators.clone() {
        let bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        T::Currency::make_free_balance_be(&who, bond_val);
        <NodleStaking<T>>::validator_join_pool(RawOrigin::Signed(who).into(), bond_val).unwrap();
    }
    validators
}

/// Grab a funded user.
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, SEED);
    let balance = T::Currency::minimum_balance() * balance_factor.into();
    T::Currency::make_free_balance_be(&user, balance);
    T::Currency::issue(balance);
    user
}

benchmarks! {
    validator_join_pool {
        let c in 1 .. T::MinSelectedValidators::get();
        log::trace!("[validator_join_pool > {:#?}]=> - Itern-{:#?}", line!(), c);
        register_validator::<T>("vjp-validator", c);
        let caller: T::AccountId = whitelisted_caller();
        let bond: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        T::Currency::make_free_balance_be(&caller, bond.clone());
    }: _(RawOrigin::Signed(caller.clone()), bond)
    verify {
        assert_last_event::<T>(
            Event::JoinedValidatorPool(caller, bond, <NodleStaking<T>>::total()).into()
        );
    }

    validator_bond_more {
        let validator = create_funded_user::<T>("vbm-validator", SEED, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let bond_additional = T::Currency::minimum_balance() * 10u32.into();
    }: _(RawOrigin::Signed(validator.clone()), bond_additional)
    verify {
        log::trace!(
            "[validator_bond_more > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        assert_last_event::<T>(
            Event::ValidatorBondedMore(
                validator.clone(),
                validator_bond_val,
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    validator_bond_less {
        let validator = create_funded_user::<T>("vbl-validator", SEED, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let bond_less = T::Currency::minimum_balance() * 2u32.into();
    }: _(RawOrigin::Signed(validator.clone()), bond_less)
    verify {
        log::trace!(
            "[validator_bond_less > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        assert_last_event::<T>(
            Event::ValidatorBondedLess(
                validator.clone(),
                validator_bond_val,
                <NodleStaking<T>>::validator_state(&validator).unwrap().bond
            ).into()
        );
    }

    validator_exit_pool {
        let validator = create_funded_user::<T>("vep-validator", SEED, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
    }: _(RawOrigin::Signed(validator.clone()))
    verify {
        log::trace!(
            "[validator_exit_pool > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        assert_last_event::<T>(
            Event::ValidatorScheduledExit(
                0,
                validator.clone(),
                2
            ).into()
        );
    }

    nominator_nominate {
        let n in 1 .. T::MaxValidatorPerNominator::get();
        log::trace!("[nominator_nominate > {:#?}]=> - Itern-{:#?}", line!(), n);
        let validator = create_funded_user::<T>("nom-validator", n, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let nominator = create_funded_user::<T>("nom-nominator", n, 100);
        whitelist_account!(nominator);
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
        log::trace!( "[nominator_nominate > {:#?}]=> - {:#?}", line!(), mock::events());

    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_val)
    verify {
        log::trace!( "[nominator_nominate > {:#?}]=> - Verif-{:#?}", line!(), mock::events());
        assert_last_event::<T>(
            Event::Nomination(
                nominator,
                nominator_bond_val,
                validator.clone(),
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    nominator_denominate {
        let n in 1 .. T::MaxValidatorPerNominator::get();
        log::trace!( "[nominator_denominate > {:#?}]=> - Itern-{:#?}", line!(), n);
        let validator = create_funded_user::<T>("nden-validator", n, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let nominator = create_funded_user::<T>("nden-nominator", n, 100);
        whitelist_account!(nominator);
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
        <NodleStaking<T>>::nominator_nominate(
            RawOrigin::Signed(nominator.clone()).into(),
            validator.clone(),
            nominator_bond_val
        ).unwrap();
        log::trace!(
            "[nominator_denominate > {:#?}]=> - Top-{:#?}",
            line!(),
            crate::mock::events()
        );
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone())
    verify {
        log::trace!(
            "[nominator_denominate > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        assert_last_event::<T>(
            Event::NominatorLeftValidator(
                nominator,
                validator.clone(),
                nominator_bond_val,
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    nominator_denominate_all {
        let n in 1 .. T::MaxValidatorPerNominator::get();
        log::trace!( "[nominator_denominate > {:#?}]=> - Itern-{:#?}", line!(), n);

        let validator_list = register_validator::<T>(
            "nda-validator",
            T::MaxValidatorPerNominator::get()
        );

        let nominator = create_funded_user::<T>("nda-nominator", n, 100);
        whitelist_account!(nominator);
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 1u32.into();

        for valid_itm in validator_list.clone() {
            <NodleStaking<T>>::nominator_nominate(
                RawOrigin::Signed(nominator.clone()).into(),
                valid_itm.clone(),
                nominator_bond_val
            ).unwrap();
        }
        log::trace!(
            "[nominator_denominate > {:#?}]=> - Top-{:#?}",
            line!(),
            crate::mock::events()
        );
    }: _(RawOrigin::Signed(nominator.clone()))
    verify {
        log::trace!(
            "[nominator_denominate > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        let verif_idx: usize = T::MaxValidatorPerNominator::get() as usize - 1;
        assert_last_event::<T>(
            Event::NominatorLeftValidator(
                nominator,
                validator_list[verif_idx].clone(),
                nominator_bond_val,
                <NodleStaking<T>>::validator_state(validator_list[verif_idx].clone()).unwrap().total
            ).into()
        );
    }

    nominator_bond_more {
        let n in 1 .. T::MaxValidatorPerNominator::get();
        log::trace!( "[nominator_bond_more > {:#?}]=> - Itern-{:#?}", line!(), n);
        let validator = create_funded_user::<T>("nbndm-validator", n, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let nominator = create_funded_user::<T>("nbndm-nominator", n, 100);
        whitelist_account!(nominator);
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
        <NodleStaking<T>>::nominator_nominate(
            RawOrigin::Signed(nominator.clone()).into(),
            validator.clone(),
            nominator_bond_val
        ).unwrap();
        log::trace!(
            "[nominator_bond_more > {:#?}]=> - Top-{:#?}",
            line!(),
            crate::mock::events()
        );
        let nominator_bond_addition: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_addition)
    verify {
        log::trace!(
            "[nominator_bond_more > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );
        assert_last_event::<T>(
            Event::NominationIncreased(
                nominator,
                validator.clone(),
                <NodleStaking<T>>::validator_state(&validator).unwrap().total - nominator_bond_addition,
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    nominator_bond_less {
        let n in 1 .. T::MaxValidatorPerNominator::get();
        log::trace!("[nominator_bond_less > {:#?}]=> - Itern-{:#?}", line!(), n);
        let validator = create_funded_user::<T>("nbndl-validator", n, 100);
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        <NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(validator.clone()).into(),
            validator_bond_val
        ).unwrap();
        let nominator = create_funded_user::<T>("nbndl-nominator", n, 100);
        whitelist_account!(nominator);
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 4u32.into();
        <NodleStaking<T>>::nominator_nominate(
            RawOrigin::Signed(nominator.clone()).into(),
            validator.clone(),
            nominator_bond_val
        ).unwrap();
        log::trace!(
            "[nominator_bond_less > {:#?}]=> - Top-{:#?}",
            line!(),
            crate::mock::events()
        );
        let nominator_bond_removal: BalanceOf<T> = T::MinNomination::get() * 1u32.into();
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_removal)
    verify {
        log::trace!(
            "[nominator_bond_less > {:#?}]=> - Verif-{:#?}",
            line!(),
            crate::mock::events()
        );

        let before = <NodleStaking<T>>::validator_state(&validator).unwrap().bond +
            <NodleStaking<T>>::validator_state(&validator).unwrap().nomi_bond_total +
            nominator_bond_removal;

        let after = <NodleStaking<T>>::validator_state(&validator).unwrap().bond +
            <NodleStaking<T>>::validator_state(&validator).unwrap().nomi_bond_total;

        assert_last_event::<T>(
            Event::NominationDecreased(
                nominator,
                validator.clone(),
                before,
                after,
            ).into()
        );
    }


}

impl_benchmark_test_suite!(
    NodleStaking,
    crate::mock::ExtBuilder::default().has_stakers(true).build(),
    crate::mock::Test,
);
