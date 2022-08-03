/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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

use crate::{polkadot_test_net::*, setup::*};

use cumulus_primitives_core::ParaId;
use frame_support::{assert_ok, traits::Currency};
use pallet_traits::ump::{XcmCall, XcmWeightFeeMisc};
use polkadot_parachain::primitives::Sibling;
use primitives::{tokens::*, AccountId, Balance, CurrencyId};
use sp_runtime::{traits::AccountIdConversion, MultiAddress};
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;

pub const RMRK_ASSET_ID: u32 = 8;
pub const RMRK_DECIMAL: u8 = 10;
pub const RMRK_WEIGHT_PER_SEC: u128 = 100000000000;
pub const EDEN_RMRK_ASSET_ID: u32 = 4187061565;
pub const STATEMINT_TOTAL_FEE_AMOUNT: u128 = 1_000_000_000; //still can be decreased further but we add some margin here
pub const FEE_IN_KUSAMA: u128 = 165_940_672;
pub const FEE_IN_STATEMINE: u128 = 10_666_664;
pub const WEIGHT_IN_STATEMINE: u64 = 4_000_000_000;

pub fn rmrk(n: f64) -> Balance {
    (n as u128) * 10u128.pow(RMRK_DECIMAL.into())
}

#[test]
fn statemint() {
    use pallet_traits::xcm::AssetType;
    let statemint_rmrk_asset_location =
        MultiLocation::new(1, X3(Parachain(1000), PalletInstance(50), GeneralIndex(8)));
    let statemint_rmrk_asset_type = AssetType::Xcm(statemint_rmrk_asset_location);
    let statemint_rmrk_asset_id: CurrencyId = statemint_rmrk_asset_type.clone().into();

    NodleNet::execute_with(|| {
        use runtime_eden::{AssetRegistry, Assets, Origin, System};
        assert_eq!(statemint_rmrk_asset_id, EDEN_RMRK_ASSET_ID);

        log::trace!("***NodleNet Pre-Events From Mock Context***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        System::reset_events();

        Assets::force_create(
            Origin::root(),
            EDEN_RMRK_ASSET_ID,
            MultiAddress::Id(AccountId::from(ALICE)),
            true,
            1,
        )
        .unwrap();
        Assets::force_set_metadata(
            Origin::root(),
            EDEN_RMRK_ASSET_ID,
            b"RMRK".to_vec(),
            b"RMRK".to_vec(),
            RMRK_DECIMAL,
            false,
        )
        .unwrap();
        assert_ok!(AssetRegistry::register_asset(
            Origin::root(),
            statemint_rmrk_asset_id,
            statemint_rmrk_asset_type.clone(),
        ));
        assert_ok!(AssetRegistry::update_asset_units_per_second(
            Origin::root(),
            statemint_rmrk_asset_type,
            RMRK_WEIGHT_PER_SEC,
        ));
        log::trace!("***NodleNet Events for RMRK asset***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        System::reset_events();
    });

    StatemintNet::execute_with(|| {
        use statemint_runtime::{Assets, Balances, Origin, PolkadotXcm, System};

        Balances::make_free_balance_be(&ALICE.into(), dot(1f64));

        // need to have some DOT to be able to receive user assets
        let para_acc: AccountId = Sibling::from(2026).into_account();
        log::trace!("Eden para account in sibling chain:{:?}", para_acc);
        Balances::make_free_balance_be(&para_acc, dot(1f64));

        // create assets and set metadata
        Assets::force_create(
            Origin::root(),
            RMRK_ASSET_ID,
            MultiAddress::Id(AccountId::from(ALICE)),
            true,
            1,
        )
        .unwrap();
        Assets::force_set_metadata(
            Origin::root(),
            RMRK_ASSET_ID,
            b"RMRK".to_vec(),
            b"RMRK".to_vec(),
            RMRK_DECIMAL,
            false,
        )
        .unwrap();

        //mint 10 rmrk to alice
        Assets::mint(
            Origin::signed(AccountId::from(ALICE)),
            RMRK_ASSET_ID,
            MultiAddress::Id(AccountId::from(ALICE)),
            rmrk(10f64),
        )
        .unwrap();

        log::trace!("***Statemint Events b4 trans***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("StatemintNet >>> {:?}", r.event));

        System::reset_events();

        //reserve transfer rmrk from statemine to eden
        assert_ok!(PolkadotXcm::reserve_transfer_assets(
            Origin::signed(ALICE.into()).clone(),
            Box::new(MultiLocation::new(1, X1(Parachain(2026))).into()),
            Box::new(
                Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any
                }
                .into()
                .into()
            ),
            Box::new((X2(PalletInstance(50), GeneralIndex(8)), rmrk(2f64)).into()),
            0
        ));

        log::trace!("***Statemint Events after trans***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("StatemintNet >>> {:?}", r.event));
        System::reset_events();
    });
}
