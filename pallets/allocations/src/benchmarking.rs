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

//! Amendments pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_std::prelude::*;

const MAX_BYTES: u32 = 1_024;
const SEED: u32 = 0;

benchmarks! {
    _ { }

    allocate {
        let u in 1 .. 1000;
        let b in 1 .. MAX_BYTES;

        let grantee: T::AccountId = account("grantee", u, SEED);
        let oracle: T::AccountId = account("oracle", u, SEED);

        Module::<T>::initialize_members(&[oracle.clone()]);
    }: _(RawOrigin::Signed(oracle), grantee, 100.into(), vec![1; b as usize])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_allocate::<Test>());
        });
    }
}
