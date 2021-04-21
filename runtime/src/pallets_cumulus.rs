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
    Call, Currencies, Event, Origin, ParachainInfo, ParachainSystem, Runtime, UnknownTokens,
    XcmHandler,
};

use cumulus_primitives_core::relay_chain::Balance as RelayChainBalance;
use frame_support::{parameter_types, traits::Get};
use frame_system::EnsureRoot;
use nodle_chain_primitives::{AccountId, Balance, CurrencyId};
use orml_xcm_support::NativePalletAssetOr;
use orml_xcm_support::{CurrencyIdConverter, IsConcreteWithGeneralKey, MultiCurrencyAdapter};
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::Convert;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_std::{collections::btree_set::BTreeSet, prelude::*};
use xcm::v0::{Junction, MultiLocation, NetworkId};
use xcm_builder::{
    AccountId32Aliases, LocationInverter, ParentIsDefault, RelayChainAsNative,
    SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
    SovereignSignedViaLocation,
};
use xcm_executor::{Config, XcmExecutor};

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type Event = Event;
    type OnValidationData = ();
    type SelfParaId = parachain_info::Module<Runtime>;
    type DownwardMessageHandlers = ();
    type HrmpMessageHandlers = ();
}

pub struct RelayToNative;
impl Convert<RelayChainBalance, Balance> for RelayToNative {
    fn convert(val: u128) -> Balance {
        // both native and relay have 12 decimals
        val
    }
}

parameter_types! {
    pub NodleNetwork: NetworkId = NetworkId::Named("nodle".into());
    pub RelayChainOrigin: Origin = cumulus_pallet_xcm_handler::Origin::Relay.into();
    pub Ancestry: MultiLocation = MultiLocation::X1(Junction::Parachain {
        id: ParachainInfo::get().into(),
    });
    pub const RelayChainCurrencyId: CurrencyId = CurrencyId::DOT;
}

pub type LocationConverter = (
    ParentIsDefault<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<NodleNetwork, AccountId>,
);

pub type LocalAssetTransactor = MultiCurrencyAdapter<
    Currencies,
    UnknownTokens,
    IsConcreteWithGeneralKey<CurrencyId, RelayToNative>,
    LocationConverter,
    AccountId,
    CurrencyIdConverter<CurrencyId, RelayChainCurrencyId>,
    CurrencyId,
>;

pub type LocalOriginConverter = (
    SovereignSignedViaLocation<LocationConverter, Origin>,
    RelayChainAsNative<RelayChainOrigin, Origin>,
    SiblingParachainAsNative<cumulus_pallet_xcm_handler::Origin, Origin>,
    SignedAccountId32AsNative<NodleNetwork, Origin>,
);

parameter_types! {
    pub NativeReserveTokens: BTreeSet<(Vec<u8>, MultiLocation)> = {
        let mut t = BTreeSet::new();
        //TODO: might need to add other assets based on orml-tokens
        t.insert(("AUSD".into(), (Junction::Parent, Junction::Parachain { id: 666 }).into()));
        t
    };
}

pub struct XcmConfig;
impl Config for XcmConfig {
    type Call = Call;
    type XcmSender = XcmHandler;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = NativePalletAssetOr<NativeReserveTokens>;
    type IsTeleporter = ();
    type LocationInverter = LocationInverter<Ancestry>;
}

impl cumulus_pallet_xcm_handler::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type UpwardMessageSender = ParachainSystem;
    type HrmpMessageSender = ParachainSystem;
    type SendXcmOrigin = EnsureRoot<AccountId>;
    type AccountIdConverter = LocationConverter;
}
