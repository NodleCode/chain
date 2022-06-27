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

use frame_support::{
	traits::{ChangeMembers, Get, InitializeMembers},
	BoundedVec,
};
use pallet_session::SessionManager;
use sp_runtime::traits::Convert;
use sp_std::prelude::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type MaxValidators: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedValidators::<T>::pre_upgrade()
		}

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migrations::v1::MigrateToBoundedValidators::<T>::on_runtime_upgrade()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedValidators::<T>::post_upgrade()
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated Validators \[new_total_validators\]
		ValidatorsUpdated(u32),
		/// Update Validators Overflow \[max_validators, requested_total_validators\]
		ValidatorsMaxOverflow(u32, u32),
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated Validators \[new_total_validators\]
		ValidatorsUpdated(u32),
		/// Update Validators Overflow \[max_validators, requested_total_validators\]
		ValidatorsMaxOverflow(u32, u32),
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated Validators \[new_total_validators\]
		ValidatorsUpdated(u32),
		/// Update Validators Overflow \[max_validators, requested_total_validators\]
		ValidatorsMaxOverflow(u32, u32),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub type Validators<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxValidators>, ValueQuery>;
}

impl<T: Config> ChangeMembers<T::AccountId> for Pallet<T> {
	fn change_members_sorted(_incoming: &[T::AccountId], _outgoing: &[T::AccountId], new: &[T::AccountId]) {
		// <Validators<T>>::put(new);

		let new_members_length: u32 = new.len() as u32;
		if new_members_length > T::MaxValidators::get() {
			Self::deposit_event(Event::ValidatorsMaxOverflow(
				T::MaxValidators::get(),
				new_members_length,
			));
		} else {
			<Validators<T>>::mutate(|maybe_oracles| {
				let new_clone: Vec<T::AccountId> = new.to_vec();

				match <BoundedVec<T::AccountId, T::MaxValidators>>::try_from(new_clone) {
					Ok(oracles) => {
						*maybe_oracles = oracles;
						Self::deposit_event(Event::ValidatorsUpdated(new_members_length));
					}
					Err(_) => {
						Self::deposit_event(Event::ValidatorsMaxOverflow(
							T::MaxValidators::get(),
							new_members_length,
						));
					}
				};
			})
		}
	}
}

impl<T: Config> InitializeMembers<T::AccountId> for Pallet<T> {
	fn initialize_members(init: &[T::AccountId]) {
		let init_members_length = init.len() as u32;

		if init_members_length > T::MaxValidators::get() {
			Self::deposit_event(Event::ValidatorsMaxOverflow(
				T::MaxValidators::get(),
				init_members_length,
			));
		} else {
			<Validators<T>>::mutate(|maybe_oracles| {
				let init_clone: Vec<T::AccountId> = init.to_vec();
				match <BoundedVec<T::AccountId, T::MaxValidators>>::try_from(init_clone) {
					Ok(oracles) => {
						*maybe_oracles = oracles;
						Self::deposit_event(Event::ValidatorsUpdated(init_members_length));
					}
					Err(_) => {
						Self::deposit_event(Event::ValidatorsMaxOverflow(
							T::MaxValidators::get(),
							init_members_length,
						));
					}
				};
			})
		}
	}
}

/// Compatibility code for the session historical code
pub type FullIdentification = u32;
pub struct FullIdentificationOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<FullIdentification>> for FullIdentificationOf<T> {
	fn convert(_validator: T::AccountId) -> Option<FullIdentification> {
		Some(0)
	}
}

type SessionIndex = u32; // A shim while waiting for this type to be exposed by `session`
impl<T: Config> SessionManager<T::AccountId> for Pallet<T> {
	fn new_session(_: SessionIndex) -> Option<Vec<T::AccountId>> {
		let all_keys = Validators::<T>::get();
		if all_keys.is_empty() {
			None
		} else {
			Some(all_keys.to_vec())
		}
	}

	fn start_session(_: SessionIndex) {}
	fn end_session(_: SessionIndex) {}
}

impl<T: Config> pallet_session::historical::SessionManager<T::AccountId, FullIdentification> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, FullIdentification)>> {
		<Self as pallet_session::SessionManager<_>>::new_session(new_index).map(|validators| {
			validators
				.into_iter()
				.map(|v| {
					let full_identification = FullIdentificationOf::<T>::convert(v.clone()).unwrap_or(0);
					(v, full_identification)
				})
				.collect()
		})
	}

	fn start_session(start_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::start_session(start_index)
	}

	fn end_session(end_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::end_session(end_index)
	}
}
