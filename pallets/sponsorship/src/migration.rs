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

use crate::weights::WeightInfo;
use crate::{
	BalanceOf, Config, Pallet, Pot, PotDetailsOf, PotMigrationCursor, PotUserMigrationPerBlock, User, UserDetailsOf,
	UserMigrationCursor, UserRegistrationCount,
};
use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::*,
	storage::generator::{StorageDoubleMap, StorageMap},
	traits::{Get, StorageVersion},
	weights::Weight,
};
use sp_runtime::Perbill;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

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
						deposit: Default::default(),
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
						deposit: Default::default(),
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

	pub fn migrate_limited<T: Config>(max_pots: usize, max_users: usize) -> Weight {
		let mut weight: Weight = T::DbWeight::get().reads(2);

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
		}

		weight
	}

	/// Return the optimum number of pots and users that can be migrated according to the given
	/// `max_weight`. It will never return 0 for either pots or users because it would then stall
	/// the migration process.
	pub fn determine_optimum_pots_users<T: Config>(max_weight: Weight) -> (usize, usize) {
		let pots_count = v0::Pot::<T>::iter().count();
		let users_count = v0::User::<T>::iter().count();

		let one_pot_migration_weight = T::WeightInfo::migrate_pots(1).ref_time();
		let one_user_migration_weight = T::WeightInfo::migrate_users(1).ref_time();

		let pots_migration_weight = one_pot_migration_weight.saturating_mul(pots_count as u64);
		let users_migration_weight = one_user_migration_weight.saturating_mul(users_count as u64);

		let total_migration_weight = pots_migration_weight.saturating_add(users_migration_weight);

		let pots_assigning_ref_time = max_weight
			.ref_time()
			.saturating_mul(pots_migration_weight)
			.checked_div(total_migration_weight);
		let max_pots = pots_assigning_ref_time
			.map(|ref_time| ref_time.checked_div(one_pot_migration_weight).unwrap_or(1))
			.unwrap_or(1) as usize;

		let users_assigning_ref_time = max_weight
			.ref_time()
			.saturating_mul(users_migration_weight)
			.checked_div(total_migration_weight);
		let max_users = users_assigning_ref_time
			.map(|ref_time| ref_time.checked_div(one_user_migration_weight).unwrap_or(1))
			.unwrap_or(1) as usize;

		(max_pots.max(1), max_users.max(1))
	}

	#[cfg(feature = "try-runtime")]
	pub fn migrate_all_in_steps<T: Config>(steps: usize) -> Weight {
		let num_of_pots = Pot::<T>::iter().count();
		let num_of_users = User::<T>::iter().count();

		let mut weight: Weight = T::DbWeight::get().reads(num_of_pots as u64 + num_of_users as u64);

		for _ in 0..steps {
			weight += migrate_limited::<T>(num_of_pots / steps + 1, num_of_users / steps + 1);
		}

		weight
	}
}

/// Call this during the next runtime upgrade for this module.
pub fn on_runtime_upgrade<T: Config>() -> Weight {
	let mut weight: Weight = T::DbWeight::get().reads(1);

	if StorageVersion::get::<Pallet<T>>() == 0 {
		PotMigrationCursor::<T>::put(Pot::<T>::prefix_hash());
		UserMigrationCursor::<T>::put(User::<T>::prefix_hash());
		weight += T::DbWeight::get().reads_writes(2, 2);

		const MIGRATION_TO_MAX_BLOCK_RATIO: Perbill = Perbill::from_percent(50);
		let (max_pots, max_users) =
			v0::determine_optimum_pots_users::<T>(T::BlockWeights::get().max_block * MIGRATION_TO_MAX_BLOCK_RATIO);
		weight += T::WeightInfo::determine_optimum_pots_users(max_pots as u32, max_users as u32);

		PotUserMigrationPerBlock::<T>::put((max_pots as u32, max_users as u32));
		weight += T::DbWeight::get().writes(1);

		// The following invocation of migration is only needed for testing the logic during the
		// try runtime. The actual migration should be called during on_initialize for the pallet.
		#[cfg(feature = "try-runtime")]
		{
			weight += v0::migrate_all_in_steps::<T>(3);
		}
	}

	weight
}

