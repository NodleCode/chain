/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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
mod benchmarking;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ShutdownOrigin: EnsureOrigin<Self::Origin>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Toggle the shutdown state if authorized to do so.
        #[pallet::weight(T::WeightInfo::toggle())]
        pub fn toggle(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            T::ShutdownOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            <Shutdown<T>>::put(!Self::shutdown());
            Self::deposit_event(Event::ShutdownToggled(Self::shutdown()));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Shutdown state was toggled, to either on or off.
        ShutdownToggled(bool),
    }

    #[pallet::storage]
    #[pallet::getter(fn shutdown)]
    pub type Shutdown<T: Config> = StorageValue<_, bool, ValueQuery>;
}
