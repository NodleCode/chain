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
    assert_noop, assert_ok, impl_outer_dispatch, impl_outer_origin, ord_parameter_types,
    parameter_types, weights::Weight,
};
use frame_system::EnsureSignedBy;
use parity_scale_codec::Encode;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Hash, IdentityLookup},
    DispatchError::BadOrigin,
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}
impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        frame_system::System,
    }
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
impl frame_system::Trait for Test {
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
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
}
parameter_types! {
    pub const MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * MaximumBlockWeight::get();
}
impl pallet_scheduler::Trait for Test {
    type Event = ();
    type Origin = Origin;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
}

ord_parameter_types! {
    pub const Proposer: u64 = 1;
    pub const Veto: u64 = 2;
    pub const Accelerator: u64 = 3;
    pub const Hacker: u64 = 4;
    pub const BlockDelay: u64 = 10;
}
impl Trait for Test {
    type Event = ();
    type Amendment = Call;
    type SubmissionOrigin = EnsureSignedBy<Proposer, u64>;
    type VetoOrigin = EnsureSignedBy<Veto, u64>;
    type AccelerationOrigin = EnsureSignedBy<Accelerator, u64>;
    type Delay = BlockDelay;
    type Scheduler = Scheduler;
}

type Amendments = Module<Test>;
type Scheduler = pallet_scheduler::Module<Test>;
type System = frame_system::Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

fn make_proposal(value: u64) -> Call {
    Call::System(frame_system::Call::remark(value.encode()))
}

#[test]
fn non_authorized_origin_cannot_trigger_amendment() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Amendments::propose(Origin::signed(Hacker::get()), make_proposal(1)),
            BadOrigin
        );
    })
}

#[test]
fn call_gets_registered_correctly() {
    new_test_ext().execute_with(|| {
        let amendment = make_proposal(1);
        assert_ok!(Amendments::propose(
            Origin::signed(Proposer::get()),
            amendment.clone()
        ));

        assert_eq!(Amendments::pending_amendments().len(), 1);

        let pending = &Amendments::pending_amendments()[0];
        assert_eq!(pending.amendment, amendment);
    })
}

#[test]
fn non_accelerator_cannot_accelerate_proposal() {}

#[test]
fn accelerator_speeds_up_proposal() {}

#[test]
fn non_veto_origin_cannot_veto() {}

#[test]
fn veto_proposal_before_delay_expired() {}

#[test]
fn call_is_not_executed_when_delay_is_not_met() {}

#[test]
fn call_is_executed_when_delay_is_met() {}
