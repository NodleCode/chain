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

//! Reserve pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]
#![allow(unused)]

use super::*;

use frame_benchmarking::{account, benchmarks_instance_pallet, impl_benchmark_test_suite, BenchmarkError};
use frame_support::traits::{EnsureOrigin, UnfilteredDispatchable};
use frame_system::RawOrigin;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

#[cfg(test)]
use crate::Pallet as Reserve;

const SEED: u32 = 0;

benchmarks_instance_pallet! {
	tip {
		let tipper = account("caller", 0, SEED);
		let smallest_usable_amount = T::Currency::minimum_balance().saturating_add(1u32.into());
		let starting_amount = smallest_usable_amount.saturating_add(smallest_usable_amount);
		let tip_amount = smallest_usable_amount;
		let _ = T::Currency::make_free_balance_be(&tipper,starting_amount);
	}: _(RawOrigin::Signed(tipper), tip_amount)

	spend {
		let dest = account("dest", 0, SEED);
		let smallest_usable_amount = T::Currency::minimum_balance();
		let starting_amount = smallest_usable_amount.saturating_add(smallest_usable_amount);

		T::Currency::make_free_balance_be(&T::PalletId::get().into_account_truncating(),starting_amount);

		let call = Call::<T, I>::spend{
			to: dest,
			amount: smallest_usable_amount
		};
		let origin = T::ExternalOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
	}: { call.dispatch_bypass_filter(origin)? }

	impl_benchmark_test_suite!(Reserve, crate::tests::new_test_ext(), crate::tests::Test,);
}