#[cfg(feature = "try-runtime")]
use ::{
	frame_support::{Blake2_128Concat, StorageHasher},
	sp_std::borrow::Borrow,
};

#[cfg(feature = "try-runtime")]
type StorageDoubleMapKey = Vec<u8>;

#[cfg(feature = "try-runtime")]
pub(crate) fn pre_upgrade<T: Config>() -> Result<Vec<u8>, TryRuntimeError> {
	if StorageVersion::get::<Pallet<T>>() > 1 {
		return Err(TryRuntimeError::Other("Storage version is not either 0 or 1"));
	}

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

	log::info!(target: "sponsorship", "pre: pots = {}, users = {}", pot_details.len(), user_details.len());
	Ok((pot_details, user_details).encode())
}

#[cfg(feature = "try-runtime")]
pub(crate) fn post_upgrade<T: Config>(state: Vec<u8>) -> Result<(), TryRuntimeError> {
	if StorageVersion::get::<Pallet<T>>() != 1 {
		return Err(TryRuntimeError::Other("Storage version is not 1"));
	}

	let (pre_pot_details, pre_user_details): (
		Vec<(T::PotId, v0::PotDetailsOf<T>)>,
		Vec<(StorageDoubleMapKey, v0::UserDetailsOf<T>)>,
	) = Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;
	let pot_details = Pot::<T>::iter().collect::<Vec<_>>();

	if pre_pot_details.len() != pot_details.len() {
		return Err(TryRuntimeError::Other("Pot count mismatch"));
	}

	for (pre, post) in pre_pot_details.iter().zip(pot_details.iter()) {
		if pre.0 != post.0 {
			return Err(TryRuntimeError::Other("Pot id mismatch"));
		}
		if pre.1.sponsor != post.1.sponsor {
			return Err(TryRuntimeError::Other("Pot sponsor mismatch"));
		}
		if pre.1.sponsorship_type != post.1.sponsorship_type {
			return Err(TryRuntimeError::Other("Pot sponsorship type mismatch"));
		}
		if pre.1.fee_quota != post.1.fee_quota {
			return Err(TryRuntimeError::Other("Pot fee quota mismatch"));
		}
		if pre.1.reserve_quota != post.1.reserve_quota {
			return Err(TryRuntimeError::Other("Pot reserve quota mismatch"));
		}
		if post.1.deposit != Default::default() {
			return Err(TryRuntimeError::Other("Pot deposit is not default"));
		}
	}

	let user_details = User::<T>::iter().collect::<Vec<_>>();

	if pre_user_details.len() != user_details.len() {
		return Err(TryRuntimeError::Other("User count mismatch"));
	}

	for (pre, post) in pre_user_details.iter().zip(user_details.iter()) {
		let key1_hashed = post.0.borrow().using_encoded(Blake2_128Concat::hash);
		let key2_hashed = post.1.borrow().using_encoded(Blake2_128Concat::hash);
		let mut final_key = Vec::new();
		final_key.extend_from_slice(key1_hashed.as_ref());
		final_key.extend_from_slice(key2_hashed.as_ref());

		if pre.0 != final_key {
			return Err(TryRuntimeError::Other("User hashed key mismatch"));
		}
		if pre.1.proxy != post.2.proxy {
			return Err(TryRuntimeError::Other("User proxy mismatch"));
		}
		if pre.1.fee_quota != post.2.fee_quota {
			return Err(TryRuntimeError::Other("User fee quota mismatch"));
		}
		if pre.1.reserve_quota != post.2.reserve_quota {
			return Err(TryRuntimeError::Other("User reserve quota mismatch"));
		}
		if post.2.deposit != Default::default() {
			return Err(TryRuntimeError::Other("User deposit is not default"));
		}
	}

	UserRegistrationCount::<T>::iter().try_for_each(|(_user, count)| {
		if count == 0 {
			return Err(TryRuntimeError::Other("User registration count is 0"));
		}
		if count > pot_details.len() as u32 {
			return Err(TryRuntimeError::Other(
				"User registration count is greater than number of pots",
			));
		}
		Ok(())
	})?;

	log::info!(target: "sponsorship", "post_upgrade: pots = {}, pot_user_count = {}, users = {}", pot_details.len(), user_details.len(), UserRegistrationCount::<T>::iter().count());
	Ok(())
}
