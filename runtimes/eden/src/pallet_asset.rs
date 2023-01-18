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

use crate::{Event, Runtime};
use frame_support::{parameter_types, traits::EnsureRoot, PalletId};

parameter_types! {
    pub const AssetDeposit: Balance = 1 * DOLLARS;
    pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 1 * DOLLARS;
    pub const MetadataDepositPerByte: Balance = 1 * CENTS;
}
impl pallet_assets::Config<pallet_assets::Instance1> for Runtime{
    type Event = Event;
    type Balance = Balance;
    type AssetId = AssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}