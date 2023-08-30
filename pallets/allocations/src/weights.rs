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

//! Autogenerated weights for pallet_allocations
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-28, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-66242306`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_allocations
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/internal_pallet_weights.hbs
// --output=temp_weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_allocations.
pub trait WeightInfo {
	fn allocate(b: u32, ) -> Weight;
	fn calc_quota() -> Weight;
	fn renew_quota() -> Weight;
	fn checked_update_session_quota() -> Weight;
	fn set_curve_starting_block() -> Weight;
}

/// Weight functions for `pallet_allocations`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Allocations SessionQuota (r:1 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:502 w:502)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 500]`.
	fn allocate(b: u32, ) -> Weight {
		// Minimum execution time: 192_860 nanoseconds.
		Weight::from_parts(65_522_044_u64, 0)
			// Standard Error: 16_461
			.saturating_add(Weight::from_parts(69_683_052_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b as u64)))
	}
	// Storage: Allocations SessionQuotaCalculationSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations NextSessionQuota (r:0 w:1)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn calc_quota() -> Weight {
		// Minimum execution time: 22_750 nanoseconds.
		Weight::from_parts(23_371_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Allocations SessionQuotaRenewSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations NextSessionQuota (r:1 w:0)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuota (r:0 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn renew_quota() -> Weight {
		// Minimum execution time: 18_150 nanoseconds.
		Weight::from_parts(18_730_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuotaCalculationSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuotaRenewSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuota (r:0 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Allocations NextSessionQuota (r:0 w:1)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn checked_update_session_quota() -> Weight {
		// Minimum execution time: 38_990 nanoseconds.
		Weight::from_parts(39_880_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations MintCurveStartingBlock (r:0 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuotaCalculationSchedule (r:0 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuotaRenewSchedule (r:0 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_curve_starting_block() -> Weight {
		// Minimum execution time: 10_670 nanoseconds.
		Weight::from_parts(11_210_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

impl WeightInfo for () {
	// Storage: Allocations SessionQuota (r:1 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:502 w:502)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 500]`.
	fn allocate(b: u32, ) -> Weight {
		// Minimum execution time: 192_860 nanoseconds.
		Weight::from_parts(65_522_044_u64, 0)
			// Standard Error: 16_461
			.saturating_add(Weight::from_parts(69_683_052_u64, 0).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(b as u64)))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(b as u64)))
	}
	// Storage: Allocations SessionQuotaCalculationSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations NextSessionQuota (r:0 w:1)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn calc_quota() -> Weight {
		// Minimum execution time: 22_750 nanoseconds.
		Weight::from_parts(23_371_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: Allocations SessionQuotaRenewSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations NextSessionQuota (r:1 w:0)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuota (r:0 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn renew_quota() -> Weight {
		// Minimum execution time: 18_150 nanoseconds.
		Weight::from_parts(18_730_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuotaCalculationSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations MintCurveStartingBlock (r:1 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances TotalIssuance (r:1 w:0)
	// Proof: Balances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations SessionQuotaRenewSchedule (r:1 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuota (r:0 w:1)
	// Proof: Allocations SessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Allocations NextSessionQuota (r:0 w:1)
	// Proof: Allocations NextSessionQuota (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn checked_update_session_quota() -> Weight {
		// Minimum execution time: 38_990 nanoseconds.
		Weight::from_parts(39_880_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Allocations MintCurveStartingBlock (r:0 w:1)
	// Proof: Allocations MintCurveStartingBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuotaCalculationSchedule (r:0 w:1)
	// Proof: Allocations SessionQuotaCalculationSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Allocations SessionQuotaRenewSchedule (r:0 w:1)
	// Proof: Allocations SessionQuotaRenewSchedule (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_curve_starting_block() -> Weight {
		// Minimum execution time: 10_670 nanoseconds.
		Weight::from_parts(11_210_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
