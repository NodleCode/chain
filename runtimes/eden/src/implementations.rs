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

use crate::{Authorship, Balances, DaoReserve};
use frame_support::traits::{
	fungible::{Balanced, Credit},
	Imbalance, OnUnbalanced,
};
use primitives::AccountId;
use support::WithAccountId;

pub struct ToAuthor;
impl OnUnbalanced<Credit<AccountId, Balances>> for ToAuthor {
	fn on_nonzero_unbalanced(amount: Credit<AccountId, Balances>) {
		if let Some(author) = Authorship::author() {
			let _ = Balances::resolve(&author, amount);
		}
	}
}

/// Splits fees 20/80 between reserve and block author.
/// Fungible implementation of `OnUnbalanced` that deals with the fees by combining tip and fee and
/// spliting the result between the author and the DaoReserve.
pub struct DealWithFees;
impl OnUnbalanced<Credit<AccountId, Balances>> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = Credit<AccountId, Balances>>) {
		if let Some(mut fees) = fees_then_tips.next() {
			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut fees);
			}
			// for fees, 20% to treasury, 80% to author
			let mut split = fees.ration(20, 80);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 20% to treasury, 80% to author (though this can be anything)
				tips.ration_merge_into(20, 80, &mut split);
			}
			let _ = Balances::resolve(&DaoReserve::account_id(), split.0);
			ToAuthor::on_unbalanced(split.1);
		}
	}
}
