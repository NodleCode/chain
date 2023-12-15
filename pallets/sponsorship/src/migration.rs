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

use crate::{
	weights::WeightInfo, BalanceOf, Config, Pallet, Pot, PotDetailsOf, PotMigrationCursor, User, UserDetailsOf,
	UserMigrationCursor, UserRegistrationCount,
};
use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::*,
	storage::generator::{StorageDoubleMap, StorageMap},
	traits::{Get, StorageVersion},
	weights::Weight,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::{traits::Zero, Perbill};
use sp_std::vec::Vec;
use support::LimitedBalance;

/// The current storage version.
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

pub(crate) mod v0 {
	use super::{Pot as V1Pot, PotDetailsOf as V1PotDetailsOf, User as V1User, UserDetailsOf as V1UserDetailsOf, *};
	use frame_support::storage_alias;
	use sp_runtime::traits::Saturating;

	#[derive(Encode, Decode, Debug)]
	pub struct PotDetails<AccountId, SponsorshipType, Balance: frame_support::traits::tokens::Balance> {
		pub sponsor: AccountId,
		pub sponsorship_type: SponsorshipType,
		pub fee_quota: LimitedBalance<Balance>,
		pub reserve_quota: LimitedBalance<Balance>,
	}

	#[derive(Encode, Decode, Debug)]
	pub struct UserDetails<AccountId, Balance: frame_support::traits::tokens::Balance> {
		pub proxy: AccountId,
		pub fee_quota: LimitedBalance<Balance>,
		pub reserve_quota: LimitedBalance<Balance>,
	}

	pub type PotDetailsOf<T> =
		PotDetails<<T as frame_system::Config>::AccountId, <T as Config>::SponsorshipType, BalanceOf<T>>;
	pub type UserDetailsOf<T> = UserDetails<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

	#[storage_alias]
	/// Details of a pot.
	pub type Pot<T: Config> =
		StorageMap<Pallet<T>, Blake2_128Concat, <T as Config>::PotId, PotDetailsOf<T>, OptionQuery>;

	#[storage_alias]
	/// User details of a pot.
	pub type User<T: Config> = StorageDoubleMap<
		Pallet<T>,
		Blake2_128Concat,
		<T as Config>::PotId,
		Blake2_128Concat,
		<T as frame_system::Config>::AccountId,
		UserDetailsOf<T>,
		OptionQuery,
	>;

	pub const BLOCK_PERCENT_USAGE: u32 = 50;

	pub fn migrate_pots<T: Config>(max_pots: usize, starting_key: Vec<u8>) -> (Option<Vec<u8>>, Weight) {
		let mut iter = Pot::<T>::iter_from(starting_key);

		let pots = iter
			.by_ref()
			.take(max_pots)
			.map(|(pot, details)| {
				(
					pot,
					V1PotDetailsOf::<T> {
						sponsor: details.sponsor,
						sponsorship_type: details.sponsorship_type,
						fee_quota: details.fee_quota,
						reserve_quota: details.reserve_quota,
						deposit: Zero::zero(),
					},
				)
			})
			.collect::<Vec<_>>();

		let num_of_pots = pots.len();

		pots.into_iter()
			.for_each(|(pot, details)| V1Pot::<T>::insert(pot, details));

		log::info!(target: "sponsorship", "migrated {} pots", num_of_pots);

		let weight = T::WeightInfo::migrate_pots(num_of_pots as u32);
		if num_of_pots == max_pots {
			(Some(iter.last_raw_key().to_vec()), weight)
		} else {
			(None, weight)
		}
	}

	pub fn migrate_users<T: Config>(max_users: usize, starting_key: Vec<u8>) -> (Option<Vec<u8>>, Weight) {
		let mut iter = User::<T>::iter_from(starting_key);

		let users = iter
			.by_ref()
			.take(max_users)
			.map(|(pot, user, details)| {
				(
					pot,
					user,
					V1UserDetailsOf::<T> {
						proxy: details.proxy,
						fee_quota: details.fee_quota,
						reserve_quota: details.reserve_quota,
						deposit: Zero::zero(),
					},
				)
			})
			.collect::<Vec<_>>();

		let users_len = users.len();

		users.into_iter().for_each(|(pot, user, details)| {
			UserRegistrationCount::<T>::mutate(&user, |count| {
				count.saturating_inc();
			});
			V1User::<T>::insert(pot, user, details);
		});

		log::info!(target: "sponsorship", "migrated {} user-in-pots", users_len);

		let weight = T::WeightInfo::migrate_users(users_len as u32);
		if users_len == max_users {
			(Some(iter.last_raw_key().to_vec()), weight)
		} else {
			(None, weight)
		}
	}

