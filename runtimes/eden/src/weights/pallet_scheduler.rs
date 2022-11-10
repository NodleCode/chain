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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-10, STEPS: `8`, REPEAT: 11, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `tama`, CPU: `11th Gen Intel(R) Core(TM) i7-11700 @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=8
// --repeat=11
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(23_484_000_u64)
			// Standard Error: 55_000
			.saturating_add(Weight::from_ref_time(25_058_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(19_081_000_u64)
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(19_547_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(19_879_000_u64)
			// Standard Error: 97_000
			.saturating_add(Weight::from_ref_time(20_443_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(23_503_000_u64)
			// Standard Error: 52_000
			.saturating_add(Weight::from_ref_time(17_523_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(10_418_000_u64)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(6_776_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(11_191_000_u64)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(3_158_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_named(s: u32, ) -> Weight {
		Weight::from_ref_time(18_995_000_u64)
			// Standard Error: 29_000
			.saturating_add(Weight::from_ref_time(12_040_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic(s: u32, ) -> Weight {
		Weight::from_ref_time(16_458_000_u64)
			// Standard Error: 42_000
			.saturating_add(Weight::from_ref_time(8_178_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named(s: u32, ) -> Weight {
		Weight::from_ref_time(20_137_000_u64)
			// Standard Error: 49_000
			.saturating_add(Weight::from_ref_time(7_322_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize(s: u32, ) -> Weight {
		Weight::from_ref_time(21_607_000_u64)
			// Standard Error: 358_000
			.saturating_add(Weight::from_ref_time(6_519_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Number (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn schedule(s: u32, ) -> Weight {
		Weight::from_ref_time(21_644_000_u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(182_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		Weight::from_ref_time(23_935_000_u64)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(581_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn schedule_named(s: u32, ) -> Weight {
		Weight::from_ref_time(25_848_000_u64)
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(267_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		Weight::from_ref_time(27_420_000_u64)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(643_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
