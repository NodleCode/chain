use frame_support::{
	traits::{OnRuntimeUpgrade, StorageVersion},
};

#[cfg(feature = "try-runtime")]
use {sp_runtime::TryRuntimeError, sp_std::prelude::*};
use crate::Runtime;
use frame_support::{traits::Get, weights::{Weight}};

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
		// Pallets with no data to migrate, just update storage version block goes here:

		//TODO check  1 key
		StorageVersion::new(4).put::<pallet_scheduler::Pallet<T>>();

		//TODO TechnicalMembership check 2 keys
		StorageVersion::new(4).put::<pallet_membership::Pallet<T, pallet_membership::pallet::Instance3>>();

		//TODO TechnicalCommittee:: check Found 4 keys (0.19s)
		StorageVersion::new(4).put::<pallet_collective::Pallet<T, pallet_collective::pallet::Instance1>>();

		// https://github.com/paritytech/substrate/pull/12813
		// moves funds to inactive if we don't need that this is OK.
		StorageVersion::new(1).put::<pallet_balances::Pallet<T>>();

		// Two keys already migrated.
		// The call to pallet_xcm::migration::v1::MigrateToV1::<Runtime>::on_runtime_upgrade() fails.
		// That migration code supposes that the value in the storage is of the old type which is not true,
		// because two new values of the new type were inserted in the VersionNotifyTargets map which is 
		// the subject of that migration. One of the new values are for Moonbeam which got inserted in 
		// the block 3351853 which is the first block after the parachain restart and the second one is
		// for Polkadot which got inserted in 3614349 16 days ago. I believe we don’t need this migration. 
		// If in the future there was any issue in any XCM interactions with Moonbeam we can force set the
		// storage entry for that single value to use proof_size = 65536 (the new default).
		StorageVersion::new(1).put::<pallet_xcm::Pallet<T>>();

		// Size of onchain storage is 0 safe to upgrade storage version
		StorageVersion::new(1).put::<pallet_preimage::Pallet<T>>();

		// TODO check 43 keys
		StorageVersion::new(12).put::<pallet_contracts::Pallet<T>>();

		// // let mut x: Weight=		<Runtime as frame_system::Config>::BlockWeights::get();
		// let x = <Runtime as frame_system::Config>::BlockWeights::set_proof_size(45);
		// // .set_ref_time(100);
		Weight::from_parts(430000000, 340000000)

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
