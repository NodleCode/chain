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

	#[cfg(feature = "try-runtime")]
	use crate::BalanceOf;

	use frame_support::{
		pallet_prelude::PhantomData,
		storage::migration::{have_storage_value, remove_storage_prefix},
		traits::{Get, OnRuntimeUpgrade},
		weights::Weight,
	};

	pub struct MigrateToBoundedOracles<T>(PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToBoundedOracles<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!(
				"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V0_0_0Legacy {
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item1_prefix: &[u8] = b"Oracles";
				let storage_item2_prefix: &[u8] = b"CoinsConsumed";

				if have_storage_value(pallet_prefix, storage_item1_prefix, &[])
					&& have_storage_value(pallet_prefix, storage_item2_prefix, &[])
				{
					remove_storage_prefix(pallet_prefix, storage_item1_prefix, &[]);
					remove_storage_prefix(pallet_prefix, storage_item2_prefix, &[]);

					<StorageVersion<T>>::put(crate::Releases::V2_0_21);

					log::info!(
						"on_runtime_upgrade[{:#?}]=> Removed Oracles & CoinsConsumed, Migrated to storage version {:?}",
						line!(),
						<StorageVersion<T>>::get()
					);
				} else {
					panic!(
						"on_runtime_upgrade[{:#?}]=> Oracles & CoinsConsumed doesn't exist",
						line!()
					);
				}

				T::DbWeight::get().reads_writes(2, 2)
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
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item1_prefix: &[u8] = b"Oracles";

				let maybe_stored_data =
					get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item1_prefix, &[]);

				if let Some(stored_data) = maybe_stored_data {
					log::info!(
						"pre_upgrade[{:#?}]=> Oracles count :: [{:#?}]",
						line!(),
						stored_data.len(),
					);
				} else {
					log::info!(
						"pre_upgrade[{:#?}]=> Oracles is_none :: [{:#?}]",
						line!(),
						maybe_stored_data.is_none(),
					);
				}
			} else {
				log::info!(
					"pre_upgrade[{:#?}]=> Version mismatch. This probably should be removed",
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
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item1_prefix: &[u8] = b"Oracles";
				let storage_item2_prefix: &[u8] = b"CoinsConsumed";

				assert!(get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item1_prefix, &[]).is_none());
				assert!(get_storage_value::<BalanceOf<T>>(pallet_prefix, storage_item2_prefix, &[]).is_none());
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
