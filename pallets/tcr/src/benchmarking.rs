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

//! Tcr pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks_instance_pallet, impl_benchmark_test_suite};
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use frame_system::RawOrigin;
use sp_std::prelude::*;

use crate::Pallet as Tcr;

const SEED: u32 = 0;
const MAX_METADATA_SIZE: u32 = 1000;

pub struct BenchmarkConfig<T: Config<I>, I: 'static> {
    applicant: T::AccountId,
    challenger: T::AccountId,
    counterer: T::AccountId,
    voter: T::AccountId,
    metadata: Vec<u8>,
    deposit_applying: BalanceOf<T, I>,
    deposit_challenging: BalanceOf<T, I>,
    deposit_countering: BalanceOf<T, I>,
    deposit_voting: BalanceOf<T, I>,
}

fn make_benchmark_config<T: Config<I>, I: 'static>(u: u32, b: u32) -> BenchmarkConfig<T, I> {
    let applicant = account("applicant", u, SEED);
    let challenger = account("challenger", u, SEED);
    let counterer = account("counterer", u, SEED);
    let voter = account("voter", u, SEED);
    let metadata = vec![1; b as usize];
    let deposit_applying = T::MinimumApplicationAmount::get();
    let deposit_challenging = T::MinimumChallengeAmount::get();
    let deposit_countering = T::MinimumCounterAmount::get();
    let deposit_voting = deposit_applying + deposit_countering;

    T::Currency::make_free_balance_be(&applicant, deposit_applying);
    T::Currency::make_free_balance_be(&counterer, deposit_countering);
    T::Currency::make_free_balance_be(&voter, deposit_voting);
    T::Currency::make_free_balance_be(&challenger, deposit_challenging);

    BenchmarkConfig {
        applicant,
        challenger,
        counterer,
        voter,
        metadata,
        deposit_applying,
        deposit_challenging,
        deposit_countering,
        deposit_voting,
    }
}

fn do_apply<T: Config<I>, I: 'static>(
    config: &BenchmarkConfig<T, I>,
) -> DispatchResultWithPostInfo {
    <Pallet<T, _>>::apply(
        RawOrigin::Signed(config.applicant.clone()).into(),
        config.metadata.clone(),
        config.deposit_applying,
    )
}

benchmarks_instance_pallet! {

    apply {
        let b in 0 .. MAX_METADATA_SIZE;

        let config = make_benchmark_config::<T, _>(0, b);
    }: _(RawOrigin::Signed(config.applicant.clone()), config.metadata.clone(), config.deposit_applying.clone())

    counter {
        let config = make_benchmark_config::<T, _>(0, 0);

        do_apply::<T, I>(&config)?;
    }: _(RawOrigin::Signed(config.counterer.clone()), config.applicant.clone(), config.deposit_countering.clone())

    vote {
        let config = make_benchmark_config::<T, _>(0, 0);
        let supporting = true;

        do_apply::<T, I>(&config)?;

        let _ = <Pallet<T, _>>::counter(
            RawOrigin::Signed(config.counterer.clone()).into(),
            config.applicant.clone(),
            config.deposit_countering
        );
    }: _(RawOrigin::Signed(config.voter.clone()), config.applicant.clone(), supporting, config.deposit_voting.clone())

    challenge {
        let config = make_benchmark_config::<T, _>(0, 0);

        do_apply::<T, I>(&config)?;

        let _ = <Pallet<T, _>>::commit_applications(
            T::FinalizeApplicationPeriod::get() + <system::Pallet<T>>::block_number()
        );
    }: _(RawOrigin::Signed(config.challenger.clone()), config.applicant.clone(), config.deposit_challenging.clone())

    impl_benchmark_test_suite!(Tcr, crate::tests::new_test_ext(), crate::tests::Test,);
}
