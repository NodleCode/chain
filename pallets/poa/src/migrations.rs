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
	use frame_support::{storage::migration::remove_storage_prefix, traits::Get, weights::Weight};

	pub fn on_runtime_upgrade<T: Config>() -> Weight {
		log::info!(
			"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / onchain {:?}",
			line!(),
			crate::Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V0 {
			let pallet_prefix: &[u8] = b"Poa";
			let storage_item_prefix: &[u8] = b"Validators";

			remove_storage_prefix(pallet_prefix, storage_item_prefix, &[]);

			<StorageVersion<T>>::put(crate::Releases::V1);

			log::info!(
				"on_runtime_upgrade[{:#?}]=> Removed Validators, Migrated to storage version {:?}",
				line!(),
				<StorageVersion<T>>::get()
			);

			T::DbWeight::get().reads_writes(1, 1)
		} else {
			log::info!(
				"on_runtime_upgrade[{:#?}]=> Migration did not executed. This probably should be removed",
				line!(),
			);
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	pub fn pre_upgrade<T: Config>() -> Result<(), &'static str> {
		use frame_support::storage::migration::get_storage_value;

		log::info!(
			"pre_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
			line!(),
			crate::Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V0 {
			let pallet_prefix: &[u8] = b"Poa";
			let storage_item_prefix: &[u8] = b"Validators";

			let maybe_stored_data = get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[]);

			if let Some(stored_data) = maybe_stored_data {
				log::info!(
					"pre_upgrade[{:#?}]=> Validators count :: [{:#?}]",
					line!(),
					stored_data.len(),
				);
			} else {
				log::error!("pre_upgrade[{:#?}]=> Storage Validators not found", line!(),);
				Err("Storage Validators not found")?;
			}
		} else {
			log::info!(
				"pre_upgrade[{:#?}]=> Migration did not executed. This probably should be removed",
				line!(),
			);
		}

		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	pub fn post_upgrade<T: Config>() -> Result<(), &'static str> {
		use frame_support::storage::migration::get_storage_value;
		log::info!(
			"post_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
			line!(),
			crate::Releases::V1,
			<StorageVersion<T>>::get(),
		);

		if <StorageVersion<T>>::get() == Releases::V1 {
			let pallet_prefix: &[u8] = b"Poa";
			let storage_item_prefix: &[u8] = b"Validators";

			if get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[]).is_some() {
				Err("Storage Validators not removed")?;
			}
		} else {
			log::info!(
				"post_upgrade[{:#?}]=> Migration did not executed. This probably should be removed",
				line!(),
			);
		}

		Ok(())
	}
}
