#![cfg_attr(not(feature = "std"), no_std)]

//! A runtime module to handle help managing validators through the `membership`,
//! support the deletion and addition of validators by a root authority n.

use frame_support::traits::{ChangeMembers, InitializeMembers};
use frame_support::{decl_module, decl_storage};
use session::SessionManager;
use sp_std::prelude::Vec;

/// The module's configuration trait.
pub trait Trait: system::Trait + session::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as AllocationsModule {
        Validators get(validators): Vec<T::AccountId>;
        Flag: bool;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Nothing, just an empty shell for declaration purposes
    }
}

impl<T: Trait> ChangeMembers<T::AccountId> for Module<T> {
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        <Validators<T>>::put(new);

        // Queue the new keys
        <session::Module<T>>::rotate_session();

        // Trigger another rotation so that the queued keys take effect
        Flag::put(true);
    }
}

impl<T: Trait> InitializeMembers<T::AccountId> for Module<T> {
    fn initialize_members(init: &[T::AccountId]) {
        <Validators<T>>::put(init);
        // Shouldn't need a flag update here as this should happen at genesis
    }
}

type SessionIndex = u32; // A shim while waiting for this type to be exposed by `session`
impl<T: Trait> SessionManager<T::AccountId> for Module<T> {
    fn new_session(_: SessionIndex) -> Option<Vec<T::AccountId>> {
        Flag::put(false);
        Some(<Validators<T>>::get())
    }

    fn end_session(_: SessionIndex) {}
}

/// tests for this module
#[cfg(test)]
mod tests {
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
    impl session::Trait for Test {
        type SessionManager = Module<Test>;
        type SessionHandler = TestSessionHandler;
        type ShouldEndSession = Module<Test>;
        type Event = ();
        type Keys = UintAuthorityId;
        type ValidatorId = <Test as system::Trait>::AccountId;
        type ValidatorIdOf = ConvertInto;
        type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    }
    impl Trait for Test {}

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
    fn change_members_sorted_set_flag() {
        new_test_ext().execute_with(|| {
            TestModule::change_members_sorted(&[], &[], &[VALIDATOR]);
            assert_eq!(Flag::get(), true);
        })
    }

    #[test]
    fn new_session_return_members_and_set_flag() {
        new_test_ext().execute_with(|| {
            TestModule::initialize_members(&[VALIDATOR]);
            Flag::put(true);
            assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR]));
            assert_eq!(Flag::get(), false);
        })
    }
}
