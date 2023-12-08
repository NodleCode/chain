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
//! DATE: 2023-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-b606df9f`, CPU: `AMD EPYC 7B13`
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
	fn migrate_users(l: u32, ) -> Weight;
	fn migrate_pots(l: u32, ) -> Weight;
}

/// Weight functions for `pallet_sponsorship`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 40_560 nanoseconds.
		Weight::from_parts(41_950_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 28_491 nanoseconds.
		Weight::from_parts(29_420_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 21_360 nanoseconds.
		Weight::from_parts(22_270_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 18_510 nanoseconds.
		Weight::from_parts(19_300_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32, ) -> Weight {
		// Minimum execution time: 70_980 nanoseconds.
		Weight::from_parts(72_350_000_u64, 0)
			// Standard Error: 11_477
			.saturating_add(Weight::from_parts(44_973_877_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32, ) -> Weight {
		// Minimum execution time: 138_040 nanoseconds.
		Weight::from_parts(139_280_000_u64, 0)
			// Standard Error: 56_451
			.saturating_add(Weight::from_parts(120_138_905_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32, ) -> Weight {
		// Minimum execution time: 30_900 nanoseconds.
		Weight::from_parts(31_630_000_u64, 0)
			// Standard Error: 10_266
			.saturating_add(Weight::from_parts(9_269_582_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
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
		// Minimum execution time: 77_370 nanoseconds.
		Weight::from_parts(78_570_000_u64, 0)
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
		// Minimum execution time: 70_220 nanoseconds.
		Weight::from_parts(70_840_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Sponsorship::User` (r:1000 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn migrate_users(l: u32, ) -> Weight {
		// Minimum execution time: 23_200 nanoseconds.
		Weight::from_parts(23_550_000_u64, 0)
			// Standard Error: 3_060
			.saturating_add(Weight::from_parts(10_562_378_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1000 w:999)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn migrate_pots(l: u32, ) -> Weight {
		// Minimum execution time: 18_460 nanoseconds.
		Weight::from_parts(18_780_000_u64, 0)
			// Standard Error: 2_632
			.saturating_add(Weight::from_parts(6_005_648_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
}

impl WeightInfo for () {
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 40_560 nanoseconds.
		Weight::from_parts(41_950_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 28_491 nanoseconds.
		Weight::from_parts(29_420_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 21_360 nanoseconds.
		Weight::from_parts(22_270_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 18_510 nanoseconds.
		Weight::from_parts(19_300_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32, ) -> Weight {
		// Minimum execution time: 70_980 nanoseconds.
		Weight::from_parts(72_350_000_u64, 0)
			// Standard Error: 11_477
			.saturating_add(Weight::from_parts(44_973_877_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32, ) -> Weight {
		// Minimum execution time: 138_040 nanoseconds.
		Weight::from_parts(139_280_000_u64, 0)
			// Standard Error: 56_451
			.saturating_add(Weight::from_parts(120_138_905_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32, ) -> Weight {
		// Minimum execution time: 30_900 nanoseconds.
		Weight::from_parts(31_630_000_u64, 0)
			// Standard Error: 10_266
			.saturating_add(Weight::from_parts(9_269_582_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 77_370 nanoseconds.
		Weight::from_parts(78_570_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 70_220 nanoseconds.
		Weight::from_parts(70_840_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: `Sponsorship::User` (r:1000 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn migrate_users(l: u32, ) -> Weight {
		// Minimum execution time: 23_200 nanoseconds.
		Weight::from_parts(23_550_000_u64, 0)
			// Standard Error: 3_060
			.saturating_add(Weight::from_parts(10_562_378_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1000 w:999)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn migrate_pots(l: u32, ) -> Weight {
		// Minimum execution time: 18_460 nanoseconds.
		Weight::from_parts(18_780_000_u64, 0)
			// Standard Error: 2_632
			.saturating_add(Weight::from_parts(6_005_648_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
}
