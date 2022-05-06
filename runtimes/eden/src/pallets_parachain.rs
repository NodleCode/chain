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

use crate::{constants, Call, Event, Origin, ParachainInfo, Runtime};
use frame_support::{dispatch::Weight, match_types, parameter_types};
use primitives::AccountId;

use xcm::latest::prelude::*;
use xcm_builder::{
    AllowUnpaidExecutionFrom, FixedWeightBounds, LocationInverter, ParentAsSuperuser,
    ParentIsPreset, SovereignSignedViaLocation,
};
use xcm_executor::XcmExecutor;

parameter_types! {
    pub const RococoLocation: MultiLocation = MultiLocation::parent();
    pub const RococoNetwork: NetworkId = NetworkId::Polkadot;
    pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// bias the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<ParentIsPreset<AccountId>, Origin>,
    // Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
    // transaction from the Root origin.
    ParentAsSuperuser<Origin>,
);

match_types! {
    pub type JustTheParent: impl Contains<MultiLocation> = { MultiLocation { parents:1, interior: Here } };
}

parameter_types! {
    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = 1_000_000_000;
    pub const MaxInstructions: u32 = 100;
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type Call = Call;
    type XcmSender = (); // sending XCM not supported
    type AssetTransactor = (); // balances not supported
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = (); // balances not supported
    type IsTeleporter = (); // balances not supported
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = AllowUnpaidExecutionFrom<JustTheParent>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>; // balances not supported
    type Trader = (); // balances not supported
    type ResponseHandler = (); // Don't handle responses for now.
    type AssetTrap = (); // don't trap for now
    type AssetClaims = (); // don't claim for now
    type SubscriptionService = (); // don't handle subscriptions for now
}

impl cumulus_pallet_xcm::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
    // We do anything the parent chain tells us in this runtime.
    pub const ReservedDmpWeight: Weight = constants::MAXIMUM_BLOCK_WEIGHT / 2;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type Event = Event;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type OutboundXcmpMessageSource = ();
    type DmpMessageHandler = cumulus_pallet_xcm::UnlimitedDmpExecution<Runtime>;
    type ReservedDmpWeight = ReservedDmpWeight;
    type XcmpMessageHandler = ();
    type ReservedXcmpWeight = ();
}

impl parachain_info::Config for Runtime {}
