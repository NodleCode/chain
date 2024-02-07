use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};

use frame_support::weights::Weight;
use sp_core::Get;
#[cfg(feature = "try-runtime")]
use {sp_runtime::TryRuntimeError, sp_std::prelude::*};

pub struct MultiMigration<T>(sp_std::marker::PhantomData<T>);

impl<T> OnRuntimeUpgrade for MultiMigration<T>
where
	T: pallet_scheduler::Config
		+ pallet_balances::Config
		+ pallet_membership::Config<pallet_membership::pallet::Instance3>
		+ pallet_collective::Config<pallet_collective::pallet::Instance1>
		+ pallet_collator_selection::Config
		+ pallet_xcm::Config
		+ pallet_preimage::Config
		+ pallet_multisig::Config
		+ pallet_contracts::Config
		+ pallet_uniques::Config
		+ pallet_nodle_uniques::Config
		+ pallet_message_queue::Config,
{
	fn on_runtime_upgrade() -> Weight {
		// Size of onchain storage is 0 safe to upgrade storage version
		// Onchain storage version = 1 in source code - unchanged any new data will be in the v1 format
		// StorageVersion::new(1).put::<pallet_preimage::Pallet<T>>();
		// pallet_uniques adding a storage version not chaning anything
		StorageVersion::new(1).put::<pallet_uniques::Pallet<T>>();

		// Store version 0 for default
		StorageVersion::new(0).put::<pallet_nodle_uniques::Pallet<T>>();
		StorageVersion::new(0).put::<pallet_message_queue::Pallet<T>>();

		// Version 1.5 Fixup
		// [2024-02-06T11:29:36Z ERROR runtime::executive] - 0. error: Failed to decode storage item `ParachainSystem::HostConfiguration`
		// [2024-02-06T11:29:36Z DEBUG runtime::executive] - 0. error: TryDecodeEntireStorageError { key: [69, 50, 61, 247, 204, 71, 21, 11, 57, 48, 226, 102, 107, 10, 163, 19, 197, 34, 35, 24, 128, 35, 138, 12, 86, 2, 27, 135, 68, 160, 7, 67], raw: Some([0, 0, 48, 0, 0, 80, 0, 0, 170, 170, 2, 0, 0, 0, 16, 0, 251, 255, 0, 0, 16, 0, 0, 0, 10, 0, 0, 0, 64, 56, 0, 0, 88, 2, 0, 0]), info: StorageInfo { pallet_name: [80, 97, 114, 97, 99, 104, 97, 105, 110, 83, 121, 115, 116, 101, 109], storage_name: [72, 111, 115, 116, 67, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110], prefix: [69, 50, 61, 247, 204, 71, 21, 11, 57, 48, 226, 102, 107, 10, 163, 19, 197, 34, 35, 24, 128, 35, 138, 12, 86, 2, 27, 135, 68, 160, 7, 67], max_values: Some(1), max_size: None } }
		// [2024-02-06T11:29:36Z ERROR runtime::executive] - 1. error: Failed to decode storage item `Preimage::StatusFor`
		// [2024-02-06T11:29:36Z DEBUG runtime::executive] - 1. error: TryDecodeEntireStorageError { key: [216, 243, 20, 183, 244, 230, 176, 149, 240, 248, 238, 70, 86, 164, 72, 37, 85, 177, 174, 142, 206, 213, 82, 47, 60, 64, 73, 188, 132, 237, 164, 168, 134, 165, 126, 209, 14, 33, 121, 241, 141, 181, 169, 30, 183, 253, 57, 95, 170, 112, 42, 188, 5, 207, 200, 241, 243, 215, 36, 138, 208, 108, 237, 135], raw: Some([0, 1, 74, 34, 11, 19, 82, 60, 166, 200, 236, 155, 46, 110, 132, 247, 154, 76, 85, 37, 199, 188, 137, 30, 55, 23, 255, 89, 202, 125, 127, 172, 65, 44, 0, 67, 52, 105, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), info: StorageInfo { pallet_name: [80, 114, 101, 105, 109, 97, 103, 101], storage_name: [83, 116, 97, 116, 117, 115, 70, 111, 114], prefix: [216, 243, 20, 183, 244, 230, 176, 149, 240, 248, 238, 70, 86, 164, 72, 37, 85, 177, 174, 142, 206, 213, 82, 47, 60, 64, 73, 188, 132, 237, 164, 168], max_values: None, max_size: None } }

		//Host Configuration
		let key = vec![
			69_u8, 50, 61, 247, 204, 71, 21, 11, 57, 48, 226, 102, 107, 10, 163, 19, 197, 34, 35, 24, 128, 35, 138, 12,
			86, 2, 27, 135, 68, 160, 7, 67,
		];
		let raw = vec![
			0_u8, 0, 48, 0, 0, 80, 0, 0, 170, 170, 2, 0, 0, 0, 16, 0, 251, 255, 0, 0, 16, 0, 0, 0, 10, 0, 0, 0, 64, 56,
			0, 0, 88, 2, 0, 0,
		];
		// let info = StorageInfo { pallet_name: [80, 97, 114, 97, 99, 104, 97, 105, 110, 83, 121, 115, 116, 101, 109], storage_name: [72, 111, 115, 116, 67, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110], prefix: [69, 50, 61, 247, 204, 71, 21, 11, 57, 48, 226, 102, 107, 10, 163, 19, 197, 34, 35, 24, 128, 35, 138, 12, 86, 2, 27, 135, 68, 160, 7, 67] };
		log::info!("{:02x?}", key);
		log::info!("{:02x?}", raw);
		
		let ans = frame_support::storage::unhashed::clear_prefix(&key, Some(1), None);
		// let x = cumulus_pallet_parachain_system::HostConfiguration{};

		log::info!(
			"Res: {:02x?} {} {} {}",
			ans.maybe_cursor,
			ans.backend,
			ans.unique,
			ans.loops
		);

		// PreImage
		let key = [
			216_u8, 243, 20, 183, 244, 230, 176, 149, 240, 248, 238, 70, 86, 164, 72, 37, 85, 177, 174, 142, 206, 213,
			82, 47, 60, 64, 73, 188, 132, 237, 164, 168, 134, 165, 126, 209, 14, 33, 121, 241, 141, 181, 169, 30, 183,
			253, 57, 95, 170, 112, 42, 188, 5, 207, 200, 241, 243, 215, 36, 138, 208, 108, 237, 135,
		];
		let raw = vec![
			0_u8, 1, 74, 34, 11, 19, 82, 60, 166, 200, 236, 155, 46, 110, 132, 247, 154, 76, 85, 37, 199, 188, 137, 30,
			55, 23, 255, 89, 202, 125, 127, 172, 65, 44, 0, 67, 52, 105, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		];
		let pallet_name = [80_u8, 114, 101, 105, 109, 97, 103, 101];
		let storage_name = [83_u8, 116, 97, 116, 117, 115, 70, 111, 114];
		let prefix = [
			216_u8, 243, 20, 183, 244, 230, 176, 149, 240, 248, 238, 70, 86, 164, 72, 37, 85, 177, 174, 142, 206, 213,
			82, 47, 60, 64, 73, 188, 132, 237, 164, 168,
		];

		// let ans = frame_support::storage::unhashed::clear_storage_prefix(&pallet_name,&storage_name,&prefix, None,None);

		let ans = frame_support::storage::unhashed::clear_prefix(&key, Some(1), None);
		// let x = cumulus_pallet_parachain_system::HostConfiguration{};

		log::info!(
			"Res: {:02x?} {} {} {}",
			ans.maybe_cursor,
			ans.backend,
			ans.unique,
			ans.loops
		);
		log::info!("{:02x?}", key);
		log::info!("{:02x?}", raw);

		T::DbWeight::get().writes(6)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		use frame_support::ensure;

		log::info!("Pre upgrade");

		ensure!(
			StorageVersion::get::<pallet_uniques::Pallet<T>>() == 0,
			TryRuntimeError::Other("pallet_uniques storage version is not 0")
		);

		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		log::info!("Post upgrade {_state:?}");

		Ok(())
	}
}
