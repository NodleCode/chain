use crate::{Runtime, RuntimeOrigin};
use frame_support::{
	migration, parameter_types,
	traits::{GetStorageVersion, OnRuntimeUpgrade},
	weights::Weight,
	BoundedVec,
};
use primitives::{AccountId, Balance};

#[cfg(feature = "try-runtime")]
use codec::{Decode, Encode};
#[cfg(feature = "try-runtime")]
use sp_std::prelude::*;

const STORAGE_VERSION_ITEM: &[u8] = b"StorageVersion";

const MIGRATION: &str = "migration";
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

		//todo
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {   
        //todo
	}
}