use crate::Runtime;
use frame_support::{
	traits::{OnRuntimeUpgrade, StorageVersion},
	weights::Weight,
};

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
		+ pallet_contracts::Config,
{
	fn on_runtime_upgrade() -> Weight {
		// Pallets with no data to migrate, just update storage version block
		StorageVersion::new(4).put::<pallet_scheduler::Pallet<T>>();
		StorageVersion::new(4).put::<pallet_membership::Pallet<T, pallet_membership::pallet::Instance3>>();
		StorageVersion::new(4).put::<pallet_collective::Pallet<T, pallet_collective::pallet::Instance1>>();

		StorageVersion::new(1).put::<pallet_balances::Pallet<T>>();
		StorageVersion::new(1).put::<pallet_collator_selection::Pallet<T>>();
		StorageVersion::new(1).put::<pallet_xcm::Pallet<T>>();
		StorageVersion::new(1).put::<pallet_preimage::Pallet<T>>();
		StorageVersion::new(1).put::<pallet_multisig::Pallet<T>>();
		StorageVersion::new(12).put::<pallet_contracts::Pallet<T>>();

		<Runtime as frame_system::Config>::BlockWeights::get().base_block
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		Ok(())
	}
}
