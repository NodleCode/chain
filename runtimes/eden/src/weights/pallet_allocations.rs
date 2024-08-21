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

//! Autogenerated weights for pallet_allocations
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_allocations
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

/// Weight functions for `pallet_allocations`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_allocations::WeightInfo for WeightInfo<T> {
	// Storage: `Allocations::SessionQuota` (r:1 w:1)
	// Proof: `Allocations::SessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:502 w:502)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 500]`.
	fn allocate(b: u32) -> Weight {
		// Minimum execution time: 114_369 nanoseconds.
		Weight::from_parts(115_700_000_u64, 0)
			// Standard Error: 13_456
			.saturating_add(Weight::from_parts(43_748_727_u64, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b as u64)))
	}
	// Storage: `Allocations::SessionQuotaCalculationSchedule` (r:1 w:1)
	// Proof: `Allocations::SessionQuotaCalculationSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::MintCurveStartingBlock` (r:1 w:1)
	// Proof: `Allocations::MintCurveStartingBlock` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::NextSessionQuota` (r:0 w:1)
	// Proof: `Allocations::NextSessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn calc_quota() -> Weight {
		// Minimum execution time: 5_130 nanoseconds.
		Weight::from_parts(5_350_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Allocations::SessionQuotaRenewSchedule` (r:1 w:1)
	// Proof: `Allocations::SessionQuotaRenewSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::MintCurveStartingBlock` (r:1 w:1)
	// Proof: `Allocations::MintCurveStartingBlock` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::NextSessionQuota` (r:1 w:0)
	// Proof: `Allocations::NextSessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `Allocations::SessionQuota` (r:0 w:1)
	// Proof: `Allocations::SessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn renew_quota() -> Weight {
		// Minimum execution time: 5_020 nanoseconds.
		Weight::from_parts(5_230_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Allocations::SessionQuotaCalculationSchedule` (r:1 w:1)
	// Proof: `Allocations::SessionQuotaCalculationSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::MintCurveStartingBlock` (r:1 w:1)
	// Proof: `Allocations::MintCurveStartingBlock` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::SessionQuotaRenewSchedule` (r:1 w:1)
	// Proof: `Allocations::SessionQuotaRenewSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::SessionQuota` (r:0 w:1)
	// Proof: `Allocations::SessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `Allocations::NextSessionQuota` (r:0 w:1)
	// Proof: `Allocations::NextSessionQuota` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn checked_update_session_quota() -> Weight {
		// Minimum execution time: 10_010 nanoseconds.
		Weight::from_parts(10_410_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Allocations::MintCurveStartingBlock` (r:0 w:1)
	// Proof: `Allocations::MintCurveStartingBlock` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::SessionQuotaCalculationSchedule` (r:0 w:1)
	// Proof: `Allocations::SessionQuotaCalculationSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Allocations::SessionQuotaRenewSchedule` (r:0 w:1)
	// Proof: `Allocations::SessionQuotaRenewSchedule` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_curve_starting_block() -> Weight {
		// Minimum execution time: 4_530 nanoseconds.
		Weight::from_parts(4_740_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
