/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![cfg(test)]

use super::*;

use frame_support::{
    assert_noop, assert_ok, impl_outer_origin, parameter_types,
    traits::{Currency, OnFinalize},
    weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
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
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
}
parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}
impl pallet_balances::Trait for Test {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type AccountStore = system::Module<Test>;
    type ExistentialDeposit = ();
}
parameter_types! {
    pub const MinimumApplicationAmount: u64 = 100;
    pub const MinimumCounterAmount: u64 = 1000;
    pub const MinimumChallengeAmount: u64 = 10000;
    pub const FinalizeApplicationPeriod: u64 = 100;
    pub const FinalizeChallengePeriod: u64 = 101; // Happens later to ease unit tests
    pub const LoosersSlash: Perbill = Perbill::from_percent(50);
}
impl pallet_tcr::Trait for Test {
    type Event = ();
    type Currency = pallet_balances::Module<Self>;
    type MinimumApplicationAmount = MinimumApplicationAmount;
    type MinimumCounterAmount = MinimumCounterAmount;
    type MinimumChallengeAmount = MinimumChallengeAmount;
    type FinalizeApplicationPeriod = FinalizeApplicationPeriod;
    type FinalizeChallengePeriod = FinalizeChallengePeriod;
    type LoosersSlash = LoosersSlash;
    type ChangeMembers = TestModule;
}
parameter_types! {
    pub const SlotBookingCost: u64 = 1000;
    pub const SlotRenewingCost: u64 = 10000;
    pub const SlotValidity: u64 = 100000;
}
impl Trait for Test {
    type Event = ();
    type Currency = pallet_balances::Module<Self>;
    type CertificateId = <Test as system::Trait>::AccountId;
    type SlotBookingCost = SlotBookingCost;
    type SlotRenewingCost = SlotRenewingCost;
    type SlotValidity = SlotValidity;
    type FundsCollector = ();
}

type BalancesModule = pallet_balances::Module<Test>;
type TcrModule = pallet_tcr::Module<Test>;
type TestModule = Module<Test>;
type TestCurrency = <Test as Trait>::Currency;

const ROOT_MANAGER: u64 = 1;
const OFFCHAIN_CERTIFICATE_SIGNER_1: u64 = 2;
const OFFCHAIN_CERTIFICATE_SIGNER_2: u64 = 3;
const OFFCHAIN_CERTIFICATE_SIGNER_3: u64 = 4;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

fn allocate_balances() {
    TestCurrency::make_free_balance_be(
        &ROOT_MANAGER,
        MinimumApplicationAmount::get() + SlotBookingCost::get() + SlotRenewingCost::get(),
    );
}

fn do_register() {
    assert_ok!(TcrModule::apply(
        Origin::signed(ROOT_MANAGER),
        vec![],
        MinimumApplicationAmount::get(),
    ));
    <TcrModule as OnFinalize<<Test as system::Trait>::BlockNumber>>::on_finalize(
        FinalizeApplicationPeriod::get() + <system::Module<Test>>::block_number(),
    );
}

#[test]
fn tcr_membership_propagate() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_eq!(TestModule::is_member(&ROOT_MANAGER), true);
        assert_eq!(TestModule::is_member(&OFFCHAIN_CERTIFICATE_SIGNER_1), false);
    })
}

#[test]
fn non_member_can_not_buy_slots() {
    new_test_ext().execute_with(|| {
        allocate_balances();

        assert_noop!(
            TestModule::book_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_1),
            Error::<Test>::NotAMember
        );
    })
}

#[test]
fn can_not_buy_slot_twice() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));
        assert_noop!(
            TestModule::book_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_1),
            Error::<Test>::SlotTaken
        );
    })
}

#[test]
fn can_not_buy_slot_if_not_enough_funds() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        BalancesModule::make_free_balance_be(&ROOT_MANAGER, 0);

        assert_noop!(
            TestModule::book_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_2),
            Error::<Test>::NotEnoughFunds
        );
    })
}

