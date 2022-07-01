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
	use crate::{Config, Oracles, Pallet};
	use frame_support::{
		dispatch::GetStorageVersion,
		pallet_prelude::PhantomData,
		storage::migration::get_storage_value,
		traits::{Get, OnRuntimeUpgrade},
		weights::Weight,
		BoundedVec,
	};
	use sp_std::convert::TryInto;
	use sp_std::vec::Vec;

	pub struct MigrateToBoundedOracles<T>(PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToBoundedOracles<T> {
		fn on_runtime_upgrade() -> Weight {
			let current = Pallet::<T>::current_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			log::info!(
				"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / onchain {:?}",
				line!(),
				current,
				onchain
			);

			if current == 1 && onchain == 0 {
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item_prefix: &[u8] = b"Oracles";

				let pre_validators: BoundedVec<T::AccountId, T::MaxOracles> =
					get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[])
						.expect("Expected Storage Element")
						.try_into()
						.expect("Could be boundedvec overflow");

				<Oracles<T>>::put(&pre_validators);

				current.put::<Pallet<T>>();

				let validators_length: u64 = <Oracles<T>>::get().len() as u64;

				log::info!(
					"on_runtime_upgrade[{:#?}]=> Upgraded {} validators, storage to version {:?}",
					line!(),
					validators_length,
					current
				);

				T::DbWeight::get().reads_writes(validators_length + 1, validators_length + 1)
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
			use frame_support::traits::OnRuntimeUpgradeHelpersExt;

			let current = Pallet::<T>::current_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			log::info!(
				"pre_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				current,
				onchain
			);

			if current == 1 && onchain == 0 {
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item_prefix: &[u8] = b"Oracles";

				let maybe_stored_data = get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[]);

				if let Some(stored_data) = maybe_stored_data {
					Self::set_temp_storage(stored_data.len() as u32, "oracles_count");
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
					"pre_upgrade[{:#?}]=> Migration did not executed. This probably should be removed",
					line!(),
				);
			}

			Ok(())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			use frame_support::traits::OnRuntimeUpgradeHelpersExt;

			let current = Pallet::<T>::current_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			log::info!(
				"post_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				current,
				onchain
			);

			if current == 1 && onchain == 1 {
				let pallet_prefix: &[u8] = b"Allocations";
				let storage_item_prefix: &[u8] = b"Oracles";

				let stored_data = get_storage_value::<Vec<T::AccountId>>(pallet_prefix, storage_item_prefix, &[])
					.expect("Expected Storage Element");

				let oracles_count = stored_data.len() as u32;

				log::info!("post_upgrade[{:#?}]=> Oracles count :: [{:#?}]", line!(), oracles_count,);

				// Check number of entries matches what was set aside in pre_upgrade
				if let Some(pre_oracles_count) = Self::get_temp_storage::<u32>("oracles_count") {
					assert!(pre_oracles_count == oracles_count);
				} else {
					log::info!(
						"post_upgrade[{:#?}]=> Pre-Migration not executed. This probably should be removed",
						line!(),
					);
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
}
