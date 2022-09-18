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
use crate::Pallet as Allocations;
use frame_benchmarking::{account, benchmarks};
use frame_support::{traits::ConstU32, BoundedVec};
use frame_system::RawOrigin;
use sp_std::prelude::*;

pub type MaxMembers = ConstU32<10>;

const SEED: u32 = 0;
const ALLOC_FACTOR: u32 = 10;

fn make_batch<T: Config>(b: u32) -> BoundedVec<(T::AccountId, BalanceOf<T>), T::MaxAllocs> {
	let mut ret = BoundedVec::with_bounded_capacity(b as usize);

	for i in 0..b {
		let account = account("grantee", i, SEED);
		let _ = ret.try_push((account, T::ExistentialDeposit::get() * ALLOC_FACTOR.into()));
	}
	ret
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	assert!(!events.is_empty());
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	batch {
		let b in 1..T::MaxAllocs::get();

		let batch_arg = make_batch::<T>(b);
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let mut members = <BenchmarkOracles<T>>::get();
		assert!(members.try_push(oracle.clone()).is_ok());
		<BenchmarkOracles<T>>::put(&members);
		<SessionQuota<T>>::put(T::ExistentialDeposit::get() * (b * ALLOC_FACTOR).into());
	}: _(RawOrigin::Signed(oracle), batch_arg)

	calc_quota {
	}: {
		Allocations::<T>::checked_calc_session_quota(Zero::zero(), true);
	}
	verify {
		assert_last_event::<T>(Event::SessionQuotaCalculated(<NextSessionQuota<T>>::get().unwrap()).into())
	}

	renew_quota {
	}: {
		Allocations::<T>::checked_renew_session_quota(Zero::zero(), true);
	}
	verify {
		assert_last_event::<T>(Event::SessionQuotaRenewed.into())
	}

	set_curve_starting_block {
	}: _(RawOrigin::Root, One::one())
	verify {
		assert_eq!(<MintCurveStartingBlock<T>>::get(), Some(One::one()));
	}

	impl_benchmark_test_suite!(
		Allocations,
		crate::tests::new_test_ext(),
		crate::tests::Test,
	);
}
