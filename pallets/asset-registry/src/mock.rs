// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use crate as pallet_asset_registry;
use parity_scale_codec::{Decode, Encode};

use frame_support::{construct_runtime, parameter_types, traits::Everything, RuntimeDebug};
use frame_system::EnsureRoot;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Hash as THash, IdentityLookup},
};
use xcm::latest::prelude::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        AssetRegistry: pallet_asset_registry::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const AssetDeposit: u64 = 1;
    pub const ApprovalDeposit: u64 = 1;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: u64 = 1;
    pub const MetadataDepositPerByte: u64 = 1;
}

parameter_types! {
    pub const StatemineParaIdInfo: u32 = 1000u32;
    pub const StatemineAssetsInstanceInfo: u8 = 50u8;
}

pub type AssetId = u32;
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum MockAssetType {
    MockAsset(AssetId),
    Xcm(MultiLocation),
}

impl Default for MockAssetType {
    fn default() -> Self {
        Self::MockAsset(0)
    }
}

impl From<MockAssetType> for AssetId {
    fn from(asset: MockAssetType) -> AssetId {
        match asset {
            MockAssetType::MockAsset(id) => id,
            MockAssetType::Xcm(id) => {
                let mut result: [u8; 4] = [0u8; 4];
                let hash: H256 = id.using_encoded(<Test as frame_system::Config>::Hashing::hash);
                result.copy_from_slice(&hash.as_fixed_bytes()[0..4]);
                u32::from_le_bytes(result)
            }
        }
    }
}

impl From<MultiLocation> for MockAssetType {
    fn from(location: MultiLocation) -> Self {
        Self::Xcm(location)
    }
}

impl Into<Option<MultiLocation>> for MockAssetType {
    fn into(self) -> Option<MultiLocation> {
        match self {
            Self::Xcm(location) => Some(location),
            _ => None,
        }
    }
}

impl Config for Test {
    type Event = Event;
    type Balance = u64;
    type AssetId = u32;
    type AssetType = MockAssetType;
    type UpdateOrigin = EnsureRoot<u64>;
    type WeightInfo = ();
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub(crate) fn events() -> Vec<super::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::AssetRegistry(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn expect_events(e: Vec<super::Event<Test>>) {
    assert_eq!(events(), e);
}
