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

use super::*;
use crate::{self as pallet_allocations};
use frame_support::{
	assert_noop, assert_ok, bounded_vec, ord_parameter_types, parameter_types,
	traits::{ConstU32, GenesisBuild},
	weights::Pays,
	PalletId,
};
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
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
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		Membership: pallet_membership::{Pallet, Call, Storage, Config<T>, Event<T>},
		Allocations: pallet_allocations::{Pallet, Call, Storage},
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
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type OnSetCode = ();
	type SystemWeightInfo = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 2;
	pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}
parameter_types! {
	pub const Oracle: u64 = 0;
	pub const Hacker: u64 = 1;
	pub const Grantee: u64 = 2;
	pub const OtherGrantee: u64 = 3;
	pub const Receiver: u64 = 4;
	pub const CoinsLimit: u64 = 1_000_000;
	pub const Fee: Perbill = Perbill::from_percent(10);
	pub const MaxAllocs: u32 = 10;
	pub const AllocPalletId: PalletId = PalletId(*b"py/alloc");
}
ord_parameter_types! {
	pub const Admin: u64 = 4;
}
impl WithAccountId<u64> for Receiver {
	fn account_id() -> u64 {
		Receiver::get()
	}
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
	type Currency = pallet_balances::Pallet<Self>;
	type PalletId = AllocPalletId;
	type ProtocolFee = Fee;
	type ProtocolFeeReceiver = Receiver;
	type MaximumSupply = CoinsLimit;
	type ExistentialDeposit = <Test as pallet_balances::Config>::ExistentialDeposit;
	type MaxAllocs = MaxAllocs;
	type OracleMembers = Membership;
	type WeightInfo = ();
}
type Errors = Error<Test>;

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
		members: vec![Oracle::get()],
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
fn non_oracle_is_rejected() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Hacker::get()), bounded_vec![(Grantee::get(), 50)]),
			Errors::OracleAccessDenied
		);
	})
}

#[test]
fn oracle_does_not_pay_fees() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![(Grantee::get(), 50)]),
			Ok(Pays::No.into())
		);
	})
}

#[test]
fn simple_allocation_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50)]
		));
		assert_eq!(Balances::free_balance(Grantee::get()), 45);
		assert_eq!(Balances::free_balance(Receiver::get()), 5);

		let alloc_account_id: u64 = AllocPalletId::get().into_account();
		assert_eq!(Balances::free_balance(alloc_account_id), 0);
	})
}

#[test]
fn batched_allocation_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Allocations::batch(
			Origin::signed(Oracle::get()),
			bounded_vec![(Grantee::get(), 50), (OtherGrantee::get(), 50)]
		));
		assert_eq!(Balances::free_balance(Grantee::get()), 45);
		assert_eq!(Balances::free_balance(OtherGrantee::get()), 45);
		assert_eq!(Balances::free_balance(Receiver::get()), 10);

		let alloc_account_id: u64 = AllocPalletId::get().into_account();
		assert_eq!(Balances::free_balance(alloc_account_id), 0);
	})
}

#[test]
fn ensure_issuance_checks() {
	new_test_ext().execute_with(|| {
		let inputs: Vec<BoundedVec<(u64, u64), MaxAllocs>> = vec![
			// overflow checks
			bounded_vec![(Grantee::get(), u64::MAX), (OtherGrantee::get(), 10)],
			// actual issuance checks
			bounded_vec![(Grantee::get(), CoinsLimit::get() + 10)],
			bounded_vec![(Grantee::get(), CoinsLimit::get()), (OtherGrantee::get(), 10)],
		];
		for input in inputs.iter().cloned() {
			assert_noop!(
				Allocations::batch(Origin::signed(Oracle::get()), input),
				Errors::TooManyCoinsToAllocate
			);
		}
	})
}

#[test]
fn ensure_existential_deposit_checks() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![(Grantee::get(), 1)]),
			Errors::DoesNotSatisfyExistentialDeposit
		);
	})
}

#[test]
fn no_issuance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Allocations::batch(Origin::signed(Oracle::get()), bounded_vec![]),
			Errors::BatchEmpty
		);
	})
}

mod deprecated_extrinsic {
	use super::*;

	#[test]
	fn non_oracle_can_not_trigger_allocation() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Allocations::allocate(Origin::signed(Hacker::get()), Grantee::get(), 50, Vec::new(),),
				Errors::OracleAccessDenied
			);
		})
	}

	#[test]
	fn oracle_does_not_pay_fees() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 50, Vec::new(),),
				Ok(Pays::No.into())
			);
		})
	}

	#[test]
	fn oracle_triggers_allocation() {
		new_test_ext().execute_with(|| {
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
	fn hacker_triggers_zero_allocation() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Allocations::allocate(Origin::signed(Hacker::get()), Grantee::get(), 0, Vec::new(),),
				Errors::OracleAccessDenied
			);
		})
	}

	#[test]
	fn allocate_the_right_amount_of_coins_to_everyone() {
		new_test_ext().execute_with(|| {
			assert_ok!(Allocations::allocate(
				Origin::signed(Oracle::get()),
				Grantee::get(),
				50,
				Vec::new(),
			));

			assert_eq!(Balances::free_balance(Grantee::get()), 45);
			assert_eq!(Balances::free_balance(Receiver::get()), 5);
		})
	}

	#[test]
	fn error_if_too_small_for_existential_deposit() {
		new_test_ext().execute_with(|| {
			// grant smaller than deposit
			assert_noop!(
				Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 1, Vec::new()),
				Errors::DoesNotSatisfyExistentialDeposit,
			);

			// grant satisfy deposit but would not be enough for both protocol and user
			assert_noop!(
				Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 2, Vec::new()),
				Errors::DoesNotSatisfyExistentialDeposit,
			);
			assert_noop!(
				Allocations::allocate(Origin::signed(Oracle::get()), Grantee::get(), 3, Vec::new()),
				Errors::DoesNotSatisfyExistentialDeposit,
			);

			assert_eq!(Balances::free_balance(Grantee::get()), 0);
			assert_eq!(Balances::free_balance(Receiver::get()), 0);
		})
	}

	#[test]
	fn can_not_allocate_more_coins_than_max() {
		new_test_ext().execute_with(|| {
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
}
