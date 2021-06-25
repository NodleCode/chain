// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

// Modified by Eliott Teissonniere (Nodle International) to contain
// only the useful code for the Nodle Chain project.

//! Auxillary struct/enums for polkadot runtime.

use crate::{Authorship, Balances, CompanyReserve};

#[cfg(feature = "with-staking")]
use crate::Staking;

use frame_support::traits::{Currency, Imbalance, OnUnbalanced};
use nodle_chain_primitives::AccountId;

/// Logic for the author to get a portion of fees.
pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
    fn on_nonzero_unbalanced(amount: NegativeImbalance) {
        Balances::resolve_creating(&Authorship::author(), amount);
    }
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

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

            #[cfg(not(feature = "with-staking"))]
            Author::on_unbalanced(split.1);

            // 80% is moved to staking pallet, when staking is enabled.
            #[cfg(feature = "with-staking")]
            Staking::on_unbalanced(split.1);
        }
    }
}
