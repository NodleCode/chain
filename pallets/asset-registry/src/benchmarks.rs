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

#![cfg(feature = "runtime-benchmarks")]
use crate::{Call, Config, Pallet};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use xcm::latest::prelude::*;

benchmarks! {
    // This where clause allows us to create assetTypes
    where_clause { where T::AssetType: From<MultiLocation> }
    register_asset {
        // does not really matter what we register
        let asset_type = T::AssetType::default();
        let asset_id: T::AssetId = asset_type.clone().into();
    }: _(RawOrigin::Root, asset_id, asset_type.clone())
    verify {
        assert_eq!(Pallet::<T>::asset_id_type(asset_id), Some(asset_type));
    }

    update_asset_units_per_second {
        let asset_type:  T::AssetType = MultiLocation::new(0, X1(GeneralIndex(0 as u128))).into();
        Pallet::<T>::register_asset(RawOrigin::Root.into(), asset_type.clone().into(), asset_type.clone())?;
        Pallet::<T>::update_asset_units_per_second(RawOrigin::Root.into(), asset_type, 1)?;

        // does not really matter what we register, as long as it is different than the previous
        let asset_type = T::AssetType::default();
        Pallet::<T>::register_asset(RawOrigin::Root.into(),  asset_type.clone().into(), asset_type.clone())?;

    }: _(RawOrigin::Root, asset_type.clone(), 1)
    verify {
        assert!(Pallet::<T>::supported_fee_payment_assets().contains(&asset_type));
        assert_eq!(Pallet::<T>::asset_type_units_per_second(asset_type), Some(1));
    }

    update_asset_type {
        let asset_type:  T::AssetType = MultiLocation::new(0, X1(GeneralIndex(0 as u128))).into();
        Pallet::<T>::register_asset(RawOrigin::Root.into(),  asset_type.clone().into(), asset_type.clone())?;
        Pallet::<T>::update_asset_units_per_second(RawOrigin::Root.into(), asset_type, 1)?;

        let new_asset_type = T::AssetType::default();
        let asset_type_to_be_changed: T::AssetType = MultiLocation::new(
            0,
            X1(GeneralIndex(0 as u128))
        ).into();
        let asset_id_to_be_changed = asset_type_to_be_changed.into();

    }: _(RawOrigin::Root, asset_id_to_be_changed, new_asset_type.clone())
    verify {
        assert_eq!(Pallet::<T>::asset_id_type(asset_id_to_be_changed), Some(new_asset_type.clone()));
        assert_eq!(Pallet::<T>::asset_type_units_per_second(&new_asset_type), Some(1));
        assert!(Pallet::<T>::supported_fee_payment_assets().contains(&new_asset_type));
    }

    remove_fee_payment_asset {
        let asset_type:  T::AssetType = MultiLocation::new(0, X1(GeneralIndex(0 as u128))).into();
        Pallet::<T>::register_asset(RawOrigin::Root.into(), asset_type.clone().into(), asset_type.clone())?;
        Pallet::<T>::update_asset_units_per_second(RawOrigin::Root.into(), asset_type, 1)?;
        let asset_type_to_be_removed: T::AssetType = MultiLocation::new(
            0,
            X1(GeneralIndex(0 as u128))
        ).into();
        // We try to remove the last asset type
    }: _(RawOrigin::Root, asset_type_to_be_removed.clone())
    verify {
        assert!(!Pallet::<T>::supported_fee_payment_assets().contains(&asset_type_to_be_removed));
        assert_eq!(Pallet::<T>::asset_type_units_per_second(asset_type_to_be_removed), None);
    }

    deregister_asset {
        let asset_type:  T::AssetType = MultiLocation::new(0, X1(GeneralIndex(0 as u128))).into();
        Pallet::<T>::register_asset(RawOrigin::Root.into(), asset_type.clone().into(), asset_type.clone())?;
        Pallet::<T>::update_asset_units_per_second(RawOrigin::Root.into(), asset_type, 1)?;

        let asset_type_to_be_removed: T::AssetType = MultiLocation::new(
            0,
            X1(GeneralIndex(0 as u128))
        ).into();
        let asset_id: T::AssetId = asset_type_to_be_removed.clone().into();
    }: _(RawOrigin::Root, asset_id)
    verify {
        assert!(Pallet::<T>::asset_id_type(asset_id).is_none());
        assert!(Pallet::<T>::asset_type_units_per_second(&asset_type_to_be_removed).is_none());
        assert!(!Pallet::<T>::supported_fee_payment_assets().contains(&asset_type_to_be_removed));
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::Test;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        TestExternalities::new(t)
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::benchmarks::tests::new_test_ext(),
    crate::mock::Test
);
