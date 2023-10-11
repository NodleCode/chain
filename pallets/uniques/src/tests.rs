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
		Uniques: pallet_nodle_uniques::{Call, Storage, Event<T, I>},
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
	type RuntimeEvent = RuntimeEvent;
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
	use frame_support::traits::Len;
	#[test]
	fn test_extra_deposit_limit_is_zero_if_not_set_explicitly() {
		new_test_ext().execute_with(|| {
			let collection_id = 0;
			let collection_owner = 1;
			let item_id = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, 100);

			assert_ok!(Uniques::create(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner,
			));
			assert_noop!(
				Uniques::mint_with_extra_deposit(
					RuntimeOrigin::signed(collection_owner),
					collection_id,
					item_id,
					item_owner,
					1
				),
				Error::<Test>::FailedToIncreaseTotalExtraDeposit
			);
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				item_id,
				item_owner,
				0
			));
			assert_eq!(
				Balances::reserved_balance(collection_owner),
				TestCollectionDeposit::get() + TestItemDeposit::get()
			);
		})
	}

	#[test]
	fn test_extra_deposit_limit_is_set_per_collection() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limits_per_collection = [(0, 53), (1, 143), (2, 0), (3, 1), (4, 71)];
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, u64::MAX);

			let mut total_extra_deposit_limit = 0;
			for (collection, extra_deposit_limit) in extra_deposit_limits_per_collection {
				assert_ok!(Uniques::create_with_extra_deposit_limit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					collection_owner,
					extra_deposit_limit
				));
				assert_noop!(
					Uniques::mint_with_extra_deposit(
						RuntimeOrigin::signed(collection_owner),
						collection,
						item,
						item_owner,
						extra_deposit_limit + 1
					),
					Error::<Test>::FailedToIncreaseTotalExtraDeposit
				);
				assert_ok!(Uniques::mint_with_extra_deposit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					item,
					item_owner,
					extra_deposit_limit
				));
				total_extra_deposit_limit += extra_deposit_limit;
			}

			assert_eq!(
				Balances::reserved_balance(collection_owner),
				(TestCollectionDeposit::get() + TestItemDeposit::get())
					* (extra_deposit_limits_per_collection.len() as u64)
					+ total_extra_deposit_limit
			);
		})
	}

	#[test]
	fn test_destroying_one_collection_does_not_impact_others() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limits_per_collection = [(0, 53), (1, 143), (2, 0), (3, 1), (4, 71)];
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, u64::MAX);

			for (collection, extra_deposit_limit) in extra_deposit_limits_per_collection {
				assert_ok!(Uniques::create_with_extra_deposit_limit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					collection_owner,
					extra_deposit_limit
				));
			}

			let witness = DestroyWitness {
				items: 0,
				item_metadatas: 0,
				attributes: 0,
			};
			assert_ok!(Uniques::destroy(RuntimeOrigin::signed(collection_owner), 1, witness));

			// remaining collections after 1 us destroyed are 0, 2, 3, 4
			let remaining_collections_and_limits = [(0, 53), (2, 0), (3, 1), (4, 71)];

			for (collection, extra_deposit_limit) in remaining_collections_and_limits {
				assert_ok!(Uniques::mint_with_extra_deposit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					item,
					item_owner,
					extra_deposit_limit
				));
			}

			assert_noop!(
				Uniques::mint_with_extra_deposit(RuntimeOrigin::signed(collection_owner), 1, item, item_owner, 1),
				pallet_uniques::Error::<Test>::UnknownCollection
			);
		})
	}

	#[test]
	fn test_extra_deposit_limit_is_maintained_when_minting_several_items() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let collection = 0;
			let collection_owner = 1;
			// The deposits below sum up to 100.
			let items_and_owners_and_deposits = [(0, 2, 53), (1, 3, 35), (2, 4, 0), (3, 5, 12)];

			Balances::make_free_balance_be(&collection_owner, u64::MAX);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));

			for (item, item_owner, deposit) in items_and_owners_and_deposits {
				assert_ok!(Uniques::mint_with_extra_deposit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					item,
					item_owner,
					deposit
				));
			}

			assert_noop!(
				Uniques::mint_with_extra_deposit(RuntimeOrigin::signed(collection_owner), collection, 4, 6, 1),
				Error::<Test>::FailedToIncreaseTotalExtraDeposit
			);
		})
	}

	#[test]
	fn test_extra_deposit_limit_is_updatable_when_over_or_equal_previous_total() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let collection = 0;
			let collection_owner = 1;
			// The deposits below sum up to 100.
			let items_and_owners_and_deposits = [(0, 2, 53), (1, 3, 35), (2, 4, 0)];

			Balances::make_free_balance_be(&collection_owner, u64::MAX);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));

			for (item, item_owner, deposit) in items_and_owners_and_deposits {
				assert_ok!(Uniques::mint_with_extra_deposit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					item,
					item_owner,
					deposit
				));
			}
			let extra_deposit_total = CollectionExtraDepositDetails::<Test>::get(collection)
				.unwrap_or_default()
				.balance();
			assert_noop!(
				Uniques::update_extra_deposit_limit(
					RuntimeOrigin::signed(collection_owner),
					collection,
					extra_deposit_total - 1
				),
				Error::<Test>::FailedToUpdateExtraDepositLimit
			);

			assert_ok!(Uniques::update_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				extra_deposit_total
			));

			assert_noop!(
				Uniques::mint_with_extra_deposit(RuntimeOrigin::signed(collection_owner), collection, 3, 5, 1),
				Error::<Test>::FailedToIncreaseTotalExtraDeposit
			);
		})
	}

	#[test]
	fn test_only_collection_owner_can_update_extra_deposit_limit() {
		new_test_ext().execute_with(|| {
			let collection_id = 0;
			let collection_owner = 1;
			let collection_admin = 2;

			let item_owner = 3;

			Balances::make_free_balance_be(&collection_owner, 100);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_admin,
				20
			));

			assert_noop!(
				Uniques::update_extra_deposit_limit(RuntimeOrigin::signed(collection_admin), collection_id, 30),
				Error::<Test>::PermissionDenied
			);
			assert_noop!(
				Uniques::update_extra_deposit_limit(RuntimeOrigin::signed(item_owner), collection_id, 30),
				Error::<Test>::PermissionDenied
			);

			assert_ok!(Uniques::update_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				30
			));
		})
	}

	#[test]
	fn test_updating_extra_deposit_limit_fails_when_collection_not_exist() {
		new_test_ext().execute_with(|| {
			let collection_id = 0;
			let collection_owner = 1;
			let collection_admin = 2;

			Balances::make_free_balance_be(&collection_owner, 100);

			assert_noop!(
				Uniques::update_extra_deposit_limit(RuntimeOrigin::signed(collection_owner), collection_id, 30),
				Error::<Test>::FailedToRetrieveCollectionOwner
			);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_admin,
				20
			));

			assert_noop!(
				Uniques::update_extra_deposit_limit(RuntimeOrigin::signed(collection_owner), collection_id + 1, 30),
				Error::<Test>::FailedToRetrieveCollectionOwner
			);

			assert_ok!(Uniques::update_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				30
			));
		})
	}

	#[test]
	fn test_extra_deposit_is_updatable_for_collections_without_previous_extra_deposit_details() {
		new_test_ext().execute_with(|| {
			let collection_id = 7;
			let collection_owner = 1;
			let collection_admin = 2;

			Balances::make_free_balance_be(&collection_owner, 100);

			assert_ok!(Uniques::create(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_admin,
			));

			let limit = 30;
			assert_ok!(Uniques::update_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				limit
			));

			assert_eq!(
				CollectionExtraDepositDetails::<Test>::get(collection_id)
					.unwrap()
					.limit(),
				limit
			);
		})
	}

	#[test]
	fn test_transfer_ownership_repatriates_extra_deposit() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let extra_deposit = extra_deposit_limit - 1;

			let collection_id = 0;
			let collection_old_owner = 1;
			let collection_new_owner = 2;

			let old_owner_reserved_balance = extra_deposit + TestCollectionDeposit::get() + TestItemDeposit::get();
			let old_owner_free_balance = 2 * old_owner_reserved_balance;
			let new_owner_free_balance = old_owner_free_balance - 1;

			let item_id = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_old_owner, old_owner_free_balance);
			Balances::make_free_balance_be(&collection_new_owner, new_owner_free_balance);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_old_owner),
				collection_id,
				collection_old_owner,
				extra_deposit_limit
			));
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_old_owner),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));
			assert_eq!(
				Balances::reserved_balance(collection_old_owner),
				old_owner_reserved_balance
			);

			assert_ok!(Uniques::set_accept_ownership(
				RuntimeOrigin::signed(collection_new_owner),
				Some(collection_id),
			));
			assert_ok!(Uniques::transfer_ownership(
				RuntimeOrigin::signed(collection_old_owner),
				collection_id,
				collection_new_owner
			));

			assert_eq!(Balances::reserved_balance(collection_old_owner), 0);
			assert_eq!(
				Balances::free_balance(collection_old_owner),
				old_owner_free_balance - old_owner_reserved_balance
			);
			assert_eq!(
				Balances::reserved_balance(collection_new_owner),
				old_owner_reserved_balance
			);
			assert_eq!(Balances::free_balance(collection_new_owner), new_owner_free_balance);
		})
	}

	#[test]
	fn test_transfer_ownership_to_self() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let extra_deposit = extra_deposit_limit - 1;

			let collection_id = 0;
			let collection_owner = 1;

			let owner_reserved_balance = extra_deposit + TestCollectionDeposit::get() + TestItemDeposit::get();
			let owner_free_balance = 2 * owner_reserved_balance;

			let item_id = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, owner_free_balance);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner,
				extra_deposit_limit
			));
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				item_id,
				item_owner,
				extra_deposit
			));
			assert_eq!(Balances::reserved_balance(collection_owner), owner_reserved_balance);

			assert_ok!(Uniques::set_accept_ownership(
				RuntimeOrigin::signed(collection_owner),
				Some(collection_id),
			));
			assert_ok!(Uniques::transfer_ownership(
				RuntimeOrigin::signed(collection_owner),
				collection_id,
				collection_owner
			));

			assert_eq!(Balances::reserved_balance(collection_owner), owner_reserved_balance);
			assert_eq!(
				Balances::free_balance(collection_owner),
				owner_free_balance - owner_reserved_balance
			);
		})
	}

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
	fn test_burn_when_extra_deposit_is_zero() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let collection = 0;
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, u64::MAX);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				item,
				item_owner,
				0
			));

			assert_ok!(Uniques::burn(RuntimeOrigin::signed(item_owner), collection, item, None));
		})
	}

	#[test]
	fn test_burn_goes_ahead_despite_partial_unreserve() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let extra_deposit = 20;
			let collection = 0;
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, 2 * extra_deposit_limit);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				item,
				item_owner,
				extra_deposit
			));

			assert_ok!(Balances::force_unreserve(
				RuntimeOrigin::root(),
				collection_owner,
				extra_deposit
			));

			assert_ok!(Uniques::burn(RuntimeOrigin::signed(item_owner), collection, item, None));
		})
	}

	#[test]
	fn test_destroy_despite_partial_unreserve() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let extra_deposit = 20;
			let collection = 0;
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, 2 * extra_deposit_limit);

			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));

			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				item,
				item_owner,
				extra_deposit
			));

			assert_ok!(Balances::force_unreserve(
				RuntimeOrigin::root(),
				collection_owner,
				extra_deposit
			));

			let witness = DestroyWitness {
				items: 1,
				item_metadatas: 0,
				attributes: 0,
			};
			assert_ok!(Uniques::destroy(
				RuntimeOrigin::signed(collection_owner),
				collection,
				witness
			));
		})
	}

	#[test]
	fn test_destroy_removes_extra_deposit_details() {
		new_test_ext().execute_with(|| {
			let extra_deposit_limit = 100;
			let extra_deposit = 20;
			let collection = 0;
			let collection_owner = 1;
			let item = 0;
			let item_owner = 2;

			Balances::make_free_balance_be(&collection_owner, 3 * extra_deposit_limit);

			assert!(!CollectionExtraDepositDetails::<Test>::contains_key(collection));
			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit
			));
			assert_eq!(
				CollectionExtraDepositDetails::<Test>::get(collection).unwrap().limit(),
				extra_deposit_limit
			);

			assert!(ItemExtraDeposits::<Test>::iter_prefix(collection).count().is_zero());
			assert_ok!(Uniques::mint_with_extra_deposit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				item,
				item_owner,
				extra_deposit
			));
			assert!(!ItemExtraDeposits::<Test>::iter_prefix(collection).count().is_zero());
			assert_eq!(
				CollectionExtraDepositDetails::<Test>::get(collection)
					.unwrap()
					.balance(),
				extra_deposit
			);

			let witness = DestroyWitness {
				items: 1,
				item_metadatas: 0,
				attributes: 0,
			};
			assert_ok!(Uniques::destroy(
				RuntimeOrigin::signed(collection_owner),
				collection,
				witness
			));
			assert!(!CollectionExtraDepositDetails::<Test>::contains_key(collection));
			assert!(ItemExtraDeposits::<Test>::iter_prefix(collection).count().is_zero());

			// Recreate the collection with higher limit
			assert_ok!(Uniques::create_with_extra_deposit_limit(
				RuntimeOrigin::signed(collection_owner),
				collection,
				collection_owner,
				extra_deposit_limit + 1
			));
			assert_eq!(
				CollectionExtraDepositDetails::<Test>::get(collection).unwrap().limit(),
				extra_deposit_limit + 1
			);
			assert_eq!(
				CollectionExtraDepositDetails::<Test>::get(collection)
					.unwrap()
					.balance(),
				0
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
