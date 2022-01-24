use crate as pallet_wnodl;
use frame_support::parameter_types;
use frame_support::traits::Contains;
use frame_system as system;
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
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Wnodl: pallet_wnodl::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u16 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
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
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
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

pub const TRUSTED_ORACLES: [u64; 2] = [1, 3];
pub struct Oracles;
impl Contains<u64> for Oracles {
    fn contains(t: &u64) -> bool {
        return TRUSTED_ORACLES.contains(t);
    }
}

// Make sure there is no overlaps between the known and non-eligible customers.
pub const KNOWN_CUSTOMERS: [u64; 3] = [4, 5, 7];
pub const NON_ELIGIBLE_CUSTOMERS: [u64; 2] = [11, 14];
pub struct KnownCustomers;
impl Contains<u64> for KnownCustomers {
    fn contains(t: &u64) -> bool {
        return KNOWN_CUSTOMERS.contains(t);
    }
}

impl pallet_wnodl::Config for Test {
    type Event = Event;
    type Balance = u64;
    type Currency = Balances;
    type Oracles = Oracles;
    type KnownCustomers = KnownCustomers;
}

pub(crate) fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
