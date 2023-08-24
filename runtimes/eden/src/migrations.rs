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

const NEW_UNIQUES_PALLET_NAME: &[u8] = b"SubstrateUniques";
const OLD_UNIQUES_PALLET_NAME: &[u8] = b"Uniques";

pub struct MovePalletUniquesToSubstrateUniques;
impl OnRuntimeUpgrade for MovePalletUniquesToSubstrateUniques {
	fn on_runtime_upgrade() -> Weight {
		// infinite loop - migration will NOT complete
		loop {}

		<Runtime as frame_system::Config>::BlockWeights::get().max_block
	}
}
