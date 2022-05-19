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

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{account, benchmarks, vec, whitelisted_caller};
use frame_support::{
	traits::{Currency, Get},
	weights::DispatchClass,
};
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, One, Saturating};

fn setup<T: Config>() -> (T::AccountId, BalanceOf<T>) {
	let amount: BalanceOf<T> = BalanceOf::<T>::one();
	let customer: T::AccountId = account("customer", 0, 0);
	WhitelistedCallers::<T>::put(vec![customer.clone()]);
	CurrencyOf::<T>::make_free_balance_be(&customer, amount.saturating_add(BalanceOf::<T>::min_value()));
	CurrentMin::<T>::put(BalanceOf::<T>::one());
	CurrentMax::<T>::put(BalanceOf::<T>::max_value());
	(customer, amount)
}

benchmarks! {
	initiate_wrapping {
		let (customer, amount) = setup::<T>();
		let eth_dest = EthAddress::from(&[0;20]);

	}: _(RawOrigin::Signed(customer), amount, eth_dest)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), 0u32.into());
		assert_eq!(<TotalRejected<T>>::get(), 0u32.into());
	}

	initiate_wrapping_reserve_fund {
		let amount: BalanceOf<T> = BalanceOf::<T>::one();
		let eth_dest = EthAddress::from(&[0;20]);
	}: _(RawOrigin::Root, amount, eth_dest)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), 0u32.into());
		assert_eq!(<TotalRejected<T>>::get(), 0u32.into());
	}

	settle {
		let (customer, amount) = setup::<T>();
		let oracle: T::AccountId = whitelisted_caller();
		let mut whitelisted_callers = WhitelistedCallers::<T>::get().unwrap();
		whitelisted_callers.push(oracle.clone());
		WhitelistedCallers::<T>::put(whitelisted_callers);

		let eth_dest = EthAddress::from(&[0;20]);
		let _ = Template::<T>::initiate_wrapping(RawOrigin::Signed(customer.clone()).into(), amount, eth_dest);

		let eth_hash = EthTxHash::from(&[0;32]);
	}: _(RawOrigin::Signed(oracle), customer, amount, eth_hash)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), amount);
		assert_eq!(<TotalRejected<T>>::get(), 0u32.into());
	}

	settle_reserve_fund {
		let amount: BalanceOf<T> = BalanceOf::<T>::one();
		let eth_dest = EthAddress::from(&[0;20]);
		let _ = Template::<T>::initiate_wrapping_reserve_fund(RawOrigin::Root.into(), amount, eth_dest);
		let eth_hash = EthTxHash::from(&[0;32]);
	}: _(RawOrigin::Root, amount, eth_hash)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), amount);
		assert_eq!(<TotalRejected<T>>::get(), 0u32.into());
	}

	reject {
		let (customer, amount) = setup::<T>();
		let oracle: T::AccountId = whitelisted_caller();
		let mut whitelisted_callers = WhitelistedCallers::<T>::get().unwrap();
		whitelisted_callers.push(oracle.clone());
		WhitelistedCallers::<T>::put(whitelisted_callers);

		let eth_dest = EthAddress::from(&[0;20]);
		let _ = Template::<T>::initiate_wrapping(RawOrigin::Signed(customer.clone()).into(), amount, eth_dest);

	}: _(RawOrigin::Signed(oracle), customer, amount, eth_dest, 2)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), 0u32.into());
		assert_eq!(<TotalRejected<T>>::get(), amount);
	}

	reject_reserve_fund {
		let b in 0 .. *T::BlockLength::get().max.get(DispatchClass::Normal) as u32;
		let reason = vec![1; b as usize];
		let amount: BalanceOf<T> = BalanceOf::<T>::one();
		let eth_dest = EthAddress::from(&[0;20]);
		let _ = Template::<T>::initiate_wrapping_reserve_fund(RawOrigin::Root.into(), amount, eth_dest);
	}: _(RawOrigin::Root, amount, eth_dest, reason)
	verify {
		assert_eq!(<TotalInitiated<T>>::get(), amount);
		assert_eq!(<TotalSettled<T>>::get(), 0u32.into());
		assert_eq!(<TotalRejected<T>>::get(), amount);
	}

	set_wrapping_limits {
		let min: BalanceOf<T> = 13u32.into();
		let max: BalanceOf<T> = 37u32.into();
	}: _(RawOrigin::Root, min, max)
	verify {
		assert_eq!(<CurrentMin<T>>::get(), Some(min));
		assert_eq!(<CurrentMax<T>>::get(), Some(max));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
