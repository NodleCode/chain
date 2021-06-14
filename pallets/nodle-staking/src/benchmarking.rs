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

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelist_account};
use frame_support::{
    assert_ok,
    traits::{Currency, EnsureOrigin, Get, UnfilteredDispatchable},
};
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::traits::Zero;
use sp_std::prelude::*;

use crate::types::UnappliedSlash;
use crate::Pallet as NodleStaking;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;
const MAX_VALIDATORS: u32 = 1000;
const MAX_SLASHES: u32 = 1000;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance: BalanceOf<T>,
) -> T::AccountId {
    let user = account(string, n, SEED);
    T::Currency::make_free_balance_be(&user, balance);
    T::Currency::issue(balance);
    user
}

fn register_validator<T: Config>(prefix: &'static str, count: u32) -> Vec<T::AccountId> {
    let mut validators: Vec<T::AccountId> = vec![];
    assert!(
        T::MinValidatorPoolStake::get() > 0u32.into(),
        "Bond cannot be zero!"
    );
    let bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
    for valid_idx in 0..count {
        let who = create_funded_user::<T>(prefix, valid_idx, bond_val);
        validators.push(who.clone());
        assert_ok!(<NodleStaking<T>>::validator_join_pool(
            RawOrigin::Signed(who).into(),
            bond_val
        ));
    }

    log::trace!(
        "[register_validator > {:#?}]=> - validators-len-{:#?}",
        line!(),
        validators.len()
    );

    validators
}

