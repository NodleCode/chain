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

#![cfg_attr(not(feature = "std"), no_std)]

//! A runtime module to handle help managing validators through the `membership`,
//! support the deletion and addition of validators by a root authority n.

#[cfg(test)]
mod tests;

mod migrations;

use codec::{Decode, Encode};

use frame_support::{pallet_prelude::MaxEncodedLen, traits::SortedMembers};
use pallet_session::SessionManager;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_staking::SessionIndex;
use sp_std::prelude::Vec;

pub use pallet::*;

// A value placed in storage that represents the current version of the POA storage.
// This value is used by the `on_runtime_upgrade` logic to determine whether we run storage
// migration logic. This should match directly with the semantic versions of the Rust crate.
#[derive(Encode, MaxEncodedLen, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
enum Releases {
	V0, // Legacy version
	V1, // Adds storage info
}

impl Default for Releases {
	fn default() -> Self {
		Releases::V0
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config {
		type ValidatorsSet: SortedMembers<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			migrations::v1::pre_upgrade::<T>()
		}

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migrations::v1::on_runtime_upgrade::<T>()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			migrations::v1::post_upgrade::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	pub(crate) type StorageVersion<T: Config> = StorageValue<_, Releases, ValueQuery>;
}

impl<T: Config> SessionManager<T::AccountId> for Pallet<T> {
	fn new_session(_: SessionIndex) -> Option<Vec<T::AccountId>> {
		let all_keys = T::ValidatorsSet::sorted_members();
		if all_keys.is_empty() {
			None
		} else {
			Some(all_keys)
		}
	}

	fn start_session(_: SessionIndex) {}
	fn end_session(_: SessionIndex) {}
}
