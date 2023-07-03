use crate::Runtime;
use frame_support::{migration, traits::OnRuntimeUpgrade, weights::Weight};

#[cfg(feature = "try-runtime")]
use codec::{Decode, Encode};
#[cfg(feature = "try-runtime")]
use frame_support::Blake2_128Concat;
#[cfg(feature = "try-runtime")]
use pallet_uniques::CollectionDetails;
#[cfg(feature = "try-runtime")]
use primitives::{AccountId, Balance};
#[cfg(feature = "try-runtime")]
use sp_std::prelude::*;
#[cfg(feature = "try-runtime")]
type CollectionId = u32;
#[cfg(feature = "try-runtime")]
const TRY_RUNTIME: &str = "try-runtime";
pub struct MovePalletUniquesToSubstrateUniques;
impl OnRuntimeUpgrade for MovePalletUniquesToSubstrateUniques {
	fn on_runtime_upgrade() -> Weight {
		//todo: replace with correct weights
		let mut weight = <Runtime as frame_system::Config>::DbWeight::get().reads(4);
		weight += <Runtime as frame_system::Config>::DbWeight::get().writes(1);
		migration::move_pallet(b"Uniques", b"SubstrateUniques");
		weight
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let iter = migration::storage_key_iter::<CollectionId, CollectionDetails<AccountId, Balance>, Blake2_128Concat>(
			b"Uniques", b"Class",
		);
		let mut collection_details: Vec<(CollectionId, CollectionDetails<AccountId, Balance>)> = Vec::new();
		for collection_detail in iter {
			log::info!(
				target: TRY_RUNTIME,
				"Collection before migration: {:?}",
				collection_detail
			);
			collection_details.push(collection_detail);
		}

		Ok(collection_details.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let previous_collection_details: Vec<(CollectionId, CollectionDetails<AccountId, Balance>)> =
			Decode::decode(&mut state.as_slice()).map_err(|_| "Unable to decode previous collection details")?;
		let mut current_collection_details = Vec::<(CollectionId, CollectionDetails<AccountId, Balance>)>::new();
		let iter = migration::storage_key_iter::<CollectionId, CollectionDetails<AccountId, Balance>, Blake2_128Concat>(
			b"SubstrateUniques",
			b"Class",
		);
		for collection_detail in iter {
			current_collection_details.push(collection_detail);
		}

		if current_collection_details != previous_collection_details {
			return Err("Pallet Uniques Migration: Collection details do not match");
		}
		Ok(())
		//todo
	}
}
