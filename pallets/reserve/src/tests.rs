use super::*;

use frame_support::{
    assert_noop, assert_ok, impl_outer_origin, ord_parameter_types, parameter_types,
    traits::Imbalance, weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchError::BadOrigin,
    Perbill,
};
use system::EnsureSignedBy;

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
    type AccountData = balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
}
impl balances::Trait for Test {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ();
    type AccountStore = system::Module<Test>;
}

ord_parameter_types! {
    pub const Admin: u64 = 1;
}
impl Trait for Test {
    type Event = ();
    type Currency = balances::Module<Self>;
    type ExternalOrigin = EnsureSignedBy<Admin, u64>;
}
type TestModule = Module<Test>;
type Balances = balances::Module<Test>;

type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn spend_error_if_bad_origin() {
    new_test_ext().execute_with(|| {
        assert_noop!(TestModule::spend(Origin::signed(0), 1, 1), BadOrigin);
    })
}

#[test]
fn spend_funds_to_target() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&TestModule::account_id(), 100);
        total_imbalance.subsume(r);

        assert_eq!(Balances::free_balance(TestModule::account_id()), 100);
        assert_eq!(Balances::free_balance(3), 0);
        assert_ok!(TestModule::spend(Origin::signed(Admin::get()), 3, 100));
        assert_eq!(Balances::free_balance(3), 100);
        assert_eq!(Balances::free_balance(TestModule::account_id()), 0);
    })
}

#[test]
fn tip() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&999, 100);
        total_imbalance.subsume(r);

        assert_ok!(TestModule::tip(Origin::signed(999), 50));
        assert_eq!(Balances::free_balance(999), 50);
        assert_eq!(Balances::free_balance(TestModule::account_id()), 50);
    })
}
