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
use crate::{self as pallet_reserve};
use frame_support::{
    assert_noop, assert_ok, ord_parameter_types, parameter_types, traits::Currency,
};
use frame_system::{EnsureSignedBy, RawOrigin};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchError::BadOrigin,
};
use sp_std::prelude::Box;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Config<T>, Storage, Event<T>},
        TestModule: pallet_reserve::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}
parameter_types! {
    pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type MaxLocks = MaxLocks;
    type ExistentialDeposit = ();
    type AccountStore = frame_system::Module<Test>;
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const Admin: u64 = 1;
}
parameter_types! {
    pub const ReserveModuleId: ModuleId = ModuleId(*b"py/resrv");
}
impl Config for Test {
    type Event = ();
    type Currency = pallet_balances::Module<Self>;
    type ExternalOrigin = EnsureSignedBy<Admin, u64>;
    type Call = Call;
    type ModuleId = ReserveModuleId;
}
type TestCurrency = <Test as Config>::Currency;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
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
        TestCurrency::make_free_balance_be(&TestModule::account_id(), 100);

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
        TestCurrency::make_free_balance_be(&999, 100);

        assert_ok!(TestModule::tip(Origin::signed(999), 50));
        assert_eq!(Balances::free_balance(999), 50);
        assert_eq!(Balances::free_balance(TestModule::account_id()), 50);
    })
}

fn make_call(value: u8) -> Box<Call> {
    Box::new(Call::System(frame_system::Call::<Test>::remark(vec![
        value,
    ])))
}

#[test]
fn apply_as_error_if_bad_origin() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TestModule::apply_as(Origin::signed(0), make_call(1)),
            BadOrigin
        );
    })
}

#[test]
fn apply_as_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(TestModule::apply_as(
            Origin::signed(Admin::get()),
            make_call(1)
        ));
    })
}

#[test]
fn try_root_if_not_admin() {
    new_test_ext().execute_with(|| {
        TestCurrency::make_free_balance_be(&TestModule::account_id(), 100);

        assert_ok!(TestModule::spend(RawOrigin::Root.into(), 3, 100));
        assert_ok!(TestModule::apply_as(RawOrigin::Root.into(), make_call(1)));
    })
}
