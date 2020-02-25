use super::*;

use frame_support::{
    assert_noop, assert_ok, impl_outer_origin, parameter_types, traits::Imbalance, weights::Weight,
};
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
impl balances::Trait for Test {
    type Balance = u64;
    type OnNewAccount = ();
    type OnReapAccount = ();
    type Event = ();
    type TransferPayment = ();
    type DustRemoval = ();
    type ExistentialDeposit = ();
    type CreationFee = ();
}
parameter_types! {
    pub const MinimumStash: u64 = 100;
    pub const SlashReward: Perbill = Perbill::from_percent(75);
}
impl Trait for Test {
    type Event = ();
    type Currency = balances::Module<Self>;
    type MinimumStash = MinimumStash;
    type SlashReward = SlashReward;
    type RemainingSlashCollector = ();
}

type SessionModule = session::Module<Test>;
type BalancesModule = balances::Module<Test>;
type TestModule = Module<Test>;

type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

pub const VALIDATOR_1: u64 = 1;
pub const VALIDATOR_2: u64 = 2;
pub const VALIDATOR_3: u64 = 3;

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
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR_1]);
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get());
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        SessionModule::rotate_session();
        let queued_keys = SessionModule::queued_keys();
        assert_eq!(queued_keys.len(), 1);
        assert_eq!(queued_keys[0].0, VALIDATOR_1);

        SessionModule::rotate_session();
        assert_eq!(SessionModule::validators(), vec![VALIDATOR_1]);
    })
}

#[test]
fn change_members_sorted_and_stake() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR_1]);
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get());
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR_1]));
    })
}

#[test]
fn new_session_return_only_members_with_valid_stake() {
    new_test_ext().execute_with(|| {
        // V1 stake OK
        // V2 stake INF.
        // V3 no stake but free balance

        let mut total_imbalance_v1 = <PositiveImbalanceOf<Test>>::zero();
        let mut total_imbalance_v2 = <PositiveImbalanceOf<Test>>::zero();
        let mut total_imbalance_v3 = <PositiveImbalanceOf<Test>>::zero();

        let r1 = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get());
        let r2 = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_2, MinimumStash::get());
        let r3 = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_3, MinimumStash::get());

        total_imbalance_v1.subsume(r1);
        total_imbalance_v2.subsume(r2);
        total_imbalance_v3.subsume(r3);

        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_2),
            MinimumStash::get() - 1
        ));

        TestModule::initialize_members(&[VALIDATOR_1, VALIDATOR_2, VALIDATOR_3]);
        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR_1]));
    })
}

#[test]
fn can_not_stake_if_already_has_stash() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get() * 2);
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_noop!(
            TestModule::stake(Origin::signed(VALIDATOR_1), MinimumStash::get()),
            Error::<Test>::StashAlreadyExists
        );
    })
}

#[test]
fn can_not_stake_dust() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get() * 2);
        total_imbalance.subsume(r);

        assert_noop!(
            TestModule::stake(
                Origin::signed(VALIDATOR_1),
                BalancesModule::minimum_balance()
            ),
            Error::<Test>::InsufficientValue
        );
    })
}

#[test]
fn can_not_stake_not_enough_free_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TestModule::stake(Origin::signed(VALIDATOR_1), MinimumStash::get()),
            Error::<Test>::NotEnoughFunds
        );
    })
}

#[test]
fn stake_extra_works() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get() * 2);
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_ok!(TestModule::stake_extra(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_eq!(
            Stashes::<Test>::get(VALIDATOR_1).total,
            MinimumStash::get() * 2
        );
    })
}

#[test]
fn can_not_stake_extra_no_stash_found() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get() * 2);
        total_imbalance.subsume(r);

        assert_noop!(
            TestModule::stake_extra(Origin::signed(VALIDATOR_1), MinimumStash::get()),
            Error::<Test>::StashNotFound
        );
    })
}

#[test]
fn can_not_stake_extra_not_enough_free_balance() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get() * 2);
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_noop!(
            TestModule::stake_extra(Origin::signed(VALIDATOR_1), MinimumStash::get() * 2),
            Error::<Test>::NotEnoughFunds
        );
    })
}

#[test]
fn new_session_execute_unstaking() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR_1]);
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get());
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR_1]));

        // Now that the validator is selected, let's try to unstake
        assert_ok!(TestModule::unstake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));
        assert_eq!(PendingUnstaking::<Test>::get().len(), 1);
        assert_eq!(TestModule::new_session(0), Some(vec![]));

        // Verify the storage status too
        assert_eq!(Stashes::<Test>::exists(VALIDATOR_1), false);
        assert_eq!(
            BalancesModule::free_balance(VALIDATOR_1),
            MinimumStash::get()
        );
        assert_eq!(PendingUnstaking::<Test>::get().len(), 0);
    })
}

#[test]
fn can_not_unstake_if_no_stash() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TestModule::unstake(Origin::signed(VALIDATOR_1), MinimumStash::get()),
            Error::<Test>::StashNotFound
        );
    })
}

#[test]
fn can_not_unstake_more_than_staked() {
    new_test_ext().execute_with(|| {
        let mut total_imbalance = <PositiveImbalanceOf<Test>>::zero();
        let r = <Test as Trait>::Currency::deposit_creating(&VALIDATOR_1, MinimumStash::get());
        total_imbalance.subsume(r);
        assert_ok!(TestModule::stake(
            Origin::signed(VALIDATOR_1),
            MinimumStash::get()
        ));

        assert_noop!(
            TestModule::unstake(Origin::signed(VALIDATOR_1), MinimumStash::get() * 2),
            Error::<Test>::TooBigUnstake
        );
    })
}
