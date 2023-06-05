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

//! A pallet allowing the registration and management of smart missions.

mod types;

// pub mod weights;
// pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::{DispatchResult, *};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Expresses who can add/remove a mission
		type WhitelistOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		// /// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn register(origin: OriginFor<T>, mission: types::MissionType) -> DispatchResult {
			T::WhitelistOrigin::ensure_origin(origin)?;

			let mission_id = NextMissionId::<T>::get();
			Missions::<T>::insert(mission_id, mission);

			NextMissionId::<T>::put(mission_id.checked_add(1).ok_or("Overflow")?);

			Ok(())
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The caller does not have the permission to call this function
		AccessDenied,
	}

	#[pallet::storage]
	#[pallet::getter(fn missions)]
	pub(crate) type Missions<T: Config> = StorageMap<_, Blake2_128Concat, u32, types::MissionType>;

	#[pallet::storage]
	#[pallet::getter(fn next_mission_id)]
	pub(crate) type NextMissionId<T: Config> = StorageValue<_, u32, ValueQuery>;
}
