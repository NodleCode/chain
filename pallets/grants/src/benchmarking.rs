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
#![allow(unused)]

use super::*;

use crate::Pallet as Grants;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, BenchmarkError};
use frame_support::traits::{EnsureOrigin, Get, UnfilteredDispatchable};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

const SEED: u32 = 0;

struct BenchmarkConfig<T: Config> {
	granter: T::AccountId,
	grantee: T::AccountId,
	grantee_lookup: <T::Lookup as StaticLookup>::Source,
	collector_lookup: <T::Lookup as StaticLookup>::Source,
	schedule: VestingSchedule<T::BlockNumber, BalanceOf<T>>,
}

fn create_shared_config<T: Config>(u: u32) -> BenchmarkConfig<T> {
	let granter: T::AccountId = account("granter", u, SEED);
	let grantee: T::AccountId = account("grantee", u, SEED);
	let collector: T::AccountId = account("collector", u, SEED);
	let grantee_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(grantee.clone());
	let collector_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(collector);

	T::Currency::make_free_balance_be(&granter, BalanceOf::<T>::max_value());

	let schedule = VestingSchedule {
		start: 0u32.into(),
		period: 10u32.into(),
		period_count: 2u32,
		per_period: T::Currency::minimum_balance(),
	};

	BenchmarkConfig {
		granter,
		grantee,
		grantee_lookup,
		collector_lookup,
		schedule,
	}
}

benchmarks! {
	add_vesting_schedule {
		let config = create_shared_config::<T>(1);

		// Add some existing schedules according to b
		for _x in 1 .. T::MaxSchedule::get() {
			Pallet::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
		}
	}:  _(RawOrigin::Signed(config.granter.clone()), config.grantee_lookup.clone(), config.schedule.clone())

	claim {
		let config = create_shared_config::<T>(1);
		Pallet::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;

		// Add some existing schedules according to b
		for _x in 1 .. T::MaxSchedule::get() {
			Pallet::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
		}
	}: _(RawOrigin::Signed(config.grantee))

	cancel_all_vesting_schedules {
	   let config = create_shared_config::<T>(1);

		// Add some existing schedules according to b
		for _x in 1 .. T::MaxSchedule::get() {
			Pallet::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
		}

		let call = Call::<T>::cancel_all_vesting_schedules{
			who: config.grantee_lookup,
			funds_collector: config.collector_lookup
		};
		let origin = T::CancelOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
	}: { call.dispatch_bypass_filter(origin)? }

	renounce {
		let config = create_shared_config::<T>(1);
		let call = Call::<T>::renounce{
			who: config.grantee_lookup,
		};
		let origin = T::CancelOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
	}: { call.dispatch_bypass_filter(origin)? }

	impl_benchmark_test_suite!(
		Grants,
		crate::mock::ExtBuilder::default()
			.one_hundred_for_alice()
			.build(),
		crate::mock::Test,
	);

}
