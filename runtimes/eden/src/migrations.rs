use crate::Runtime;
use frame_support::{migration, parameter_types, traits::OnRuntimeUpgrade, weights::Weight, BoundedVec};
use primitives::AccountId;

#[cfg(feature = "try-runtime")]
use frame_support::traits::OnRuntimeUpgradeHelpersExt;

// MaxMembers is chosen based on what used to be the MaxMembers param for the pallet ValidaorsSet
// We have intentionally used the same number for MaxInvulnerables for the pallet CollatorSelection
// So the BoundedVec storage in these two pallets remain compatible.
parameter_types! {
	pub const MaxMembers: u32 = 50;
}

const VALIDATORS_SET_MODULE: &[u8] = b"ValidatorsSet";
const MEMBERS_ITEM: &[u8] = b"Members";
const PRIME_ITEM: &[u8] = b"Prime";

const POA_MODULE: &[u8] = b"Poa";
const STORAGE_VERSION_ITEM: &[u8] = b"StorageVersion";

const COLLATOR_SELECTION_MODULE: &[u8] = b"CollatorSelection";
const INVULNERABLES_ITEM: &[u8] = b"Invulnerables";

const EMPTY_HASH: &[u8] = &[];

const MIGRATION: &str = "migration";
#[cfg(feature = "try-runtime")]
const TRY_RUNTIME: &str = "try-runtime";

pub struct MoveValidatorsSetToInvulnerables;
impl OnRuntimeUpgrade for MoveValidatorsSetToInvulnerables {
	fn on_runtime_upgrade() -> Weight {
		if let Some(validators) = migration::take_storage_value::<BoundedVec<AccountId, MaxMembers>>(
			VALIDATORS_SET_MODULE,
			MEMBERS_ITEM,
			EMPTY_HASH,
		) {
			log::info!(target: MIGRATION, "ValidatorsSet::Members are {:?}", validators);

			migration::put_storage_value(COLLATOR_SELECTION_MODULE, INVULNERABLES_ITEM, EMPTY_HASH, validators);

			let clear_prime_result =
				migration::clear_storage_prefix(VALIDATORS_SET_MODULE, PRIME_ITEM, EMPTY_HASH, None, None);
			if clear_prime_result.maybe_cursor.is_none() {
				log::info!(
					target: MIGRATION,
					"ValidatorsSet::Prime with {} unique entries is removed",
					clear_prime_result.unique
				);
			} else {
				log::error!(target: MIGRATION, "Failed to remove ValidatorsSet::Prime completely");
			}

			let clear_poa_result =
				migration::clear_storage_prefix(POA_MODULE, STORAGE_VERSION_ITEM, EMPTY_HASH, None, None);
			if clear_poa_result.maybe_cursor.is_none() {
				log::info!(
					target: MIGRATION,
					"Poa::StorageVersion with {} unique entries is removed",
					clear_poa_result.unique
				);
			} else {
				log::error!(target: MIGRATION, "Failed to remove Poa::StorageVersion completely");
			}

			<Runtime as frame_system::Config>::DbWeight::get().writes(4)
		} else {
			<Runtime as frame_system::Config>::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		if let Some(validators) = migration::get_storage_value::<BoundedVec<AccountId, MaxMembers>>(
			VALIDATORS_SET_MODULE,
			MEMBERS_ITEM,
			EMPTY_HASH,
		) {
			Self::set_temp_storage(validators, "ValidatorsSet::Members");
			Ok(())
		} else {
			Err("Remove the runtime upgrade code")
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		if migration::have_storage_value(VALIDATORS_SET_MODULE, MEMBERS_ITEM, EMPTY_HASH)
			|| migration::have_storage_value(VALIDATORS_SET_MODULE, PRIME_ITEM, EMPTY_HASH)
			|| migration::have_storage_value(POA_MODULE, STORAGE_VERSION_ITEM, EMPTY_HASH)
		{
			Err("Failed to remove ValidatorsSet and/or Poa")
		} else {
			let invulnerables = pallet_collator_selection::pallet::Pallet::<Runtime>::invulnerables();
			log::info!(
				target: TRY_RUNTIME,
				"CollatorSelection::Invulnerables are {:?}",
				invulnerables
			);
			let validators =
				Self::get_temp_storage::<BoundedVec<AccountId, MaxMembers>>("ValidatorsSet::Members").unwrap();
			if invulnerables == validators {
				log::info!(target: TRY_RUNTIME, "MoveValidatorsSetToInvulnerables was successful");
				Ok(())
			} else {
				Err("CollatorSelection::Invulnerables are not the same as ValidatorsSet::Members")
			}
		}
	}
}
