use frame_support::{parameter_types, sp_tracing};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		PalletBalances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		ParachainInfo: parachain_info,
	}
);

impl parachain_info::Config for Test {}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

pub type AccountId = u128;
impl frame_system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
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
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 50;
}
type Balance = u64;
impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

#[derive(Default)]
pub struct ExtBuilder {
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		sp_tracing::try_init_simple();

		let storage = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap_or_else(|err| {
				panic!(
					"new_test_ext:[{:#?}] - FrameSystem GenesisConfig Err:[{:#?}]!!!",
					line!(),
					err
				)
			});

		let mut ext = sp_io::TestExternalities::from(storage);

		ext.execute_with(|| {
			System::set_block_number(1);
		});

		ext
	}
}
