/// A runtime module to handle Nodle Cash allocations to network
/// contributors, has a list of oracles that can submit Merkle
/// Root Hashes to be paid for.

use support::{decl_module, decl_storage, decl_event, ensure, StorageValue, dispatch::Result};
use system::ensure_signed;

use crate::errors;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as AllocationsModule {
		Oracles get(oracles) config(): Vec<T::AccountId>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		// Add an oracle to the list, sudo only
		pub fn add_oracle(oracle: T::AccountId) -> Result {
			ensure!(!Self::is_oracle(oracle.clone()), errors::ALLOCATIONS_ALREADY_ORACLE);

			<Oracles<T>>::mutate(|mem| mem.push(oracle.clone()));
			Self::deposit_event(RawEvent::OracleAdded(oracle));

			Ok(())
		}

		// Remove an oracle to the list, sudo only
		pub fn remove_oracle(oracle: T::AccountId) -> Result {
			ensure!(Self::is_oracle(oracle.clone()), errors::ALLOCATIONS_NOT_ORACLE);

			<Oracles<T>>::mutate(|mem| mem.retain(|m| m != &oracle));
			Self::deposit_event(RawEvent::OracleRemoved(oracle));

			Ok(())
		}

		// As an oracle, submit a merkle root for reward
		pub fn submit_reward(origin, merkle_root_hash: Vec<u8>, who: T::AccountId, amount: u64) -> Result {
			Self::ensure_oracle(origin)?;

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		OracleAdded(AccountId),
		OracleRemoved(AccountId),
	}
);

impl<T: Trait> Module<T> {
	pub fn is_oracle(who: T::AccountId) -> bool {
		Self::oracles().contains(&who)
	}

	fn ensure_oracle(origin: T::Origin) -> Result {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), errors::ALLOCATIONS_ORACLE_ACCESS_DENIED);

		Ok(())
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, assert_noop};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type AllocationsModule = Module<Test>;

	pub const ORACLE: u64 = 0;
	pub const NON_ORACLE: u64 = 1;

	pub const REWARD_TARGET: u64 = 2;
	pub const REWARD_AMOUNT: u64 = 100;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		let mut genesis = system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap()
			.0;

		genesis.extend(
			GenesisConfig::<Test> {
				oracles: vec![ORACLE],
			}
			.build_storage()
			.unwrap()
			.0,
		);

		genesis.into()
	}

	#[test]
	fn remove_add_oracle() {
		with_externalities(&mut new_test_ext(), || {
			assert_ok!(AllocationsModule::remove_oracle(ORACLE));
			assert!(!AllocationsModule::is_oracle(ORACLE));

			assert_ok!(AllocationsModule::add_oracle(ORACLE));
			assert!(AllocationsModule::is_oracle(ORACLE));
		})
	}

	#[test]
	fn can_not_add_oracle_twice() {
		with_externalities(&mut new_test_ext(), || {
			assert_noop!(AllocationsModule::add_oracle(ORACLE), errors::ALLOCATIONS_ALREADY_ORACLE);
		})
	}

	#[test]
	fn can_not_remove_oracle_twice() {
		with_externalities(&mut new_test_ext(), || {
			assert_noop!(AllocationsModule::remove_oracle(NON_ORACLE), errors::ALLOCATIONS_NOT_ORACLE);
		})
	}

	#[test]
	fn non_oracle_can_not_submit_reward() {
		with_externalities(&mut new_test_ext(), || {
			assert_noop!(
				AllocationsModule::submit_reward(Origin::signed(NON_ORACLE), (0..10).collect(), REWARD_TARGET, REWARD_AMOUNT),
				errors::ALLOCATIONS_ORACLE_ACCESS_DENIED
			);
		})
	}

	#[test]
	fn oracle_submit_reward() {
		with_externalities(&mut new_test_ext(), || {
			assert_ok!(AllocationsModule::submit_reward(Origin::signed(ORACLE), (0..10).collect(), REWARD_TARGET, REWARD_AMOUNT));
		})
	}
}
