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

//! Autogenerated weights for pallet_balances
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-2aa7e463`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_allow_death() -> Weight {
		// Minimum execution time: 103_240 nanoseconds.
		Weight::from_parts(105_360_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 80_500 nanoseconds.
		Weight::from_parts(82_280_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn force_set_balance_creating() -> Weight {
		// Minimum execution time: 29_300 nanoseconds.
		Weight::from_parts(30_180_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn force_set_balance_killing() -> Weight {
		// Minimum execution time: 43_980 nanoseconds.
		Weight::from_parts(45_820_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn force_transfer() -> Weight {
		// Minimum execution time: 106_620 nanoseconds.
		Weight::from_parts(109_260_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_all() -> Weight {
		// Minimum execution time: 97_770 nanoseconds.
		Weight::from_parts(99_880_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	fn force_unreserve() -> Weight {
		// Minimum execution time: 34_440 nanoseconds.
		Weight::from_parts(35_831_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:999 w:999)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Minimum execution time: 34_080 nanoseconds.
		Weight::from_parts(34_500_000_u64, 0)
			// Standard Error: 13_151
			.saturating_add(Weight::from_parts(25_726_758_u64, 0).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u as u64)))
	}
}
