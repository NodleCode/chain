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

//! Autogenerated weights for pallet_nodle_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nodle_uniques
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

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
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		// Minimum execution time: 3_222_090 nanoseconds.
		Weight::from_parts(3_243_240_000_u64, 0)
			// Standard Error: 28_996
			.saturating_add(Weight::from_parts(33_040_167_u64, 0).saturating_mul(n as u64))
			// Standard Error: 28_996
			.saturating_add(Weight::from_parts(232_229_u64, 0).saturating_mul(m as u64))
			// Standard Error: 28_996
			.saturating_add(Weight::from_parts(338_093_u64, 0).saturating_mul(a as u64))
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
		// Minimum execution time: 80_700 nanoseconds.
		Weight::from_parts(81_790_000_u64, 0)
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
		// Minimum execution time: 80_280 nanoseconds.
		Weight::from_parts(82_150_000_u64, 0)
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
		// Minimum execution time: 46_090 nanoseconds.
		Weight::from_parts(47_440_000_u64, 0)
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
		// Minimum execution time: 75_790 nanoseconds.
		Weight::from_parts(78_289_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `NodleUniques::CollectionExtraDepositDetails` (r:1 w:1)
	// Proof: `NodleUniques::CollectionExtraDepositDetails` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn update_extra_deposit_limit() -> Weight {
		// Minimum execution time: 24_900 nanoseconds.
		Weight::from_parts(25_740_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
