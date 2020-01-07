/// A runtime module to handle Nodle Cash allocations to network
/// contributors, has a list of oracles that can submit Merkle
/// Root Hashes to be paid for.

use rstd::prelude::Vec;
use support::{decl_module, decl_storage, decl_event, ensure, StorageValue, dispatch::Result};
use support::traits::{Currency, OnUnbalanced, Imbalance};
use system::ensure_signed;

use crate::errors;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// Currency minting
	type Currency: Currency<Self::AccountId>;
	type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
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
		pub fn submit_reward(origin, merkle_root_hash: T::Hash, who: T::AccountId, amount: BalanceOf<T>) -> Result {
			Self::ensure_oracle(origin)?;

			let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();
			let r = T::Currency::deposit_into_existing(&who, amount).ok();
			total_imbalance.maybe_subsume(r);
			T::Reward::on_unbalanced(total_imbalance);

			Self::deposit_event(RawEvent::RewardAllocated(who, amount, merkle_root_hash));

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Balance = BalanceOf<T>,
		Hash = <T as system::Trait>::Hash,
	{
		OracleAdded(AccountId),
		OracleRemoved(AccountId),

		RewardAllocated(AccountId, Balance, Hash),
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

	use balances;
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
	impl balances::Trait for Test {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = ();
		type TransferPayment = ();
		type TransactionPayment = ();
		type DustRemoval = ();
	}
	impl Trait for Test {
		type Event = ();

		type Currency = balances::Module<Self>;
		type Reward = ();
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
				AllocationsModule::submit_reward(Origin::signed(NON_ORACLE), H256::random(), REWARD_TARGET, REWARD_AMOUNT),
				errors::ALLOCATIONS_ORACLE_ACCESS_DENIED
			);
		})
	}

	#[test]
	fn oracle_submit_reward() {
		// Init balance check

		with_externalities(&mut new_test_ext(), || {
			assert_ok!(AllocationsModule::submit_reward(Origin::signed(ORACLE), H256::random(), REWARD_TARGET, REWARD_AMOUNT));
		})

		// Verify balance
	}
}
