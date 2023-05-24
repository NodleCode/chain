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

#![cfg_attr(not(feature = "std"), no_std)]

//! Handle the ability to notify other pallets that they should stop all



pub use pallet::*;
use pallet_uniques::DestroyWitness;
use sp_runtime::{
	traits::{StaticLookup},
};
use sp_std::prelude::*;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::DispatchResult;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Issue a new collection of non-fungible items from a public origin.
		///
		/// This new collection has no items initially and its owner is the origin.
		///
		/// The origin must conform to the configured `CreateOrigin` and have sufficient funds free.
		///
		/// `ItemDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `collection`: The identifier of the new collection. This must not be currently in use.
		/// - `admin`: The admin of this collection. The admin is the initial address of each
		/// member of the collection's admin team.
		///
		/// Emits `Created` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			admin: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::create(origin, collection, admin)
		}
		/// Issue a new collection of non-fungible items from a privileged origin.
		///
		/// This new collection has no items initially.
		///
		/// The origin must conform to `ForceOrigin`.
		///
		/// Unlike `create`, no funds are reserved.
		///
		/// - `collection`: The identifier of the new item. This must not be currently in use.
		/// - `owner`: The owner of this collection of items. The owner has full superuser
		///   permissions
		/// over this item, but may later change and configure the permissions using
		/// `transfer_ownership` and `set_team`.
		///
		/// Emits `ForceCreated` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn force_create(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			owner: AccountIdLookupOf<T>,
			free_holding: bool,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::force_create(origin, collection, owner, free_holding)
		}

		/// Destroy a collection of fungible items.
		///
		/// The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the
		/// owner of the `collection`.
		///
		/// - `collection`: The identifier of the collection to be destroyed.
		/// - `witness`: Information on the items minted in the collection. This must be
		/// correct.
		///
		/// Emits `Destroyed` event when successful.
		///
		/// Weight: `O(n + m)` where:
		/// - `n = witness.items`
		/// - `m = witness.item_metadatas`
		/// - `a = witness.attributes`
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn destroy(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			witness: DestroyWitness,
		) -> DispatchResultWithPostInfo {
			pallet_uniques::Pallet::<T>::destroy(origin, collection, witness)
		}

		/// Mint an item of a particular collection.
		///
		/// The origin must be Signed and the sender must be the Issuer of the `collection`.
		///
		/// - `collection`: The collection of the item to be minted.
		/// - `item`: The item value of the item to be minted.
		/// - `beneficiary`: The initial owner of the minted item.
		///
		/// Emits `Issued` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn mint(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			owner: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::mint(origin, collection, item, owner)
		}

		/// Destroy a single item.
		///
		/// Origin must be Signed and the signing account must be either:
		/// - the Admin of the `collection`;
		/// - the Owner of the `item`;
		///
		/// - `collection`: The collection of the item to be burned.
		/// - `item`: The item of the item to be burned.
		/// - `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the
		///   item is owned by this value.
		///
		/// Emits `Burned` with the actual amount burned.
		///
		/// Weight: `O(1)`
		/// Modes: `check_owner.is_some()`.
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn burn(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			check_owner: Option<AccountIdLookupOf<T>>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::burn(origin, collection, item, check_owner)
		}

		/// Move an item from the sender account to another.
		///
		/// This resets the approved account of the item.
		///
		/// Origin must be Signed and the signing account must be either:
		/// - the Admin of the `collection`;
		/// - the Owner of the `item`;
		/// - the approved delegate for the `item` (in this case, the approval is reset).
		///
		/// Arguments:
		/// - `collection`: The collection of the item to be transferred.
		/// - `item`: The item of the item to be transferred.
		/// - `dest`: The account to receive ownership of the item.
		///
		/// Emits `Transferred`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			dest: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::transfer(origin, collection, item, dest)
		}

		/// Reevaluate the deposits on some items.
		///
		/// Origin must be Signed and the sender should be the Owner of the `collection`.
		///
		/// - `collection`: The collection to be frozen.
		/// - `items`: The items of the collection whose deposits will be reevaluated.
		///
		/// NOTE: This exists as a best-effort function. Any items which are unknown or
		/// in the case that the owner account does not have reservable funds to pay for a
		/// deposit increase are ignored. Generally the owner isn't going to call this on items
		/// whose existing deposit is less than the refreshed deposit as it would only cost them,
		/// so it's of little consequence.
		///
		/// It will still return an error in the case that the collection is unknown of the signer
		/// is not permitted to call it.
		///
		/// Weight: `O(items.len())`
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn redeposit(origin: OriginFor<T>, collection: T::CollectionId, items: Vec<T::ItemId>) -> DispatchResult {
			pallet_uniques::Pallet::<T>::redeposit(origin, collection, items)
		}

		/// Disallow further unprivileged transfer of an item.
		///
		/// Origin must be Signed and the sender should be the Freezer of the `collection`.
		///
		/// - `collection`: The collection of the item to be frozen.
		/// - `item`: The item of the item to be frozen.
		///
		/// Emits `Frozen`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn freeze(origin: OriginFor<T>, collection: T::CollectionId, item: T::ItemId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::freeze(origin, collection, item)
		}

		/// Re-allow unprivileged transfer of an item.
		///
		/// Origin must be Signed and the sender should be the Freezer of the `collection`.
		///
		/// - `collection`: The collection of the item to be thawed.
		/// - `item`: The item of the item to be thawed.
		///
		/// Emits `Thawed`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(8)]
		#[pallet::weight(0)]
		pub fn thaw(origin: OriginFor<T>, collection: T::CollectionId, item: T::ItemId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::thaw(origin, collection, item)
		}

		/// Disallow further unprivileged transfers for a whole collection.
		///
		/// Origin must be Signed and the sender should be the Freezer of the `collection`.
		///
		/// - `collection`: The collection to be frozen.
		///
		/// Emits `CollectionFrozen`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(9)]
		#[pallet::weight(0)]
		pub fn freeze_collection(origin: OriginFor<T>, collection: T::CollectionId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::freeze_collection(origin, collection)
		}

		/// Re-allow unprivileged transfers for a whole collection.
		///
		/// Origin must be Signed and the sender should be the Admin of the `collection`.
		///
		/// - `collection`: The collection to be thawed.
		///
		/// Emits `CollectionThawed`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(10)]
		#[pallet::weight(0)]
		pub fn thaw_collection(origin: OriginFor<T>, collection: T::CollectionId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::thaw_collection(origin, collection)
		}

		/// Change the Owner of a collection.
		///
		/// Origin must be Signed and the sender should be the Owner of the `collection`.
		///
		/// - `collection`: The collection whose owner should be changed.
		/// - `owner`: The new Owner of this collection. They must have called
		///   `set_accept_ownership` with `collection` in order for this operation to succeed.
		///
		/// Emits `OwnerChanged`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(11)]
		#[pallet::weight(0)]
		pub fn transfer_ownership(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			owner: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::transfer_ownership(origin, collection, owner)
		}

		/// Change the Issuer, Admin and Freezer of a collection.
		///
		/// Origin must be Signed and the sender should be the Owner of the `collection`.
		///
		/// - `collection`: The collection whose team should be changed.
		/// - `issuer`: The new Issuer of this collection.
		/// - `admin`: The new Admin of this collection.
		/// - `freezer`: The new Freezer of this collection.
		///
		/// Emits `TeamChanged`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(12)]
		#[pallet::weight(0)]
		pub fn set_team(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			issuer: AccountIdLookupOf<T>,
			admin: AccountIdLookupOf<T>,
			freezer: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_team(origin, collection, issuer, admin, freezer)
		}

		/// Approve an item to be transferred by a delegated third-party account.
		///
		/// The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be
		/// either the owner of the `item` or the admin of the collection.
		///
		/// - `collection`: The collection of the item to be approved for delegated transfer.
		/// - `item`: The item of the item to be approved for delegated transfer.
		/// - `delegate`: The account to delegate permission to transfer the item.
		///
		/// Important NOTE: The `approved` account gets reset after each transfer.
		///
		/// Emits `ApprovedTransfer` on success.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(13)]
		#[pallet::weight(0)]
		pub fn approve_transfer(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			delegate: AccountIdLookupOf<T>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::approve_transfer(origin, collection, item, delegate)
		}

		/// Cancel the prior approval for the transfer of an item by a delegate.
		///
		/// Origin must be either:
		/// - the `Force` origin;
		/// - `Signed` with the signer being the Admin of the `collection`;
		/// - `Signed` with the signer being the Owner of the `item`;
		///
		/// Arguments:
		/// - `collection`: The collection of the item of whose approval will be cancelled.
		/// - `item`: The item of the item of whose approval will be cancelled.
		/// - `maybe_check_delegate`: If `Some` will ensure that the given account is the one to
		///   which permission of transfer is delegated.
		///
		/// Emits `ApprovalCancelled` on success.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(14)]
		#[pallet::weight(0)]
		pub fn cancel_approval(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			maybe_check_delegate: Option<AccountIdLookupOf<T>>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::cancel_approval(origin, collection, item, maybe_check_delegate)
		}

		/// Alter the attributes of a given item.
		///
		/// Origin must be `ForceOrigin`.
		///
		/// - `collection`: The identifier of the item.
		/// - `owner`: The new Owner of this item.
		/// - `issuer`: The new Issuer of this item.
		/// - `admin`: The new Admin of this item.
		/// - `freezer`: The new Freezer of this item.
		/// - `free_holding`: Whether a deposit is taken for holding an item of this collection.
		/// - `is_frozen`: Whether this collection is frozen except for permissioned/admin
		/// instructions.
		///
		/// Emits `ItemStatusChanged` with the identity of the item.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(15)]
		#[pallet::weight(0)]
		pub fn force_item_status(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			owner: AccountIdLookupOf<T>,
			issuer: AccountIdLookupOf<T>,
			admin: AccountIdLookupOf<T>,
			freezer: AccountIdLookupOf<T>,
			free_holding: bool,
			is_frozen: bool,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::force_item_status(
				origin,
				collection,
				owner,
				issuer,
				admin,
				freezer,
				free_holding,
				is_frozen,
			)
		}

		/// Set an attribute for a collection or item.
		///
		/// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
		/// `collection`.
		///
		/// If the origin is Signed, then funds of signer are reserved according to the formula:
		/// `MetadataDepositBase + DepositPerByte * (key.len + value.len)` taking into
		/// account any already reserved funds.
		///
		/// - `collection`: The identifier of the collection whose item's metadata to set.
		/// - `maybe_item`: The identifier of the item whose metadata to set.
		/// - `key`: The key of the attribute.
		/// - `value`: The value to which to set the attribute.
		///
		/// Emits `AttributeSet`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(16)]
		#[pallet::weight(0)]
		pub fn set_attribute(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			maybe_item: Option<T::ItemId>,
			key: BoundedVec<u8, T::KeyLimit>,
			value: BoundedVec<u8, T::ValueLimit>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_attribute(origin, collection, maybe_item, key, value)
		}

		/// Clear an attribute for a collection or item.
		///
		/// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
		/// `collection`.
		///
		/// Any deposit is freed for the collection's owner.
		///
		/// - `collection`: The identifier of the collection whose item's metadata to clear.
		/// - `maybe_item`: The identifier of the item whose metadata to clear.
		/// - `key`: The key of the attribute.
		///
		/// Emits `AttributeCleared`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(17)]
		#[pallet::weight(0)]
		pub fn clear_attribute(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			maybe_item: Option<T::ItemId>,
			key: BoundedVec<u8, T::KeyLimit>,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::clear_attribute(origin, collection, maybe_item, key)
		}

		/// Set the metadata for an item.
		///
		/// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
		/// `collection`.
		///
		/// If the origin is Signed, then funds of signer are reserved according to the formula:
		/// `MetadataDepositBase + DepositPerByte * data.len` taking into
		/// account any already reserved funds.
		///
		/// - `collection`: The identifier of the collection whose item's metadata to set.
		/// - `item`: The identifier of the item whose metadata to set.
		/// - `data`: The general information of this item. Limited in length by `StringLimit`.
		/// - `is_frozen`: Whether the metadata should be frozen against further changes.
		///
		/// Emits `MetadataSet`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(18)]
		#[pallet::weight(0)]
		pub fn set_metadata(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			item: T::ItemId,
			data: BoundedVec<u8, T::StringLimit>,
			is_frozen: bool,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_metadata(origin, collection, item, data, is_frozen)
		}

		/// Clear the metadata for an item.
		///
		/// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
		/// `item`.
		///
		/// Any deposit is freed for the collection's owner.
		///
		/// - `collection`: The identifier of the collection whose item's metadata to clear.
		/// - `item`: The identifier of the item whose metadata to clear.
		///
		/// Emits `MetadataCleared`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(19)]
		#[pallet::weight(0)]
		pub fn clear_metadata(origin: OriginFor<T>, collection: T::CollectionId, item: T::ItemId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::clear_metadata(origin, collection, item)
		}

		/// Set the metadata for a collection.
		///
		/// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
		/// the `collection`.
		///
		/// If the origin is `Signed`, then funds of signer are reserved according to the formula:
		/// `MetadataDepositBase + DepositPerByte * data.len` taking into
		/// account any already reserved funds.
		///
		/// - `collection`: The identifier of the item whose metadata to update.
		/// - `data`: The general information of this item. Limited in length by `StringLimit`.
		/// - `is_frozen`: Whether the metadata should be frozen against further changes.
		///
		/// Emits `CollectionMetadataSet`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(20)]
		#[pallet::weight(0)]
		pub fn set_collection_metadata(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			data: BoundedVec<u8, T::StringLimit>,
			is_frozen: bool,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_collection_metadata(origin, collection, data, is_frozen)
		}

		/// Clear the metadata for a collection.
		///
		/// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
		/// the `collection`.
		///
		/// Any deposit is freed for the collection's owner.
		///
		/// - `collection`: The identifier of the collection whose metadata to clear.
		///
		/// Emits `CollectionMetadataCleared`.
		///
		/// Weight: `O(1)`
		#[pallet::call_index(21)]
		#[pallet::weight(0)]
		pub fn clear_collection_metadata(origin: OriginFor<T>, collection: T::CollectionId) -> DispatchResult {
			pallet_uniques::Pallet::<T>::clear_collection_metadata(origin, collection)
		}

		/// Set (or reset) the acceptance of ownership for a particular account.
		///
		/// Origin must be `Signed` and if `maybe_collection` is `Some`, then the signer must have a
		/// provider reference.
		///
		/// - `maybe_collection`: The identifier of the collection whose ownership the signer is
		///   willing to accept, or if `None`, an indication that the signer is willing to accept no
		///   ownership transferal.
		///
		/// Emits `OwnershipAcceptanceChanged`.
		#[pallet::call_index(22)]
		#[pallet::weight(0)]
		pub fn set_accept_ownership(origin: OriginFor<T>, maybe_collection: Option<T::CollectionId>) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_accept_ownership(origin, maybe_collection)
		}

		/// Set the maximum amount of items a collection could have.
		///
		/// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
		/// the `collection`.
		///
		/// Note: This function can only succeed once per collection.
		///
		/// - `collection`: The identifier of the collection to change.
		/// - `max_supply`: The maximum amount of items a collection could have.
		///
		/// Emits `CollectionMaxSupplySet` event when successful.
		#[pallet::call_index(23)]
		#[pallet::weight(0)]
		pub fn set_collection_max_supply(
			origin: OriginFor<T>,
			collection: T::CollectionId,
			max_supply: u32,
		) -> DispatchResult {
			pallet_uniques::Pallet::<T>::set_collection_max_supply(origin, collection, max_supply)
		}

		// /// Set (or reset) the price for an item.
		// ///
		// /// Origin must be Signed and must be the owner of the asset `item`.
		// ///
		// /// - `collection`: The collection of the item.
		// /// - `item`: The item to set the price for.
		// /// - `price`: The price for the item. Pass `None`, to reset the price.
		// /// - `buyer`: Restricts the buy operation to a specific account.
		// ///
		// /// Emits `ItemPriceSet` on success if the price is not `None`.
		// /// Emits `ItemPriceRemoved` on success if the price is `None`.
		// #[pallet::call_index(24)]
		// #[pallet::weight(0)]
		// pub fn set_price(
		// 	origin: OriginFor<T>,
		// 	collection: T::CollectionId,
		// 	item: T::ItemId,
		// 	price: Option<ItemPrice<T, I>>,
		// 	whitelisted_buyer: Option<AccountIdLookupOf<T>>,
		// ) -> DispatchResult {
		// 	pallet_uniques::Pallet::<T>::set_price(origin,collection,item,price,whitelisted_buyer)

		// }

		// /// Allows to buy an item if it's up for sale.
		// ///
		// /// Origin must be Signed and must not be the owner of the `item`.
		// ///
		// /// - `collection`: The collection of the item.
		// /// - `item`: The item the sender wants to buy.
		// /// - `bid_price`: The price the sender is willing to pay.
		// ///
		// /// Emits `ItemBought` on success.
		// #[pallet::call_index(25)]
		// #[pallet::weight(0)]
		// #[transactional]
		// pub fn buy_item(
		// 	origin: OriginFor<T>,
		// 	collection: T::CollectionId,
		// 	item: T::ItemId,
		// 	bid_price: ItemPrice<T, I>,
		// ) -> DispatchResult {
		// 	pallet_uniques::Pallet::<T>::buy_item(origin,collection,item,bid_price)
		// }
	}
}
