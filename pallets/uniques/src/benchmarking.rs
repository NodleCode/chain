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
use frame_support::{
	dispatch::UnfilteredDispatchable,
	traits::{EnsureOrigin, Get},
	BoundedVec,
};
use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Pallet as Uniques;
const SEED: u32 = 0;

fn get_config<T: Config<I>, I: 'static>() -> (T::CollectionId, T::AccountId, AccountIdLookupOf<T>) {
	let collection_owner: T::AccountId = account("colletction_owner", 0, SEED);
	let collection_owner_lookup = T::Lookup::unlookup(collection_owner.clone());
	let collection_id = T::Helper::collection(0);
	(collection_id, collection_owner, collection_owner_lookup)
}
fn create_collection<T: Config<I>, I: 'static>() -> (T::CollectionId, T::AccountId, AccountIdLookupOf<T>) {
	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();
	T::Currency::make_free_balance_be(&collection_owner, BalanceOf::<T, I>::max_value());
	assert!(Uniques::<T, I>::force_create(
		SystemOrigin::Root.into(),
		collection_id.clone(),
		collection_owner_lookup.clone(),
		false,
	)
	.is_ok());
	(collection_id, collection_owner, collection_owner_lookup)
}
fn add_collection_metadata<T: Config<I>, I: 'static>() -> (T::AccountId, AccountIdLookupOf<T>) {
	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();

	let caller = collection_owner;
	let caller_lookup = collection_owner_lookup;
	assert!(Uniques::<T, I>::set_collection_metadata(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
		false,
	)
	.is_ok());
	(caller, caller_lookup)
}
// fn add_item_metadata<T: Config<I>, I: 'static>(item: T::ItemId) -> (T::AccountId, AccountIdLookupOf<T>) {
// 	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();

// 	let caller = collection_owner;
// 	let caller_lookup = collection_owner_lookup;
// 	assert!(Uniques::<T, I>::set_metadata(
// 		SystemOrigin::Signed(caller.clone()).into(),
// 		T::Helper::collection(0),
// 		item,
// 		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
// 		false,
// 	)
// 	.is_ok());
// 	(caller, caller_lookup)
// }
// fn add_item_attribute<T: Config<I>, I: 'static>(
// 	item: T::ItemId,
// ) -> (BoundedVec<u8, T::KeyLimit>, T::AccountId, AccountIdLookupOf<T>) {
// 	let (collection_id, collection_owner, collection_owner_lookup) = get_config::<T, I>();

// 	let caller = collection_owner;
// 	let caller_lookup = collection_owner_lookup;
// 	let caller_lookup = T::Lookup::unlookup(caller.clone());
// 	let key: BoundedVec<_, _> = vec![0; T::KeyLimit::get() as usize].try_into().unwrap();
// 	assert!(Uniques::<T, I>::set_attribute(
// 		SystemOrigin::Signed(caller.clone()).into(),
// 		T::Helper::collection(0),
// 		Some(item),
// 		key.clone(),
// 		vec![0; T::ValueLimit::get() as usize].try_into().unwrap(),
// 	)
// 	.is_ok());
// 	(key, caller, caller_lookup)
// }
fn mint_item_with_extra_deposit<T: Config<I>, I: 'static>(
	index: u16,
) -> (T::ItemId, T::AccountId, AccountIdLookupOf<T>) {
	let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T, I>();
	let item = T::Helper::item(index);
	assert!(Uniques::<T, I>::mint_with_extra_deposit(
		SystemOrigin::Signed(collection_owner.clone()).into(),
		collection_id,
		item,
		collection_owner_lookup.clone(),
		index.into(),
	)
	.is_ok());
	(item, collection_owner, collection_owner_lookup)
}

benchmarks_instance_pallet! {

	// destroy {
	// 	let n in 0 .. 1_000;
	// 	let m in 0 .. 1_000;
	// 	let a in 0 .. 1_000;

	// 	let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T, I>();
	// 	add_collection_metadata::<T, I>();
	// 	for i in 0..n {
	// 		mint_item_with_extra_deposit::<T, I>(i as u16);
	// 	}
	// 	for i in 0..m {
	// 		add_item_metadata::<T, I>(T::Helper::item(i as u16));
	// 	}
	// 	for i in 0..a {
	// 		add_item_attribute::<T, I>(T::Helper::item(i as u16));
	// 	}
	// 	let witness = Uniques2::Collection::<T, I>::get(collection.clone()).unwrap().destroy_witness();
	// }: _(SystemOrigin::Signed(collection_owner), collection_id, witness)

	mint_with_extra_deposit {
		let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T, I>();
		let item = T::Helper::item(0);
		let deposit = BalanceOf::<T,I>::max_value();
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id.clone(), item, collection_owner_lookup, deposit)


	burn {
		let (collection_id, collection_owner, collection_owner_lookup) = create_collection::<T,I>();
		let (item, ..) = mint_item_with_extra_deposit::<T, I>(0);
	}: _(SystemOrigin::Signed(collection_owner.clone()), collection_id.clone(), item, Some(collection_owner_lookup))




	impl_benchmark_test_suite!(Uniques, crate::tests::new_test_ext(), crate::tests::Test);
}
