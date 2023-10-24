use core::marker::PhantomData;

use crate::Runtime;
use frame_support::{
	migration,
	traits::{OnRuntimeUpgrade, PalletInfoAccess, StorageVersion},
	weights::Weight,
};

#[cfg(feature = "try-runtime")]
use {
	codec::{Decode, Encode},
	frame_support::Blake2_128Concat,
	pallet_uniques::CollectionDetails,
	primitives::{AccountId, Balance},
	sp_runtime::TryRuntimeError,
	sp_std::prelude::*,
};

// #[cfg(feature = "try-runtime")]
// type CollectionId = u32;

// #[cfg(feature = "try-runtime")]
// const UNIQUES_CLASS_PREFIX: &[u8] = b"Class";

// const NEW_UNIQUES_PALLET_NAME: &[u8] = b"SubstrateUniques";
// const OLD_UNIQUES_PALLET_NAME: &[u8] = b"Uniques";

pub struct MultiMigration<T>(sp_std::marker::PhantomData<T>);

impl<T> OnRuntimeUpgrade for MultiMigration<T>
where
	T: pallet_scheduler::Config
		+ pallet_balances::Config
		+ pallet_membership::Config<pallet_membership::pallet::Instance3>
		+ pallet_collective::Config<pallet_collective::pallet::Instance1>,
{
	fn on_runtime_upgrade() -> Weight {
		// Pallets with no data to migrate, just update storage version block
		StorageVersion::new(4).put::<pallet_scheduler::Pallet<T>>();
		StorageVersion::new(4).put::<pallet_membership::Pallet<T, pallet_membership::pallet::Instance3>>();
		StorageVersion::new(4).put::<pallet_collective::Pallet<T, pallet_collective::pallet::Instance1>>();

		StorageVersion::new(1).put::<pallet_balances::Pallet<T>>();

		<Runtime as frame_system::Config>::BlockWeights::get().max_block
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		// let collection_details = migration::storage_key_iter::<
		// 	CollectionId,
		// 	CollectionDetails<AccountId, Balance>,
		// 	Blake2_128Concat,
		// >(OLD_UNIQUES_PALLET_NAME, UNIQUES_CLASS_PREFIX)
		// .collect::<Vec<(CollectionId, CollectionDetails<AccountId, Balance>)>>();
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
		// let previous_collection_details: Vec<(CollectionId, CollectionDetails<AccountId, Balance>)> =
		// 	Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;

		// let current_collection_details = migration::storage_key_iter::<
		// 	CollectionId,
		// 	CollectionDetails<AccountId, Balance>,
		// 	Blake2_128Concat,
		// >(NEW_UNIQUES_PALLET_NAME, UNIQUES_CLASS_PREFIX)
		// .collect::<Vec<_>>();

		// if current_collection_details != previous_collection_details {
		// 	return Err("Pallet Uniques Migration: Collection details do not match");
		// }

		Ok(())
	}
}
