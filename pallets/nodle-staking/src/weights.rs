/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

//! Autogenerated weights for pallet_nodle_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-13, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/nodle-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nodle_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/nodle-staking/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nodle_staking.
pub trait WeightInfo {
	fn set_invulnerables(c: u32, ) -> Weight;
	fn set_total_validator_per_round(c: u32, ) -> Weight;
	fn validator_join_pool() -> Weight;
	fn validator_bond_more() -> Weight;
	fn validator_bond_less() -> Weight;
	fn validator_exit_pool() -> Weight;
	fn nominator_nominate() -> Weight;
	fn nominator_denominate() -> Weight;
	fn nominator_bond_more() -> Weight;
	fn nominator_bond_less() -> Weight;
	fn nominator_denominate_all() -> Weight;
	fn withdraw_unbonded() -> Weight;
	fn slash_cancel_deferred(s: u32, c: u32, ) -> Weight;
}

/// Weights for pallet_nodle_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn set_invulnerables(c: u32, ) -> Weight {
		(20_119_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((461_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_total_validator_per_round(_c: u32, ) -> Weight {
		(21_044_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn validator_join_pool() -> Weight {
		(79_859_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn validator_bond_more() -> Weight {
		(74_830_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn validator_bond_less() -> Weight {
		(49_343_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn validator_exit_pool() -> Weight {
		(50_965_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn nominator_nominate() -> Weight {
		(96_521_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn nominator_denominate() -> Weight {
		(79_620_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn nominator_bond_more() -> Weight {
		(82_795_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn nominator_bond_less() -> Weight {
		(58_730_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn nominator_denominate_all() -> Weight {
		(1_180_123_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(33 as Weight))
			.saturating_add(T::DbWeight::get().writes(30 as Weight))
	}
	fn withdraw_unbonded() -> Weight {
		(34_254_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn slash_cancel_deferred(s: u32, c: u32, ) -> Weight {
		(6_954_811_000 as Weight)
			// Standard Error: 431_000
			.saturating_add((8_524_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 431_000
			.saturating_add((7_113_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_invulnerables(c: u32, ) -> Weight {
		(20_119_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((461_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_total_validator_per_round(_c: u32, ) -> Weight {
		(21_044_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn validator_join_pool() -> Weight {
		(79_859_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn validator_bond_more() -> Weight {
		(74_830_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn validator_bond_less() -> Weight {
		(49_343_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn validator_exit_pool() -> Weight {
		(50_965_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn nominator_nominate() -> Weight {
		(96_521_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn nominator_denominate() -> Weight {
		(79_620_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn nominator_bond_more() -> Weight {
		(82_795_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn nominator_bond_less() -> Weight {
		(58_730_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn nominator_denominate_all() -> Weight {
		(1_180_123_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(33 as Weight))
			.saturating_add(RocksDbWeight::get().writes(30 as Weight))
	}
	fn withdraw_unbonded() -> Weight {
		(34_254_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn slash_cancel_deferred(s: u32, c: u32, ) -> Weight {
		(6_954_811_000 as Weight)
			// Standard Error: 431_000
			.saturating_add((8_524_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 431_000
			.saturating_add((7_113_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
