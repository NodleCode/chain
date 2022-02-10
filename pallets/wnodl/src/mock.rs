use crate as pallet_wnodl;
use frame_support::{ord_parameter_types, parameter_types, traits::Contains, PalletId};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use support::WithAccountId;
use system::EnsureSignedBy;

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
        Reserve: pallet_reserve::{Pallet, Call, Storage, Event<T>},
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

ord_parameter_types! {
    pub const Admin: u64 = 1;
}
parameter_types! {
    pub const ReserveModuleId: PalletId = PalletId(*b"py/resrv");
}
impl pallet_reserve::Config for Test {
    type Event = Event;
    type Currency = pallet_balances::Pallet<Self>;
    type ExternalOrigin = EnsureSignedBy<Admin, u64>;
    type Call = Call;
    type PalletId = ReserveModuleId;
    type WeightInfo = ();
}

pub const TRUSTED_ORACLES: [u64; 2] = [1, 3];
pub const NON_ELIGIBLE_ORACLES: [u64; 3] = [4, 5, 11];
pub struct Oracles;
impl Contains<u64> for Oracles {
    fn contains(t: &u64) -> bool {
        return TRUSTED_ORACLES.contains(t);
    }
}

// Make sure there is no overlaps between the known and non-eligible customers.
pub const KNOWN_CUSTOMERS: [u64; 4] = [4, 5, 7, 1];
pub const CUSTOMER_BALANCE: u64 = 50u64;
pub const MIN_WRAP_AMOUNT: u64 = 5u64;
pub const MAX_WRAP_AMOUNT: u64 = 100u64;
pub const RESERVE_BALANCE: u64 = 1000u64;

pub const NON_ELIGIBLE_CUSTOMERS: [u64; 2] = [11, 14];
pub struct KnownCustomers;
impl Contains<u64> for KnownCustomers {
    fn contains(t: &u64) -> bool {
        return KNOWN_CUSTOMERS.contains(t);
    }
}

impl pallet_wnodl::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type Oracles = Oracles;
    type KnownCustomers = KnownCustomers;
    type Reserve = Reserve;
    type WeightInfo = ();
}

pub(crate) fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let mut balances: Vec<(u64, u64)> = KNOWN_CUSTOMERS
        .iter()
        .map(|x| (*x, CUSTOMER_BALANCE))
        .collect();
    balances.push((Reserve::account_id(), RESERVE_BALANCE));
    pallet_balances::GenesisConfig::<Test> { balances }
        .assimilate_storage(&mut t)
        .unwrap();

    pallet_wnodl::GenesisConfig::<Test> {
        min_wrapping: MIN_WRAP_AMOUNT,
        max_wrapping: MAX_WRAP_AMOUNT,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
