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

//! Autogenerated weights for pallet_xcm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-b606df9f`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn send() -> Weight {
		// Minimum execution time: 31_720 nanoseconds.
		Weight::from_parts(32_770_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Benchmark::Override` (r:0 w:0)
	// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn teleport_assets() -> Weight {
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_parts(18_446_744_073_709_551_000_u64, 0)
	}
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn reserve_transfer_assets() -> Weight {
		// Minimum execution time: 27_900 nanoseconds.
		Weight::from_parts(28_691_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `Benchmark::Override` (r:0 w:0)
	// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn execute() -> Weight {
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_parts(18_446_744_073_709_551_000_u64, 0)
	}
	// Storage: `PolkadotXcm::SupportedVersion` (r:0 w:1)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_xcm_version() -> Weight {
		// Minimum execution time: 11_590 nanoseconds.
		Weight::from_parts(12_210_000_u64, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:0 w:1)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_default_xcm_version() -> Weight {
		// Minimum execution time: 3_760 nanoseconds.
		Weight::from_parts(4_040_000_u64, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifiers` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_subscribe_version_notify() -> Weight {
		// Minimum execution time: 37_500 nanoseconds.
		Weight::from_parts(38_470_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifiers` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_unsubscribe_version_notify() -> Weight {
		// Minimum execution time: 38_670 nanoseconds.
		Weight::from_parts(40_060_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `PolkadotXcm::XcmExecutionSuspended` (r:0 w:1)
	// Proof: `PolkadotXcm::XcmExecutionSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_suspension() -> Weight {
		// Minimum execution time: 3_750 nanoseconds.
		Weight::from_parts(4_190_000_u64, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `PolkadotXcm::SupportedVersion` (r:4 w:2)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_supported_version() -> Weight {
		// Minimum execution time: 24_860 nanoseconds.
		Weight::from_parts(25_670_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifiers` (r:4 w:2)
	// Proof: `PolkadotXcm::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_version_notifiers() -> Weight {
		// Minimum execution time: 24_860 nanoseconds.
		Weight::from_parts(25_500_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifyTargets` (r:5 w:0)
	// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn already_notified_target() -> Weight {
		// Minimum execution time: 26_330 nanoseconds.
		Weight::from_parts(26_950_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifyTargets` (r:2 w:1)
	// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn notify_current_targets() -> Weight {
		// Minimum execution time: 35_560 nanoseconds.
		Weight::from_parts(36_130_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifyTargets` (r:3 w:0)
	// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn notify_target_migration_fail() -> Weight {
		// Minimum execution time: 14_100 nanoseconds.
		Weight::from_parts(14_590_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifyTargets` (r:4 w:2)
	// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_version_notify_targets() -> Weight {
		// Minimum execution time: 25_470 nanoseconds.
		Weight::from_parts(26_130_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `PolkadotXcm::VersionNotifyTargets` (r:4 w:2)
	// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn migrate_and_notify_old_targets() -> Weight {
		// Minimum execution time: 45_190 nanoseconds.
		Weight::from_parts(46_140_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn new_query() -> Weight {
		//TODO
		Weight::from_parts(46_140_000_u64, 0)
		.saturating_add(T::DbWeight::get().reads(9_u64))
		.saturating_add(T::DbWeight::get().writes(4_u64))
		}

	fn take_response() -> Weight {
		//TODO
		Weight::from_parts(46_140_000_u64, 0)
		.saturating_add(T::DbWeight::get().reads(9_u64))
		.saturating_add(T::DbWeight::get().writes(4_u64))
		}

	fn transfer_assets() -> Weight {
			//TODO
			Weight::from_parts(46_140_000_u64, 0)
	}
}
