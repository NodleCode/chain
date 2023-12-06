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

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-66242306`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 16_190 nanoseconds.
		Weight::from_parts(16_966_734_u64, 0)
			// Standard Error: 3
			.saturating_add(Weight::from_parts(480_u64, 0).saturating_mul(z as u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 62_060 nanoseconds.
		Weight::from_parts(54_342_367_u64, 0)
			// Standard Error: 1_547
			.saturating_add(Weight::from_parts(96_670_u64, 0).saturating_mul(s as u64))
			// Standard Error: 15
			.saturating_add(Weight::from_parts(1_478_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 37_170 nanoseconds.
		Weight::from_parts(30_260_457_u64, 0)
			// Standard Error: 714
			.saturating_add(Weight::from_parts(84_956_u64, 0).saturating_mul(s as u64))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(1_493_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 68_550 nanoseconds.
		Weight::from_parts(57_955_218_u64, 0)
			// Standard Error: 1_304
			.saturating_add(Weight::from_parts(117_458_u64, 0).saturating_mul(s as u64))
			// Standard Error: 12
			.saturating_add(Weight::from_parts(1_572_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 49_839 nanoseconds.
		Weight::from_parts(52_059_834_u64, 0)
			// Standard Error: 1_208
			.saturating_add(Weight::from_parts(105_081_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 27_751 nanoseconds.
		Weight::from_parts(28_703_013_u64, 0)
			// Standard Error: 747
			.saturating_add(Weight::from_parts(83_996_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 51_040 nanoseconds.
		Weight::from_parts(52_839_594_u64, 0)
			// Standard Error: 1_037
			.saturating_add(Weight::from_parts(90_131_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
