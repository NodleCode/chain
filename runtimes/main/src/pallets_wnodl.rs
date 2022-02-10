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

use crate::{
    pallets_governance::{FinancialCollective, TechnicalCollective},
    Balances, Event, InternationalReserve, KnownCustomerMembership, Runtime, WnodlOracleMembership,
};
use frame_support::parameter_types;
use primitives::AccountId;
use sp_core::u32_trait::{_1, _2};
pub use sp_runtime::{Perbill, Perquintill};

parameter_types! {
    pub const MaxWnodlOracles: u32 = u32::MAX;
    pub const MaxKnownCustomers: u32 = u32::MAX;
}

impl pallet_membership::Config<pallet_membership::Instance6> for Runtime {
    type Event = Event;
    type AddOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;
    type MembershipInitialized = ();
    type MembershipChanged = ();
    type MaxMembers = MaxWnodlOracles;
    type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

impl pallet_membership::Config<pallet_membership::Instance7> for Runtime {
    type Event = Event;
    type AddOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type RemoveOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type SwapOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type ResetOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type PrimeOrigin =
        pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, FinancialCollective>;
    type MembershipInitialized = ();
    type MembershipChanged = ();
    type MaxMembers = MaxKnownCustomers;
    type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

impl pallet_wnodl::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type Oracles = WnodlOracleMembership;
    type KnownCustomers = KnownCustomerMembership;
    type Reserve = InternationalReserve;
    type WeightInfo = pallet_wnodl::weights::SubstrateWeight<Runtime>;
}
