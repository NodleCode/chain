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
	use crate::{Config, Releases, StorageVersion, VestingScheduleOf, VestingSchedules};
	use frame_support::{
		pallet_prelude::PhantomData,
		storage::migration::storage_key_iter,
		traits::{Get, OnRuntimeUpgrade},
		weights::Weight,
		Blake2_128Concat, BoundedVec,
	};
	use sp_runtime::traits::Saturating;
	use sp_std::convert::TryInto;
	use sp_std::vec::Vec;

	pub struct MigrateToBoundedVestingSchedules<T>(PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToBoundedVestingSchedules<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!(
				"on_runtime_upgrade[{:#?}]=> Running migration with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V0_0_0Legacy {
				let pallet_prefix: &[u8] = b"Vesting";
				let storage_item_prefix: &[u8] = b"VestingSchedules";

				// Check number of entries, and set it aside in temp storage
				let stored_data: Vec<_> =
					storage_key_iter::<T::AccountId, Vec<VestingScheduleOf<T>>, Blake2_128Concat>(
						pallet_prefix,
						storage_item_prefix,
					)
					.collect();

				let mut translated = 0u64;
				let mut max_schedules: usize = 0;

				assert!(!stored_data.is_empty());

				// Write to the new storage with removed and added fields
				for (account, old_vesting) in stored_data {
					translated.saturating_inc();
					max_schedules = max_schedules.max(old_vesting.len());
					let new_vesting: BoundedVec<VestingScheduleOf<T>, T::MaxSchedule> = old_vesting
						.clone()
						.try_into()
						.map_err(|err| {
							log::error!(
								"on_runtime_upgrade[{:#?}]=> Schedule length :: {:#?} Max :: {:#?}",
								line!(),
								old_vesting.len(),
								T::MaxSchedule::get()
							);
							err
						})
						.expect("Could be boundedvec Overflow");

					<VestingSchedules<T>>::insert(account, new_vesting);
				}

				<StorageVersion<T>>::put(crate::Releases::V2_0_21);

				log::info!(
					"on_runtime_upgrade[{:#?}]=> Upgraded {} schedules, Max {}, storage to version {:?}",
					line!(),
					translated,
					max_schedules,
					<StorageVersion<T>>::get()
				);

				T::DbWeight::get().reads_writes(translated + 1, translated + 1)
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

			log::info!(
				"pre_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V0_0_0Legacy {
				let pallet_prefix: &[u8] = b"Vesting";
				let storage_item_prefix: &[u8] = b"VestingSchedules";

				// Check number of entries, and set it aside in temp storage
				let stored_data: Vec<_> =
					storage_key_iter::<T::AccountId, Vec<VestingScheduleOf<T>>, Blake2_128Concat>(
						pallet_prefix,
						storage_item_prefix,
					)
					.collect();

				let mapping_count: u32 = stored_data.len() as u32;
				Self::set_temp_storage(mapping_count, "mapping_count");

				stored_data
					.iter()
					.for_each(|(_account, old_vesting)| assert!(old_vesting.len() <= T::MaxSchedule::get() as usize));

				log::info!(
					"pre_upgrade[{:#?}]=> VestingSchedules map count :: [{:#?}]",
					line!(),
					mapping_count,
				);
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

			log::info!(
				"post_upgrade[{:#?}]=> with current storage version {:?} / onchain {:?}",
				line!(),
				crate::Releases::V2_0_21,
				<StorageVersion<T>>::get(),
			);

			if <StorageVersion<T>>::get() == Releases::V2_0_21 {
				let pallet_prefix: &[u8] = b"Vesting";
				let storage_item_prefix: &[u8] = b"VestingSchedules";

				let mapping_count: u32 = storage_key_iter::<
					T::AccountId,
					BoundedVec<VestingScheduleOf<T>, T::MaxSchedule>,
					Blake2_128Concat,
				>(pallet_prefix, storage_item_prefix)
				.count() as u32;

				log::info!(
					"post_upgrade[{:#?}]=> VestingSchedules map count :: [{:#?}]",
					line!(),
					mapping_count,
				);

				assert!(Some(mapping_count) == Self::get_temp_storage::<u32>("mapping_count"));
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
