// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Uniques pallet benchmarking.

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

fn assert_last_event<T: Config<I>, I: 'static>(generic_event: <T as pallet_uniques::Config<I>>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
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
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id, collection_owner_lookup.clone(), BalanceOf::<T, I>::max_value())
	verify {
		assert_last_event::<T, I>(pallet_uniques::Event::Created { collection: <T as pallet_uniques::Config<I>>::Helper::collection(0), creator: collection_owner.clone(), owner: collection_owner }.into());
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
		assert_last_event::<T, I>(pallet_uniques::Event::OwnerChanged { collection, new_owner: target }.into());
	}

	impl_benchmark_test_suite!(Uniques, crate::tests::new_test_ext(), crate::tests::Test);
}
