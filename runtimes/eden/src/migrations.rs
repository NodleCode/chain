use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};

use frame_support::weights::Weight;
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
		// Pallets with no data to migrate, just update storage version block goes here:

		// Pallet_scheduler:  1 key
		// Changed storage version to 3 and executed the v3 to v4 migration
		// [2023-12-01T03:32:38Z INFO  runtime::scheduler::migration] Trying to migrate 0 agendas...
		// [2023-12-01T03:32:38Z INFO  runtime::scheduler::migration] Migrated 0 agendas.
		// *** No v3 agendas to migrate

		// The one present key is identified as
		// 0x3db7a24cfdc9de785974746c14a99df94e7b9012096b41c4eb3aaf947f6ea429: Raw
		// scheduler.palletVersion: u16 = 0

		// v2 -> v3 code changed:
		// 5e50e0bc2c7 (Gavin Wood           2021-12-11 15:55:23 +0100 323)                        StorageVersion::<T>::put(Releases::V3);
		// *** Adding support for preimage, StorageMap format changed for Agenda
		//     Since chain contains 0 agendas it should be safe to write new storage version.

		StorageVersion::new(4).put::<pallet_scheduler::Pallet<T>>();

		// TechnicalMembership --  2 keys
		// Storage version unchanged since 2021-09-07
		// 03b294641ef substrate/frame/membership/src/lib.rs (Qinxuan Chen         2021-09-07 20:17:26 +0800
		// No migration needed just update storage version
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

		// give it some weight not zero
		Weight::from_parts(10000, 0x10000)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		log::info!("Pre upgrade");
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		log::info!("Post upgrade {_state:?}");
		Ok(())
	}
}
