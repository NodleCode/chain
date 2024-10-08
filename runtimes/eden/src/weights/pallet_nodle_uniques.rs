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

//! Autogenerated weights for pallet_nodle_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_nodle_uniques
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

/// Weight functions for `pallet_nodle_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nodle_uniques::WeightInfo for WeightInfo<T> {
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::ItemExtraDeposits` (r:1001 w:1000)
	// Proof: `NodleUniques::ItemExtraDeposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1001 w:1000)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1000 w:1000)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Attribute` (r:1000 w:1000)
	// Proof: `Uniques::Attribute` (`max_values`: None, `max_size`: Some(364), added: 2839, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassMetadataOf` (r:0 w:1)
	// Proof: `Uniques::ClassMetadataOf` (`max_values`: None, `max_size`: Some(167), added: 2642, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1000)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Uniques::CollectionMaxSupply` (r:0 w:1)
	// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:0 w:1)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32) -> Weight {
		// Minimum execution time: 3_331_959 nanoseconds.
		Weight::from_parts(3_354_649_000_u64, 0)
			// Standard Error: 30_555
			.saturating_add(Weight::from_parts(30_984_038_u64, 0).saturating_mul(n as u64))
			// Standard Error: 30_555
			.saturating_add(Weight::from_parts(231_805_u64, 0).saturating_mul(m as u64))
			// Standard Error: 30_555
			.saturating_add(Weight::from_parts(389_386_u64, 0).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a as u64)))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a as u64)))
	}
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:1 w:1)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::ItemExtraDeposits` (r:0 w:1)
	// Proof: `NodleUniques::ItemExtraDeposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn mint_with_extra_deposit() -> Weight {
		// Minimum execution time: 67_890 nanoseconds.
		Weight::from_parts(70_670_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::ItemExtraDeposits` (r:1 w:1)
	// Proof: `NodleUniques::ItemExtraDeposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Minimum execution time: 68_270 nanoseconds.
		Weight::from_parts(69_660_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:0 w:1)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn create_with_extra_deposit_limit() -> Weight {
		// Minimum execution time: 37_910 nanoseconds.
		Weight::from_parts(38_950_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Uniques::OwnershipAcceptance` (r:1 w:1)
	// Proof: `Uniques::OwnershipAcceptance` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:1 w:0)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:2)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 62_030 nanoseconds.
		Weight::from_parts(63_100_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:1 w:1)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn update_extra_deposit_limit() -> Weight {
		// Minimum execution time: 18_650 nanoseconds.
		Weight::from_parts(19_210_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
