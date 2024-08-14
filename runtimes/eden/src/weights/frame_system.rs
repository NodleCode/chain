/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2024  Nodle International
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

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=frame_system
// --extrinsic=*
// --steps=50
// --repeat=20
// --genesis-builder=runtime
// --runtime=./target/release/wbuild/runtime-eden/runtime_eden.wasm
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32) -> Weight {
		// Minimum execution time: 2_330 nanoseconds.
		Weight::from_parts(1_445_505_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(289_u64, 0).saturating_mul(b as u64))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32) -> Weight {
		// Minimum execution time: 6_590 nanoseconds.
		Weight::from_parts(5_195_281_u64, 0)
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_748_u64, 0).saturating_mul(b as u64))
	}
	// Storage: `System::Digest` (r:1 w:1)
	// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
	// Proof: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 4_060 nanoseconds.
		Weight::from_parts(4_340_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::UpgradeRestrictionSignal` (r:1 w:0)
	// Proof: `ParachainSystem::UpgradeRestrictionSignal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingValidationCode` (r:1 w:1)
	// Proof: `ParachainSystem::PendingValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::NewValidationCode` (r:0 w:1)
	// Proof: `ParachainSystem::NewValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::DidSetValidationCode` (r:0 w:1)
	// Proof: `ParachainSystem::DidSetValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_code() -> Weight {
		// Minimum execution time: 138_225_574 nanoseconds.
		Weight::from_parts(143_983_033_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32) -> Weight {
		// Minimum execution time: 2_300 nanoseconds.
		Weight::from_parts(2_480_000_u64, 0)
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(910_842_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32) -> Weight {
		// Minimum execution time: 2_280 nanoseconds.
		Weight::from_parts(2_360_000_u64, 0)
			// Standard Error: 1_062
			.saturating_add(Weight::from_parts(658_041_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32) -> Weight {
		// Minimum execution time: 4_060 nanoseconds.
		Weight::from_parts(4_120_000_u64, 0)
			// Standard Error: 1_174
			.saturating_add(Weight::from_parts(1_183_487_u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
	// Storage: `System::AuthorizedUpgrade` (r:0 w:1)
	// Proof: `System::AuthorizedUpgrade` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	fn authorize_upgrade() -> Weight {
		// Minimum execution time: 10_090 nanoseconds.
		Weight::from_parts(10_730_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `System::AuthorizedUpgrade` (r:1 w:1)
	// Proof: `System::AuthorizedUpgrade` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::UpgradeRestrictionSignal` (r:1 w:0)
	// Proof: `ParachainSystem::UpgradeRestrictionSignal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingValidationCode` (r:1 w:1)
	// Proof: `ParachainSystem::PendingValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::NewValidationCode` (r:0 w:1)
	// Proof: `ParachainSystem::NewValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::DidSetValidationCode` (r:0 w:1)
	// Proof: `ParachainSystem::DidSetValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn apply_authorized_upgrade() -> Weight {
		// Minimum execution time: 140_211_847 nanoseconds.
		Weight::from_parts(142_492_816_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
