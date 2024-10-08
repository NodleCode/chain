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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --pallet=pallet_scheduler
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

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: `Scheduler::IncompleteSince` (r:1 w:1)
	// Proof: `Scheduler::IncompleteSince` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 1_510 nanoseconds.
		Weight::from_parts(1_580_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32) -> Weight {
		// Minimum execution time: 3_100 nanoseconds.
		Weight::from_parts(5_776_694_u64, 0)
			// Standard Error: 3_483
			.saturating_add(Weight::from_parts(527_329_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 3_609 nanoseconds.
		Weight::from_parts(3_740_000_u64, 0)
	}
	// Storage: `Preimage::PreimageFor` (r:1 w:1)
	// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
	// Storage: `Preimage::StatusFor` (r:1 w:0)
	// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32) -> Weight {
		// Minimum execution time: 19_690 nanoseconds.
		Weight::from_parts(20_050_000_u64, 0)
			// Standard Error: 11
			.saturating_add(Weight::from_parts(1_024_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Lookup` (r:0 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn service_task_named() -> Weight {
		// Minimum execution time: 5_491 nanoseconds.
		Weight::from_parts(5_729_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 3_570 nanoseconds.
		Weight::from_parts(3_730_000_u64, 0)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 2_460 nanoseconds.
		Weight::from_parts(2_680_000_u64, 0)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 2_420 nanoseconds.
		Weight::from_parts(2_650_000_u64, 0)
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32) -> Weight {
		// Minimum execution time: 11_100 nanoseconds.
		Weight::from_parts(14_446_796_u64, 0)
			// Standard Error: 3_336
			.saturating_add(Weight::from_parts(569_829_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Lookup` (r:0 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32) -> Weight {
		// Minimum execution time: 18_260 nanoseconds.
		Weight::from_parts(17_186_629_u64, 0)
			// Standard Error: 4_517
			.saturating_add(Weight::from_parts(864_238_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32) -> Weight {
		// Minimum execution time: 15_430 nanoseconds.
		Weight::from_parts(19_645_376_u64, 0)
			// Standard Error: 3_824
			.saturating_add(Weight::from_parts(598_193_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32) -> Weight {
		// Minimum execution time: 20_700 nanoseconds.
		Weight::from_parts(20_383_151_u64, 0)
			// Standard Error: 4_807
			.saturating_add(Weight::from_parts(914_227_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 50]`.
	fn schedule_retry(s: u32) -> Weight {
		// Minimum execution time: 10_660 nanoseconds.
		Weight::from_parts(11_365_922_u64, 0)
			// Standard Error: 895
			.saturating_add(Weight::from_parts(33_425_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:0)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn set_retry() -> Weight {
		// Minimum execution time: 30_890 nanoseconds.
		Weight::from_parts(31_570_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:0)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:0)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn set_retry_named() -> Weight {
		// Minimum execution time: 40_189 nanoseconds.
		Weight::from_parts(41_590_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:0)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel_retry() -> Weight {
		// Minimum execution time: 29_570 nanoseconds.
		Weight::from_parts(31_170_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:0)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:0)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Retries` (r:0 w:1)
	// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel_retry_named() -> Weight {
		// Minimum execution time: 39_200 nanoseconds.
		Weight::from_parts(40_340_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
