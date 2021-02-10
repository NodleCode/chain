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

use crate::{Balances, Call, CompanyReserve};
use frame_support::{
    traits::{Currency, Imbalance, InstanceFilter, OnUnbalanced},
    RuntimeDebug,
};
use nodle_chain_primitives::AccountId;
use parity_scale_codec::{Decode, Encode};

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// All to the company reserve.
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        if let Some(fees) = fees_then_tips.next() {
            let mut total = fees;
            if let Some(tips) = fees_then_tips.next() {
                tips.merge_into(&mut total);
            }
            CompanyReserve::on_unbalanced(total);
        }
    }
}

/// The type used to represent the kinds of proxying allowed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Governance,
}
impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}
impl InstanceFilter<Call> for ProxyType {
    fn filter(&self, c: &Call) -> bool {
        match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => !matches!(
                c,
                Call::Balances(..)
                    | Call::Grants(..)
                    | Call::Indices(pallet_indices::Call::transfer(..))
            ),
            ProxyType::Governance => matches!(
                c,
                Call::FinancialCommittee(..)
                    | Call::RootCommittee(..)
                    | Call::TechnicalCommittee(..)
                    | Call::CompanyReserve(..)
                    | Call::UsaReserve(..)
                    | Call::InternationalReserve(..)
            ),
        }
    }
    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            (ProxyType::NonTransfer, _) => true,
            _ => false,
        }
    }
}
