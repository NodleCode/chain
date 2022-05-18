/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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

use sp_std::prelude::*;

use codec::Encode;
use frame_support::{
	assert_noop, assert_ok, ord_parameter_types, parameter_types, traits::EqualPrivilegeOnly, weights::Weight,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError::BadOrigin,
	Perbill,
};

use crate::{self as amendments, Config};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>},
		Amendments: amendments::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1_000_000);
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
	// type AccountData = pallet_balances::AccountData<u64>;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type OnSetCode = ();
	type SystemWeightInfo = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}
impl pallet_scheduler::Config for Test {
	type Event = ();
	type Origin = Origin;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type ScheduleOrigin = EnsureRoot<u64>;
	type PalletsOrigin = OriginCaller;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type WeightInfo = ();
	type PreimageProvider = ();
	type NoPreimagePostponement = ();
}

ord_parameter_types! {
	pub const Proposer: u64 = 1;
	pub const Veto: u64 = 2;
	pub const Hacker: u64 = 3;
	pub const BlockDelay: u64 = 10;
}
impl Config for Test {
	type Event = ();
	type Amendment = Call;
	type SubmissionOrigin = EnsureSignedBy<Proposer, u64>;
	type VetoOrigin = EnsureSignedBy<Veto, u64>;
	type Delay = BlockDelay;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

fn make_proposal(value: u64) -> Box<Call> {
	Box::new(Call::System(frame_system::Call::remark { remark: value.encode() }))
}

#[test]
fn non_authorized_origin_cannot_trigger_amendment() {
	new_test_ext().execute_with(|| {
		let proposal = make_proposal(1);
		assert_noop!(Amendments::propose(Origin::signed(Hacker::get()), proposal), BadOrigin,);
	})
}

#[test]
fn call_gets_registered_correctly() {
	new_test_ext().execute_with(|| {
		let proposal = make_proposal(1);
		assert_ok!(Amendments::propose(Origin::signed(Proposer::get()), proposal,));
	})
}

#[test]
fn non_veto_origin_cannot_veto() {
	new_test_ext().execute_with(|| {
		assert_noop!(Amendments::veto(Origin::signed(Hacker::get()), 0), BadOrigin);
	})
}

#[test]
fn veto_proposal_before_delay_expired() {
	new_test_ext().execute_with(|| {
		let proposal = make_proposal(1);
		assert_ok!(Amendments::propose(Origin::signed(Proposer::get()), proposal,));

		assert_ok!(Amendments::veto(Origin::signed(Veto::get()), 0));
	})
}
