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
//! DATE: 2023-05-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `tama`, CPU: `11th Gen Intel(R) Core(TM) i7-11700 @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
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
		// Minimum execution time: 3_987 nanoseconds.
		Weight::from_parts(35_461_151_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(328_u64, 0).saturating_mul(b as u64))
	}
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Minimum execution time: 14_412 nanoseconds.
		Weight::from_parts(200_986_555_u64, 0)
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_161_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: unknown `0x3a686561707061676573` (r:0 w:1)
	// Proof Skipped: unknown `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 5_275 nanoseconds.
		Weight::from_parts(5_555_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Minimum execution time: 2_771 nanoseconds.
		Weight::from_parts(2_859_000_u64, 0)
			// Standard Error: 1_377
			.saturating_add(Weight::from_parts(679_848_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Minimum execution time: 2_732 nanoseconds.
		Weight::from_parts(2_805_000_u64, 0)
			// Standard Error: 609
			.saturating_add(Weight::from_parts(475_171_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Minimum execution time: 5_088 nanoseconds.
		Weight::from_parts(5_204_000_u64, 0)
			// Standard Error: 793
			.saturating_add(Weight::from_parts(919_301_u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
}
