/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, traits::EnsureOrigin,
};
use frame_system::ensure_root;

/// The module's configuration trait.
pub trait Config: frame_system::Config {
    type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
    type ShutdownOrigin: EnsureOrigin<Self::Origin>;
}

decl_storage! {
    trait Store for Module<T: Config> as EmergencyShutdown {
        pub Shutdown get(fn shutdown): bool;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Toggle the shutdown state if authorized to do so.
        #[weight = 10_000_000]
        pub fn toggle(origin) -> DispatchResult {
            T::ShutdownOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            Shutdown::put(!Self::shutdown());
            Self::deposit_event(Event::ShutdownToggled(Self::shutdown()));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event {
        /// Shutdown state was toggled, to either on or off.
        ShutdownToggled(bool),
    }
);
