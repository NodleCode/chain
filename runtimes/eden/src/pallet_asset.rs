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

use crate::{constants::deposit, Balances, Event, Runtime};
use frame_support::{parameter_types, traits::ConstU128};
use frame_system::EnsureRoot;
use primitives::{AccountId, Balance};

pub const UNITS: Balance = 100_000_000_000;
type AssetId = u32;

parameter_types! {
	pub const AssetDeposit: Balance = 1 * UNITS;
	pub const ApprovalDeposit: Balance = 0;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance =  deposit(1, 68);
	pub const MetadataDepositPerByte: Balance =  deposit(0, 1);
}

impl pallet_assets::Config<pallet_assets::Instance1> for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type AssetAccountDeposit = ConstU128<{ deposit(1, 18) }>; // TODO: check this
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}
