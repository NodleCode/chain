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

use frame_benchmarking::{account, benchmarks};
use frame_support::traits::UnfilteredDispatchable;
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

const MAX_SCHEDULES: u32 = 100;
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
        let u in 1 .. 1000;
        let b in 0 .. MAX_SCHEDULES;

        let config = create_shared_config::<T>(u);

        // Add some existing schedules according to b
        for x in 0 .. b {
            Module::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
        }
    }:  _(RawOrigin::Signed(config.granter), config.grantee_lookup, config.schedule)

    claim {
        let u in 1 .. 1000;
        let b in 0 .. MAX_SCHEDULES;

        let config = create_shared_config::<T>(u);
        Module::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;

        // Add some existing schedules according to b
        for x in 0 .. b {
            Module::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
        }
    }: _(RawOrigin::Signed(config.grantee))

    cancel_all_vesting_schedules {
        let u in 1 .. 1000;
        let b in 0 .. MAX_SCHEDULES;

       let config = create_shared_config::<T>(u);

        // Add some existing schedules according to b
        for x in 0 .. b {
            Module::<T>::do_add_vesting_schedule(&config.granter, &config.grantee, config.schedule.clone())?;
        }

        let call = Call::<T>::cancel_all_vesting_schedules(config.grantee_lookup, config.collector_lookup);
        let origin = T::CancelOrigin::successful_origin();
    }: { call.dispatch_bypass_filter(origin)? }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{ExtBuilder, Test as Runtime};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_add_vesting_schedule::<Runtime>());
            assert_ok!(test_benchmark_claim::<Runtime>());
            assert_ok!(test_benchmark_cancel_all_vesting_schedules::<Runtime>());
        });
    }
}
