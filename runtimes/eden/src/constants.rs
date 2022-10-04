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

use frame_support::{
	parameter_types,
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_PER_SECOND},
		DispatchClass, Weight,
	},
};
use frame_system::limits::BlockWeights;
use primitives::{Balance, BlockNumber};
pub use sp_runtime::{Perbill, Perquintill};
use static_assertions::const_assert;

/// Money matters.
pub const NODL: Balance = 100_000_000_000;
pub const MILLI_NODL: Balance = NODL / 1_000;
pub const MICRO_NODL: Balance = MILLI_NODL / 1_000;
pub const NANO_NODL: Balance = MICRO_NODL / 1_000;

pub const EXISTENTIAL_DEPOSIT: Balance = 100 * NANO_NODL;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 1_500 * MICRO_NODL + (bytes as Balance) * 600 * MICRO_NODL
}

/// Time and blocks.
pub const MILLISECS_PER_BLOCK: u64 = 12000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
pub const EPOCH_DURATION_IN_SLOTS: u64 = {
	const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;
	(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
};

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const MILLISECS_PER_RELAY_CHAIN_BLOCK: u64 = MILLISECS_PER_BLOCK / 2;
pub const MINUTES_RELAY_CHAIN: BlockNumber = 60_000 / (MILLISECS_PER_RELAY_CHAIN_BLOCK as BlockNumber);
pub const HOURS_RELAY_CHAIN: BlockNumber = MINUTES_RELAY_CHAIN * 60;
pub const DAYS_RELAY_CHAIN: BlockNumber = HOURS_RELAY_CHAIN * 24;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// Fee-related.
/// The block saturation level. Fees will be updates based on this value.
pub const TARGET_BLOCK_FULLNESS: Perquintill = Perquintill::from_percent(25);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for .5 seconds of compute with a 16 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_div(2);

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};

parameter_types! {
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constants_did_not_change() {
		const DOLLARS: Balance = NODL / 100; // = 10 MILLI
		const CENTS: Balance = DOLLARS / 100; // = 100 MICRO
		const MILLICENTS: Balance = CENTS / 1_000; // = 100 NANO

		assert_eq!(10 * MILLI_NODL, DOLLARS);
		assert_eq!(100 * MICRO_NODL, CENTS);
		assert_eq!(100 * NANO_NODL, MILLICENTS);
	}
}