#[test]
fn member_can_buy_slots() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).key,
            OFFCHAIN_CERTIFICATE_SIGNER_1
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).owner,
            ROOT_MANAGER
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).created,
            <system::Module<Test>>::block_number()
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).renewed,
            <system::Module<Test>>::block_number()
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).revoked,
            false
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).validity,
            SlotValidity::get(),
        );
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).child_revocations,
            vec![],
        );

        assert_eq!(
            BalancesModule::free_balance(ROOT_MANAGER),
            MinimumApplicationAmount::get() + SlotRenewingCost::get()
        ); // Took SlotBookingCost
    })
}

#[test]
fn root_certificate_is_valid() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            true
        );
    })
}

#[test]
fn root_certificate_not_valid_if_revoked() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        let now = <system::Module<Test>>::block_number();
        <Slots<Test>>::insert(
            &OFFCHAIN_CERTIFICATE_SIGNER_1,
            RootCertificate {
                owner: ROOT_MANAGER,
                key: OFFCHAIN_CERTIFICATE_SIGNER_1,
                created: now,
                renewed: now,
                revoked: true,
                validity: SlotValidity::get(),
                child_revocations: vec![],
            },
        );

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );
    })
}

#[test]
fn root_certificate_not_valid_if_expired() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        <system::Module<Test>>::set_block_number(SlotValidity::get() + 1);

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );
    })
}

#[test]
fn root_certificate_not_valid_if_owner_is_no_longer_a_member() {
    new_test_ext().execute_with(|| {
        let now = <system::Module<Test>>::block_number();
        <Slots<Test>>::insert(
            &OFFCHAIN_CERTIFICATE_SIGNER_1,
            RootCertificate {
                owner: ROOT_MANAGER,
                key: OFFCHAIN_CERTIFICATE_SIGNER_1,
                created: now,
                renewed: now,
                revoked: false,
                validity: SlotValidity::get(),
                child_revocations: vec![],
            },
        );

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );
    })
}

#[test]
fn root_certificate_not_valid_if_does_not_exists() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );
    })
}

#[test]
fn child_certificate_still_valid_if_revoked_under_non_parent_certificate() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        let now = <system::Module<Test>>::block_number();
        <Slots<Test>>::insert(
            &OFFCHAIN_CERTIFICATE_SIGNER_3,
            RootCertificate {
                owner: ROOT_MANAGER,
                key: OFFCHAIN_CERTIFICATE_SIGNER_3,
                created: now,
                renewed: now,
                revoked: false,
                validity: SlotValidity::get(),
                child_revocations: vec![OFFCHAIN_CERTIFICATE_SIGNER_2],
            },
        );

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            true
        );

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_3),
            true
        );

        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            true
        );
    })
}

#[test]
fn child_certificate_not_valid_if_revoked_in_root_certificate() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        let now = <system::Module<Test>>::block_number();
        <Slots<Test>>::insert(
            &OFFCHAIN_CERTIFICATE_SIGNER_1,
            RootCertificate {
                owner: ROOT_MANAGER,
                key: OFFCHAIN_CERTIFICATE_SIGNER_1,
                created: now,
                renewed: now,
                revoked: false,
                validity: SlotValidity::get(),
                child_revocations: vec![OFFCHAIN_CERTIFICATE_SIGNER_2],
            },
        );

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            true
        );

        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            false
        );
    })
}

#[test]
fn child_certificate_not_valid_if_root_certificate_not_valid() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );

        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            false
        );
    })
}

#[test]
fn child_certificate_is_valid() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            true
        );
    })
}

#[test]
fn child_invalid_if_equal_root() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_1
            ),
            false
        );
    })
}

#[test]
fn renew_update_fields() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_ok!(TestModule::renew_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));
        assert_eq!(
            TestModule::slots(OFFCHAIN_CERTIFICATE_SIGNER_1).renewed,
            <system::Module<Test>>::block_number()
        );
        assert_eq!(
            BalancesModule::free_balance(ROOT_MANAGER),
            MinimumApplicationAmount::get()
        ); // Took SlotBookingCost + SlotRenewingCost
    })
}

