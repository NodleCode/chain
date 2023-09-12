use crate::Runtime;
use frame_support::{migration, traits::OnRuntimeUpgrade, weights::Weight};

#[cfg(feature = "try-runtime")]
use {
	codec::{Decode, Encode},
	frame_support::Blake2_128Concat,
	pallet_uniques::CollectionDetails,
	primitives::{AccountId, Balance},
	sp_std::prelude::*,
};

#[cfg(feature = "try-runtime")]
type CollectionId = u32;

#[cfg(feature = "try-runtime")]
const UNIQUES_CLASS_PREFIX: &[u8] = b"Class";

// Migrate back and revert 2.2.2 to same state as mainnet.
const NEW_UNIQUES_PALLET_NAME: &[u8] = b"Uniques";
const OLD_UNIQUES_PALLET_NAME: &[u8] = b"SubstrateUniques";

pub struct MovePalletUniquesToSubstrateUniques;
impl OnRuntimeUpgrade for MovePalletUniquesToSubstrateUniques {
	fn on_runtime_upgrade() -> Weight {
		migration::move_pallet(OLD_UNIQUES_PALLET_NAME, NEW_UNIQUES_PALLET_NAME);
		<Runtime as frame_system::Config>::BlockWeights::get().max_block
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let collection_details = migration::storage_key_iter::<
			CollectionId,
			CollectionDetails<AccountId, Balance>,
			Blake2_128Concat,
		>(OLD_UNIQUES_PALLET_NAME, UNIQUES_CLASS_PREFIX)
		.collect::<Vec<(CollectionId, CollectionDetails<AccountId, Balance>)>>();
		Ok(collection_details.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let previous_collection_details: Vec<(CollectionId, CollectionDetails<AccountId, Balance>)> =
			Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;

		let current_collection_details = migration::storage_key_iter::<
			CollectionId,
			CollectionDetails<AccountId, Balance>,
			Blake2_128Concat,
		>(NEW_UNIQUES_PALLET_NAME, UNIQUES_CLASS_PREFIX)
		.collect::<Vec<_>>();

		if current_collection_details != previous_collection_details {
			return Err("Pallet Uniques Migration: Collection details do not match");
		}
		Ok(())
	}
}
