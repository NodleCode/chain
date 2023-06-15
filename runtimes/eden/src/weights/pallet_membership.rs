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

//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-012bd056`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
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

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 49]`.
	fn add_member(m: u32, ) -> Weight {
		// Minimum execution time: 25_700 nanoseconds.
		Weight::from_parts(26_936_324_u64, 0)
			// Standard Error: 1_088
			.saturating_add(Weight::from_parts(65_278_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 50]`.
	fn remove_member(m: u32, ) -> Weight {
		// Minimum execution time: 29_470 nanoseconds.
		Weight::from_parts(30_891_100_u64, 0)
			// Standard Error: 1_691
			.saturating_add(Weight::from_parts(59_368_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 50]`.
	fn swap_member(m: u32, ) -> Weight {
		// Minimum execution time: 29_890 nanoseconds.
		Weight::from_parts(31_136_747_u64, 0)
			// Standard Error: 1_632
			.saturating_add(Weight::from_parts(85_361_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 50]`.
	fn reset_member(m: u32, ) -> Weight {
		// Minimum execution time: 29_470 nanoseconds.
		Weight::from_parts(31_706_138_u64, 0)
			// Standard Error: 2_451
			.saturating_add(Weight::from_parts(233_174_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalMembership Prime (r:1 w:1)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 50]`.
	fn change_key(m: u32, ) -> Weight {
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_parts(32_510_851_u64, 0)
			// Standard Error: 1_632
			.saturating_add(Weight::from_parts(86_350_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 50]`.
	fn set_prime(m: u32, ) -> Weight {
		// Minimum execution time: 10_930 nanoseconds.
		Weight::from_parts(11_445_262_u64, 0)
			// Standard Error: 520
			.saturating_add(Weight::from_parts(14_444_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 50]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Minimum execution time: 4_950 nanoseconds.
		Weight::from_parts(5_244_125_u64, 0)
			// Standard Error: 295
			.saturating_add(Weight::from_parts(2_502_u64, 0).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
