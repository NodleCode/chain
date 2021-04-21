/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

#![cfg_attr(not(feature = "std"), no_std)]

use crate::{
    pallets_governance::CompanyReserveAccount, Balances, Event, ParachainInfo, Runtime, XcmHandler,
};

use cumulus_primitives_core::relay_chain::Balance as RelayChainBalance;
use frame_support::parameter_types;
use nodle_chain_primitives::{AccountId, Amount, Balance, BlockNumber, CurrencyId};
use orml_currencies::BasicCurrencyAdapter;
use orml_traits::parameter_type_with_key;
use orml_xcm_support::XcmHandler as XcmHandlerT;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
    traits::{Convert, Zero},
    DispatchResult,
};
use xcm::v0::{NetworkId, Xcm};

parameter_types! {
    pub const PolkadotNetworkId: NetworkId = NetworkId::Polkadot;
}

pub struct AccountId32Convert;
impl Convert<AccountId, [u8; 32]> for AccountId32Convert {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

pub struct NativeToRelay;
impl Convert<Balance, RelayChainBalance> for NativeToRelay {
    fn convert(val: u128) -> Balance {
        // both native and relay have 12 decimals
        val
    }
}

pub struct HandleXcm;
impl XcmHandlerT<AccountId> for HandleXcm {
    fn execute_xcm(origin: AccountId, xcm: Xcm) -> DispatchResult {
        XcmHandler::execute_xcm(origin, xcm)
    }
}

impl orml_xtokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type ToRelayChainBalance = NativeToRelay;
    type AccountId32Convert = AccountId32Convert;
    type RelayChainNetworkId = PolkadotNetworkId;
    type ParaId = ParachainInfo;
    type XcmHandler = HandleXcm;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        Zero::zero()
    };
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, CompanyReserveAccount>;
}

parameter_types! {
    pub const GetNodleTokenId: CurrencyId = CurrencyId::NODL;
}

pub type NodleToken = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;

impl orml_currencies::Config for Runtime {
    type Event = Event;
    type MultiCurrency = orml_tokens::Pallet<Runtime>;
    type NativeCurrency = NodleToken;
    type GetNativeCurrencyId = GetNodleTokenId;
    type WeightInfo = ();
}

impl orml_unknown_tokens::Config for Runtime {
    type Event = Event;
}
