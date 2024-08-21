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

//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_membership
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

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: `AllocationsOracles::Members` (r:1 w:1)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 49]`.
	/// The range of component `m` is `[1, 49]`.
	fn add_member(m: u32) -> Weight {
		// Minimum execution time: 10_460 nanoseconds.
		Weight::from_parts(11_000_163_u64, 0)
			// Standard Error: 354
			.saturating_add(Weight::from_parts(29_701_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `AllocationsOracles::Members` (r:1 w:1)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AllocationsOracles::Prime` (r:1 w:0)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 50]`.
	/// The range of component `m` is `[2, 50]`.
	fn remove_member(m: u32) -> Weight {
		// Minimum execution time: 13_760 nanoseconds.
		Weight::from_parts(14_297_992_u64, 0)
			// Standard Error: 388
			.saturating_add(Weight::from_parts(24_171_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `AllocationsOracles::Members` (r:1 w:1)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AllocationsOracles::Prime` (r:1 w:0)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 50]`.
	/// The range of component `m` is `[2, 50]`.
	fn swap_member(m: u32) -> Weight {
		// Minimum execution time: 13_730 nanoseconds.
		Weight::from_parts(14_340_781_u64, 0)
			// Standard Error: 518
			.saturating_add(Weight::from_parts(47_037_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `AllocationsOracles::Members` (r:1 w:1)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AllocationsOracles::Prime` (r:1 w:0)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 50]`.
	/// The range of component `m` is `[1, 50]`.
	fn reset_members(m: u32) -> Weight {
		// Minimum execution time: 13_640 nanoseconds.
		Weight::from_parts(14_724_703_u64, 0)
			// Standard Error: 912
			.saturating_add(Weight::from_parts(169_605_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `AllocationsOracles::Members` (r:1 w:1)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AllocationsOracles::Prime` (r:1 w:1)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 50]`.
	/// The range of component `m` is `[1, 50]`.
	fn change_key(m: u32) -> Weight {
		// Minimum execution time: 14_560 nanoseconds.
		Weight::from_parts(15_214_764_u64, 0)
			// Standard Error: 573
			.saturating_add(Weight::from_parts(46_586_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `AllocationsOracles::Members` (r:1 w:0)
	// Proof: `AllocationsOracles::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `AllocationsOracles::Prime` (r:0 w:1)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 50]`.
	/// The range of component `m` is `[1, 50]`.
	fn set_prime(m: u32) -> Weight {
		// Minimum execution time: 5_460 nanoseconds.
		Weight::from_parts(5_769_924_u64, 0)
			// Standard Error: 258
			.saturating_add(Weight::from_parts(11_372_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `AllocationsOracles::Prime` (r:0 w:1)
	// Proof: `AllocationsOracles::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn clear_prime() -> Weight {
		// Minimum execution time: 2_750 nanoseconds.
		Weight::from_parts(2_900_000_u64, 0).saturating_add(T::DbWeight::get().writes(2_u64))
	}
}