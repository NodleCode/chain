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

// Tests for AssetRegistry Pallet
use crate::*;
use mock::*;

use frame_support::{assert_noop, assert_ok};

#[test]
fn registering_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_eq!(
            AssetRegistry::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(1)
        );
        assert_eq!(
            AssetRegistry::asset_type_id(MockAssetType::MockAsset(1)).unwrap(),
            1
        );
        expect_events(vec![crate::Event::AssetRegistered {
            asset_id: 1,
            asset_type: MockAssetType::MockAsset(1),
        }])
    });
}

#[test]
fn test_asset_exists_error() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_eq!(
            AssetRegistry::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(1)
        );
        assert_noop!(
            AssetRegistry::register_asset(
                Origin::root(),
                MockAssetType::MockAsset(1).into(),
                MockAssetType::MockAsset(1),
            ),
            Error::<Test>::AssetAlreadyExists
        );
    });
}

#[test]
fn test_root_can_change_units_per_second() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_eq!(
            AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
            200
        );
        assert!(
            AssetRegistry::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
        );

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
        ])
    });
}

#[test]
fn test_regular_user_cannot_call_extrinsics() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AssetRegistry::register_asset(
                Origin::signed(1),
                MockAssetType::MockAsset(1).into(),
                MockAssetType::MockAsset(1),
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        assert_noop!(
            AssetRegistry::update_asset_units_per_second(
                Origin::signed(1),
                MockAssetType::MockAsset(1),
                200u128.into(),
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        assert_noop!(
            AssetRegistry::update_asset_type(Origin::signed(1), 1, MockAssetType::MockAsset(2),),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_root_can_change_asset_id_type() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetRegistry::update_asset_type(
            Origin::root(),
            1,
            MockAssetType::MockAsset(2),
        ));

        // New one contains the new asset type units per second
        assert_eq!(
            AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(2)).unwrap(),
            200
        );

        // Old one does not contain units per second
        assert!(AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        // New associations are stablished
        assert_eq!(
            AssetRegistry::asset_id_type(1).unwrap(),
            MockAssetType::MockAsset(2)
        );
        assert_eq!(
            AssetRegistry::asset_type_id(MockAssetType::MockAsset(2)).unwrap(),
            1
        );

        // Old ones are deleted
        assert!(AssetRegistry::asset_type_id(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::AssetTypeUpdated {
                asset_id: 1,
                new_asset_type: MockAssetType::MockAsset(2),
            },
        ])
    });
}

#[test]
fn test_change_units_per_second_after_setting_it_once() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_eq!(
            AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
            200
        );
        assert!(
            AssetRegistry::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
        );

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            100u128.into(),
        ));

        assert_eq!(
            AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
            100
        );
        assert!(
            AssetRegistry::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
        );

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 100,
            },
        ]);
    });
}

#[test]
fn test_root_can_change_units_per_second_and_then_remove() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_eq!(
            AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).unwrap(),
            200
        );
        assert!(
            AssetRegistry::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
        );

        assert_ok!(AssetRegistry::remove_fee_payment_asset(
            Origin::root(),
            MockAssetType::MockAsset(1),
        ));

        assert!(
            !AssetRegistry::supported_fee_payment_assets().contains(&MockAssetType::MockAsset(1))
        );

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::FeePaymentAssetRemoved {
                asset_type: MockAssetType::MockAsset(1),
            },
        ]);
    });
}

#[test]
fn test_weight_hint_error() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetRegistry::remove_fee_payment_asset(
            Origin::root(),
            MockAssetType::MockAsset(1)
        ));
    });
}

#[test]
fn test_asset_id_non_existent_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AssetRegistry::update_asset_units_per_second(
                Origin::root(),
                MockAssetType::MockAsset(1),
                200u128.into(),
            ),
            Error::<Test>::AssetDoesNotExist
        );
        assert_noop!(
            AssetRegistry::update_asset_type(Origin::root(), 1, MockAssetType::MockAsset(2),),
            Error::<Test>::AssetDoesNotExist
        );
    });
}

#[test]
fn test_root_can_remove_asset_association() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            MockAssetType::MockAsset(1),
            200u128.into(),
        ));

        assert_ok!(AssetRegistry::deregister_asset(Origin::root(), 1,));

        // Mappings are deleted
        assert!(AssetRegistry::asset_type_id(MockAssetType::MockAsset(1)).is_none());
        assert!(AssetRegistry::asset_id_type(1).is_none());

        // Units per second removed
        assert!(AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::UnitsPerSecondUpdated {
                asset_type: MockAssetType::MockAsset(1),
                units_per_second: 200,
            },
            crate::Event::AssetDeregisteredd {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
        ])
    });
}

#[test]
fn test_removing_without_asset_units_per_second_does_not_panic() {
    new_test_ext().execute_with(|| {
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            MockAssetType::MockAsset(1).into(),
            MockAssetType::MockAsset(1),
        ));

        assert_ok!(AssetRegistry::deregister_asset(Origin::root(), 1,));

        // Mappings are deleted
        assert!(AssetRegistry::asset_type_id(MockAssetType::MockAsset(1)).is_none());
        assert!(AssetRegistry::asset_id_type(1).is_none());

        // Units per second removed
        assert!(AssetRegistry::asset_type_units_per_second(MockAssetType::MockAsset(1)).is_none());

        expect_events(vec![
            crate::Event::AssetRegistered {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
            crate::Event::AssetDeregisteredd {
                asset_id: 1,
                asset_type: MockAssetType::MockAsset(1),
            },
        ])
    });
}
