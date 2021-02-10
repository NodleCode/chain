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

//! Minimal Pallet that injects a ParachainId into Runtime storage.

use cumulus_primitives::ParaId;
use frame_support::{decl_module, decl_storage, traits::Get};

/// Configuration trait of this pallet.
pub trait Config: frame_system::Config {}

impl<T: Config> Get<ParaId> for Module<T> {
    fn get() -> ParaId {
        Self::parachain_id()
    }
}

decl_storage! {
    trait Store for Module<T: Config> as ParachainInfo {
        ParachainId get(fn parachain_id) config(): ParaId = 100.into();
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {}
}
