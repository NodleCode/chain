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

//! Auxillary struct/enums for polkadot runtime.

use crate::{Balances, CollatorSelection, CompanyReserve};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};
use primitives::{AccountId, BlockNumber};
use sp_runtime::traits::BlockNumberProvider;

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// Implementation of `OnUnbalanced` that deposits the fees into a staking pot for later payout.
pub struct ToStakingPot;
impl OnUnbalanced<NegativeImbalance> for ToStakingPot {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		let staking_pot = CollatorSelection::account_id();
		Balances::resolve_creating(&staking_pot, amount);
	}
}

/// Splits fees 20/80 between reserve and block author.
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 20% to treasury, 80% to author
			let mut split = fees.ration(20, 80);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 20% to treasury, 80% to author (though this can be anything)
				tips.ration_merge_into(20, 80, &mut split);
			}
			CompanyReserve::on_unbalanced(split.0);
			ToStakingPot::on_unbalanced(split.1);
		}
	}
}

pub struct RelayChainBlockNumberProvider<T>(sp_std::marker::PhantomData<T>);

impl<T: cumulus_pallet_parachain_system::Config> BlockNumberProvider for RelayChainBlockNumberProvider<T> {
	type BlockNumber = BlockNumber;

	fn current_block_number() -> Self::BlockNumber {
		cumulus_pallet_parachain_system::Pallet::<T>::validation_data()
			.map(|d| d.relay_parent_number)
			.unwrap_or_default()
	}
}
