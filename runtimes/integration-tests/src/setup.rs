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

use frame_support::traits::GenesisBuild;

use primitives::{tokens::*, AccountId, Balance};
use sp_runtime::MultiAddress;

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const DOT_DECIMAL: u32 = 10;

pub fn dot(n: f64) -> Balance {
    (n as u128) * 10u128.pow(DOT_DECIMAL)
}

pub struct ExtBuilder {
    pub parachain_id: u32,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self { parachain_id: 2085 }
    }
}

impl ExtBuilder {
    pub fn parachain_id(mut self, parachain_id: u32) -> Self {
        self.parachain_id = parachain_id;
        self
    }

    pub fn polkadot_build(self) -> sp_io::TestExternalities {
        use runtime_eden::{Assets, Origin, Runtime, System};
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        <parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &parachain_info::GenesisConfig {
                parachain_id: self.parachain_id.into(),
            },
            &mut t,
        )
        .unwrap();

        <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_xcm::GenesisConfig {
                safe_xcm_version: Some(2),
            },
            &mut t,
        )
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| {
            System::set_block_number(1);
            Assets::force_create(
                Origin::root(),
                DOT,
                MultiAddress::Id(AccountId::from(ALICE)),
                true,
                1,
            )
            .unwrap();
            Assets::force_set_metadata(
                Origin::root(),
                DOT,
                b"Polkadot".to_vec(),
                b"DOT".to_vec(),
                12,
                false,
            )
            .unwrap();
            Assets::mint(
                Origin::signed(AccountId::from(ALICE)),
                DOT,
                MultiAddress::Id(AccountId::from(ALICE)),
                dot(100f64),
            )
            .unwrap();
        });
        ext
    }
}
