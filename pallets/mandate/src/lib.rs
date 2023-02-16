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

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::GetDispatchInfo,
		pallet_prelude::*,
		traits::{EnsureOrigin, UnfilteredDispatchable},
		Parameter,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::DispatchResult;
	use sp_std::prelude::Box;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type RuntimeCall: Parameter + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin> + GetDispatchInfo;

		/// Origin that can call this module and execute sudo actions. Typically
		/// the `collective` module.
		type ExternalOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[allow(clippy::boxed_local)]
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Let the configured origin dispatch a call as root
		#[pallet::call_index(0)]
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(
				dispatch_info.weight.saturating_add(Weight::from_ref_time(10_000)),
				dispatch_info.class,
			)
		})]
		pub fn apply(origin: OriginFor<T>, call: Box<<T as Config>::RuntimeCall>) -> DispatchResultWithPostInfo {
			T::ExternalOrigin::ensure_origin(origin)?;

			// Shamelessly stollen from the `sudo` module
			let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
			Self::deposit_event(Event::RootOp(res.map(|_| ()).map_err(|e| e.error)));

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A root operation was executed, show result
		RootOp(DispatchResult),
	}
}
