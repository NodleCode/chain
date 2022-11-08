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

//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collator_selection.
pub trait WeightInfo {
	fn set_invulnerables(b: u32, ) -> Weight;
	fn set_desired_candidates() -> Weight;
	fn set_candidacy_bond() -> Weight;
	fn register_as_candidate(c: u32, ) -> Weight;
	fn leave_intent(c: u32, ) -> Weight;
	fn note_author() -> Weight;
	fn new_session(r: u32, c: u32, ) -> Weight;}

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn set_invulnerables(b: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 154_000
			.saturating_add(Weight::from_ref_time(8_171_000).saturating_mul(b as u64))			.saturating_add(T::DbWeight::get().reads(4_u64))			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))	}
	fn set_desired_candidates() -> Weight {
		Weight::from_ref_time(30_710_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))			.saturating_add(T::DbWeight::get().writes(3_u64))	}
	fn set_candidacy_bond() -> Weight {
		Weight::from_ref_time(31_870_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))			.saturating_add(T::DbWeight::get().writes(3_u64))	}
	fn register_as_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(147_325_000_u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(187_000).saturating_mul(c as u64))			.saturating_add(T::DbWeight::get().reads(9_u64))			.saturating_add(T::DbWeight::get().writes(4_u64))	}
	fn leave_intent(c: u32, ) -> Weight {
		Weight::from_ref_time(120_154_000_u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(136_000).saturating_mul(c as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().writes(4_u64))	}
	fn note_author() -> Weight {
		Weight::from_ref_time(49_520_000_u64)
			.saturating_add(T::DbWeight::get().reads(7_u64))			.saturating_add(T::DbWeight::get().writes(6_u64))	}
	fn new_session(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 1_562_000
			.saturating_add(Weight::from_ref_time(12_048_000).saturating_mul(r as u64))			// Standard Error: 1_562_000
			.saturating_add(Weight::from_ref_time(45_520_000).saturating_mul(c as u64))			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c as u64)))			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as u64)))			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as u64)))	}}

// For backwards compatibility and tests
impl WeightInfo for () {	
	fn set_invulnerables(b: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)

			// Standard Error: 154_000
			.saturating_add(Weight::from_ref_time(8_171_000).saturating_mul(b as u64))			.saturating_add(RocksDbWeight::get().reads(4_u64))			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(b as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))	}	
	fn set_desired_candidates() -> Weight {
		Weight::from_ref_time(30_710_000_u64)

			.saturating_add(RocksDbWeight::get().reads(4_u64))			.saturating_add(RocksDbWeight::get().writes(3_u64))	}	
	fn set_candidacy_bond() -> Weight {
		Weight::from_ref_time(31_870_000_u64)

			.saturating_add(RocksDbWeight::get().reads(4_u64))			.saturating_add(RocksDbWeight::get().writes(3_u64))	}	
	fn register_as_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(147_325_000_u64)

			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(187_000).saturating_mul(c as u64))			.saturating_add(RocksDbWeight::get().reads(9_u64))			.saturating_add(RocksDbWeight::get().writes(4_u64))	}	
	fn leave_intent(c: u32, ) -> Weight {
		Weight::from_ref_time(120_154_000_u64)

			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(136_000).saturating_mul(c as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().writes(4_u64))	}	
	fn note_author() -> Weight {
		Weight::from_ref_time(49_520_000_u64)

			.saturating_add(RocksDbWeight::get().reads(7_u64))			.saturating_add(RocksDbWeight::get().writes(6_u64))	}	
	fn new_session(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)

			// Standard Error: 1_562_000
			.saturating_add(Weight::from_ref_time(12_048_000).saturating_mul(r as u64))			// Standard Error: 1_562_000
			.saturating_add(Weight::from_ref_time(45_520_000).saturating_mul(c as u64))			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c as u64)))			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as u64)))			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as u64)))	}}
