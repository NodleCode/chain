use super::*;
use crate as pallet_nodle_uniques;

use frame_support::{
	assert_noop, assert_ok, construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64},
};
use pallet_uniques::DestroyWitness;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

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
		Uniques: pallet_nodle_uniques::{Call, Storage},
		Uniques2: pallet_uniques::{Pallet, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type MaxLocks = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	#[doc = " The ID type for holds."]
	type HoldIdentifier = [u8; 8];
	#[doc = " The ID type for freezes."]
	type FreezeIdentifier = [u8; 8];
	#[doc = " The maximum number of holds that can exist on an account at any time."]
	type MaxHolds = ();
	#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
	type MaxFreezes = ();
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
impl Config for Test {
	type WeightInfo = ();
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
#[cfg(test)]
mod test_cases {

	use super::*;
	#[test]
	fn test_mint_with_extra_deposit() {
		new_test_ext().execute_with(|| {
			let extra_deposit = 20;
			let collection_id = 0;
			let item_id = 10;
			let item_id2 = 12;
			let collection_owner_id = 1;
			let item_owner = 42;
			Balances::make_free_balance_be(&collection_owner_id, 100);
			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				collection_owner_id,
				100
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get()
			);
			assert_ok!(Uniques::set_collection_metadata(
				RuntimeOrigin::signed(collection_owner_id),
				0,
				bvec![0, 0],
				false
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));

			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + TestItemDeposit::get() + extra_deposit + 3
			);

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				item_id2,
				item_owner,
				extra_deposit
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + 2 * TestItemDeposit::get() + 2 * extra_deposit + 3
			);
		})
	}

	#[test]
	fn test_mint_and_burn_with_extra_deposit() {
		new_test_ext().execute_with(|| {
			let extra_deposit = 20;
			let collection_id = 0;
			let item_id = 10;
			let collection_owner_id = 1;
			let item_owner = 42;
			let init_balance = 100;
			Balances::make_free_balance_be(&collection_owner_id, init_balance);
			Balances::make_free_balance_be(&item_owner, init_balance);
			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				collection_owner_id,
				100
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get()
			);
			assert_ok!(Uniques::set_collection_metadata(
				RuntimeOrigin::signed(collection_owner_id),
				0,
				bvec![0, 0],
				false
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));

			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + TestItemDeposit::get() + extra_deposit + 3
			);

			assert_ok!(Uniques::burn(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				item_id,
				None
			));

			// check if extra deposit is freed as well as the item deposit
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + 3
			);
			// check that the owner of the collection does not recover the reserved amount of the burnt item
			assert_eq!(
				Balances::free_balance(collection_owner_id),
				init_balance - (TestCollectionDeposit::get() + 3 + extra_deposit)
			);
			// extra deposit transferred to the item owner free balance
			assert_eq!(Balances::free_balance(item_owner), init_balance + extra_deposit);
		})
	}
	#[test]
	fn test_mint_and_burn_wrong_origin_with_extra_deposit() {
		new_test_ext().execute_with(|| {
			let extra_deposit = 20;
			let collection_id = 0;
			let item_id = 10;
			let collection_owner_id = 1;
			let not_collection_owner_id = 255;
			let item_owner = 42;
			let init_balance = 100;
			Balances::make_free_balance_be(&collection_owner_id, init_balance);
			Balances::make_free_balance_be(&item_owner, init_balance);
			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				collection_owner_id,
				100
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get()
			);
			assert_ok!(Uniques::set_collection_metadata(
				RuntimeOrigin::signed(1),
				0,
				bvec![0, 0],
				false
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner_id),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));

			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + TestItemDeposit::get() + extra_deposit + 3
			);

			assert_noop!(
				Uniques::burn(
					RuntimeOrigin::signed(not_collection_owner_id),
					collection_id,
					item_id,
					None
				),
				pallet_uniques::Error::<Test>::NoPermission
			);

			// reserved balance should not have changed
			assert_eq!(
				Balances::reserved_balance(collection_owner_id),
				TestCollectionDeposit::get() + TestItemDeposit::get() + extra_deposit + 3
			);
		})
	}

	#[test]
	fn test_destroy_collection() {
		new_test_ext().execute_with(|| {
			let collection_id = 0;
			let items = [10, 12, 15];
			let extra_deposits = [20, 30, 40];
			let collection_owner = 1;
			let owners = [42, 43];
			let init_balance = 100;

			let total_extra_deposit = extra_deposits.into_iter().reduce(|a, b| a + b).unwrap();

			Balances::make_free_balance_be(&collection_owner, init_balance);
			Balances::make_free_balance_be(&owners[0], init_balance);
			Balances::make_free_balance_be(&owners[1], init_balance);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner,
				100
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner),
				TestCollectionDeposit::get()
			);

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				items[0],
				owners[0],
				extra_deposits[0]
			));
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				items[1],
				owners[0],
				extra_deposits[1]
			));
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				items[2],
				owners[1],
				extra_deposits[2]
			));

			assert_eq!(
				Balances::reserved_balance(collection_owner),
				total_extra_deposit + TestCollectionDeposit::get() + TestItemDeposit::get() * items.len() as u64
			);
			assert_eq!(
				ItemExtraDeposits::<Test>::iter_prefix(collection_id).count(),
				items.len()
			);

			let witness = DestroyWitness {
				items: items.len() as u32,
				item_metadatas: 0,
				attributes: 0,
			};
			assert_ok!(Uniques::destroy(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				witness
			));

			assert_eq!(ItemExtraDeposits::<Test>::iter_prefix(collection_id).count(), 0);
			assert_eq!(Balances::reserved_balance(collection_owner), 0);
			assert_eq!(
				Balances::free_balance(collection_owner),
				init_balance - total_extra_deposit
			);
			assert_eq!(
				Balances::free_balance(owners[0]),
				init_balance + extra_deposits[0] + extra_deposits[1]
			);
			assert_eq!(Balances::free_balance(owners[1]), init_balance + extra_deposits[2]);
		})
	}

	#[test]
	fn test_no_storage_change_happens_if_destroy_fails() {
		new_test_ext().execute_with(|| {
			let extra_deposit = 20;
			let collection_id = 0;
			let item_id = 10;
			let collection_owner = 1;
			let item_owner = 42;
			let init_balance = 100;

			Balances::make_free_balance_be(&collection_owner, init_balance);
			Balances::make_free_balance_be(&item_owner, init_balance);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner,
				100
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));

			// This wrong witness should make the destroy fail
			let witness = DestroyWitness {
				items: 2,
				item_metadatas: 0,
				attributes: 0,
			};

			assert_noop!(
				Uniques::destroy(RuntimeOrigin::signed(1), collection_id, witness),
				pallet_uniques::Error::<Test>::BadWitness
			);

			assert_eq!(
				ItemExtraDeposits::<Test>::get(collection_id, item_id).unwrap(),
				extra_deposit
			);
		})
	}

	#[test]
	fn test_no_storage_change_happens_if_burn_fails() {
		new_test_ext().execute_with(|| {
			let extra_deposit = 20;
			let collection_id = 0;
			let item_id = 10;
			let collection_owner = 1;
			let non_owner = 2;
			let item_owner = 42;
			let init_balance = 100;

			Balances::make_free_balance_be(&collection_owner, init_balance);
			Balances::make_free_balance_be(&item_owner, init_balance);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner,
				100
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));

			assert_noop!(
				Uniques::burn(RuntimeOrigin::signed(non_owner), collection_id, item_id, None),
				pallet_uniques::Error::<Test>::NoPermission
			);

			assert_eq!(
				ItemExtraDeposits::<Test>::get(collection_id, item_id).unwrap(),
				extra_deposit
			);
		})
	}
}
