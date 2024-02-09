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

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		// Minimum execution time: 3_100 nanoseconds.
		Weight::from_parts(3_597_431_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(289_u64, 0).saturating_mul(b as u64))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Minimum execution time: 8_020 nanoseconds.
		Weight::from_parts(17_421_834_u64, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_728_u64, 0).saturating_mul(b as u64))
	}
	// Storage: `System::Digest` (r:1 w:1)
	// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
	// Proof: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 5_260 nanoseconds.
		Weight::from_parts(5_560_000_u64, 0)
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
		// Minimum execution time: 148_021_601 nanoseconds.
		Weight::from_parts(152_962_972_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Minimum execution time: 3_090 nanoseconds.
		Weight::from_parts(3_190_000_u64, 0)
			// Standard Error: 2_529
			.saturating_add(Weight::from_parts(979_179_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Minimum execution time: 3_160 nanoseconds.
		Weight::from_parts(3_220_000_u64, 0)
			// Standard Error: 988
			.saturating_add(Weight::from_parts(664_002_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Minimum execution time: 5_880 nanoseconds.
		Weight::from_parts(6_030_000_u64, 0)
			// Standard Error: 1_152
			.saturating_add(Weight::from_parts(1_205_441_u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
}
