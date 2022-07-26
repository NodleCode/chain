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

pub mod v1 {
	use crate::{Config, Releases, StorageVersion};
	use frame_support::{storage::migration::clear_storage_prefix, traits::Get, weights::Weight};

	pub fn on_runtime_upgrade<T: Config>() -> Weight {
		log::info!(
			"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / on-chain {:?}",
			line!(),
			Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V0 {
			let pallet_prefix: &[u8] = b"Allocations";
			let storage_item_prefix: &[u8] = b"Oracles";

			let remove_result = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

			<StorageVersion<T>>::put(crate::Releases::V1);

			log::info!(
				"on_runtime_upgrade[{:#?}]=> Clear Result loops[{:#?}], backend[{:#?}]",
				line!(),
				remove_result.loops,
				remove_result.backend
			);

			log::info!(
				"on_runtime_upgrade[{:#?}]=> Removed Oracles, Migrated to storage version {:#?}",
				line!(),
				<StorageVersion<T>>::get()
			);

			T::DbWeight::get().reads_writes(remove_result.loops.into(), remove_result.backend.into())
		} else {
			log::info!(
				"on_runtime_upgrade[{:#?}]=> Migration did not execute. This probably should be removed",
				line!(),
			);
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	pub fn pre_upgrade<T: Config>() -> Result<(), &'static str> {
		use frame_support::storage::migration::get_storage_value;

		log::info!(
			"pre_upgrade[{:#?}]=> with current storage version {:?} / on-chain {:?}",
			line!(),
			crate::Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V0 {
			let pallet_prefix: &[u8] = b"Allocations";
			let storage_item_prefix: &[u8] = b"Oracles";
			let stored_data = get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[])
				.ok_or("No oracle storage")?;
			log::info!(
				"pre_upgrade[{:#?}]=> Oracles count :: [{:#?}]",
				line!(),
				stored_data.len(),
			);
		} else {
			log::info!("pallet-grants::pre_upgrade: No migration is expected");
		}

		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	pub fn post_upgrade<T: Config>() -> Result<(), &'static str> {
		use frame_support::storage::migration::get_storage_value;
		log::info!(
			"post_upgrade[{:#?}]=> with current storage version {:?} / on-chain {:?}",
			line!(),
			crate::Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V1 {
			let pallet_prefix: &[u8] = b"Allocations";
			let storage_item_prefix: &[u8] = b"Oracles";

			if get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[]).is_some() {
				Err("Oracle storage not removed")?;
			}
		} else {
			log::info!(
				"post_upgrade[{:#?}]=> Migration did not execute. This probably should be removed",
				line!(),
			);
		}

		Ok(())
	}
}
