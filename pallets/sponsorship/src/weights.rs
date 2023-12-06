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

//! Autogenerated weights for pallet_sponsorship
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
// --pallet=pallet_sponsorship
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/internal_pallet_weights.hbs
// --output=temp_weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_sponsorship.
pub trait WeightInfo {
	fn create_pot() -> Weight;
	fn remove_pot() -> Weight;
	fn update_pot_limits() -> Weight;
	fn update_sponsorship_type() -> Weight;
	fn register_users(l: u32, ) -> Weight;
	fn remove_users(l: u32, ) -> Weight;
	fn update_users_limits(l: u32, ) -> Weight;
	fn pre_sponsor() -> Weight;
	fn post_sponsor() -> Weight;
}

/// Weight functions for `pallet_sponsorship`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 19_010 nanoseconds.
		Weight::from_parts(19_790_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 25_700 nanoseconds.
		Weight::from_parts(27_050_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 21_200 nanoseconds.
		Weight::from_parts(22_010_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 19_300 nanoseconds.
		Weight::from_parts(20_310_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:2 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32, ) -> Weight {
		// Minimum execution time: 57_500 nanoseconds.
		Weight::from_parts(58_100_000_u64, 0)
			// Standard Error: 6_407
			.saturating_add(Weight::from_parts(34_097_426_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:2 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32, ) -> Weight {
		// Minimum execution time: 129_720 nanoseconds.
		Weight::from_parts(130_569_000_u64, 0)
			// Standard Error: 44_816
			.saturating_add(Weight::from_parts(107_138_039_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32, ) -> Weight {
		// Minimum execution time: 31_280 nanoseconds.
		Weight::from_parts(31_660_000_u64, 0)
			// Standard Error: 9_779
			.saturating_add(Weight::from_parts(9_259_454_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 84_951 nanoseconds.
		Weight::from_parts(86_070_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 77_460 nanoseconds.
		Weight::from_parts(78_909_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
}

impl WeightInfo for () {
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 19_010 nanoseconds.
		Weight::from_parts(19_790_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 25_700 nanoseconds.
		Weight::from_parts(27_050_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 21_200 nanoseconds.
		Weight::from_parts(22_010_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 19_300 nanoseconds.
		Weight::from_parts(20_310_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `Sponsorship::Pot` (r:2 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32, ) -> Weight {
		// Minimum execution time: 57_500 nanoseconds.
		Weight::from_parts(58_100_000_u64, 0)
			// Standard Error: 6_407
			.saturating_add(Weight::from_parts(34_097_426_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:2 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32, ) -> Weight {
		// Minimum execution time: 129_720 nanoseconds.
		Weight::from_parts(130_569_000_u64, 0)
			// Standard Error: 44_816
			.saturating_add(Weight::from_parts(107_138_039_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32, ) -> Weight {
		// Minimum execution time: 31_280 nanoseconds.
		Weight::from_parts(31_660_000_u64, 0)
			// Standard Error: 9_779
			.saturating_add(Weight::from_parts(9_259_454_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 84_951 nanoseconds.
		Weight::from_parts(86_070_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::TotalIssuance` (r:1 w:0)
	// Proof: `Balances::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Number` (r:1 w:0)
	// Proof: `System::Number` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::ExecutionPhase` (r:1 w:0)
	// Proof: `System::ExecutionPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	// Storage: `System::EventCount` (r:1 w:1)
	// Proof: `System::EventCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `System::Events` (r:1 w:1)
	// Proof: `System::Events` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: Some(164), added: 2639, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 77_460 nanoseconds.
		Weight::from_parts(78_909_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}
