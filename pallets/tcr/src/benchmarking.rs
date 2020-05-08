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

use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_std::prelude::*;

const SEED_APPLICANT: u32 = 0;
const SEED_COUNTERER: u32 = 1;
const SEED_VOTER: u32 = 2;
const SEED_CHALLENGER: u32 = 3;

benchmarks! {
    _ { }

    apply {
        let u in 0 .. 1000;

        let applicant = account("applicant", u, SEED_APPLICANT);
        let metadata = (0..100).map(|v| v).collect();
        let deposit_applying = T::MinimumApplicationAmount::get();
        let _ = T::Currency::make_free_balance_be(&applicant, deposit_applying);
    }: _(RawOrigin::Signed(applicant), metadata, deposit_applying)

    counter {
        let u in 0 .. 1000;

        let applicant = account("applicant", u, SEED_APPLICANT);
        let counterer = account("counterer", u, SEED_COUNTERER);
        let metadata = (0..100).map(|v| v).collect();
        let deposit_applying = T::MinimumApplicationAmount::get();
        let deposit_countering = T::MinimumCounterAmount::get();

        let _ = T::Currency::make_free_balance_be(&applicant, deposit_applying);
        let _ = T::Currency::make_free_balance_be(&counterer, deposit_countering);

        let _ = <Module<T>>::apply(RawOrigin::Signed(
            applicant.clone()).into(),
            metadata,
            deposit_applying
        );
    }: _(RawOrigin::Signed(counterer), applicant, deposit_countering)

    vote {
        let u in 0 .. 1000;

        let applicant = account("applicant", u, SEED_APPLICANT);
        let counterer = account("counterer", u, SEED_COUNTERER);
        let voter = account("voter", u, SEED_VOTER);
        let metadata = (0..100).map(|v| v).collect();
        let deposit_applying = T::MinimumApplicationAmount::get();
        let deposit_countering = T::MinimumCounterAmount::get();
        let deposit_voting = deposit_applying + deposit_countering;
        let supporting = u % 2 == 0;

        let _ = T::Currency::make_free_balance_be(&applicant, deposit_applying);
        let _ = T::Currency::make_free_balance_be(&counterer, deposit_countering);
        let _ = T::Currency::make_free_balance_be(&voter, deposit_voting);

        let _ = <Module<T>>::apply(
            RawOrigin::Signed(applicant.clone()).into(),
            metadata,
            deposit_applying
        );

        let _ = <Module<T>>::counter(
            RawOrigin::Signed(counterer.clone()).into(),
            applicant.clone(),
            deposit_countering
        );
    }: _(RawOrigin::Signed(voter), applicant, supporting, deposit_voting)

    challenge {
        let u in 0 .. 1000;

        let applicant = account("applicant", u, SEED_APPLICANT);
        let challenger = account("challenger", u, SEED_CHALLENGER);
        let metadata = (0..100).map(|v| v).collect();
        let deposit_applying = T::MinimumApplicationAmount::get();
        let deposit_challenging = T::MinimumChallengeAmount::get();

        let _ = T::Currency::make_free_balance_be(&applicant, deposit_applying);
        let _ = T::Currency::make_free_balance_be(&challenger, deposit_challenging);

        let _ = <Module<T>>::apply(
            RawOrigin::Signed(applicant.clone()).into(),
            metadata,
            deposit_applying
        );

        let _ = <Module<T>>::commit_applications(
            T::FinalizeApplicationPeriod::get() + <system::Module<T>>::block_number()
        );
    }: _(RawOrigin::Signed(challenger), applicant, deposit_challenging)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_apply::<Test>());
            assert_ok!(test_benchmark_counter::<Test>());
            assert_ok!(test_benchmark_vote::<Test>());
            assert_ok!(test_benchmark_challenge::<Test>());
        });
    }
}
