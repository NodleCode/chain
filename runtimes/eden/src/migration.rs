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
		// Fixup two unmigrated storage blocks found on mainnet.
		let parachain_system_host_configuration_key = [
			69_u8, 50, 61, 247, 204, 71, 21, 11, 57, 48, 226, 102, 107, 10, 163, 19, 197, 34, 35, 24, 128, 35, 138, 12,
			86, 2, 27, 135, 68, 160, 7, 67,
		];
		let _ = frame_support::storage::unhashed::clear_prefix(&parachain_system_host_configuration_key, Some(1), None);
		let pre_image_status_for_key = [
			216_u8, 243, 20, 183, 244, 230, 176, 149, 240, 248, 238, 70, 86, 164, 72, 37, 85, 177, 174, 142, 206, 213,
			82, 47, 60, 64, 73, 188, 132, 237, 164, 168, 134, 165, 126, 209, 14, 33, 121, 241, 141, 181, 169, 30, 183,
			253, 57, 95, 170, 112, 42, 188, 5, 207, 200, 241, 243, 215, 36, 138, 208, 108, 237, 135,
		];
		let _ = frame_support::storage::unhashed::clear_prefix(&pre_image_status_for_key, Some(1), None);

		StorageVersion::new(1).put::<pallet_uniques::Pallet<T>>();

		T::DbWeight::get().writes(3)
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
