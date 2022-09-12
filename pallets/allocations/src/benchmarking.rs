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
use frame_support::{
	traits::{ConstU32, Hooks},
	BoundedVec,
};
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

	on_initialize {
		let c in 0..1;
		let r in 0..1;
		Allocations::<T>::on_initialize(One::one());
		let n = if c == 1 {
					if r == 1 {
						T::MintCurve::get().session_period() * T::MintCurve::get().fiscal_period()
					}
					else {
						T::MintCurve::get().fiscal_period()
					}
				}
				else {
					if r == 1 {
						T::MintCurve::get().session_period()
					}
					else {
						T::MintCurve::get().session_period() * T::MintCurve::get().fiscal_period() + One::one()
					}
				};
	}: {
		Allocations::<T>::on_initialize(n);
	}
	verify {
		if r == 1 {
			assert_last_event::<T>(Event::SessionQuotaRenewed.into())
		}
		else if c == 1 {
			assert_last_event::<T>(Event::SessionQuotaCalculated(Zero::zero()).into())
		}
		else {
			assert!(frame_system::Pallet::<T>::events().is_empty())
		}
	}

	impl_benchmark_test_suite!(
		Allocations,
		crate::tests::new_test_ext(),
		crate::tests::Test,
	);
}
