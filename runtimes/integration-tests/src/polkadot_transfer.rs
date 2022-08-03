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

use cumulus_primitives_core::ParaId;
use frame_support::{assert_ok, traits::Currency};
use primitives::{tokens::*, AccountId};
use runtime_eden::Assets;
use sp_runtime::traits::AccountIdConversion;
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation};
use xcm_emulator::TestExt;

use crate::{polkadot_test_net::*, setup::*};

#[test]
fn transfer_from_relay_chain() {
    NodleNet::execute_with(|| {
        use runtime_eden::System;
        log::trace!("***Pre-Events From Nodle Mock Context***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        System::reset_events();
        log::trace!("***Pre NodleNet Asset Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Assets::balance(DOT, &AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Assets::balance(DOT, &AccountId::from(BOB)));
    });

    PolkadotNet::execute_with(|| {
        use polkadot_runtime::{Balances, System};
        log::trace!("***Pre-Events From PolkadotNet Mock Context***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("PolkadotNet >>> {:?}", r.event));
        System::reset_events();
        log::trace!("***Pre Polkadot Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Balances::free_balance(AccountId::from(ALICE))
        );
        log::trace!(
            "para id :: {:#?}",
            Balances::free_balance(&ParaId::from(2026 as u32).into_account())
        );

        assert_ok!(polkadot_runtime::XcmPallet::reserve_transfer_assets(
            polkadot_runtime::Origin::signed(ALICE.into()),
            Box::new(VersionedMultiLocation::V1(X1(Parachain(2026)).into())),
            Box::new(VersionedMultiLocation::V1(
                X1(Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any
                })
                .into()
            )),
            Box::new(VersionedMultiAssets::V1((Here, dot(1f64)).into())),
            0,
        ));
    });

    PolkadotNet::execute_with(|| {
        use polkadot_runtime::{Balances, System};
        log::trace!("***Polkadot Events Post Transfer***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("PolkadotNet >>> {:?}", r.event));
        log::trace!("***Post Polkadot Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Balances::free_balance(AccountId::from(ALICE))
        );
        log::trace!(
            "para id :: {:#?}",
            Balances::free_balance(&ParaId::from(2026 as u32).into_account())
        );
    });

    NodleNet::execute_with(|| {
        use runtime_eden::System;
        log::trace!("***Nodle Events Post Transfer***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        assert_eq!(Assets::balance(DOT, &AccountId::from(BOB)), 9_999_904_000);
        //dot fee in nodle is 96_000
        log::trace!("***Post NodleNet Asset Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Assets::balance(DOT, &AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Assets::balance(DOT, &AccountId::from(BOB)));
    });
}

#[test]
fn transfer_to_relay_chain() {
    use runtime_eden::{Origin, XTokens};

    NodleNet::execute_with(|| {
        use runtime_eden::{Assets, System};
        log::trace!("***Pre-Events From Nodle Mock Context***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        System::reset_events();
        log::trace!("***Pre NodleNet Asset Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Assets::balance(DOT, &AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Assets::balance(DOT, &AccountId::from(BOB)));
    });

    PolkadotNet::execute_with(|| {
        use polkadot_runtime::{Balances, System};
        log::trace!("***Pre Polkadot Events***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("PolkadotNet >>> {:?}", r.event));
        log::trace!("***Pre Polkadot Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Balances::free_balance(AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Balances::free_balance(AccountId::from(BOB)));
        log::trace!(
            "Para id :: {:#?}",
            Balances::free_balance(&ParaId::from(2026 as u32).into_account())
        );
    });

    NodleNet::execute_with(|| {
        assert_ok!(XTokens::transfer(
            Origin::signed(ALICE.into()),
            DOT,
            dot(10f64),
            Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
                1,
                X1(Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any
                })
            ))),
            4_000_000_000
        ));
    });

    NodleNet::execute_with(|| {
        use runtime_eden::{Assets, System};
        log::trace!("***Post Nodle Events***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("NodleNet >>> {:?}", r.event));
        log::trace!("***Post NodleNet Asset Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Assets::balance(DOT, &AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Assets::balance(DOT, &AccountId::from(BOB)));
    });

    PolkadotNet::execute_with(|| {
        use polkadot_runtime::{Balances, System};
        log::trace!("***Post Polkadot Events***");
        System::events()
            .iter()
            .for_each(|r| log::trace!("PolkadotNet >>> {:?}", r.event));
        log::trace!("***Post Polkadot Balance Info***");
        log::trace!(
            "Alice :: {:#?}",
            Balances::free_balance(AccountId::from(ALICE))
        );
        log::trace!("Bob :: {:#?}", Balances::free_balance(AccountId::from(BOB)));
        log::trace!(
            "Para id :: {:#?}",
            Balances::free_balance(&ParaId::from(2026 as u32).into_account())
        );
        assert_eq!(
            polkadot_runtime::Balances::free_balance(&AccountId::from(BOB)),
            99_530_582_548,
        );
    });
}
