use crate::{AccountId, Runtime};
use frame_system::EnsureRoot;

impl pallet_missions_registry::Config for Runtime {
	type WhitelistOrigin = EnsureRoot<AccountId>;
}
