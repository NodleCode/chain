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

//! # Asset registry pallet
//!
//! This pallet allows to register new assets
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub mod tests;
pub mod weights;
pub use pallet::*;
pub use weights::WeightInfo;

#[pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::HasCompact;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Asset Id. This will be used to register the asset in Assets
        type AssetId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// The Asset Kind.
        type AssetType: Parameter + Member + Ord + PartialOrd + Into<Self::AssetId> + Default;

        /// The units in which we record balances.
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

        /// Origin that is allowed to create and modify asset information
        type UpdateOrigin: EnsureOrigin<Self::Origin>;

        type WeightInfo: WeightInfo;
    }

    /// An error that can occur while executing the mapping pallet's logic.
    #[pallet::error]
    pub enum Error<T> {
        AssetAlreadyExists,
        AssetDoesNotExist,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New asset with the asset manager is registered
        AssetRegistered {
            asset_id: T::AssetId,
            asset_type: T::AssetType,
        },
        /// Changed the amount of units we are charging per execution second for a given asset
        UnitsPerSecondUpdated {
            asset_type: T::AssetType,
            units_per_second: u128,
        },
        /// Changed the xcm type mapping for a given asset id
        AssetTypeUpdated {
            asset_id: T::AssetId,
            new_asset_type: T::AssetType,
        },
        /// Removed all information related to an assetId
        AssetDeregisteredd {
            asset_id: T::AssetId,
            asset_type: T::AssetType,
        },
        /// Supported asset type for fee payment removed
        FeePaymentAssetRemoved { asset_type: T::AssetType },
    }

    /// Mapping from an asset id to asset type.
    /// This is mostly used when receiving transaction specifying an asset directly,
    /// like transferring an asset from this chain to another.
    #[pallet::storage]
    #[pallet::getter(fn asset_id_type)]
    pub type AssetIdType<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::AssetType>;

    /// Reverse mapping of AssetIdType. Mapping from an asset type to an asset id.
    /// This is mostly used when receiving a multilocation XCM message to retrieve
    /// the corresponding asset in which tokens should me minted.
    #[pallet::storage]
    #[pallet::getter(fn asset_type_id)]
    pub type AssetTypeId<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetType, T::AssetId>;

    /// Stores the units per second for local execution for a AssetType.
    /// This is used to know how to charge for XCM execution in a particular
    /// asset
    /// Not all assets might contain units per second, hence the different storage
    #[pallet::storage]
    #[pallet::getter(fn asset_type_units_per_second)]
    pub type AssetTypeUnitsPerSecond<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AssetType, u128>;

    // Supported fee asset payments
    #[pallet::storage]
    #[pallet::getter(fn supported_fee_payment_assets)]
    pub type SupportedFeePaymentAssets<T: Config> = StorageValue<_, Vec<T::AssetType>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register new asset with the asset registry
        #[pallet::weight(T::WeightInfo::register_asset())]
        pub fn register_asset(
            origin: OriginFor<T>,
            asset_id: T::AssetId,
            asset_type: T::AssetType,
        ) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            ensure!(
                !AssetIdType::<T>::contains_key(&asset_id),
                Error::<T>::AssetAlreadyExists
            );

            AssetIdType::<T>::insert(&asset_id, &asset_type);
            AssetTypeId::<T>::insert(&asset_type, &asset_id);

            Self::deposit_event(Event::AssetRegistered {
                asset_id,
                asset_type,
            });
            Ok(())
        }

        /// Change the amount of units we are charging per execution second for a given AssetType
        #[pallet::weight(T::WeightInfo::update_asset_units_per_second())]
        pub fn update_asset_units_per_second(
            origin: OriginFor<T>,
            asset_type: T::AssetType,
            units_per_second: u128,
        ) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            ensure!(
                AssetTypeId::<T>::contains_key(&asset_type),
                Error::<T>::AssetDoesNotExist
            );

            // Grab supported assets
            let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

            // Only if the asset is not supported we need to push it
            if let Err(next_index) = supported_assets.binary_search(&asset_type) {
                supported_assets.insert(next_index, asset_type.clone());
                SupportedFeePaymentAssets::<T>::put(supported_assets);
            }

            AssetTypeUnitsPerSecond::<T>::insert(&asset_type, &units_per_second);

            Self::deposit_event(Event::UnitsPerSecondUpdated {
                asset_type,
                units_per_second,
            });
            Ok(())
        }

        /// Change the xcm type mapping for a given assetId
        /// We also change this if the previous units per second where pointing at the old
        /// assetType
        #[pallet::weight(T::WeightInfo::update_asset_type())]
        pub fn update_asset_type(
            origin: OriginFor<T>,
            asset_id: T::AssetId,
            new_asset_type: T::AssetType,
        ) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            // Grab supported assets
            let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

            let previous_asset_type =
                AssetIdType::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

            // Insert new asset type info
            AssetIdType::<T>::insert(&asset_id, &new_asset_type);
            AssetTypeId::<T>::insert(&new_asset_type, &asset_id);

            // Remove previous asset type info
            AssetTypeId::<T>::remove(&previous_asset_type);

            // Change AssetTypeUnitsPerSecond
            if let Some(units) = AssetTypeUnitsPerSecond::<T>::get(&previous_asset_type) {
                // Only if the old asset is supported we need to remove it
                if let Ok(index) = supported_assets.binary_search(&previous_asset_type) {
                    supported_assets.remove(index);
                }

                // Only if the new asset is not supported we need to push it
                if let Err(index) = supported_assets.binary_search(&new_asset_type) {
                    supported_assets.insert(index, new_asset_type.clone());
                }

                // Insert supported fee payment assets
                SupportedFeePaymentAssets::<T>::put(supported_assets);

                // Remove previous asset type info
                AssetTypeUnitsPerSecond::<T>::remove(&previous_asset_type);
                AssetTypeUnitsPerSecond::<T>::insert(&new_asset_type, units);
            }

            Self::deposit_event(Event::AssetTypeUpdated {
                asset_id,
                new_asset_type,
            });
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::remove_fee_payment_asset())]
        pub fn remove_fee_payment_asset(
            origin: OriginFor<T>,
            asset_type: T::AssetType,
        ) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            // Grab supported assets
            let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

            // Only if the old asset is supported we need to remove it
            if let Ok(index) = supported_assets.binary_search(&asset_type) {
                supported_assets.remove(index);
            }

            // Insert
            SupportedFeePaymentAssets::<T>::put(supported_assets);

            // Remove
            AssetTypeUnitsPerSecond::<T>::remove(&asset_type);

            Self::deposit_event(Event::FeePaymentAssetRemoved { asset_type });
            Ok(())
        }

        /// Remove a given assetId -> assetType association
        #[pallet::weight(T::WeightInfo::deregister_asset())]
        pub fn deregister_asset(origin: OriginFor<T>, asset_id: T::AssetId) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            // Grab supported assets
            let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

            let asset_type =
                AssetIdType::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

            // Remove from AssetIdType
            AssetIdType::<T>::remove(&asset_id);
            // Remove from AssetTypeId
            AssetTypeId::<T>::remove(&asset_type);
            // Remove previous asset type units per second
            AssetTypeUnitsPerSecond::<T>::remove(&asset_type);

            // Only if the old asset is supported we need to remove it
            if let Ok(index) = supported_assets.binary_search(&asset_type) {
                supported_assets.remove(index);
            }

            // Insert
            SupportedFeePaymentAssets::<T>::put(supported_assets);

            Self::deposit_event(Event::AssetDeregisteredd {
                asset_id,
                asset_type,
            });
            Ok(())
        }
    }
}

// We implement this trait to be able to get the AssetType and units per second registered
impl<T: Config> pallet_traits::xcm::AssetTypeGetter<T::AssetId, T::AssetType> for Pallet<T> {
    fn get_asset_type(asset_id: T::AssetId) -> Option<T::AssetType> {
        AssetIdType::<T>::get(asset_id)
    }

    fn get_asset_id(asset_type: T::AssetType) -> Option<T::AssetId> {
        AssetTypeId::<T>::get(asset_type)
    }
}

impl<T: Config> pallet_traits::xcm::UnitsToWeightRatio<T::AssetType> for Pallet<T> {
    fn payment_is_supported(asset_type: T::AssetType) -> bool {
        SupportedFeePaymentAssets::<T>::get()
            .binary_search(&asset_type)
            .is_ok()
    }
    fn get_units_per_second(asset_type: T::AssetType) -> Option<u128> {
        AssetTypeUnitsPerSecond::<T>::get(asset_type)
    }
}
