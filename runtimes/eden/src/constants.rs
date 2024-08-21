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
	dispatch::DispatchClass,
	parameter_types,
	weights::constants::{BlockExecutionWeight, ExtrinsicBaseWeight},
};
use frame_system::limits::BlockWeights;
use pallet_contracts::DebugInfo;
pub use parachains_common::{
	AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT, MILLISECS_PER_BLOCK, MINUTES,
	NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};
use primitives::{Balance, BlockNumber};
pub use sp_runtime::{Perbill, Perquintill};

/// Money matters.
pub const NODL: Balance = 100_000_000_000;
pub const MILLI_NODL: Balance = NODL / 1_000;
pub const MICRO_NODL: Balance = MILLI_NODL / 1_000;
pub const NANO_NODL: Balance = MICRO_NODL / 1_000;

pub const EXISTENTIAL_DEPOSIT: Balance = 100 * NANO_NODL;
pub const POLKADOT_EXISTENTIAL_DEPOSIT: Balance = 10_000_000_000;
pub const POLKADOT_CENT: Balance = 100_000_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 1_500 * MICRO_NODL + (bytes as Balance) * 600 * MICRO_NODL
}

/// Time and blocks.
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
pub const EPOCH_DURATION_IN_SLOTS: u64 = {
	const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;
	(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
};

pub const MINUTES_RELAY_CHAIN: BlockNumber = 60_000 / (RELAY_CHAIN_SLOT_DURATION_MILLIS as BlockNumber);
pub const HOURS_RELAY_CHAIN: BlockNumber = MINUTES_RELAY_CHAIN * 60;
pub const DAYS_RELAY_CHAIN: BlockNumber = HOURS_RELAY_CHAIN * 24;

/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
/// into the relay chain.
pub const UNINCLUDED_SEGMENT_CAPACITY: u32 = 1;
/// How many parachain blocks are processed by the relay chain per parent. Limits the
/// number of blocks authored per slot.
pub const BLOCK_PROCESSING_VELOCITY: u32 = 1;
/// Relay chain slot duration, in milliseconds.
pub const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// Fee-related.
/// The block saturation level. Fees will be updates based on this value.
pub const TARGET_BLOCK_FULLNESS: Perquintill = Perquintill::from_percent(25);

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

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: DebugInfo = DebugInfo::UnsafeDebug;

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
		assert_eq!(EXISTENTIAL_DEPOSIT, 10_000);
		assert_eq!(NANO_NODL, 100);
		assert_eq!(MICRO_NODL, 100_000);
		assert_eq!(NODL, 1e11 as u128);
	}

	#[test]
	fn test_deposit() {
		assert_eq!(deposit(0, 0), 0 as Balance);
		assert_eq!(deposit(1000, 0), 150000000000 as Balance);
		assert_eq!(deposit(0, 1000), 60000000000 as Balance);
		assert_eq!(deposit(0xFFFF_FFFF, 0xFFFF_FFFF), 901943131950000000 as Balance);
	}

	#[test]
	fn polkadot_constants() {
		assert_eq!(100 * POLKADOT_CENT, POLKADOT_EXISTENTIAL_DEPOSIT);
	}
}
