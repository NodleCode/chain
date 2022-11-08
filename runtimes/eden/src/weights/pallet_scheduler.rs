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
//! DATE: 2022-11-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_scheduler.
pub trait WeightInfo {
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight;
	fn on_initialize_named_resolved(s: u32, ) -> Weight;
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight;
	fn on_initialize_resolved(s: u32, ) -> Weight;
	fn on_initialize_named_aborted(s: u32, ) -> Weight;
	fn on_initialize_aborted(s: u32, ) -> Weight;
	fn on_initialize_periodic_named(s: u32, ) -> Weight;
	fn on_initialize_periodic(s: u32, ) -> Weight;
	fn on_initialize_named(s: u32, ) -> Weight;
	fn on_initialize(s: u32, ) -> Weight;
	fn schedule(s: u32, ) -> Weight;
	fn cancel(s: u32, ) -> Weight;
	fn schedule_named(s: u32, ) -> Weight;
	fn cancel_named(s: u32, ) -> Weight;}

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(79_056_000_u64)
			// Standard Error: 1_622_000
			.saturating_add(Weight::from_ref_time(43_087_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(s as u64)))	}
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(408_143_000_u64)
			// Standard Error: 650_000
			.saturating_add(Weight::from_ref_time(14_976_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(s as u64)))	}
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 984_000
			.saturating_add(Weight::from_ref_time(57_870_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(s as u64)))	}
	fn on_initialize_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(25_966_000_u64)
			// Standard Error: 44_000
			.saturating_add(Weight::from_ref_time(46_670_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(s as u64)))	}
	fn on_initialize_named_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(18_704_000_u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(19_665_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(2_u64))			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(2_u64))			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}
	fn on_initialize_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(24_465_000_u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(8_538_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(2_u64))			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(2_u64))	}
	fn on_initialize_periodic_named(s: u32, ) -> Weight {
		Weight::from_ref_time(29_230_000_u64)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(32_730_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(s as u64)))	}
	fn on_initialize_periodic(s: u32, ) -> Weight {
		Weight::from_ref_time(31_128_000_u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(21_426_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}
	fn on_initialize_named(s: u32, ) -> Weight {
		Weight::from_ref_time(31_248_000_u64)
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(20_127_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().writes(3_u64))			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}
	fn on_initialize(s: u32, ) -> Weight {
		Weight::from_ref_time(31_074_000_u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(15_644_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().writes(3_u64))	}
	fn schedule(s: u32, ) -> Weight {
		Weight::from_ref_time(49_528_000_u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(111_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().writes(3_u64))	}
	fn cancel(s: u32, ) -> Weight {
		Weight::from_ref_time(47_443_000_u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(1_260_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(5_u64))			.saturating_add(T::DbWeight::get().writes(4_u64))	}
	fn schedule_named(s: u32, ) -> Weight {
		Weight::from_ref_time(61_046_000_u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(247_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(6_u64))			.saturating_add(T::DbWeight::get().writes(4_u64))	}
	fn cancel_named(s: u32, ) -> Weight {
		Weight::from_ref_time(56_234_000_u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(1_347_000).saturating_mul(s as u64))			.saturating_add(T::DbWeight::get().reads(6_u64))			.saturating_add(T::DbWeight::get().writes(4_u64))	}}

// For backwards compatibility and tests
impl WeightInfo for () {	
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(79_056_000_u64)

			// Standard Error: 1_622_000
			.saturating_add(Weight::from_ref_time(43_087_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(408_143_000_u64)

			// Standard Error: 650_000
			.saturating_add(Weight::from_ref_time(14_976_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)

			// Standard Error: 984_000
			.saturating_add(Weight::from_ref_time(57_870_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_resolved(s: u32, ) -> Weight {
		Weight::from_ref_time(25_966_000_u64)

			// Standard Error: 44_000
			.saturating_add(Weight::from_ref_time(46_670_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_named_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(18_704_000_u64)

			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(19_665_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(2_u64))			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(2_u64))			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_aborted(s: u32, ) -> Weight {
		Weight::from_ref_time(24_465_000_u64)

			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(8_538_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(2_u64))			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(2_u64))	}	
	fn on_initialize_periodic_named(s: u32, ) -> Weight {
		Weight::from_ref_time(29_230_000_u64)

			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(32_730_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_periodic(s: u32, ) -> Weight {
		Weight::from_ref_time(31_128_000_u64)

			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(21_426_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s as u64)))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}	
	fn on_initialize_named(s: u32, ) -> Weight {
		Weight::from_ref_time(31_248_000_u64)

			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(20_127_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().writes(3_u64))			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s as u64)))	}	
	fn on_initialize(s: u32, ) -> Weight {
		Weight::from_ref_time(31_074_000_u64)

			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(15_644_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().writes(3_u64))	}	
	fn schedule(s: u32, ) -> Weight {
		Weight::from_ref_time(49_528_000_u64)

			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(111_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().writes(3_u64))	}	
	fn cancel(s: u32, ) -> Weight {
		Weight::from_ref_time(47_443_000_u64)

			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(1_260_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(5_u64))			.saturating_add(RocksDbWeight::get().writes(4_u64))	}	
	fn schedule_named(s: u32, ) -> Weight {
		Weight::from_ref_time(61_046_000_u64)

			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(247_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(6_u64))			.saturating_add(RocksDbWeight::get().writes(4_u64))	}	
	fn cancel_named(s: u32, ) -> Weight {
		Weight::from_ref_time(56_234_000_u64)

			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(1_347_000).saturating_mul(s as u64))			.saturating_add(RocksDbWeight::get().reads(6_u64))			.saturating_add(RocksDbWeight::get().writes(4_u64))	}}
