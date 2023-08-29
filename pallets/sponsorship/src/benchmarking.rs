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
use frame_support::assert_ok;
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
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		#[extrinsic_call]
		create_pot(
			RawOrigin::Signed(caller),
			pot,
			pot_details.sponsorship_type.clone(),
			pot_details.fee_quota.limit(),
			pot_details.reserve_quota.limit(),
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
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		#[extrinsic_call]
		remove_pot(RawOrigin::Signed(caller), pot);

		assert_eq!(Pot::<T>::get(pot), None);
	}

	#[benchmark]
	fn update_pot_limits() {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		#[extrinsic_call]
		update_pot_limits(RawOrigin::Signed(caller), pot, 8u32.into(), 19u32.into());

		let updated_pot = Pot::<T>::get(pot).unwrap();
		assert_eq!(updated_pot.fee_quota.limit(), 8u32.into());
		assert_eq!(updated_pot.reserve_quota.limit(), 19u32.into());
	}

	#[benchmark]
	fn update_sponsorship_type() {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		#[extrinsic_call]
		update_sponsorship_type(RawOrigin::Signed(caller), pot, T::SponsorshipType::default());

		let updated_pot = Pot::<T>::get(pot).unwrap();
		assert_eq!(updated_pot.fee_quota.limit(), 5u32.into());
		assert_eq!(updated_pot.reserve_quota.limit(), 7u32.into());
	}

	#[benchmark]
	fn register_users(l: Linear<1, 1_000>) {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let users: Vec<T::AccountId> = (0..l).map(|i| account("user", i, SEED)).collect();

		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		#[extrinsic_call]
		register_users(RawOrigin::Signed(caller), pot, users, 8u32.into(), 19u32.into());

		assert_eq!(User::<T>::iter_prefix_values(pot).count() as u32, l);
	}

	#[benchmark]
	fn remove_users(l: Linear<1, 1_000>) {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let users: Vec<T::AccountId> = (0..l).map(|i| account("user", i, SEED)).collect();

		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		assert_ok!(Pallet::<T>::register_users(
			RawOrigin::Signed(caller.clone()).into(),
			pot,
			users.clone(),
			5u32.into(),
			11u32.into(),
		),);

		let user_free_balance = T::Currency::minimum_balance() * 100u32.into();
		for user in &users {
			let user_detail = User::<T>::get(pot, user).unwrap();
			T::Currency::make_free_balance_be(&user_detail.proxy, user_free_balance);
		}

		#[extrinsic_call]
		remove_users(RawOrigin::Signed(caller), pot, users.clone());

		assert_eq!(User::<T>::iter_prefix_values(pot).count() as u32, 0);
		for user in &users {
			assert_eq!(T::Currency::free_balance(user), user_free_balance);
		}
	}

	#[benchmark]
	fn update_users_limits(l: Linear<1, 1_000>) {
		let caller: T::AccountId = whitelisted_caller();
		let pot = 0u32.into();
		let users: Vec<T::AccountId> = (0..l).map(|i| account("user", i, SEED)).collect();

		let pot_details = PotDetailsOf::<T> {
			sponsor: caller.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(5u32.into()),
			reserve_quota: LimitedBalance::with_limit(7u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		assert_ok!(Pallet::<T>::register_users(
			RawOrigin::Signed(caller.clone()).into(),
			pot,
			users.clone(),
			5u32.into(),
			11u32.into(),
		),);

		#[extrinsic_call]
		update_users_limits(RawOrigin::Signed(caller), pot, 7u32.into(), 13u32.into(), users.clone());

		for user in users {
			let user_detail = User::<T>::get(pot, user).unwrap();
			assert_eq!(user_detail.fee_quota.limit(), 7u32.into());
			assert_eq!(user_detail.reserve_quota.limit(), 13u32.into());
		}
	}

	#[benchmark]
	fn pre_sponsor() {
		let sponsor: T::AccountId = account("sponsor", 0, SEED);
		let pot = 11u32.into();
		let user: T::AccountId = account("user", 0, SEED);

		let sponsor_free_balance = T::Currency::minimum_balance() * 18_000_000u32.into();
		T::Currency::make_free_balance_be(&sponsor, sponsor_free_balance);

		let pot_details = PotDetailsOf::<T> {
			sponsor: sponsor.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(T::Currency::minimum_balance() * 5_000_000u32.into()),
			reserve_quota: LimitedBalance::with_limit(T::Currency::minimum_balance() * 13_000_000u32.into()),
		};
		Pot::<T>::insert(pot, pot_details);

		assert_ok!(Pallet::<T>::register_users(
			RawOrigin::Signed(sponsor).into(),
			pot,
			vec![user.clone()],
			T::Currency::minimum_balance() * 5_000u32.into(),
			T::Currency::minimum_balance() * 13_000u32.into(),
		),);

		#[block]
		{
			assert!(pallet::Pallet::<T>::pre_sponsor_for(user, pot).is_ok());
		}
	}

	#[benchmark]
	fn post_sponsor() {
		let sponsor: T::AccountId = account("sponsor", 0, SEED);
		let pot = 11u32.into();
		let user: T::AccountId = account("user", 0, SEED);

		let sponsor_free_balance = T::Currency::minimum_balance() * 18_000_000u32.into();
		T::Currency::make_free_balance_be(&sponsor, sponsor_free_balance);

		let mut pot_details = PotDetailsOf::<T> {
			sponsor: sponsor.clone(),
			sponsorship_type: T::SponsorshipType::default(),
			fee_quota: LimitedBalance::with_limit(T::Currency::minimum_balance() * 5_000_000u32.into()),
			reserve_quota: LimitedBalance::with_limit(T::Currency::minimum_balance() * 13_000_000u32.into()),
		};
		Pot::<T>::insert(pot, pot_details.clone());

		assert_ok!(Pallet::<T>::register_users(
			RawOrigin::Signed(sponsor).into(),
			pot,
			vec![user.clone()],
			T::Currency::minimum_balance() * 5_000u32.into(),
			T::Currency::minimum_balance() * 13_000u32.into(),
		),);

		let mut user_details = User::<T>::get(pot, &user).unwrap();

		let paid = T::Currency::minimum_balance() * 12_000u32.into();
		pot_details.reserve_quota.saturating_add(paid);
		user_details.reserve_quota.saturating_add(paid);

		let proxy_balance = T::Currency::minimum_balance() * 11_000u32.into();
		let new_proxy_balance = T::Currency::minimum_balance() * 12_000u32.into();
		T::Currency::make_free_balance_be(&user_details.proxy, new_proxy_balance);

		#[block]
		{
			assert_ok!(pallet::Pallet::<T>::post_sponsor_for(
				user.clone(),
				pot,
				pot_details,
				user_details,
				paid,
				proxy_balance
			));
		}

		let user_detail = User::<T>::get(pot, &user).unwrap();
		assert_eq!(user_detail.reserve_quota.balance(), T::Currency::minimum_balance());
	}

	impl_benchmark_test_suite!(Sponsorship, crate::mock::new_test_ext(), crate::mock::Test);
}
