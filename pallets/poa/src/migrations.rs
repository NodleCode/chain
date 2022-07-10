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
	use frame_support::{
		pallet_prelude::PhantomData,
		storage::migration::{have_storage_value, remove_storage_prefix},
		traits::{Get, OnRuntimeUpgrade},
		weights::Weight,
	};

	pub struct MigrateToBoundedValidators<T>(PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToBoundedValidators<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!(
				"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V0_0_0Legacy {
				let pallet_prefix: &[u8] = b"Poa";
				let storage_item_prefix: &[u8] = b"Validators";

				if have_storage_value(pallet_prefix, storage_item_prefix, &[]) {
					remove_storage_prefix(pallet_prefix, storage_item_prefix, &[]);

					<StorageVersion<T>>::put(crate::Releases::V2_0_21);

					log::info!(
						"on_runtime_upgrade[{:#?}]=> Removed Validators, Migrated to storage version {:?}",
						line!(),
						<StorageVersion<T>>::get()
					);
				} else {
					panic!("on_runtime_upgrade[{:#?}]=> Validators doesn't exist", line!());
				}

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
		fn pre_upgrade() -> Result<(), &'static str> {
			use frame_support::storage::migration::get_storage_value;

			log::info!(
				"pre_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V0_0_0Legacy {
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
					log::info!(
						"pre_upgrade[{:#?}]=> Validators is_none :: [{:#?}]",
						line!(),
						maybe_stored_data.is_none(),
					);
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
		fn post_upgrade() -> Result<(), &'static str> {
			use frame_support::storage::migration::get_storage_value;
			log::info!(
				"post_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V2_0_21 {
				let pallet_prefix: &[u8] = b"Poa";
				let storage_item_prefix: &[u8] = b"Validators";

				assert!(get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[]).is_none());
			} else {
				log::info!(
					"post_upgrade[{:#?}]=> Migration did not executed. This probably should be removed",
					line!(),
				);
			}

			Ok(())
		}
	}
}
