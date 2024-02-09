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
		+ pallet_nodle_uniques::Config,
{
	fn on_runtime_upgrade() -> Weight {
		// Size of onchain storage is 0 safe to upgrade storage version
		// Onchain storage version = 1 in source code - unchanged any new data will be in the v1 format
		StorageVersion::new(1).put::<pallet_preimage::Pallet<T>>();
		// pallet_uniques adding a storage version not chaning anything
		StorageVersion::new(1).put::<pallet_uniques::Pallet<T>>();

		// Store version 0 for default
		StorageVersion::new(0).put::<pallet_nodle_uniques::Pallet<T>>();

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
