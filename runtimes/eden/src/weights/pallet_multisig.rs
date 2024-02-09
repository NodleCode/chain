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
		// Minimum execution time: 15_760 nanoseconds.
		Weight::from_parts(16_529_622_u64, 0)
			// Standard Error: 3
			.saturating_add(Weight::from_parts(449_u64, 0).saturating_mul(z as u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 49_650 nanoseconds.
		Weight::from_parts(42_368_971_u64, 0)
			// Standard Error: 901
			.saturating_add(Weight::from_parts(85_459_u64, 0).saturating_mul(s as u64))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_433_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 31_640 nanoseconds.
		Weight::from_parts(24_328_772_u64, 0)
			// Standard Error: 546
			.saturating_add(Weight::from_parts(81_815_u64, 0).saturating_mul(s as u64))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_434_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 56_530 nanoseconds.
		Weight::from_parts(45_446_816_u64, 0)
			// Standard Error: 1_037
			.saturating_add(Weight::from_parts(121_788_u64, 0).saturating_mul(s as u64))
			// Standard Error: 10
			.saturating_add(Weight::from_parts(1_514_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 38_770 nanoseconds.
		Weight::from_parts(40_592_382_u64, 0)
			// Standard Error: 983
			.saturating_add(Weight::from_parts(88_401_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 21_990 nanoseconds.
		Weight::from_parts(22_915_813_u64, 0)
			// Standard Error: 542
			.saturating_add(Weight::from_parts(83_022_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Multisig::Multisigs` (r:1 w:1)
	// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 39_340 nanoseconds.
		Weight::from_parts(41_198_187_u64, 0)
			// Standard Error: 1_005
			.saturating_add(Weight::from_parts(88_533_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
