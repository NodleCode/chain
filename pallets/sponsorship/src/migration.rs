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

use crate::{BalanceOf, Config, Pallet, Pot, User, UserRegistrationCount};
use codec::{Decode, Encode};
use frame_support::{
	traits::{Get, StorageVersion},
	weights::Weight,
};
use support::LimitedBalance;

/// The current storage version.
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

mod v0 {
	use super::*;
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
}

/// Call this during the next runtime upgrade for this module.
pub fn on_runtime_upgrade<T: Config>() -> Weight {
	let mut weight: Weight = T::DbWeight::get().reads(1);

	let translate_pot_details = |pre: v0::PotDetailsOf<T>| -> crate::PotDetailsOf<T> {
		let mut fee_quota = LimitedBalance::with_limit(pre.fee_quota.limit());
		fee_quota.saturating_add(pre.fee_quota.balance());
		let mut reserve_quota = LimitedBalance::with_limit(pre.reserve_quota.limit());
		reserve_quota.saturating_add(pre.reserve_quota.balance());
		crate::PotDetailsOf::<T> {
			sponsor: pre.sponsor,
			sponsorship_type: pre.sponsorship_type,
			fee_quota,
			reserve_quota,
			deposit: Default::default(),
		}
	};
	let translate_user_details = |pre: v0::UserDetailsOf<T>| -> crate::UserDetailsOf<T> {
		let mut fee_quota = LimitedBalance::with_limit(pre.fee_quota.limit());
		fee_quota.saturating_add(pre.fee_quota.balance());
		let mut reserve_quota = LimitedBalance::with_limit(pre.reserve_quota.limit());
		reserve_quota.saturating_add(pre.reserve_quota.balance());
		crate::UserDetailsOf::<T> {
			proxy: pre.proxy,
			fee_quota,
			reserve_quota,
			deposit: Default::default(),
		}
	};

	if StorageVersion::get::<Pallet<T>>() == 0 {
		Pot::<T>::translate(|_key, pre| Some(translate_pot_details(pre)));
		let pot_count = Pot::<T>::iter().count() as u64;
		weight = weight.saturating_add(T::DbWeight::get().reads_writes(pot_count, pot_count));

		User::<T>::translate(|_key1, _key2, pre| Some(translate_user_details(pre)));
		let pot_user_count = User::<T>::iter().count() as u64;
		weight = weight.saturating_add(T::DbWeight::get().reads_writes(pot_user_count, pot_user_count));

		for (_pot, user, _details) in User::<T>::iter() {
			UserRegistrationCount::<T>::mutate(user, |count| *count += 1);
		}
		weight = weight.saturating_add(T::DbWeight::get().reads_writes(pot_user_count, pot_user_count));

		StorageVersion::new(1).put::<Pallet<T>>();
	}

	weight
}

#[cfg(feature = "try-runtime")]
use ::{
	frame_support::{
		storage::generator::{StorageDoubleMap, StorageMap},
		Blake2_128Concat, StorageHasher,
	},
	sp_std::{borrow::Borrow, vec::Vec},
};

#[cfg(feature = "try-runtime")]
type StorageDoubleMapKey = Vec<u8>;

#[cfg(feature = "try-runtime")]
pub(crate) fn pre_upgrade<T: Config>() -> Result<Vec<u8>, &'static str> {
	if StorageVersion::get::<Pallet<T>>() > 1 {
		return Err("Storage version is not either 0 or 1");
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
pub(crate) fn post_upgrade<T: Config>(state: Vec<u8>) -> Result<(), &'static str> {
	if StorageVersion::get::<Pallet<T>>() != 1 {
		return Err("Storage version is not 1");
	}

	let (pre_pot_details, pre_user_details): (
		Vec<(T::PotId, v0::PotDetailsOf<T>)>,
		Vec<(StorageDoubleMapKey, v0::UserDetailsOf<T>)>,
	) = Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;
	let pot_details = Pot::<T>::iter().collect::<Vec<_>>();

	if pre_pot_details.len() != pot_details.len() {
		return Err("Pot count mismatch");
	}

	for (pre, post) in pre_pot_details.iter().zip(pot_details.iter()) {
		if pre.0 != post.0 {
			return Err("Pot id mismatch");
		}
		if pre.1.sponsor != post.1.sponsor {
			return Err("Pot sponsor mismatch");
		}
		if pre.1.sponsorship_type != post.1.sponsorship_type {
			return Err("Pot sponsorship type mismatch");
		}
		if pre.1.fee_quota != post.1.fee_quota {
			return Err("Pot fee quota mismatch");
		}
		if pre.1.reserve_quota != post.1.reserve_quota {
			return Err("Pot reserve quota mismatch");
		}
		if post.1.deposit != Default::default() {
			return Err("Pot deposit is not default");
		}
	}

	let user_details = User::<T>::iter().collect::<Vec<_>>();

	if pre_user_details.len() != user_details.len() {
		return Err("User count mismatch");
	}

	for (pre, post) in pre_user_details.iter().zip(user_details.iter()) {
		let key1_hashed = post.0.borrow().using_encoded(Blake2_128Concat::hash);
		let key2_hashed = post.1.borrow().using_encoded(Blake2_128Concat::hash);
		let mut final_key = Vec::new();
		final_key.extend_from_slice(key1_hashed.as_ref());
		final_key.extend_from_slice(key2_hashed.as_ref());

		if pre.0 != final_key {
			return Err("User hashed key mismatch");
		}
		if pre.1.proxy != post.2.proxy {
			return Err("User proxy mismatch");
		}
		if pre.1.fee_quota != post.2.fee_quota {
			return Err("User fee quota mismatch");
		}
		if pre.1.reserve_quota != post.2.reserve_quota {
			return Err("User reserve quota mismatch");
		}
		if post.2.deposit != Default::default() {
			return Err("User deposit is not default");
		}
	}

	UserRegistrationCount::<T>::iter().try_for_each(|(_user, count)| {
		if count == 0 {
			return Err("User registration count is 0");
		}
		if count > pot_details.len() as u32 {
			return Err("User registration count is greater than number of pots");
		}
		Ok(())
	})?;

	log::info!(target: "sponsorship", "post_upgrade: pots = {}, pot_user_count = {}, users = {}", pot_details.len(), user_details.len(), UserRegistrationCount::<T>::iter().count());
	Ok(())
}
