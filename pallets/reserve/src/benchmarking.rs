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

//! Reserve pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks_instance_pallet, impl_benchmark_test_suite};
use frame_support::traits::{EnsureOrigin, UnfilteredDispatchable};
use frame_system::RawOrigin;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

use crate::Pallet as Reserve;

const SEED: u32 = 0;

benchmarks_instance_pallet! {
    tip {
        let tipper = account("caller", 0, SEED);
        let value = 100u32.into();
        let _ = T::Currency::make_free_balance_be(&tipper, value);
    }: _(RawOrigin::Signed(tipper), value)

    spend {
        let dest = account("dest", 0, SEED);
        let value = T::Currency::minimum_balance().saturating_mul(100u32.into());

        let call = Call::<T, I>::spend{
            to: dest,
            amount: value
        };
        let origin = T::ExternalOrigin::successful_origin();
    }: { call.dispatch_bypass_filter(origin)? }

    impl_benchmark_test_suite!(Reserve, crate::tests::new_test_ext(), crate::tests::Test,);
}
