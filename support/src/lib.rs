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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	traits::tokens::Balance,
	RuntimeDebug,
};

pub trait WithAccountId<AccountId> {
	fn account_id() -> AccountId;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct LimitedBalance<T: Balance> {
	/// The cap for the balance
	///
	/// `balance` should never go higher than `limit`.
	limit: T,
	/// The balance so far
	balance: T,
}

pub enum LimitedBalanceError {
	/// Overflow adding to the balance
	Overflow,
	/// The balance exceeds the limit
	BalanceExceedsLimit,
	/// The limit is below the current commitment
	LimitBelowCommitment,
}

impl<T: Balance> LimitedBalance<T> {
	pub fn with_limit(limit: T) -> Self {
		Self {
			limit,
			..Default::default()
		}
	}
	pub fn add(&mut self, value: T) -> Result<(), LimitedBalanceError> {
		let new_total = self.balance.checked_add(&value).ok_or(LimitedBalanceError::Overflow)?;
		if new_total <= self.limit {
			self.balance = new_total;
			Ok(())
		} else {
			Err(LimitedBalanceError::BalanceExceedsLimit)
		}
	}
	pub fn saturating_add(&mut self, value: T) {
		self.balance = self.balance.saturating_add(value);
		if self.balance > self.limit {
			self.balance = self.limit;
		}
	}
	pub fn saturating_sub(&mut self, value: T) {
		self.balance = self.balance.saturating_sub(value);
	}
	pub fn update_limit(&mut self, new_limit: T) -> Result<(), LimitedBalanceError> {
		if new_limit >= self.balance {
			self.limit = new_limit;
			Ok(())
		} else {
			Err(LimitedBalanceError::LimitBelowCommitment)
		}
	}
	pub fn balance(&self) -> T {
		self.balance
	}
	pub fn limit(&self) -> T {
		self.limit
	}
	/// Returns the amount to add to the balance to reach the limit.
	pub fn available_margin(&self) -> T {
		self.limit.saturating_sub(self.balance)
	}
}