#[test]
fn can_not_renew_if_not_owner() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_noop!(
            TestModule::renew_slot(
                Origin::signed(OFFCHAIN_CERTIFICATE_SIGNER_1),
                OFFCHAIN_CERTIFICATE_SIGNER_1
            ),
            Error::<Test>::NotTheOwner
        );
    })
}

#[test]
fn can_not_renew_if_invalid() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        <system::Module<Test>>::set_block_number(SlotValidity::get() + 1);

        assert_noop!(
            TestModule::renew_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_1),
            Error::<Test>::NoLongerValid
        );
    })
}

#[test]
fn can_not_renew_if_not_enough_funds() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        BalancesModule::make_free_balance_be(&ROOT_MANAGER, 0);

        assert_noop!(
            TestModule::renew_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_1),
            Error::<Test>::NotEnoughFunds
        );
    })
}

#[test]
fn revoke_slot_works() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_ok!(TestModule::revoke_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            false
        );
        assert_eq!(
            <Slots<Test>>::get(&OFFCHAIN_CERTIFICATE_SIGNER_1).revoked,
            true
        );
        assert_eq!(TestModule::is_root_certificate_valid(&ROOT_MANAGER), false);
    })
}

#[test]
fn can_not_revoke_slot_if_not_owner() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_noop!(
            TestModule::revoke_slot(
                Origin::signed(OFFCHAIN_CERTIFICATE_SIGNER_1),
                OFFCHAIN_CERTIFICATE_SIGNER_1
            ),
            Error::<Test>::NotTheOwner
        );
    })
}

#[test]
fn can_not_revoke_slot_if_not_valid_anymore() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        // Best to way to make it invalid would be to revoke it once already!
        assert_ok!(TestModule::revoke_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_noop!(
            TestModule::revoke_slot(Origin::signed(ROOT_MANAGER), OFFCHAIN_CERTIFICATE_SIGNER_1),
            Error::<Test>::NoLongerValid
        );
    })
}

#[test]
fn revoke_child_works() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_ok!(TestModule::revoke_child(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1,
            OFFCHAIN_CERTIFICATE_SIGNER_2
        ));

        assert_eq!(
            TestModule::is_root_certificate_valid(&OFFCHAIN_CERTIFICATE_SIGNER_1),
            true
        );
        assert_eq!(
            TestModule::is_child_certificate_valid(
                &OFFCHAIN_CERTIFICATE_SIGNER_1,
                &OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            false
        );
        assert_eq!(
            <Slots<Test>>::get(&OFFCHAIN_CERTIFICATE_SIGNER_1)
                .child_revocations
                .contains(&OFFCHAIN_CERTIFICATE_SIGNER_2),
            true
        );
    })
}

#[test]
fn can_not_revoke_child_if_not_owner() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_noop!(
            TestModule::revoke_child(
                Origin::signed(OFFCHAIN_CERTIFICATE_SIGNER_1),
                OFFCHAIN_CERTIFICATE_SIGNER_1,
                OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            Error::<Test>::NotTheOwner
        );
    })
}

#[test]
fn can_not_revoke_child_if_root_not_valid_anymore() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_ok!(TestModule::revoke_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_noop!(
            TestModule::revoke_child(
                Origin::signed(ROOT_MANAGER),
                OFFCHAIN_CERTIFICATE_SIGNER_1,
                OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            Error::<Test>::NoLongerValid
        );
    })
}

#[test]
fn can_not_revoke_child_if_not_valid_anymore() {
    new_test_ext().execute_with(|| {
        allocate_balances();
        do_register();

        assert_ok!(TestModule::book_slot(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1
        ));

        assert_ok!(TestModule::revoke_child(
            Origin::signed(ROOT_MANAGER),
            OFFCHAIN_CERTIFICATE_SIGNER_1,
            OFFCHAIN_CERTIFICATE_SIGNER_2
        ));

        assert_noop!(
            TestModule::revoke_child(
                Origin::signed(ROOT_MANAGER),
                OFFCHAIN_CERTIFICATE_SIGNER_1,
                OFFCHAIN_CERTIFICATE_SIGNER_2
            ),
            Error::<Test>::NoLongerValid
        );
    })
}