benchmarks! {
    // Benchmark `set_invulnerables` extrinsic with the best possible conditions:
    // * Origin of the Call may be from CancelOrigin or ROOT account.
    // * Call will create the validator account.
    set_invulnerables {
        let c in 1 .. T::MinSelectedValidators::get();
        log::trace!("[set_invulnerables > {:#?}]=> - Itern-{:#?}", line!(), c);
        let inv_validators = register_validator::<T>("sinv-validator", c);
        let caller = T::CancelOrigin::successful_origin();
        let call = Call::<T>::set_invulnerables(inv_validators.clone());
    }: { call.dispatch_bypass_filter(caller)? }
    verify {
        assert_last_event::<T>(
            Event::NewInvulnerables(inv_validators).into()
        );
    }

    // Benchmark `set_total_validator_per_round` extrinsic with the best possible conditions:
    // * Origin of the Call may be from CancelOrigin or ROOT account.
    set_total_validator_per_round {
       let c in 5 .. T::MinSelectedValidators::get() * 2;
       let caller = T::CancelOrigin::successful_origin();
       let call = Call::<T>::set_total_validator_per_round(c);
       let old = <TotalSelected<T>>::get();
   }: { call.dispatch_bypass_filter(caller)? }
   verify {
       assert_last_event::<T>(
           Event::TotalSelectedSet(old, c).into()
       );
   }

    // Benchmark `validator_join_pool` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator account.
   validator_join_pool {
       let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
       let validator = create_funded_user::<T>("vjp-validator", SEED, validator_bond_val);
   }: _(RawOrigin::Signed(validator.clone()), validator_bond_val)
   verify {
       assert_last_event::<T>(
           Event::JoinedValidatorPool(
               validator,
               validator_bond_val,
               <NodleStaking<T>>::total()
           ).into()
       );
   }

       // Benchmark `validator_bond_more` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator account.
   validator_bond_more {
       let validator_bal: BalanceOf<T> = T::MinValidatorPoolStake::get() * 3u32.into();
       let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
       let validator = create_funded_user::<T>("vbm-validator", SEED, validator_bal);
       assert_ok!(
           <NodleStaking<T>>::validator_join_pool(
           RawOrigin::Signed(validator.clone()).into(),
           validator_bond_val)
       );
       let bond_additional = T::MinValidatorPoolStake::get() * 1u32.into();
   }: _(RawOrigin::Signed(validator.clone()), bond_additional)
   verify {
       assert_last_event::<T>(
           Event::ValidatorBondedMore(
               validator.clone(),
               validator_bond_val,
               <NodleStaking<T>>::validator_state(&validator).unwrap().total
           ).into()
       );
   }

       // Benchmark `validator_bond_less` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator account.
   validator_bond_less {
       let validator_bal: BalanceOf<T> = T::MinValidatorPoolStake::get() * 3u32.into();
       let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
       let validator = create_funded_user::<T>("vbl-validator", SEED, validator_bal);
       assert_ok!(
           <NodleStaking<T>>::validator_join_pool(
               RawOrigin::Signed(validator.clone()).into(),
               validator_bond_val
           )
       );
       let bond_less = T::MinValidatorPoolStake::get() * 1u32.into();
   }: _(RawOrigin::Signed(validator.clone()), bond_less)
   verify {
       assert_last_event::<T>(
           Event::ValidatorBondedLess(
               validator.clone(),
               validator_bond_val,
               <NodleStaking<T>>::validator_state(&validator).unwrap().bond
           ).into()
       );
   }

       // Benchmark `validator_exit_pool` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator account.
   validator_exit_pool {
       let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
       let validator = create_funded_user::<T>("vep-validator", SEED, validator_bond_val);
       assert_ok!(
           <NodleStaking<T>>::validator_join_pool(
               RawOrigin::Signed(validator.clone()).into(),
               validator_bond_val
           )
       );
   }: _(RawOrigin::Signed(validator.clone()))
   verify {
       assert_eq!(
           <NodleStaking<T>>::validator_state(&validator).unwrap().is_leaving(),
           true
       );
   }

    // Benchmark `nominator_nominate` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
   nominator_nominate {
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        let validator = create_funded_user::<T>("nom-validator", SEED, validator_bond_val);
        assert_ok!(
            <NodleStaking<T>>::validator_join_pool(
                RawOrigin::Signed(validator.clone()).into(),
                validator_bond_val
            )
        );
        let nominator_bond_val: BalanceOf<T> = T::MinNominatorStake::get() * 2u32.into();
        let nominator = create_funded_user::<T>("nom-nominator", SEED, nominator_bond_val);
        whitelist_account!(nominator);
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_val)
    verify {
        assert_last_event::<T>(
            Event::Nomination(
                nominator,
                nominator_bond_val,
                validator.clone(),
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    // Benchmark `nominator_denominate` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
    nominator_denominate {

        let validator_list = register_validator::<T>(
            "nda-validator",
            T::MaxValidatorPerNominator::get()
        );

        let nominator_bond_val: BalanceOf<T> = T::MinNominatorStake::get() * 2u32.into();
        let nominator = create_funded_user::<T>(
            "nden-nominator",
            SEED,
            nominator_bond_val * T::MaxValidatorPerNominator::get().into(),
        );
        whitelist_account!(nominator);

        for validator in validator_list.clone() {
            assert_ok!(
                <NodleStaking<T>>::nominator_nominate(
                    RawOrigin::Signed(nominator.clone()).into(),
                    validator.clone(),
                    nominator_bond_val
                )
            );
        }
        let validator_to_exit = validator_list[0].clone();
    }: _(RawOrigin::Signed(nominator.clone()), validator_to_exit.clone())
    verify {
        assert_last_event::<T>(
            Event::NominatorLeftValidator(
                nominator,
                validator_to_exit.clone(),
                nominator_bond_val,
                <NodleStaking<T>>::validator_state(&validator_to_exit).unwrap().total
            ).into()
        );
    }

    // Benchmark `nominator_bond_more` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
    nominator_bond_more {
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        let validator = create_funded_user::<T>("nbndm-validator", SEED, validator_bond_val);
        assert_ok!(
            <NodleStaking<T>>::validator_join_pool(
                RawOrigin::Signed(validator.clone()).into(),
                validator_bond_val
            )
        );
        let nominator_balance: BalanceOf<T> = T::MinNomination::get() * 4u32.into();
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
        let nominator = create_funded_user::<T>("nbndm-nominator", SEED, nominator_balance);
        whitelist_account!(nominator);
        assert_ok!(
            <NodleStaking<T>>::nominator_nominate(
                RawOrigin::Signed(nominator.clone()).into(),
                validator.clone(),
                nominator_bond_val
            )
        );
        let nominator_bond_addition: BalanceOf<T> = T::MinNomination::get() * 2u32.into();
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_addition)
    verify {
        assert_last_event::<T>(
            Event::NominationIncreased(
                nominator,
                validator.clone(),
                <NodleStaking<T>>::validator_state(&validator).unwrap().total - nominator_bond_addition,
                <NodleStaking<T>>::validator_state(&validator).unwrap().total
            ).into()
        );
    }

    // Benchmark `nominator_bond_less` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
    nominator_bond_less {
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        let validator = create_funded_user::<T>("nbndl-validator", SEED, validator_bond_val);
        assert_ok!(
            <NodleStaking<T>>::validator_join_pool(
                RawOrigin::Signed(validator.clone()).into(),
                validator_bond_val
            )
        );
        let nominator_balance: BalanceOf<T> = T::MinNomination::get() * 4u32.into();
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 4u32.into();
        let nominator = create_funded_user::<T>("nbndl-nominator", SEED, nominator_balance);
        whitelist_account!(nominator);
        assert_ok!(
            <NodleStaking<T>>::nominator_nominate(
                RawOrigin::Signed(nominator.clone()).into(),
                validator.clone(),
                nominator_bond_val
            )
        );
        let nominator_bond_removal: BalanceOf<T> = T::MinNomination::get() * 1u32.into();
    }: _(RawOrigin::Signed(nominator.clone()), validator.clone(), nominator_bond_removal)
    verify {
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

    // Benchmark `nominator_denominate_all` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
    nominator_denominate_all {
        let validator_list = register_validator::<T>(
            "nda-validator",
            T::MaxValidatorPerNominator::get()
        );
        let nominator_bond_val: BalanceOf<T> = T::MinNomination::get() * 1u32.into();
        let nominator = create_funded_user::<T>(
            "nda-nominator",
            SEED,
            nominator_bond_val * T::MaxValidatorPerNominator::get().into()
        );
        whitelist_account!(nominator);
        for valid_itm in validator_list.clone() {
            assert_ok!(
                <NodleStaking<T>>::nominator_nominate(
                    RawOrigin::Signed(nominator.clone()).into(),
                    valid_itm.clone(),
                    nominator_bond_val
                )
            );
        }
    }: _(RawOrigin::Signed(nominator.clone()))
    verify {
        assert_eq!(
            <NominatorState<T>>::get(nominator.clone()).unwrap().active_bond,
            Zero::zero()
        );
        assert_eq!(
            <NominatorState<T>>::get(nominator.clone()).unwrap().unlocking.len(),
            T::MaxValidatorPerNominator::get() as usize
        );
    }

    // Benchmark `withdraw_unbonded` extrinsic with the best possible conditions:
    // * Origin of the Call is from signed origin.
    // * Call will create the validator & nominator account.
    withdraw_unbonded {
        let validator_bond_val: BalanceOf<T> = T::MinValidatorPoolStake::get() * 2u32.into();
        let validator = create_funded_user::<T>("wdu-validator", SEED, validator_bond_val);
        assert_ok!(
            <NodleStaking<T>>::validator_join_pool(
                RawOrigin::Signed(validator.clone()).into(),
                validator_bond_val
            )
        );
    }: _(RawOrigin::Signed(validator.clone()))
    verify {
        assert_last_event::<T>(
            Event::Withdrawn(
                validator,
                Zero::zero()
            ).into()
        );
    }

    // Benchmark `withdraw_unbonded` extrinsic with the best possible conditions:
    // * Origin of the Call must be Root.
    // * Call will create the validator & nominator account.
    slash_cancel_deferred {

        let s in 1 .. MAX_SLASHES;
        let c in 1 .. MAX_VALIDATORS;
        let mut unapplied_slashes = Vec::new();
        let session_idx = 1u32;

        let reg_validators = register_validator::<T>("def-validator", MAX_VALIDATORS);
        let mut deferred_validators = Vec::new();

        for idx in 0 .. MAX_SLASHES {

            let unapl_slainst = UnappliedSlash::<T::AccountId, BalanceOf<T>>{
                validator: reg_validators[idx as usize].clone(),
                ..Default::default()
            };

            if idx % 2 == 0 { deferred_validators.push(reg_validators[idx as usize].clone()) };

            unapplied_slashes.push(unapl_slainst);
        }

        <UnappliedSlashes<T>>::insert(
            session_idx.saturating_add(T::SlashDeferDuration::get()),
            &unapplied_slashes
        );

        let slash_indices: Vec<u32> = (0 .. s).collect();
    }: _(RawOrigin::Root, session_idx, deferred_validators)
    verify {
        assert_eq!(
            <UnappliedSlashes<T>>::get(
                session_idx.saturating_add(T::SlashDeferDuration::get()),
            ).len(),
            (MAX_VALIDATORS / 2) as usize
        );
    }

}

impl_benchmark_test_suite!(
    NodleStaking,
    crate::mock::ExtBuilder::default().has_stakers(true).build(),
    crate::mock::Test,
);
