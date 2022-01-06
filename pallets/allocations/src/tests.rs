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
use crate::{self as pallet_allocations};
use frame_support::{
    assert_noop, assert_ok, assert_storage_noop, ord_parameter_types, parameter_types,
    weights::Pays,
};
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        EmergencyShutdown: pallet_emergency_shutdown::{Pallet, Call, Storage, Event<T>},
        Allocations: pallet_allocations::{Pallet, Call, Storage, Event<T>},
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
    type BaseCallFilter = frame_support::traits::Everything;
    type OnSetCode = ();
    type SystemWeightInfo = ();
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 2;
    pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type AccountStore = frame_system::Pallet<Test>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}
ord_parameter_types! {
    pub const ShutdownAdmin: u64 = 21;
}
impl pallet_emergency_shutdown::Config for Test {
    type Event = ();
    type ShutdownOrigin = EnsureSignedBy<ShutdownAdmin, u64>;
    type WeightInfo = ();
}

parameter_types! {
    pub const Oracle: u64 = 0;
    pub const Hacker: u64 = 1;
    pub const Grantee: u64 = 2;
    pub const Receiver: u64 = 3;
    pub const CoinsLimit: u64 = 1_000_000;
    pub const Fee: Perbill = Perbill::from_percent(10);
}
impl WithAccountId<u64> for Receiver {
    fn account_id() -> u64 {
        Receiver::get()
    }
}
impl Config for Test {
    type Event = ();
    type Currency = pallet_balances::Pallet<Self>;
    type ProtocolFee = Fee;
    type ProtocolFeeReceiver = Receiver;
    type MaximumCoinsEverAllocated = CoinsLimit;
    type ExistentialDeposit = <Test as pallet_balances::Config>::ExistentialDeposit;
    type WeightInfo = ();
}
type Errors = Error<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn non_oracle_can_not_trigger_allocation() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Allocations::allocate(
                Origin::signed(Hacker::get()),
                Grantee::get(),
                50,
                Vec::new(),
            ),
            Errors::OracleAccessDenied
        );
    })
}

#[test]
fn oracle_does_not_pay_fees() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);
        assert_eq!(
            Allocations::allocate(
                Origin::signed(Oracle::get()),
                Grantee::get(),
                50,
                Vec::new(),
            ),
            Ok(Pays::No.into())
        );
    })
}

#[test]
fn oracle_triggers_allocation() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);
        assert_eq!(Allocations::is_oracle(Oracle::get()), true);

        assert_ok!(Allocations::allocate(
            Origin::signed(Oracle::get()),
            Grantee::get(),
            50,
            Vec::new(),
        ));
    })
}

#[test]
fn oracle_triggers_zero_allocation() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_storage_noop!(assert_ok!(Allocations::allocate(
            Origin::signed(Oracle::get()),
            Grantee::get(),
            0,
            Vec::new(),
        )));
    })
}

#[test]
fn hacker_triggers_zero_allocation() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_noop!(
            Allocations::allocate(Origin::signed(Hacker::get()), Grantee::get(), 0, Vec::new(),),
            Errors::OracleAccessDenied
        );
    })
}

#[test]
fn oracle_triggers_zero_allocation_under_emergency_shutdown() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_ok!(EmergencyShutdown::toggle(Origin::signed(
            ShutdownAdmin::get()
        )));

        assert_noop!(
            Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 0, Vec::new(),),
            Errors::UnderShutdown
        );
    })
}

#[test]
fn allocate_the_right_amount_of_coins_to_everyone() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_eq!(Allocations::coins_consumed(), 0);
        assert_ok!(Allocations::allocate(
            Origin::signed(Oracle::get()),
            Grantee::get(),
            50,
            Vec::new(),
        ));

        assert_eq!(Balances::free_balance(Grantee::get()), 45);
        assert_eq!(Balances::free_balance(Receiver::get()), 5);
        assert_eq!(Allocations::coins_consumed(), 50);
    })
}

#[test]
fn error_if_too_small_for_existential_deposit() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_noop!(
            Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 1, Vec::new()),
            Errors::DoesNotSatisfyExistentialDeposit,
        );

        assert_eq!(Balances::free_balance(Grantee::get()), 0);
        assert_eq!(Balances::free_balance(Receiver::get()), 0);
        assert_eq!(Allocations::coins_consumed(), 0);
    })
}

#[test]
fn do_not_error_if_too_small_for_existential_deposit_but_balance_ok() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        Balances::make_free_balance_be(&Grantee::get(), ExistentialDeposit::get());
        Balances::make_free_balance_be(&Receiver::get(), ExistentialDeposit::get());

        assert_ok!(Allocations::allocate(
            Origin::signed(Oracle::get()),
            Grantee::get(),
            10,
            Vec::new()
        ),);

        assert_eq!(
            Balances::free_balance(Grantee::get()),
            ExistentialDeposit::get().saturating_add(9)
        );
        assert_eq!(
            Balances::free_balance(Receiver::get()),
            ExistentialDeposit::get().saturating_add(1)
        );
        assert_eq!(Allocations::coins_consumed(), 10);
    })
}

#[test]
fn can_not_allocate_more_coins_than_max() {
    new_test_ext().execute_with(|| {
        Allocations::initialize_members(&[Oracle::get()]);

        assert_noop!(
            Allocations::allocate(
                Origin::signed(Oracle::get()),
                Grantee::get(),
                CoinsLimit::get() + 1,
                Vec::new(),
            ),
            Errors::TooManyCoinsToAllocate
        );
    })
}

#[test]
fn emergency_shutdown() {
    new_test_ext().execute_with(|| {
        assert_ok!(EmergencyShutdown::toggle(Origin::signed(
            ShutdownAdmin::get()
        )));
        Allocations::initialize_members(&[Oracle::get()]);

        assert_noop!(
            Allocations::allocate(
                Origin::signed(Oracle::get()),
                Grantee::get(),
                42,
                Vec::new(),
            ),
            Errors::UnderShutdown
        );
    })
}
