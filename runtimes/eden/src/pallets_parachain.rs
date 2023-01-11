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
	constants, pallets_governance::EnsureRootOrMoreThanHalfOfTechComm, xcm_config::XcmConfig, DmpQueue, Event, Runtime,
	XcmpQueue,
};
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use frame_support::{dispatch::Weight, match_types, parameter_types};
use xcm::latest::prelude::*;

use xcm_executor::XcmExecutor;

match_types! {
	pub type JustTheParent: impl Contains<MultiLocation> = { MultiLocation { parents:1, interior: Here } };
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrMoreThanHalfOfTechComm;
}

parameter_types! {
	pub const ReservedDmpWeight: Weight = constants::MAXIMUM_BLOCK_WEIGHT.saturating_div(2);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type Event = Event;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ();
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}
