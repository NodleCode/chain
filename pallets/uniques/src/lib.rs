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

//! Handle the ability to notify other pallets that they should stop all
//! operations, or resume them

use frame_support::traits::nonfungibles::Inspect;
pub use pallet::*;
use sp_runtime::traits::StaticLookup;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
type CollectionIdOf<T> =
	<<T as pallet::Config>::NonFungible as Inspect<<T as frame_system::Config>::AccountId>>::CollectionId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::nonfungibles::{Create, Destroy, InspectEnumerable, Mutate, Transfer},
			EnsureOriginWithArg,
		},
		Parameter,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::DispatchResult;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Standard collection creation is only allowed if the origin attempting it and the
		/// collection are in this set.
		type CreateOrigin: EnsureOriginWithArg<Self::RuntimeOrigin, CollectionIdOf<Self>, Success = Self::AccountId>;

		type NonFungible: Mutate<Self::AccountId>
			+ Transfer<Self::AccountId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ InspectEnumerable<Self::AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		CollectionIdOf<T>: Member + Parameter + MaxEncodedLen + Copy,
	{
		/// Issue a new collection of non-fungible items from a public origin.
		///
		/// This new collection has no items initially and its owner is the origin.
		///
		/// The origin must conform to the configured `CreateOrigin` and have sufficient funds free.
		///
		/// `ItemDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `collection`: The identifier of the new collection. This must not be currently in use.
		/// - `admin`: The admin of this collection. The admin is the initial address of each
		/// member of the collection's admin team.
		///
		/// Emits `Created` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create(
			origin: OriginFor<T>,
			collection: CollectionIdOf<T>,
			admin: AccountIdLookupOf<T>,
		) -> DispatchResult {
			let owner = T::CreateOrigin::ensure_origin(origin, &collection)?;
			let admin = T::Lookup::lookup(admin)?;

			<T::NonFungible as Create<T::AccountId>>::create_collection(&collection, &owner, &admin)
		}
	}
}
