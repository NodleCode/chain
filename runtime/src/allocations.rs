//! A runtime module to handle Nodle Cash allocations to network
//! contributors, has a list of oracles that can submit Merkle
//! Root Hashes to be paid for.

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult, ensure};
use frame_support::traits::{Currency, OnUnbalanced, Imbalance};
use sp_std::prelude::Vec;
use system::{ensure_signed, ensure_root};

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

decl_storage! {
	trait Store for Module<T: Trait> as AllocationsModule {
		Oracles get(oracles) config(): Vec<T::AccountId>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		// Add an oracle to the list, sudo only
		pub fn add_oracle(origin, oracle: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!Self::is_oracle(oracle.clone()), errors::ALLOCATIONS_ALREADY_ORACLE);

			<Oracles<T>>::mutate(|mem| mem.push(oracle.clone()));
			Self::deposit_event(RawEvent::OracleAdded(oracle));

			Ok(())
		}

		// Remove an oracle to the list, sudo only
		pub fn remove_oracle(origin, oracle: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::is_oracle(oracle.clone()), errors::ALLOCATIONS_NOT_ORACLE);

			<Oracles<T>>::mutate(|mem| mem.retain(|m| m != &oracle));
			Self::deposit_event(RawEvent::OracleRemoved(oracle));

			Ok(())
		}

		// As an oracle, submit a merkle root for reward
		pub fn submit_reward(origin, merkle_root_hash: T::Hash, who: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			Self::ensure_oracle(origin)?;

			let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();
			let r = T::Currency::deposit_creating(&who, amount);
			total_imbalance.subsume(r);
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

	fn ensure_oracle(origin: T::Origin) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), errors::ALLOCATIONS_ORACLE_ACCESS_DENIED);

		Ok(())
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl balances::Trait for Test {
		type Balance = u64;
		type OnNewAccount = ();
		type OnFreeBalanceZero = ();
		type Event = ();
		type TransferPayment = ();
		type DustRemoval = ();
		type ExistentialDeposit = ();
		type TransferFee = ();
		type CreationFee = ();
	}
	impl Trait for Test {
		type Event = ();

		type Currency = balances::Module<Self>;
		type Reward = ();
	}
	type AllocationsModule = Module<Test>;

	pub type Balances = balances::Module<Test>;

	pub const ORACLE: u64 = 0;
	pub const NON_ORACLE: u64 = 1;

	pub const REWARD_TARGET: u64 = 2;
	pub const REWARD_AMOUNT: u64 = 100;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		GenesisConfig::<Test>{
			oracles: vec![ORACLE],
		}.assimilate_storage(&mut t).unwrap();
		t.into()
	}

	#[test]
	fn remove_add_oracle() {
		new_test_ext().execute_with(|| {
			assert_ok!(AllocationsModule::remove_oracle(Origin::ROOT, ORACLE));
			assert!(!AllocationsModule::is_oracle(ORACLE));

			assert_ok!(AllocationsModule::add_oracle(Origin::ROOT, ORACLE));
			assert!(AllocationsModule::is_oracle(ORACLE));
		})
	}

	#[test]
	fn can_not_add_oracle_twice() {
		new_test_ext().execute_with(|| {
			assert_noop!(AllocationsModule::add_oracle(Origin::ROOT, ORACLE), errors::ALLOCATIONS_ALREADY_ORACLE);
		})
	}

	#[test]
	fn can_not_remove_oracle_twice() {
		new_test_ext().execute_with(|| {
			assert_noop!(AllocationsModule::remove_oracle(Origin::ROOT, NON_ORACLE), errors::ALLOCATIONS_NOT_ORACLE);
		})
	}

	#[test]
	fn non_oracle_can_not_submit_reward() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				AllocationsModule::submit_reward(Origin::signed(NON_ORACLE), H256::random(), REWARD_TARGET, REWARD_AMOUNT),
				errors::ALLOCATIONS_ORACLE_ACCESS_DENIED
			);
		})
	}

	#[test]
	fn oracle_submit_reward() {
		new_test_ext().execute_with(|| {
			assert_eq!(Balances::free_balance(REWARD_TARGET), 0);
			assert_ok!(AllocationsModule::submit_reward(Origin::signed(ORACLE), H256::random(), REWARD_TARGET, REWARD_AMOUNT));
			assert_eq!(Balances::free_balance(REWARD_TARGET), REWARD_AMOUNT);
		})
	}
}
