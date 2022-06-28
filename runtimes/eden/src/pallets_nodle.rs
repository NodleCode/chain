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
use crate::{
	constants, pallets_governance::MoreThanHalfOfTechComm, Allocations, Balances, CompanyReserve, Event, Runtime,
};
use frame_support::{parameter_types, PalletId};
use primitives::Balance;
use sp_runtime::Perbill;

parameter_types! {
	pub const ProtocolFee: Perbill = Perbill::from_percent(20);
	pub const MaximumSupply: Balance = 21_000_000_000 * constants::NODL; // 21B NODL
	pub const AllocPalletId: PalletId = PalletId(*b"py/alloc");
	pub const MaxAllocs: u32 = 500;
}

impl pallet_allocations::Config for Runtime {
	type Currency = Balances;
	type PalletId = AllocPalletId;
	type ProtocolFee = ProtocolFee;
	type ProtocolFeeReceiver = CompanyReserve;
	type MaximumSupply = MaximumSupply;
	type ExistentialDeposit = <Runtime as pallet_balances::Config>::ExistentialDeposit;
	type MaxAllocs = MaxAllocs;
	type WeightInfo = pallet_allocations::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxMembers: u32 = 50;
}

impl pallet_membership::Config<pallet_membership::Instance2> for Runtime {
	type Event = Event;
	type AddOrigin = MoreThanHalfOfTechComm;
	type RemoveOrigin = MoreThanHalfOfTechComm;
	type SwapOrigin = MoreThanHalfOfTechComm;
	type ResetOrigin = MoreThanHalfOfTechComm;
	type PrimeOrigin = MoreThanHalfOfTechComm;
	type MembershipInitialized = Allocations;
	type MembershipChanged = Allocations;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}
