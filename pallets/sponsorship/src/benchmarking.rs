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

//! Benchmarking setup for pallet-sponsorship
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Sponsorship;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_pot() {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			remained_fee_quota: 5u32.into(),
			remained_reserve_quota: 7u32.into(),
		};
		#[extrinsic_call]
		create_pot(
			RawOrigin::Signed(caller),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.remained_fee_quota,
			pot_details.remained_reserve_quota,
		);

		assert_eq!(Pot::<T>::get(pot), Some(pot_details));
	}

	#[benchmark]
	fn remove_pot() {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			remained_fee_quota: 5u32.into(),
			remained_reserve_quota: 7u32.into(),
		};
		Pot::<T>::insert(pot, pot_details.clone());

		#[extrinsic_call]
		remove_pot(RawOrigin::Signed(caller.clone()), pot);

		assert_eq!(Pot::<T>::get(pot), None);
	}

	#[benchmark]
	fn register_users(l: Linear<1, 1_000>) {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let users: Vec<T::AccountId> = (0..l).map(|i| account("user", i, SEED)).collect();

		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			remained_fee_quota: 5u32.into(),
			remained_reserve_quota: 7u32.into(),
		};
		Pot::<T>::insert(pot, pot_details.clone());

		#[extrinsic_call]
		register_users(RawOrigin::Signed(caller), pot, users, 8u32.into(), 19u32.into());

		assert_eq!(User::<T>::iter_prefix_values(pot).count() as u32, l);
	}

	impl_benchmark_test_suite!(Sponsorship, crate::mock::new_test_ext(), crate::mock::Test);
}
