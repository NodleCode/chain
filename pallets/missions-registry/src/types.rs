use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
/// `MissionType` encodes the type of a smart mission. This allows the categorization
/// of available missions which then allow clients to load specialized UIs for each
/// mission type as necessary.
pub enum MissionType {
	/// A demo mission to prove out our concept. Will most likely be removed in the future.
	DemoTakeAPicture,
}
