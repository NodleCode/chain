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

#![cfg(feature = "runtime-benchmarks")]

//! Amendments pallet benchmarks

use super::*;
use crate::BalanceOf;
#[cfg(test)]
use crate::Pallet as Allocations;
use frame_benchmarking::{account, benchmarks};
use frame_support::{traits::ConstU32, BoundedVec};
use frame_system::RawOrigin;
use sp_std::prelude::*;

pub type MaxMembers = ConstU32<10>;

const SEED: u32 = 0;

fn make_batch<T: Config>(b: u32) -> BoundedVec<(T::AccountId, BalanceOf<T>), T::MaxAllocs> {
	let mut ret = BoundedVec::with_bounded_capacity(b as usize);

	for i in 0..b {
		let account = account("grantee", i, SEED);
		let _ = ret.try_push((account, T::ExistentialDeposit::get() * 10u32.into()));
	}
	ret
}

benchmarks! {
	batch {
		let b in 1..T::MaxAllocs::get();

		let batch_arg = make_batch::<T>(b);
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let mut members = <BenchmarkOracles<T>>::get();
		assert!(members.try_push(oracle.clone()).is_ok());
		<BenchmarkOracles<T>>::put(&members);
	}: _(RawOrigin::Signed(oracle), batch_arg)

	impl_benchmark_test_suite!(
		Allocations,
		crate::tests::new_test_ext(),
		crate::tests::Test,
	);
}
