//! A runtime module to manage vested grants, allows `ExternalOrigin` to create or delete
//! grants.

use frame_support::{
	decl_module, dispatch::DispatchResult, traits::{Currency, VestingCurrency}
};
use sp_runtime::traits::EnsureOrigin;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	type ExternalOrigin: EnsureOrigin<Self::Origin>;
	type Currency: Currency<Self::AccountId>
		+ VestingCurrency<Self::AccountId, Moment=Self::BlockNumber>;
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Adds a vesting schedule to a given account.
		fn add_vesting_schedule(origin,
			who: T::AccountId,
			locked: BalanceOf<T>,
			per_block: BalanceOf<T>,
			starting_block: T::BlockNumber
		) -> DispatchResult {
			T::ExternalOrigin::ensure_origin(origin)?;
			T::Currency::add_vesting_schedule(&who, locked, per_block, starting_block)
		}

		/// Remove a vesting schedule for a given account.
		fn remove_vesting_schedule(origin, who: T::AccountId) {
			T::ExternalOrigin::ensure_origin(origin)?;
			T::Currency::remove_vesting_schedule(&who)
		}
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use system::EnsureSignedBy;
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill, DispatchError::BadOrigin,
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

	parameter_types! {
		pub const Admin: u64 = 1;
	}
	impl Trait for Test {
		type Currency = balances::Module<Self>;
		type ExternalOrigin = EnsureSignedBy<Admin, u64>;
	}
	type TestModule = Module<Test>;
	type Balances = balances::Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn non_external_origin_cannot_create_vesting_schedule() {
		new_test_ext().execute_with(|| {
			assert_noop!(TestModule::add_vesting_schedule(Origin::signed(0), 0, 0, 0, 0), BadOrigin);
		})
	}

	#[test]
	fn non_external_origin_cannot_remove_vesting_schedule() {
		new_test_ext().execute_with(|| {
			assert_noop!(TestModule::remove_vesting_schedule(Origin::signed(0), 0), BadOrigin);
		})
	}

	#[test]
	fn adding_grant() {
		new_test_ext().execute_with(|| {
			// Vesting schedule on account 0
			assert_eq!(Balances::vesting(0).is_some(), false);
			assert_ok!(TestModule::add_vesting_schedule(Origin::signed(1), 0, 2, 2, 2));
			assert_eq!(Balances::vesting(0).is_some(), true);
		})
	}

	#[test]
	fn removing_grant() {
		new_test_ext().execute_with(|| {
			// Vesting schedule on account 0
			assert_ok!(TestModule::add_vesting_schedule(Origin::signed(1), 0, 2, 2, 2));
			assert_ok!(TestModule::remove_vesting_schedule(Origin::signed(1), 0));
			assert_eq!(Balances::vesting(0).is_some(), false);
		})
	}
}
