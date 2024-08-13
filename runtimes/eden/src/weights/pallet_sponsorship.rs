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

//! Autogenerated weights for pallet_sponsorship
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-13, STEPS: `4`, REPEAT: 4, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `tama`, CPU: `11th Gen Intel(R) Core(TM) i7-11700 @ 2.50GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_sponsorship
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

/// Weight functions for `pallet_sponsorship`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sponsorship::WeightInfo for WeightInfo<T> {
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 30_575 nanoseconds.
		Weight::from_parts(32_811_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 19_230 nanoseconds.
		Weight::from_parts(21_537_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 13_589 nanoseconds.
		Weight::from_parts(15_360_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 11_705 nanoseconds.
		Weight::from_parts(13_505_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1000 w:1000)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:1000 w:1000)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2000 w:2000)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32) -> Weight {
		// Minimum execution time: 51_574 nanoseconds.
		Weight::from_parts(89_663_570_u64, 0)
			// Standard Error: 184_182
			.saturating_add(Weight::from_parts(34_558_929_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1000 w:1000)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2000 w:2000)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:1000 w:1000)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32) -> Weight {
		// Minimum execution time: 112_572 nanoseconds.
		Weight::from_parts(770_458_785_u64, 0)
			// Standard Error: 1_052_101
			.saturating_add(Weight::from_parts(91_652_814_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1000 w:1000)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32) -> Weight {
		// Minimum execution time: 20_110 nanoseconds.
		Weight::from_parts(20_400_000_u64, 0)
			// Standard Error: 32_703
			.saturating_add(Weight::from_parts(7_295_531_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 57_491 nanoseconds.
		Weight::from_parts(59_508_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 52_084 nanoseconds.
		Weight::from_parts(54_223_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
