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
//! DATE: 2024-08-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_collator_selection
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

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: `Session::NextKeys` (r:20 w:0)
	// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 20]`.
	fn set_invulnerables(b: u32) -> Weight {
		// Minimum execution time: 11_530 nanoseconds.
		Weight::from_parts(9_355_847_u64, 0)
			// Standard Error: 7_625
			.saturating_add(Weight::from_parts(3_850_481_u64, 0).saturating_mul(b as u64))
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
		// Minimum execution time: 46_020 nanoseconds.
		Weight::from_parts(45_927_817_u64, 0)
			// Standard Error: 7_281
			.saturating_add(Weight::from_parts(98_231_u64, 0).saturating_mul(b as u64))
			// Standard Error: 1_380
			.saturating_add(Weight::from_parts(110_735_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[4, 20]`.
	fn remove_invulnerable(b: u32) -> Weight {
		// Minimum execution time: 11_360 nanoseconds.
		Weight::from_parts(11_938_928_u64, 0)
			// Standard Error: 2_139
			.saturating_add(Weight::from_parts(95_752_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 5_490 nanoseconds.
		Weight::from_parts(5_780_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
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
		// Minimum execution time: 10_360 nanoseconds.
		Weight::from_parts(10_410_000_u64, 0)
			// Standard Error: 215_838
			.saturating_add(Weight::from_parts(7_097_647_u64, 0).saturating_mul(c as u64))
			// Standard Error: 215_838
			.saturating_add(Weight::from_parts(6_844_231_u64, 0).saturating_mul(k as u64))
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
		// Minimum execution time: 30_650 nanoseconds.
		Weight::from_parts(32_766_915_u64, 0)
			// Standard Error: 1_434
			.saturating_add(Weight::from_parts(101_154_u64, 0).saturating_mul(c as u64))
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
		// Minimum execution time: 39_020 nanoseconds.
		Weight::from_parts(42_735_931_u64, 0)
			// Standard Error: 1_740
			.saturating_add(Weight::from_parts(121_935_u64, 0).saturating_mul(c as u64))
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
		// Minimum execution time: 59_089 nanoseconds.
		Weight::from_parts(62_504_960_u64, 0)
			// Standard Error: 2_418
			.saturating_add(Weight::from_parts(137_818_u64, 0).saturating_mul(c as u64))
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
		// Minimum execution time: 33_370 nanoseconds.
		Weight::from_parts(36_119_522_u64, 0)
			// Standard Error: 1_716
			.saturating_add(Weight::from_parts(115_219_u64, 0).saturating_mul(c as u64))
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
		// Minimum execution time: 50_400 nanoseconds.
		Weight::from_parts(51_610_000_u64, 0)
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
	// Storage: `System::Account` (r:97 w:97)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `c` is `[1, 100]`.
	fn new_session(_r: u32, c: u32) -> Weight {
		// Minimum execution time: 18_960 nanoseconds.
		Weight::from_parts(19_620_000_u64, 0)
			// Standard Error: 374_512
			.saturating_add(Weight::from_parts(15_602_383_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as u64)))
	}
}
