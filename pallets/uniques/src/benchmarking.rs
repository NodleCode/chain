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

//! Nodle uniques pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v1::{account, benchmarks_instance_pallet};
use frame_support::{assert_ok, traits::Get, BoundedVec};
use frame_system::RawOrigin as SystemOrigin;
use pallet_uniques::{BenchmarkHelper, DestroyWitness};
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Pallet as Uniques;
const SEED: u32 = 0;

fn get_config<T: Config<I>, I: 'static>() -> (T::CollectionId, T::AccountId, AccountIdLookupOf<T>) {
	let collection_owner: T::AccountId = account("collection_owner", 0, SEED);
	let collection_owner_lookup = T::Lookup::unlookup(collection_owner.clone());
	let collection_id = <T as pallet_uniques::Config<I>>::Helper::collection(0);
	(collection_id, collection_owner, collection_owner_lookup)
}

fn create_collection<T: Config<I>, I: 'static>(
	extra_deposit_limit: BalanceOf<T, I>,
) -> (T::CollectionId, T::AccountId, AccountIdLookupOf<T>) {
	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();
	T::Currency::make_free_balance_be(&collection_owner, BalanceOf::<T, I>::max_value());
	assert_ok!(Uniques::<T, I>::create_with_extra_deposit_limit(
		SystemOrigin::Signed(collection_owner.clone()).into(),
		collection_id,
		collection_owner_lookup.clone(),
		extra_deposit_limit
	));
	(collection_id, collection_owner, collection_owner_lookup)
}
fn add_collection_metadata<T: Config<I>, I: 'static>() -> (T::AccountId, AccountIdLookupOf<T>) {
	let (.., collection_owner, collection_owner_lookup) = get_config::<T, I>();

	let caller = collection_owner;
	let caller_lookup = collection_owner_lookup;
	assert!(Uniques::<T, I>::set_collection_metadata(
		SystemOrigin::Signed(caller.clone()).into(),
		<T as pallet_uniques::Config<I>>::Helper::collection(0),
		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
		false,
	)
	.is_ok());
	(caller, caller_lookup)
}
fn add_item_metadata<T: Config<I>, I: 'static>(item: T::ItemId) -> (T::AccountId, AccountIdLookupOf<T>) {
	let (.., collection_owner, collection_owner_lookup) = get_config::<T, I>();

	let caller = collection_owner;
	let caller_lookup = collection_owner_lookup;
	assert!(Uniques::<T, I>::set_metadata(
		SystemOrigin::Signed(caller.clone()).into(),
		<T as pallet_uniques::Config<I>>::Helper::collection(0),
		item,
		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
		false,
	)
	.is_ok());
	(caller, caller_lookup)
}
fn add_item_attribute<T: Config<I>, I: 'static>(
	item: T::ItemId,
) -> (BoundedVec<u8, T::KeyLimit>, T::AccountId, AccountIdLookupOf<T>) {
	let (.., collection_owner, collection_owner_lookup) = get_config::<T, I>();

	let key: BoundedVec<_, _> = vec![0; T::KeyLimit::get() as usize].try_into().unwrap();
	assert!(Uniques::<T, I>::set_attribute(
		SystemOrigin::Signed(collection_owner.clone()).into(),
		<T as pallet_uniques::Config<I>>::Helper::collection(0),
		Some(item),
		key.clone(),
		vec![0; T::ValueLimit::get() as usize].try_into().unwrap(),
	)
	.is_ok());
	(key, collection_owner, collection_owner_lookup)
}
fn mint_item_with_extra_deposit<T: Config<I>, I: 'static>(
	index: u16,
	deposit: BalanceOf<T, I>,
) -> (T::ItemId, T::AccountId, AccountIdLookupOf<T>) {
	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();
	let item = T::Helper::item(index);
	assert!(Uniques::<T, I>::mint_with_extra_deposit(
		SystemOrigin::Signed(collection_owner.clone()).into(),
		collection_id,
		item,
		collection_owner_lookup.clone(),
		deposit,
	)
	.is_ok());
	(item, collection_owner, collection_owner_lookup)
}

fn assert_last_event<T: Config<I>, I: 'static>(system_event: <T as frame_system::Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks_instance_pallet! {
	destroy {
		let n in 0 .. 1_000;
		let m in 0 .. 1_000;
		let a in 0 .. 1_000;

		let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T, I>(BalanceOf::<T, I>::max_value());

		add_collection_metadata::<T, I>();

		for i in 0..n {
			mint_item_with_extra_deposit::<T, I>(i as u16, T::Currency::minimum_balance());
		}
		for i in 0..m {
			add_item_metadata::<T, I>(T::Helper::item(i as u16));
		}
		for i in 0..a {
			add_item_attribute::<T, I>(T::Helper::item(i as u16));
		}
		let witness = DestroyWitness{
			items: n,
			item_metadatas: m,
			attributes: a,
		};
	}: _(SystemOrigin::Signed(collection_owner), collection_id, witness)

	mint_with_extra_deposit {
		let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T, I>(BalanceOf::<T, I>::max_value());
		let item = T::Helper::item(0);
		let deposit = 5u32.into();
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id, item, collection_owner_lookup, deposit)

	burn {
		let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T,I>(BalanceOf::<T, I>::max_value());
		let (item, ..) = mint_item_with_extra_deposit::<T, I>(0, T::Currency::minimum_balance());
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id, item, Some(collection_owner_lookup))

	create_with_extra_deposit_limit {
		let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();
		T::Currency::make_free_balance_be(&collection_owner, BalanceOf::<T, I>::max_value());
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id, collection_owner_lookup, BalanceOf::<T, I>::max_value())
	verify {
		let event: <T as pallet_uniques::Config<I>>::RuntimeEvent = pallet_uniques::Event::Created { collection: <T as pallet_uniques::Config<I>>::Helper::collection(0), creator: collection_owner.clone(), owner: collection_owner }.into();
		assert_last_event::<T, I>(event.into());
	}

	transfer_ownership {
		let (collection, collection_owner, _) = create_collection::<T, I>(BalanceOf::<T, I>::max_value());
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		T::Currency::make_free_balance_be(&target, T::Currency::minimum_balance());
		let origin = SystemOrigin::Signed(target.clone()).into();
		Uniques::<T, I>::set_accept_ownership(origin, Some(collection))?;
	}: _(SystemOrigin::Signed(collection_owner), collection, target_lookup)
	verify {
		let event: <T as pallet_uniques::Config<I>>::RuntimeEvent = pallet_uniques::Event::OwnerChanged { collection, new_owner: target }.into();
		assert_last_event::<T, I>(event.into());
	}

	update_extra_deposit_limit {
		let (collection, collection_owner, _) = create_collection::<T, I>(BalanceOf::<T, I>::max_value());
	}: _(SystemOrigin::Signed(collection_owner), collection, BalanceOf::<T, I>::min_value())
	verify {
		let event: <T as Config<I>>::RuntimeEvent = Event::ExtraDepositLimitUpdated { collection, limit: BalanceOf::<T, I>::min_value() }.into();
		assert_last_event::<T, I>(event.into());
	}

	impl_benchmark_test_suite!(Uniques, crate::tests::new_test_ext(), crate::tests::Test);
}
