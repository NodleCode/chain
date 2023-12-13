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

//! Autogenerated weights for pallet_grants
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-b606df9f`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_grants
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/internal_pallet_weights.hbs
// --output=temp_weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_grants.
pub trait WeightInfo {
	fn add_vesting_schedule() -> Weight;
	fn claim() -> Weight;
	fn cancel_all_vesting_schedules() -> Weight;
	fn renounce() -> Weight;
}

/// Weight functions for `pallet_grants`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn add_vesting_schedule() -> Weight {
		// Minimum execution time: 120_040 nanoseconds.
		Weight::from_parts(124_240_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:0)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Minimum execution time: 52_300 nanoseconds.
		Weight::from_parts(53_550_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Vesting::Renounced` (r:1 w:0)
	// Proof: `Vesting::Renounced` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Vesting::CounterForVestingSchedules` (r:1 w:1)
	// Proof: `Vesting::CounterForVestingSchedules` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn cancel_all_vesting_schedules() -> Weight {
		// Minimum execution time: 168_070 nanoseconds.
		Weight::from_parts(173_320_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: `Vesting::Renounced` (r:0 w:1)
	// Proof: `Vesting::Renounced` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn renounce() -> Weight {
		// Minimum execution time: 13_340 nanoseconds.
		Weight::from_parts(14_080_000_u64, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

impl WeightInfo for () {
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn add_vesting_schedule() -> Weight {
		// Minimum execution time: 120_040 nanoseconds.
		Weight::from_parts(124_240_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:0)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Minimum execution time: 52_300 nanoseconds.
		Weight::from_parts(53_550_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: `Vesting::Renounced` (r:1 w:0)
	// Proof: `Vesting::Renounced` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	// Storage: `Balances::Locks` (r:1 w:1)
	// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	// Storage: `Balances::Freezes` (r:1 w:0)
	// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Vesting::CounterForVestingSchedules` (r:1 w:1)
	// Proof: `Vesting::CounterForVestingSchedules` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn cancel_all_vesting_schedules() -> Weight {
		// Minimum execution time: 168_070 nanoseconds.
		Weight::from_parts(173_320_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: `Vesting::Renounced` (r:0 w:1)
	// Proof: `Vesting::Renounced` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn renounce() -> Weight {
		// Minimum execution time: 13_340 nanoseconds.
		Weight::from_parts(14_080_000_u64, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
