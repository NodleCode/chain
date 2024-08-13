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

//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-13, STEPS: `4`, REPEAT: 4, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `tama`, CPU: `11th Gen Intel(R) Core(TM) i7-11700 @ 2.50GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_collator_selection
// --extrinsic=*
// --steps=4
// --repeat=4
// --genesis-builder=runtime
// --runtime=./target/release/wbuild/runtime-eden/runtime_eden.wasm
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: `Session::NextKeys` (r:20 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 20]`.
	fn set_invulnerables(b: u32) -> Weight {
		// Minimum execution time: 14_758 nanoseconds.
		Weight::from_parts(11_466_327_u64, 0)
			// Standard Error: 83_285
			.saturating_add(Weight::from_parts(4_122_053_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 19]`.
	/// The range of component `c` is `[1, 99]`.
	fn add_invulnerable(b: u32, c: u32) -> Weight {
		// Minimum execution time: 54_546 nanoseconds.
		Weight::from_parts(52_411_987_u64, 0)
			// Standard Error: 60_903
			.saturating_add(Weight::from_parts(133_516_u64, 0).saturating_mul(b as u64))
			// Standard Error: 11_153
			.saturating_add(Weight::from_parts(107_107_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[4, 20]`.
	fn remove_invulnerable(b: u32) -> Weight {
		// Minimum execution time: 14_038 nanoseconds.
		Weight::from_parts(13_891_476_u64, 0)
			// Standard Error: 23_032
			.saturating_add(Weight::from_parts(135_150_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 6_591 nanoseconds.
		Weight::from_parts(8_667_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:1)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:100 w:100)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:100)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `k` is `[0, 100]`.
	fn set_candidacy_bond(c: u32, k: u32) -> Weight {
		// Minimum execution time: 11_670 nanoseconds.
		Weight::from_parts(12_255_000_u64, 0)
			// Standard Error: 2_131_639
			.saturating_add(Weight::from_parts(7_255_395_u64, 0).saturating_mul(c as u64))
			// Standard Error: 2_131_639
			.saturating_add(Weight::from_parts(7_092_400_u64, 0).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k as u64)))
	}
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[4, 100]`.
	fn update_bond(c: u32) -> Weight {
		// Minimum execution time: 34_090 nanoseconds.
		Weight::from_parts(37_031_856_u64, 0)
			// Standard Error: 15_554
			.saturating_add(Weight::from_parts(61_610_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 99]`.
	fn register_as_candidate(c: u32) -> Weight {
		// Minimum execution time: 42_136 nanoseconds.
		Weight::from_parts(44_663_408_u64, 0)
			// Standard Error: 13_339
			.saturating_add(Weight::from_parts(111_230_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `Session::NextKeys` (r:1 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:2)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[4, 100]`.
	fn take_candidate_slot(c: u32) -> Weight {
		// Minimum execution time: 64_408 nanoseconds.
		Weight::from_parts(65_210_631_u64, 0)
			// Standard Error: 8_514
			.saturating_add(Weight::from_parts(130_504_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[4, 100]`.
	fn leave_intent(c: u32) -> Weight {
		// Minimum execution time: 36_751 nanoseconds.
		Weight::from_parts(39_454_481_u64, 0)
			// Standard Error: 29_224
			.saturating_add(Weight::from_parts(109_642_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::BlockWeight` (r:1 w:1)
	// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Minimum execution time: 52_749 nanoseconds.
		Weight::from_parts(55_297_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::LastAuthoredBlock` (r:100 w:0)
	// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::BlockWeight` (r:1 w:1)
	// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:67 w:67)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `c` is `[1, 100]`.
	fn new_session(r: u32, c: u32) -> Weight {
		// Minimum execution time: 20_957 nanoseconds.
		Weight::from_parts(203_153_996_u64, 0)
			// Standard Error: 4_373_851
			.saturating_add(Weight::from_parts(9_617_643_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(14_u64))
	}
}
