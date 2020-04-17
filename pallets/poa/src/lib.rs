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

//! A runtime module to handle help managing validators through the `membership`,
//! support the deletion and addition of validators by a root authority n.

#[cfg(test)]
mod tests;

use frame_support::{
    decl_module, decl_storage,
    traits::{ChangeMembers, InitializeMembers},
};
use frame_system as system;
use pallet_session::SessionManager;
use sp_runtime::traits::Convert;
use sp_std::prelude::Vec;

/// The module's configuration trait.
pub trait Trait: system::Trait + pallet_session::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as AllocationsModule {
        Validators get(validators): Vec<T::AccountId>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Nothing, just an empty shell for declaration purposes
    }
}

impl<T: Trait> ChangeMembers<T::AccountId> for Module<T> {
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        <Validators<T>>::put(new);
    }
}

impl<T: Trait> InitializeMembers<T::AccountId> for Module<T> {
    fn initialize_members(init: &[T::AccountId]) {
        <Validators<T>>::put(init);
        // Shouldn't need a flag update here as this should happen at genesis
    }
}

/// Compatibility code for the session historical code
pub type FullIdentification = u32;
pub struct FullIdentificationOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Trait> Convert<T::AccountId, Option<FullIdentification>> for FullIdentificationOf<T> {
    fn convert(_validator: T::AccountId) -> Option<FullIdentification> {
        Some(0)
    }
}

type SessionIndex = u32; // A shim while waiting for this type to be exposed by `session`
impl<T: Trait> SessionManager<T::AccountId> for Module<T> {
    fn new_session(_: SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(<Validators<T>>::get())
    }

    fn start_session(_: SessionIndex) {}
    fn end_session(_: SessionIndex) {}
}

impl<T: Trait> pallet_session::historical::SessionManager<T::AccountId, FullIdentification>
    for Module<T>
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, FullIdentification)>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index).map(|validators| {
            validators
                .into_iter()
                .map(|v| {
                    let full_identification =
                        FullIdentificationOf::<T>::convert(v.clone()).unwrap_or(0);
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
