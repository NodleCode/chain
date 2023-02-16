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
	constants, implementations::RelayChainBlockNumberProvider, pallets_governance::MoreThanHalfOfTechComm,
	AllocationsOracles, Balances, CompanyReserve, Runtime, RuntimeEvent,
};
use frame_support::{parameter_types, PalletId};
use lazy_static::lazy_static;
use pallet_allocations::MintCurve;
use sp_runtime::Perbill;

lazy_static! {
	static ref EDEN_MINT_CURVE: MintCurve<Runtime> = MintCurve::new(
		constants::DAYS_RELAY_CHAIN,
		91 * constants::DAYS_RELAY_CHAIN + 6 * constants::HOURS_RELAY_CHAIN,
		&[
			Perbill::from_perthousand(1),
			Perbill::from_perthousand(2),
			Perbill::from_perthousand(3),
			Perbill::from_perthousand(4),
			Perbill::from_perthousand(6),
			Perbill::from_perthousand(9),
			Perbill::from_perthousand(11),
			Perbill::from_perthousand(14),
			Perbill::from_perthousand(17),
			Perbill::from_perthousand(20),
			Perbill::from_perthousand(24),
			Perbill::from_perthousand(26),
			Perbill::from_perthousand(32),
			Perbill::from_perthousand(34),
			Perbill::from_perthousand(36),
			Perbill::from_perthousand(37),
			Perbill::from_perthousand(38),
			Perbill::from_perthousand(39),
			Perbill::from_perthousand(39),
			Perbill::from_perthousand(39),
			Perbill::from_perthousand(38),
			Perbill::from_perthousand(38),
			Perbill::from_perthousand(37),
			Perbill::from_perthousand(35),
			Perbill::from_perthousand(34),
			Perbill::from_perthousand(32),
			Perbill::from_perthousand(30),
			Perbill::from_perthousand(28),
			Perbill::from_perthousand(26),
			Perbill::from_perthousand(24),
			Perbill::from_perthousand(22),
			Perbill::from_perthousand(20),
			Perbill::from_perthousand(18),
			Perbill::from_perthousand(16),
			Perbill::from_perthousand(14),
			Perbill::from_perthousand(12),
			Perbill::from_perthousand(11),
			Perbill::from_perthousand(9),
			Perbill::from_perthousand(7),
			Perbill::from_perthousand(6),
			Perbill::from_perthousand(5),
			Perbill::from_perthousand(4),
			Perbill::from_perthousand(3),
			Perbill::from_perthousand(2),
			Perbill::from_perthousand(1),
		],
		21_000_000_000 * constants::NODL
	);
}

parameter_types! {
	pub const ProtocolFee: Perbill = Perbill::from_percent(20);
	pub const AllocPalletId: PalletId = PalletId(*b"py/alloc");
	pub const MaxAllocs: u32 = 500;
	pub EdenMintCurve: &'static MintCurve<Runtime> = &EDEN_MINT_CURVE;
}

impl pallet_allocations::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type PalletId = AllocPalletId;
	type ProtocolFee = ProtocolFee;
	type ProtocolFeeReceiver = CompanyReserve;
	type MintCurve = EdenMintCurve;
	type ExistentialDeposit = <Runtime as pallet_balances::Config>::ExistentialDeposit;
	type MaxAllocs = MaxAllocs;
	type OracleMembers = AllocationsOracles;
	type BlockNumberProvider = RelayChainBlockNumberProvider<Runtime>;
	type WeightInfo = pallet_allocations::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxMembers: u32 = 50;
}

impl pallet_membership::Config<pallet_membership::Instance2> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = MoreThanHalfOfTechComm;
	type RemoveOrigin = MoreThanHalfOfTechComm;
	type SwapOrigin = MoreThanHalfOfTechComm;
	type ResetOrigin = MoreThanHalfOfTechComm;
	type PrimeOrigin = MoreThanHalfOfTechComm;
	type MembershipInitialized = ();
	type MembershipChanged = ();
	type MaxMembers = MaxMembers;
	type WeightInfo = crate::weights::pallet_membership::WeightInfo<Runtime>;
}