	pub fn migrate_limited<T: Config>(max_weight: Weight) -> Weight {
		let mut weight: Weight = Zero::zero();

		loop {
			weight += min_weight::<T>();

			let max_pots = max_weight
				.saturating_sub(weight)
				.ref_time()
				.checked_div(T::WeightInfo::migrate_pots(1).ref_time())
				.unwrap_or(1) as usize;
			if max_pots == 0 {
				break;
			}

			let pot_migration_in_progress = if let Some(starting_key) = PotMigrationCursor::<T>::get() {
				let (end_cursor, migration_weight) = migrate_pots::<T>(max_pots, starting_key);
				weight += migration_weight + T::DbWeight::get().writes(1);
				match end_cursor {
					Some(last_key) => {
						PotMigrationCursor::<T>::put(last_key);
						true
					}
					None => {
						PotMigrationCursor::<T>::kill();
						false
					}
				}
			} else {
				false
			};

			let max_users = max_weight
				.saturating_sub(weight)
				.ref_time()
				.checked_div(T::WeightInfo::migrate_users(1).ref_time())
				.unwrap_or(1) as usize;
			if max_users == 0 {
				break;
			}

			let user_migration_in_progress = if let Some(starting_key) = UserMigrationCursor::<T>::get() {
				let (end_cursor, migration_weight) = migrate_users::<T>(max_users, starting_key);
				weight += migration_weight + T::DbWeight::get().writes(1);
				match end_cursor {
					Some(last_key) => {
						UserMigrationCursor::<T>::put(last_key);
						true
					}
					None => {
						UserMigrationCursor::<T>::kill();
						false
					}
				}
			} else {
				false
			};

			if !pot_migration_in_progress && !user_migration_in_progress {
				weight += T::DbWeight::get().writes(1);
				STORAGE_VERSION.put::<Pallet<T>>();
				break;
			}
		}

		weight
	}

	/// Return the minimum overhead of attempting to migrate the storage.
	pub fn min_weight<T: Config>() -> Weight {
		// 2 reads: PotMigrationCursor, UserMigrationCursor
		// Fixed: 40_000_000 as a pessimistic estimation for non benchmarked logic with trivial cost
		// during each loop of migrate_limited
		T::DbWeight::get()
			.reads(2)
			.saturating_add(Weight::from_parts(40_000_000_u64, 0))
	}

	/// Return the maximum overhead of attempting to migrate the storage.
	pub fn max_weight<T: Config>() -> Weight {
		T::BlockWeights::get().max_block * Perbill::from_percent(BLOCK_PERCENT_USAGE)
	}
}

/// Call this during on_initialize for the pallet.
pub fn on_initialize<T: Config>(_n: BlockNumberFor<T>) -> Weight {
	v0::migrate_limited::<T>(v0::max_weight::<T>())
}

/// Call this during the next runtime upgrade for this module.
pub fn on_runtime_upgrade<T: Config>() -> Weight {
	let mut weight: Weight = T::DbWeight::get().reads(1);

	if StorageVersion::get::<Pallet<T>>() == 0 {
		PotMigrationCursor::<T>::put(&Pot::<T>::prefix_hash());
		UserMigrationCursor::<T>::put(&User::<T>::prefix_hash());
		weight += T::DbWeight::get().reads_writes(2, 2);

		// The following invocation of migration is only needed for testing the logic during the
		// try runtime. The actual migration should be called during on_initialize for the pallet.
		#[cfg(feature = "try-runtime")]
		while StorageVersion::get::<Pallet<T>>() == 0 {
			weight += v0::migrate_limited::<T>(v0::max_weight::<T>());
		}
	}

	weight
}

#[cfg(feature = "try-runtime")]
use ::{
	frame_support::{Blake2_128Concat, StorageHasher},
	sp_runtime::TryRuntimeError,
	sp_std::borrow::Borrow,
};

#[cfg(feature = "try-runtime")]
type StorageDoubleMapKey = Vec<u8>;

