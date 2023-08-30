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

use crate as pallet_sponsorship;
use frame_support::{
	pallet_prelude::{ConstU32, Decode, Encode, MaxEncodedLen, RuntimeDebug},
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU64, ConstU8, InstanceFilter},
	weights::IdentityFee,
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Uniques: pallet_uniques,
		SponsorshipModule: pallet_sponsorship,
		TransactionPayment: pallet_transaction_payment,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type HoldIdentifier = ();
	type MaxHolds = ();
}

impl pallet_transaction_payment::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<u64>;
	type LengthToFee = IdentityFee<u64>;
	type FeeMultiplierUpdate = ();
}

parameter_types! {
	pub TestCollectionDeposit:  u64 = 2;
	pub TestItemDeposit:  u64 = 1;
}
impl pallet_uniques::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type Locker = ();
	type CollectionDeposit = TestCollectionDeposit;
	type ItemDeposit = TestItemDeposit;
	type MetadataDepositBase = ConstU64<1>;
	type AttributeDepositBase = ConstU64<1>;
	type DepositPerByte = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum SponsorshipType {
	AnySafe,
	Balances,
	Uniques,
	UniquesMint,
}
impl InstanceFilter<RuntimeCall> for SponsorshipType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			// Just for the sake of tests we assume sponsoring Balance calls are not safe but anything else is fine.
			SponsorshipType::AnySafe => !matches!(c, RuntimeCall::Balances { .. }),
			SponsorshipType::Balances => matches!(c, RuntimeCall::Balances { .. }),
			SponsorshipType::Uniques => matches!(c, RuntimeCall::Uniques { .. }),
			SponsorshipType::UniquesMint => {
				matches!(c, RuntimeCall::Uniques(pallet_uniques::Call::mint { .. }))
			}
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		(self == &SponsorshipType::AnySafe)
			|| (self == &SponsorshipType::Uniques && o == &SponsorshipType::UniquesMint)
			|| (self == o)
	}
}
impl Default for SponsorshipType {
	fn default() -> Self {
		Self::AnySafe
	}
}

impl pallet_sponsorship::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type PotId = u32;
	type SponsorshipType = SponsorshipType;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
