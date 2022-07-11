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

//! Amendments pallet benchmarks

use super::*;

use crate::{Pallet as Allocations, ValidatorSet};
use frame_benchmarking::impl_benchmark_test_suite;
use frame_benchmarking::{account, benchmarks};
use frame_support::{
	assert_ok,
	pallet_prelude::PhantomData,
	parameter_types,
	traits::{ConstU32, EnsureOrigin},
};
use frame_system::RawOrigin;
use pallet_membership::Pallet as Membership;
use sp_std::{prelude::*, str};

pub type MaxMembers = ConstU32<10>;

const MAX_BYTES: u32 = 1_024;
const SEED: u32 = 0;

pub struct BenchmarkConfig<T: Config> {
	grantee: T::AccountId,
	oracle: T::AccountId,
}

fn make_benchmark_config<T: Config>(u: u32) -> BenchmarkConfig<T> {
	let grantee: T::AccountId = account("grantee", u, SEED);
	let oracle: T::AccountId = account("oracle", u, SEED);

	// let add_origin = <T as pallet_membership::Config<I>>::AddOrigin::successful_origin();
	// assert_ok!(<Membership<T, I>>::add_member(add_origin, oracle.clone()));

	let mut members = <ValidatorSet<T>>::get();
	assert!(members.try_push(oracle.clone()).is_ok());
	<ValidatorSet<T>>::put(&members);

	let deposit_applying = T::ExistentialDeposit::get();

	T::Currency::make_free_balance_be(&grantee, deposit_applying);
	T::Currency::make_free_balance_be(&oracle, deposit_applying);

	BenchmarkConfig { grantee, oracle }
}

benchmarks! {
	allocate {
		let b in 1 .. MAX_BYTES;

		let config = make_benchmark_config::<T>(0);

	}: _(RawOrigin::Signed(config.oracle.clone()), config.grantee.clone(), 40000u32.into(), vec![1; b as usize])

	impl_benchmark_test_suite!(
		Allocations,
		crate::tests::new_test_ext(),
		crate::tests::Test,
	);
}
