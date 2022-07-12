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

use super::*;
use crate::{self as pallet_poa};
use frame_support::{
	assert_ok, ord_parameter_types, parameter_types,
	traits::{ConstU32, GenesisBuild},
};
use frame_system::EnsureSignedBy;
use sp_core::{crypto::key_types, H256};
use sp_runtime::{
	testing::{Header, UintAuthorityId},
	traits::{BlakeTwo256, ConvertInto, IdentityLookup, OpaqueKeys},
	KeyTypeId, Perbill,
};

pub(crate) type AccountId = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		SessionModule: pallet_session::{Pallet, Call, Config<T>, Storage, Event},
		Membership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		TestModule: pallet_poa::{Pallet, Storage},
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
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
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
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}
pub type AuthorityId = u64;
pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AuthorityId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

	fn on_new_session<Ks: OpaqueKeys>(
		_changed: bool,
		_validators: &[(AuthorityId, Ks)],
		_queued_validators: &[(AuthorityId, Ks)],
	) {
	}

	fn on_disabled(_validator_index: u32) {}

	fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AuthorityId, Ks)]) {}
}
impl pallet_session::ShouldEndSession<u64> for TestSessionHandler {
	fn should_end_session(_now: u64) -> bool {
		false
	}
}
impl pallet_session::Config for Test {
	type SessionManager = Pallet<Test>;
	type SessionHandler = TestSessionHandler;
	type ShouldEndSession = TestSessionHandler;
	type NextSessionRotation = ();
	type Event = Event;
	type Keys = UintAuthorityId;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ConvertInto;
	type WeightInfo = ();
}
ord_parameter_types! {
	pub const Admin: u64 = 4;
}
impl pallet_membership::Config for Test {
	type Event = Event;
	type AddOrigin = EnsureSignedBy<Admin, u64>;
	type RemoveOrigin = EnsureSignedBy<Admin, u64>;
	type SwapOrigin = EnsureSignedBy<Admin, u64>;
	type ResetOrigin = EnsureSignedBy<Admin, u64>;
	type PrimeOrigin = EnsureSignedBy<Admin, u64>;
	type MembershipInitialized = ();
	type MembershipChanged = ();
	type MaxMembers = ConstU32<10>;
	type WeightInfo = ();
}

impl Config for Test {
	type ValidatorsSet = Membership;
}

parameter_types! {
	pub const Validator01: AccountId = 1;
	pub const Validator02: AccountId = 2;
	pub const Validator03: AccountId = 3;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_tracing::try_init_simple();

	let mut storage = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap_or_else(|err| {
			panic!(
				"new_test_ext:[{:#?}] - FrameSystem GenesisConfig Err:[{:#?}]!!!",
				line!(),
				err
			)
		});

	let _ = pallet_membership::GenesisConfig::<Test> {
		members: vec![Validator01::get(), Validator02::get(), Validator03::get()],
		..Default::default()
	}
	.assimilate_storage(&mut storage)
	.map_err(|err| {
		panic!(
			"new_test_ext:[{:#?}] - Membership GenesisConfig Err [{:#?}]!!!",
			line!(),
			err
		);
	});

	let mut ext = sp_io::TestExternalities::from(storage);

	ext.execute_with(|| {
		System::set_block_number(1);
	});

	ext
}

#[test]
fn validators_update_propagate() {
	new_test_ext().execute_with(|| {
		assert_eq!(SessionModule::validators().len(), 0);
		System::inc_providers(&1); // set_keys adds 1 consumer which needs 1 provider
		assert_ok!(SessionModule::set_keys(Origin::signed(1), UintAuthorityId(1), vec![]));

		SessionModule::rotate_session();
		let queued_keys = SessionModule::queued_keys();
		assert_eq!(queued_keys.len(), 1);
		assert_eq!(queued_keys[0].0, Validator01::get());

		SessionModule::rotate_session();
		assert_eq!(SessionModule::validators(), vec![Validator01::get()]);
	})
}

#[test]
fn new_session_return_members() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			TestModule::new_session(0),
			Some(vec![Validator01::get(), Validator02::get(), Validator03::get()]),
		);
	})
}
