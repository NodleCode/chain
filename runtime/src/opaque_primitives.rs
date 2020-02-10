//! Opaque types. These are used by the CLI to instantiate machinery that don't need to know
//! the specifics of the runtime. They can then be made to be agnostic over specific formats
//! of data like extrinsics, allowing for them to continue syncing the network through upgrades
//! to even the core datastructures.

use crate::{AuthorityDiscovery, Babe, Grandpa, ImOnline};
#[cfg(feature = "std")]
// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use frame_support::{
    construct_runtime, parameter_types, traits::Randomness, weights::Weight, StorageValue,
};
use sp_runtime::traits::{IdentifyAccount, Verify};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{generic, impl_opaque_keys, traits::BlakeTwo256, MultiSignature};
pub use sp_runtime::{Perbill, Permill};
use sp_std::prelude::*;
pub use timestamp::Call as TimestampCall;

pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub babe: Babe,
        pub grandpa: Grandpa,
        pub im_online: ImOnline,
        pub authority_discovery: AuthorityDiscovery,
    }
}
