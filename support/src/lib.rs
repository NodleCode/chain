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

#[derive(PartialEq, Debug)]
pub enum LimitedBalanceError {
	/// Overflow adding to the balance
	Overflow,
	/// The balance exceeds the limit
	BalanceExceedsLimit,
	/// The limit is below the current commitment
	LimitBelowCommitment,
}

impl<T: Balance> LimitedBalance<T> {
	/// Create a new `LimitedBalance` with the given limit.
	pub fn with_limit(limit: T) -> Self {
		Self {
			limit,
			..Default::default()
		}
	}
	/// Add to the balance, returning an error if the limit is exceeded.
	pub fn add(&mut self, value: T) -> Result<(), LimitedBalanceError> {
		let new_total = self.balance.checked_add(&value).ok_or(LimitedBalanceError::Overflow)?;
		if new_total <= self.limit {
			self.balance = new_total;
			Ok(())
		} else {
			Err(LimitedBalanceError::BalanceExceedsLimit)
		}
	}
	/// Add to the balance, saturating at the limit.
	pub fn saturating_add(&mut self, value: T) {
		self.balance = self.balance.saturating_add(value);
		if self.balance > self.limit {
			self.balance = self.limit;
		}
	}
	/// Subtract from the balance, saturating at 0.
	pub fn saturating_sub(&mut self, value: T) {
		self.balance = self.balance.saturating_sub(value);
	}
	/// Update the limit, returning an error if the new limit is below the current commitment,
	/// meaning that the new limit would not be able to cover the current balance.
	pub fn update_limit(&mut self, new_limit: T) -> Result<(), LimitedBalanceError> {
		if new_limit >= self.balance {
			self.limit = new_limit;
			Ok(())
		} else {
			Err(LimitedBalanceError::LimitBelowCommitment)
		}
	}
	/// Returns the current balance.
	pub fn balance(&self) -> T {
		self.balance
	}
	/// Returns the current limit.
	pub fn limit(&self) -> T {
		self.limit
	}
	/// Returns the amount to add to the balance to reach the limit.
	pub fn available_margin(&self) -> T {
		self.limit.saturating_sub(self.balance)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn create_limited_balance() {
		let lb = LimitedBalance::<u32>::with_limit(100);
		assert_eq!(lb.balance(), 0);
		assert_eq!(lb.limit(), 100);
		assert_eq!(lb.available_margin(), 100);
	}

	#[test]
	fn add_to_balance() {
		let mut lb = LimitedBalance::<u32>::with_limit(100);
		assert_eq!(lb.add(50), Ok(()));
		assert_eq!(lb.balance(), 50);
		assert_eq!(lb.available_margin(), 50);
		assert_eq!(lb.add(20), Ok(()));
		assert_eq!(lb.balance(), 70);
		assert_eq!(lb.limit(), 100);
		assert_eq!(lb.available_margin(), 30);
	}

	#[test]
	fn exceed_limit() {
		let mut lb = LimitedBalance::<u32>::with_limit(100);
		assert_eq!(lb.add(100), Ok(()));
		assert_eq!(lb.add(1), Err(LimitedBalanceError::BalanceExceedsLimit));
		assert_eq!(lb.balance(), 100);
		assert_eq!(lb.limit(), 100);
		assert_eq!(lb.available_margin(), 0);
	}

	#[test]
	fn update_limit() {
		let mut lb = LimitedBalance::<u32>::with_limit(100);
		assert_eq!(lb.add(100), Ok(()));
		assert_eq!(lb.update_limit(200), Ok(()));
		assert_eq!(lb.balance(), 100);
		assert_eq!(lb.limit(), 200);
		assert_eq!(lb.available_margin(), 100);
		assert_eq!(lb.update_limit(100), Ok(()));
		assert_eq!(lb.balance(), 100);
		assert_eq!(lb.limit(), 100);
		assert_eq!(lb.available_margin(), 0);
		assert_eq!(lb.update_limit(99), Err(LimitedBalanceError::LimitBelowCommitment));
		assert_eq!(lb.balance(), 100);
		assert_eq!(lb.limit(), 100);
		assert_eq!(lb.available_margin(), 0);
	}

	#[test]
	fn saturating_add() {
		let mut lb = LimitedBalance::<u32>::with_limit(100);
		lb.saturating_add(50);
		assert_eq!(lb.balance(), 50);
		assert_eq!(lb.available_margin(), 50);
		lb.saturating_add(100);
		assert_eq!(lb.balance(), 100);
		assert_eq!(lb.available_margin(), 0);
	}

	#[test]
	fn saturating_sub() {
		let mut lb = LimitedBalance::<u32>::with_limit(100);
		lb.saturating_add(50);
		lb.saturating_sub(1);
		assert_eq!(lb.balance(), 49);
		lb.saturating_sub(50);
		assert_eq!(lb.balance(), 0);
		lb.saturating_sub(1);
		assert_eq!(lb.balance(), 0);
		assert_eq!(lb.available_margin(), 100);
	}
}