#[cfg(feature = "try-runtime")]
pub(crate) fn pre_upgrade<T: Config>() -> Result<Vec<u8>, TryRuntimeError> {
	ensure!(
		StorageVersion::get::<Pallet<T>>() == 0,
		TryRuntimeError::Other("Storage version is not 0")
	);

	let block_usage = v0::max_weight::<T>();
	ensure!(
		block_usage.all_gt(v0::min_weight::<T>()),
		TryRuntimeError::Other("Block usage is set too low")
	);
	log::info!(target: "sponsorship", "pre_upgrade: block_usage = ({ref_time}, {proof_size})", ref_time=block_usage.ref_time(), proof_size=block_usage.proof_size());

	let max_pots_per_block = block_usage
		.saturating_sub(v0::min_weight::<T>())
		.ref_time()
		.checked_div(T::WeightInfo::migrate_pots(1).ref_time())
		.unwrap_or(1) as usize;
	let max_users_per_block = block_usage
		.saturating_sub(v0::min_weight::<T>())
		.ref_time()
		.checked_div(T::WeightInfo::migrate_users(1).ref_time())
		.unwrap_or(1) as usize;
	ensure!(
		max_pots_per_block > 0 && max_users_per_block > 0,
		TryRuntimeError::Other("Migration allowed weight is too low")
	);
	log::info!(target: "sponsorship", "pre_upgrade: max_pots_per_block = {max_pots_per_block}, max_users_per_block = {max_users_per_block}");

	let pot_details = frame_support::migration::storage_key_iter::<
		T::PotId,
		v0::PotDetailsOf<T>,
		frame_support::Blake2_128Concat,
	>(Pot::<T>::module_prefix(), Pot::<T>::storage_prefix())
	.collect::<Vec<_>>();
	let user_details = frame_support::migration::storage_iter::<v0::UserDetailsOf<T>>(
		User::<T>::module_prefix(),
		User::<T>::storage_prefix(),
	)
	.collect::<Vec<_>>();
	log::info!(target: "sponsorship", "pre_upgrade: pots = {pot_details_len}, users = {user_details_len}", pot_details_len = pot_details.len(), user_details_len = user_details.len());

	let total_consumed_weight = v0::min_weight::<T>()
		+ T::WeightInfo::migrate_pots(pot_details.len() as u32)
		+ T::WeightInfo::migrate_users(user_details.len() as u32)
		+ T::DbWeight::get().writes(1);
	let blocks = total_consumed_weight
		.ref_time()
		.checked_div(block_usage.ref_time())
		.ok_or("Unable to calculate blocks")?
		+ 1;
	log::info!(target: "sponsorship", "pre_upgrade: total_consumed_weight = ({ref_time}, {proof_size}), blocks = {blocks:?}", ref_time=total_consumed_weight.ref_time(), proof_size=total_consumed_weight.proof_size());

	Ok((pot_details, user_details).encode())
}

#[cfg(feature = "try-runtime")]
pub(crate) fn post_upgrade<T: Config>(state: Vec<u8>) -> Result<(), TryRuntimeError> {
	ensure!(
		StorageVersion::get::<Pallet<T>>() == 1,
		TryRuntimeError::Other("Storage version not fixed")
	);

	let (pre_pot_details, pre_user_details): (
		Vec<(T::PotId, v0::PotDetailsOf<T>)>,
		Vec<(StorageDoubleMapKey, v0::UserDetailsOf<T>)>,
	) = Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;
	let pot_details = Pot::<T>::iter().collect::<Vec<_>>();

	ensure!(
		pre_pot_details.len() == pot_details.len(),
		TryRuntimeError::Other("Pot count mismatch")
	);

	for (pre, post) in pre_pot_details.iter().zip(pot_details.iter()) {
		ensure!(pre.0 == post.0, TryRuntimeError::Other("Pot id mismatch"));
		ensure!(
			pre.1.sponsor == post.1.sponsor,
			TryRuntimeError::Other("Pot sponsor mismatch")
		);
		ensure!(
			pre.1.sponsorship_type == post.1.sponsorship_type,
			TryRuntimeError::Other("Pot sponsorship type mismatch")
		);
		ensure!(
			pre.1.fee_quota == post.1.fee_quota,
			TryRuntimeError::Other("Pot fee quota mismatch")
		);
		ensure!(
			pre.1.reserve_quota == post.1.reserve_quota,
			TryRuntimeError::Other("Pot reserve quota mismatch")
		);
		ensure!(
			post.1.deposit == Default::default(),
			TryRuntimeError::Other("Pot deposit is not default")
		);
	}

	let user_details = User::<T>::iter().collect::<Vec<_>>();
	ensure!(
		pre_user_details.len() == user_details.len(),
		TryRuntimeError::Other("User count mismatch")
	);

	for (pre, post) in pre_user_details.iter().zip(user_details.iter()) {
		let key1_hashed = post.0.borrow().using_encoded(Blake2_128Concat::hash);
		let key2_hashed = post.1.borrow().using_encoded(Blake2_128Concat::hash);
		let mut final_key = Vec::new();
		final_key.extend_from_slice(key1_hashed.as_ref());
		final_key.extend_from_slice(key2_hashed.as_ref());

		ensure!(pre.0 == final_key, TryRuntimeError::Other("User key mismatch"));
		ensure!(
			pre.1.proxy == post.2.proxy,
			TryRuntimeError::Other("User proxy mismatch")
		);
		ensure!(
			pre.1.fee_quota == post.2.fee_quota,
			TryRuntimeError::Other("User fee quota mismatch")
		);
		ensure!(
			pre.1.reserve_quota == post.2.reserve_quota,
			TryRuntimeError::Other("User reserve quota mismatch")
		);
		ensure!(
			post.2.deposit == Default::default(),
			TryRuntimeError::Other("User deposit is not default")
		);
	}

	UserRegistrationCount::<T>::iter().try_for_each(|(_user, count)| {
		ensure!(count > 0, TryRuntimeError::Other("User registration count is 0"));
		ensure!(
			count <= pot_details.len() as u32,
			TryRuntimeError::Other("User registration count is greater than number of pots")
		);
		Ok::<(), TryRuntimeError>(())
	})?;

	log::info!(target: "sponsorship", "post_upgrade: pots = {}, pot_user_count = {}, users = {}", pot_details.len(), user_details.len(), UserRegistrationCount::<T>::iter().count());
	Ok(())
}
