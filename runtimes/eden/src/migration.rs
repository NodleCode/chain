/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2024  Nodle International
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

use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};
use frame_support::weights::Weight;
use sp_core::Get;
#[cfg(feature = "try-runtime")]
use {frame_support::ensure, sp_runtime::TryRuntimeError, sp_std::prelude::*};

pub struct FromSpec27<T>(sp_std::marker::PhantomData<T>);
impl<T> OnRuntimeUpgrade for FromSpec27<T>
where
	T: pallet_uniques::Config,
{
	fn on_runtime_upgrade() -> Weight {
		StorageVersion::new(1).put::<pallet_uniques::Pallet<T>>();

		T::DbWeight::get().writes(1)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		ensure!(
			StorageVersion::get::<pallet_uniques::Pallet<T>>() == 0,
			TryRuntimeError::Other("pallet_uniques storage version is not 0")
		);
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		ensure!(
			StorageVersion::get::<pallet_uniques::Pallet<T>>() == 1,
			TryRuntimeError::Other("pallet_uniques post upgrade storage version is not 1")
		);
		Ok(())
	}
}
