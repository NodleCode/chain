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
//! DATE: 2022-11-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-012bd056`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 49]`.
	fn add_member(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(27_262_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(66_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[2, 50]`.
	fn remove_member(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(30_896_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(59_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[2, 50]`.
	fn swap_member(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(30_952_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(75_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 50]`.
	fn reset_member(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(30_795_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(201_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 50]`.
	fn change_key(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(32_024_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(79_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 50]`.
	fn set_prime(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(11_878_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(18_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 50]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(7_410_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
