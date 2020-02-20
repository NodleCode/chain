use super::*;

use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::{crypto::key_types, H256};
use sp_runtime::{
    testing::{Header, UintAuthorityId},
    traits::{BlakeTwo256, ConvertInto, IdentityLookup, OpaqueKeys},
    KeyTypeId, Perbill,
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
parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}
pub type AuthorityId = u64;
pub struct TestSessionHandler;
impl session::SessionHandler<AuthorityId> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AuthorityId, Ks)],
        _queued_validators: &[(AuthorityId, Ks)],
    ) {
    }

    fn on_disabled(_validator_index: usize) {}

    fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AuthorityId, Ks)]) {}
}
impl session::ShouldEndSession<u64> for TestSessionHandler {
    fn should_end_session(_now: u64) -> bool {
        false
    }
}
impl session::Trait for Test {
    type SessionManager = Module<Test>;
    type SessionHandler = TestSessionHandler;
    type ShouldEndSession = TestSessionHandler;
    type Event = ();
    type Keys = UintAuthorityId;
    type ValidatorId = <Test as system::Trait>::AccountId;
    type ValidatorIdOf = ConvertInto;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
}
impl Trait for Test {}

type SessionModule = session::Module<Test>;
type TestModule = Module<Test>;

pub const VALIDATOR: u64 = 1;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn validators_update_propagate() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR]);

        SessionModule::rotate_session();
        let queued_keys = SessionModule::queued_keys();
        assert_eq!(queued_keys.len(), 1);
        assert_eq!(queued_keys[0].0, VALIDATOR);

        SessionModule::rotate_session();
        assert_eq!(SessionModule::validators(), vec![VALIDATOR]);
    })
}

#[test]
fn change_members_sorted() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR]);
        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR]));
    })
}

#[test]
fn new_session_return_members() {
    new_test_ext().execute_with(|| {
        TestModule::initialize_members(&[VALIDATOR]);
        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR]));
    })
}
