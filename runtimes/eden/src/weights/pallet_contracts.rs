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

//! Autogenerated weights for pallet_contracts
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_contracts
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

/// Weight functions for `pallet_contracts`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_contracts::weights::WeightInfo for WeightInfo<T> {
	// Storage: `Contracts::DeletionQueueCounter` (r:1 w:0)
	// Proof: `Contracts::DeletionQueueCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	fn on_process_deletion_queue_batch() -> Weight {
		// Minimum execution time: 700 nanoseconds.
		Weight::from_parts(770_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `k` is `[0, 1024]`.
	fn on_initialize_per_trie_key(k: u32) -> Weight {
		// Minimum execution time: 13_660 nanoseconds.
		Weight::from_parts(14_040_000_u64, 0)
			// Standard Error: 3_206
			.saturating_add(Weight::from_parts(1_139_124_u64, 0).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(k as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k as u64)))
	}
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:2 w:1)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:2 w:1)
	/// The range of component `c` is `[0, 125952]`.
	fn v9_migration_step(c: u32) -> Weight {
		// Minimum execution time: 6_850 nanoseconds.
		Weight::from_parts(5_787_029_u64, 0)
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_491_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::ContractInfoOf` (r:2 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	fn v10_migration_step() -> Weight {
		// Minimum execution time: 19_120 nanoseconds.
		Weight::from_parts(19_620_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::DeletionQueue` (r:1 w:1025)
	// Proof: `Contracts::DeletionQueue` (`max_values`: None, `max_size`: Some(142), added: 2617, mode: `Measured`)
	// Storage: `Contracts::DeletionQueueCounter` (r:0 w:1)
	// Proof: `Contracts::DeletionQueueCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	/// The range of component `k` is `[0, 1024]`.
	fn v11_migration_step(k: u32) -> Weight {
		// Minimum execution time: 2_280 nanoseconds.
		Weight::from_parts(2_409_000_u64, 0)
			// Standard Error: 802
			.saturating_add(Weight::from_parts(1_607_861_u64, 0).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k as u64)))
	}
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553053f13fd319a03c211337c76e0fe776df` (r:2 w:0)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553053f13fd319a03c211337c76e0fe776df` (r:2 w:0)
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:1 w:1)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:1 w:1)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:0 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	/// The range of component `c` is `[0, 125952]`.
	fn v12_migration_step(c: u32) -> Weight {
		// Minimum execution time: 16_640 nanoseconds.
		Weight::from_parts(19_794_654_u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(382_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Contracts::ContractInfoOf` (r:2 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	fn v13_migration_step() -> Weight {
		// Minimum execution time: 14_650 nanoseconds.
		Weight::from_parts(15_060_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::CodeInfoOf` (r:2 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	// Storage: `Balances::Holds` (r:1 w:0)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	fn v14_migration_step() -> Weight {
		// Minimum execution time: 56_520 nanoseconds.
		Weight::from_parts(58_191_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::ContractInfoOf` (r:2 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	fn v15_migration_step() -> Weight {
		// Minimum execution time: 69_600 nanoseconds.
		Weight::from_parts(71_389_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Contracts::ContractInfoOf` (r:2 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	fn v16_migration_step() -> Weight {
		// Minimum execution time: 14_100 nanoseconds.
		Weight::from_parts(14_460_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:1)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	fn migration_noop() -> Weight {
		// Minimum execution time: 1_790 nanoseconds.
		Weight::from_parts(1_920_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:1)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:1)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:1)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:0)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	fn migrate() -> Weight {
		// Minimum execution time: 20_620 nanoseconds.
		Weight::from_parts(21_520_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	fn on_runtime_upgrade_noop() -> Weight {
		// Minimum execution time: 5_010 nanoseconds.
		Weight::from_parts(5_250_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	fn on_runtime_upgrade_in_progress() -> Weight {
		// Minimum execution time: 6_910 nanoseconds.
		Weight::from_parts(7_150_000_u64, 0).saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Contracts::MigrationInProgress` (r:1 w:1)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	fn on_runtime_upgrade() -> Weight {
		// Minimum execution time: 6_850 nanoseconds.
		Weight::from_parts(7_230_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	/// The range of component `c` is `[0, 125952]`.
	fn call_with_code_per_byte(c: u32) -> Weight {
		// Minimum execution time: 338_520 nanoseconds.
		Weight::from_parts(357_967_349_u64, 0)
			// Standard Error: 119
			.saturating_add(Weight::from_parts(43_171_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Balances::Holds` (r:2 w:2)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	// Storage: `Contracts::Nonce` (r:1 w:1)
	// Proof: `Contracts::Nonce` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:0 w:1)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	/// The range of component `c` is `[0, 125952]`.
	/// The range of component `i` is `[0, 1048576]`.
	/// The range of component `s` is `[0, 1048576]`.
	fn instantiate_with_code(c: u32, i: u32, s: u32) -> Weight {
		// Minimum execution time: 5_100_100 nanoseconds.
		Weight::from_parts(733_341_039_u64, 0)
			// Standard Error: 151
			.saturating_add(Weight::from_parts(86_385_u64, 0).saturating_mul(c as u64))
			// Standard Error: 18
			.saturating_add(Weight::from_parts(2_109_u64, 0).saturating_mul(i as u64))
			// Standard Error: 18
			.saturating_add(Weight::from_parts(2_103_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	// Storage: `Contracts::Nonce` (r:1 w:1)
	// Proof: `Contracts::Nonce` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	// Storage: `Balances::Holds` (r:1 w:1)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	/// The range of component `i` is `[0, 1048576]`.
	/// The range of component `s` is `[0, 1048576]`.
	fn instantiate(i: u32, s: u32) -> Weight {
		// Minimum execution time: 2_536_600 nanoseconds.
		Weight::from_parts(405_055_705_u64, 0)
			// Standard Error: 6
			.saturating_add(Weight::from_parts(2_138_u64, 0).saturating_mul(i as u64))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(2_044_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	fn call() -> Weight {
		// Minimum execution time: 265_260 nanoseconds.
		Weight::from_parts(274_010_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Balances::Holds` (r:1 w:1)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:0 w:1)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	/// The range of component `c` is `[0, 125952]`.
	fn upload_code_determinism_enforced(c: u32) -> Weight {
		// Minimum execution time: 310_080 nanoseconds.
		Weight::from_parts(301_203_754_u64, 0)
			// Standard Error: 341
			.saturating_add(Weight::from_parts(45_080_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Balances::Holds` (r:1 w:1)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:0 w:1)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	/// The range of component `c` is `[0, 125952]`.
	fn upload_code_determinism_relaxed(c: u32) -> Weight {
		// Minimum execution time: 331_530 nanoseconds.
		Weight::from_parts(339_994_489_u64, 0)
			// Standard Error: 37
			.saturating_add(Weight::from_parts(44_464_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Balances::Holds` (r:1 w:1)
	// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:0 w:1)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	fn remove_code() -> Weight {
		// Minimum execution time: 51_840 nanoseconds.
		Weight::from_parts(52_970_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:2 w:2)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	fn set_code() -> Weight {
		// Minimum execution time: 28_480 nanoseconds.
		Weight::from_parts(29_750_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// The range of component `r` is `[0, 1600]`.
	fn noop_host_fn(r: u32) -> Weight {
		// Minimum execution time: 13_980 nanoseconds.
		Weight::from_parts(14_791_291_u64, 0)
			// Standard Error: 65
			.saturating_add(Weight::from_parts(76_552_u64, 0).saturating_mul(r as u64))
	}
	fn seal_caller() -> Weight {
		// Minimum execution time: 940 nanoseconds.
		Weight::from_parts(1_010_000_u64, 0)
	}
	// Storage: `Contracts::ContractInfoOf` (r:1 w:0)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	fn seal_is_contract() -> Weight {
		// Minimum execution time: 7_370 nanoseconds.
		Weight::from_parts(7_710_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `Contracts::ContractInfoOf` (r:1 w:0)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	fn seal_code_hash() -> Weight {
		// Minimum execution time: 8_990 nanoseconds.
		Weight::from_parts(9_300_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn seal_own_code_hash() -> Weight {
		// Minimum execution time: 1_140 nanoseconds.
		Weight::from_parts(1_230_000_u64, 0)
	}
	fn seal_caller_is_origin() -> Weight {
		// Minimum execution time: 540 nanoseconds.
		Weight::from_parts(620_000_u64, 0)
	}
	fn seal_caller_is_root() -> Weight {
		// Minimum execution time: 530 nanoseconds.
		Weight::from_parts(550_000_u64, 0)
	}
	fn seal_address() -> Weight {
		// Minimum execution time: 910 nanoseconds.
		Weight::from_parts(980_000_u64, 0)
	}
	fn seal_gas_left() -> Weight {
		// Minimum execution time: 1_000 nanoseconds.
		Weight::from_parts(1_120_000_u64, 0)
	}
	fn seal_balance() -> Weight {
		// Minimum execution time: 4_820 nanoseconds.
		Weight::from_parts(5_170_000_u64, 0)
	}
	fn seal_value_transferred() -> Weight {
		// Minimum execution time: 890 nanoseconds.
		Weight::from_parts(950_000_u64, 0)
	}
	fn seal_minimum_balance() -> Weight {
		// Minimum execution time: 910 nanoseconds.
		Weight::from_parts(970_000_u64, 0)
	}
	fn seal_block_number() -> Weight {
		// Minimum execution time: 870 nanoseconds.
		Weight::from_parts(980_000_u64, 0)
	}
	fn seal_now() -> Weight {
		// Minimum execution time: 890 nanoseconds.
		Weight::from_parts(931_000_u64, 0)
	}
	// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `Measured`)
	fn seal_weight_to_fee() -> Weight {
		// Minimum execution time: 2_300 nanoseconds.
		Weight::from_parts(2_400_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// The range of component `n` is `[0, 1048572]`.
	fn seal_input(n: u32) -> Weight {
		// Minimum execution time: 660 nanoseconds.
		Weight::from_parts(847_117_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(142_u64, 0).saturating_mul(n as u64))
	}
	/// The range of component `n` is `[0, 1048572]`.
	fn seal_return(n: u32) -> Weight {
		// Minimum execution time: 530 nanoseconds.
		Weight::from_parts(514_629_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(215_u64, 0).saturating_mul(n as u64))
	}
	// Storage: `Contracts::DeletionQueueCounter` (r:1 w:1)
	// Proof: `Contracts::DeletionQueueCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:33 w:33)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::DeletionQueue` (r:0 w:1)
	// Proof: `Contracts::DeletionQueue` (`max_values`: None, `max_size`: Some(142), added: 2617, mode: `Measured`)
	/// The range of component `n` is `[0, 32]`.
	fn seal_terminate(n: u32) -> Weight {
		// Minimum execution time: 16_550 nanoseconds.
		Weight::from_parts(17_597_636_u64, 0)
			// Standard Error: 7_045
			.saturating_add(Weight::from_parts(4_149_734_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}
	// Storage: `RandomnessCollectiveFlip::RandomMaterial` (r:1 w:0)
	// Proof: `RandomnessCollectiveFlip::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `Measured`)
	fn seal_random() -> Weight {
		// Minimum execution time: 2_680 nanoseconds.
		Weight::from_parts(2_760_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `System::EventTopics` (r:4 w:4)
	// Proof: `System::EventTopics` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[0, 4]`.
	/// The range of component `n` is `[0, 16384]`.
	fn seal_deposit_event(t: u32, n: u32) -> Weight {
		// Minimum execution time: 4_540 nanoseconds.
		Weight::from_parts(4_944_184_u64, 0)
			// Standard Error: 8_103
			.saturating_add(Weight::from_parts(2_542_719_u64, 0).saturating_mul(t as u64))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(21_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t as u64)))
	}
	/// The range of component `i` is `[0, 1048576]`.
	fn seal_debug_message(i: u32) -> Weight {
		// Minimum execution time: 620 nanoseconds.
		Weight::from_parts(1_928_062_u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(795_u64, 0).saturating_mul(i as u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 16384]`.
	/// The range of component `o` is `[0, 16384]`.
	fn seal_set_storage(n: u32, o: u32) -> Weight {
		// Minimum execution time: 9_500 nanoseconds.
		Weight::from_parts(7_598_150_u64, 0)
			// Standard Error: 8
			.saturating_add(Weight::from_parts(364_u64, 0).saturating_mul(n as u64))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(123_u64, 0).saturating_mul(o as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 16384]`.
	fn seal_clear_storage(n: u32) -> Weight {
		// Minimum execution time: 6_890 nanoseconds.
		Weight::from_parts(8_239_518_u64, 0)
			// Standard Error: 12
			.saturating_add(Weight::from_parts(222_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 16384]`.
	fn seal_get_storage(n: u32) -> Weight {
		// Minimum execution time: 6_380 nanoseconds.
		Weight::from_parts(8_214_505_u64, 0)
			// Standard Error: 17
			.saturating_add(Weight::from_parts(821_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 16384]`.
	fn seal_contains_storage(n: u32) -> Weight {
		// Minimum execution time: 5_680 nanoseconds.
		Weight::from_parts(7_000_916_u64, 0)
			// Standard Error: 12
			.saturating_add(Weight::from_parts(233_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: `Skipped::Metadata` (r:0 w:0)
	// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 16384]`.
	fn seal_take_storage(n: u32) -> Weight {
		// Minimum execution time: 7_700 nanoseconds.
		Weight::from_parts(9_783_668_u64, 0)
			// Standard Error: 20
			.saturating_add(Weight::from_parts(771_u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn seal_transfer() -> Weight {
		// Minimum execution time: 10_451 nanoseconds.
		Weight::from_parts(10_720_000_u64, 0)
	}
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	/// The range of component `t` is `[0, 1]`.
	/// The range of component `i` is `[0, 1048576]`.
	fn seal_call(t: u32, i: u32) -> Weight {
		// Minimum execution time: 207_240 nanoseconds.
		Weight::from_parts(213_979_899_u64, 0)
			// Standard Error: 248_924
			.saturating_add(Weight::from_parts(50_945_757_u64, 0).saturating_mul(t as u64))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t as u64)))
	}
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	fn seal_delegate_call() -> Weight {
		// Minimum execution time: 198_520 nanoseconds.
		Weight::from_parts(204_580_000_u64, 0).saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	// Storage: `Contracts::Nonce` (r:1 w:0)
	// Proof: `Contracts::Nonce` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `Measured`)
	/// The range of component `t` is `[0, 1]`.
	/// The range of component `i` is `[0, 983040]`.
	/// The range of component `s` is `[0, 983040]`.
	fn seal_instantiate(t: u32, i: u32, s: u32) -> Weight {
		// Minimum execution time: 2_213_270 nanoseconds.
		Weight::from_parts(189_831_305_u64, 0)
			// Standard Error: 6_285_217
			.saturating_add(Weight::from_parts(133_320_105_u64, 0).saturating_mul(t as u64))
			// Standard Error: 10
			.saturating_add(Weight::from_parts(1_922_u64, 0).saturating_mul(i as u64))
			// Standard Error: 10
			.saturating_add(Weight::from_parts(2_078_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// The range of component `n` is `[0, 1048576]`.
	fn seal_hash_sha2_256(n: u32) -> Weight {
		// Minimum execution time: 1_190 nanoseconds.
		Weight::from_parts(1_250_000_u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_322_u64, 0).saturating_mul(n as u64))
	}
	/// The range of component `n` is `[0, 1048576]`.
	fn seal_hash_keccak_256(n: u32) -> Weight {
		// Minimum execution time: 1_630 nanoseconds.
		Weight::from_parts(1_650_000_u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(3_386_u64, 0).saturating_mul(n as u64))
	}
	/// The range of component `n` is `[0, 1048576]`.
	fn seal_hash_blake2_256(n: u32) -> Weight {
		// Minimum execution time: 1_110 nanoseconds.
		Weight::from_parts(1_150_000_u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_666_u64, 0).saturating_mul(n as u64))
	}
	/// The range of component `n` is `[0, 1048576]`.
	fn seal_hash_blake2_128(n: u32) -> Weight {
		// Minimum execution time: 1_080 nanoseconds.
		Weight::from_parts(1_120_000_u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_666_u64, 0).saturating_mul(n as u64))
	}
	/// The range of component `n` is `[0, 125697]`.
	fn seal_sr25519_verify(n: u32) -> Weight {
		// Minimum execution time: 44_280 nanoseconds.
		Weight::from_parts(43_060_432_u64, 0)
			// Standard Error: 7
			.saturating_add(Weight::from_parts(5_253_u64, 0).saturating_mul(n as u64))
	}
	fn seal_ecdsa_recover() -> Weight {
		// Minimum execution time: 44_070 nanoseconds.
		Weight::from_parts(45_300_000_u64, 0)
	}
	fn seal_ecdsa_to_eth_address() -> Weight {
		// Minimum execution time: 12_560 nanoseconds.
		Weight::from_parts(12_700_000_u64, 0)
	}
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `Measured`)
	fn seal_set_code_hash() -> Weight {
		// Minimum execution time: 21_240 nanoseconds.
		Weight::from_parts(21_780_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `Measured`)
	fn lock_delegate_dependency() -> Weight {
		// Minimum execution time: 10_460 nanoseconds.
		Weight::from_parts(10_760_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Contracts::CodeInfoOf` (r:1 w:1)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn unlock_delegate_dependency() -> Weight {
		// Minimum execution time: 9_210 nanoseconds.
		Weight::from_parts(9_480_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn seal_reentrance_count() -> Weight {
		// Minimum execution time: 560 nanoseconds.
		Weight::from_parts(590_000_u64, 0)
	}
	fn seal_account_reentrance_count() -> Weight {
		// Minimum execution time: 550 nanoseconds.
		Weight::from_parts(590_000_u64, 0)
	}
	// Storage: `Contracts::Nonce` (r:1 w:0)
	// Proof: `Contracts::Nonce` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `Measured`)
	fn seal_instantiation_nonce() -> Weight {
		// Minimum execution time: 2_650 nanoseconds.
		Weight::from_parts(2_810_000_u64, 0).saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// The range of component `r` is `[0, 5000]`.
	fn instr_i64_load_store(r: u32) -> Weight {
		// Minimum execution time: 1_109 nanoseconds.
		Weight::from_parts(1_485_072_u64, 0)
			// Standard Error: 7
			.saturating_add(Weight::from_parts(15_437_u64, 0).saturating_mul(r as u64))
	}
}
