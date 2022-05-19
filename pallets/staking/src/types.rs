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

use super::{ActiveSession, AtStake, BalanceOf, Config, Pallet};
use crate::set::OrderedSet;
use codec::{Decode, Encode, HasCompact};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Convert, Saturating, Zero},
	RuntimeDebug,
};
use sp_staking::SessionIndex;
use sp_std::{cmp::Ordering, convert::From, prelude::*};

/// The index of a slashing span - unique to each controller.
pub(crate) type SpanIndex = u32;

/// The type define for validators reward
pub(crate) type RewardPoint = u32;

#[derive(Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct Bond<AccountId, Balance> {
	pub owner: AccountId,
	pub amount: Balance,
}

impl<A, B: Default> Bond<A, B> {
	pub(crate) fn from_owner(owner: A) -> Self {
		Bond {
			owner,
			amount: B::default(),
		}
	}
}

impl<AccountId: Ord, Balance> Eq for Bond<AccountId, Balance> {}

impl<AccountId: Ord, Balance> PartialEq for Bond<AccountId, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.owner == other.owner
	}
}

impl<AccountId: Ord, Balance> Ord for Bond<AccountId, Balance> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.owner.cmp(&other.owner)
	}
}

impl<AccountId: Ord, Balance> PartialOrd for Bond<AccountId, Balance> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, Copy, scale_info::TypeInfo)]
pub struct UnlockChunk<Balance> {
	/// Amount of funds to be unlocked.
	pub(crate) value: Balance,
	/// Session number at which point it'll be unlocked.
	pub(crate) session_idx: SessionIndex,
}

pub(crate) type StakeReward<Balance> = UnlockChunk<Balance>;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
/// The activity status of the validator
pub enum ValidatorStatus {
	/// Committed to be online and producing valid blocks
	Active,
	/// Temporarily inactive
	Idle,
	/// Bonded until the inner round
	Leaving(SessionIndex),
}

impl Default for ValidatorStatus {
	fn default() -> ValidatorStatus {
		ValidatorStatus::Active
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, scale_info::TypeInfo)]
/// Global validator state with commission fee, bonded stake, and nominations
pub struct Validator<AccountId, Balance> {
	pub id: AccountId,
	pub bond: Balance,
	pub nomi_bond_total: Balance,
	pub nominators: OrderedSet<Bond<AccountId, Balance>>,
	pub total: Balance,
	pub state: ValidatorStatus,
	pub unlocking: Vec<UnlockChunk<Balance>>,
}

impl<
		A: Ord + Clone + sp_std::fmt::Debug,
		B: AtLeast32BitUnsigned + Ord + Copy + sp_std::ops::AddAssign + sp_std::ops::SubAssign + Default,
	> Validator<A, B>
{
	pub fn new(id: A, bond: B) -> Self {
		let total = bond;
		Validator {
			id,
			bond,
			nomi_bond_total: Zero::zero(),
			nominators: OrderedSet::new(),
			total,
			state: ValidatorStatus::default(), // default active
			unlocking: Vec::new(),
		}
	}
	pub fn is_active(&self) -> bool {
		self.state == ValidatorStatus::Active
	}
	pub fn is_leaving(&self) -> bool {
		matches!(self.state, ValidatorStatus::Leaving(_))
	}
	pub fn bond_more(&mut self, more: B) {
		self.bond = self.bond.saturating_add(more);
		self.total = self.total.saturating_add(more);
	}
	// Returns None if underflow or less == self.bond (in which case validator should leave)
	pub fn bond_less(&mut self, less: B) -> Option<B> {
		if self.bond > less {
			self.bond = self.bond.saturating_sub(less);
			Some(self.bond)
		} else {
			None
		}
	}
	pub fn inc_nominator(&mut self, nominator: A, more: B) {
		if let Ok(loc) = self.nominators.0.binary_search(&Bond::from_owner(nominator)) {
			let nom_bond = &mut self.nominators.0[loc];
			nom_bond.amount = nom_bond.amount.saturating_add(more);
			self.nomi_bond_total = self.nomi_bond_total.saturating_add(more);
			self.total = self.total.saturating_add(more);
		};
	}
	pub fn dec_nominator(&mut self, nominator: A, less: B) {
		if let Ok(loc) = self.nominators.0.binary_search(&Bond::from_owner(nominator)) {
			let nom_bond = &mut self.nominators.0[loc];
			nom_bond.amount = nom_bond.amount.saturating_sub(less);
			self.nomi_bond_total = self.nomi_bond_total.saturating_sub(less);
			self.total = self.total.saturating_sub(less);
		};
	}
	pub fn go_offline(&mut self) {
		self.state = ValidatorStatus::Idle;
	}
	pub fn go_online(&mut self) {
		self.state = ValidatorStatus::Active;
	}
	pub fn leave_validators_pool(&mut self, round: SessionIndex) {
		self.state = ValidatorStatus::Leaving(round);
	}
}

impl<AccountId, Balance> Validator<AccountId, Balance>
where
	Balance: AtLeast32BitUnsigned + Saturating + Copy + sp_std::ops::AddAssign + sp_std::ops::SubAssign,
{
	/// Slash the validator for a given amount of balance. This can grow the value
	/// of the slash in the case that the validator has less than `minimum_balance`
	/// active funds. Returns the amount of funds actually slashed.
	///
	/// Slashes from `active` funds first, and then `unlocking`, starting with the
	/// chunks that are closest to unlocking.
	pub(crate) fn slash(&mut self, mut value: Balance, minimum_balance: Balance) -> Balance {
		let pre_total = self.total;
		let total = &mut self.total;
		let active = &mut self.bond;

		let slash_out_of = |total_remaining: &mut Balance, target: &mut Balance, value: &mut Balance| {
			let mut slash_from_target = (*value).min(*target);

			if !slash_from_target.is_zero() {
				*target = target.saturating_sub(slash_from_target);

				// Make sure not drop below ED
				if *target <= minimum_balance {
					let diff_val = minimum_balance.saturating_sub(*target);
					*target = target.saturating_add(diff_val);
					slash_from_target = slash_from_target.saturating_sub(diff_val);
				}
				*total_remaining = total_remaining.saturating_sub(slash_from_target);
				*value = value.saturating_sub(slash_from_target);
			}
		};

		slash_out_of(total, active, &mut value);

		let i = self
			.unlocking
			.iter_mut()
			.map(|chunk| {
				slash_out_of(total, &mut chunk.value, &mut value);
				chunk.value
			})
			.take_while(|value| value.is_zero()) // take all fully-consumed chunks out.
			.count();

		// kill all drained chunks.
		let _ = self.unlocking.drain(..i);

		pre_total.saturating_sub(*total)
	}
	/// Remove entries from `unlocking` that are sufficiently old and reduce the
	/// total by the sum of their balances.
	pub fn consolidate_unlocked(&mut self, current_session: SessionIndex) -> Balance {
		let mut total = self.total;
		self.unlocking.retain(|&chunk| {
			if chunk.session_idx > current_session {
				true
			} else {
				total = total.saturating_sub(chunk.value);
				false
			}
		});
		let unlocked_val = self.total.saturating_sub(total);
		self.total = total;
		unlocked_val
	}
}

#[derive(Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
/// Snapshot of validator state at the start of the round for which they are selected
pub struct ValidatorSnapshot<AccountId, Balance> {
	pub bond: Balance,
	pub nominators: Vec<Bond<AccountId, Balance>>,
	pub total: Balance,
}

impl<A: Clone, B: Copy + sp_std::ops::AddAssign + sp_std::ops::Add<Output = B> + sp_std::ops::SubAssign>
	From<Validator<A, B>> for ValidatorSnapshot<A, B>
{
	fn from(other: Validator<A, B>) -> ValidatorSnapshot<A, B> {
		ValidatorSnapshot {
			bond: other.bond,
			nominators: other.nominators.0,
			total: other.bond + other.nomi_bond_total,
		}
	}
}

impl<AccountId, Balance: Default + HasCompact> Default for ValidatorSnapshot<AccountId, Balance> {
	fn default() -> Self {
		Self {
			bond: Default::default(),
			nominators: vec![],
			total: Default::default(),
		}
	}
}

impl<AccountId: Ord, Balance: Ord> Eq for ValidatorSnapshot<AccountId, Balance> {}

impl<AccountId: Ord, Balance: Ord> PartialEq for ValidatorSnapshot<AccountId, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.nominators == other.nominators
	}
}

impl<AccountId: Ord, Balance: Ord> Ord for ValidatorSnapshot<AccountId, Balance> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.total.cmp(&other.total)
	}
}

impl<AccountId: Ord, Balance: Ord> PartialOrd for ValidatorSnapshot<AccountId, Balance> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
		// Some(self.total.cmp(&other.total))
	}
}

/// A typed conversion from stash account ID to the active exposure of nominators
/// on that account.
///
/// Active exposure is the exposure of the validator set currently validating, i.e. in
/// `active_era`. It can differ from the latest planned exposure in `current_era`.
pub struct ValidatorSnapshotOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<ValidatorSnapshot<T::AccountId, BalanceOf<T>>>>
	for ValidatorSnapshotOf<T>
{
	fn convert(validator: T::AccountId) -> Option<ValidatorSnapshot<T::AccountId, BalanceOf<T>>> {
		let now = <ActiveSession<T>>::get();
		if <AtStake<T>>::contains_key(now, &validator) {
			Some(<Pallet<T>>::at_stake(now, &validator))
		} else {
			None
		}
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, scale_info::TypeInfo)]
pub struct Nominator<AccountId, Balance> {
	pub nominations: OrderedSet<Bond<AccountId, Balance>>,
	pub total: Balance,
	pub active_bond: Balance,
	pub frozen_bond: Balance,
	pub unlocking: Vec<UnlockChunk<Balance>>,
}

impl<
		AccountId: Ord + Clone,
		Balance: Copy
			+ AtLeast32BitUnsigned
			+ Saturating
			+ sp_std::ops::AddAssign
			+ sp_std::ops::Add<Output = Balance>
			+ sp_std::ops::SubAssign
			+ PartialOrd
			+ Default,
	> Nominator<AccountId, Balance>
{
	pub fn new(validator: AccountId, amount: Balance) -> Self {
		Nominator {
			nominations: OrderedSet::from(vec![Bond {
				owner: validator,
				amount,
			}]),
			total: amount,
			active_bond: amount,
			frozen_bond: Zero::zero(),
			unlocking: Vec::new(),
		}
	}
	pub fn add_nomination(&mut self, bond: Bond<AccountId, Balance>, unfreeze_bond: bool) -> bool {
		let amt = bond.amount;
		if self.nominations.insert(bond) {
			if unfreeze_bond {
				self.frozen_bond = Zero::zero();
			}
			self.total = self.total.saturating_add(amt);
			self.active_bond = self.active_bond.saturating_add(amt);
			true
		} else {
			false
		}
	}
	// Returns Some(remaining balance), must be more than MinNominatorStake
	// Returns None if nomination not found
	pub fn rm_nomination(&mut self, validator: AccountId, freeze_bond: bool) -> Option<Balance> {
		let mut amt: Option<Balance> = None;
		let nominations = self
			.nominations
			.0
			.iter()
			.filter_map(|x| {
				if x.owner == validator {
					amt = Some(x.amount);
					None
				} else {
					Some(x.clone())
				}
			})
			.collect();
		if let Some(balance) = amt {
			self.nominations = OrderedSet::from(nominations);
			self.active_bond = self.active_bond.saturating_sub(balance);
			if freeze_bond {
				self.frozen_bond = self.frozen_bond.saturating_add(balance);
			}
			Some(self.active_bond)
		} else {
			None
		}
	}

	pub fn unbond_frozen(&mut self) -> Option<Balance> {
		if self.frozen_bond > Zero::zero() {
			let frozen_bond = self.frozen_bond;
			self.total = self.total.saturating_sub(frozen_bond);
			self.frozen_bond = Zero::zero();
			Some(frozen_bond)
		} else {
			None
		}
	}

	// Returns None if nomination not found
	pub fn inc_nomination(&mut self, validator: AccountId, more: Balance, unfreeze_bond: bool) -> Option<Balance> {
		match self.nominations.0.binary_search(&Bond::from_owner(validator)) {
			Ok(loc) => {
				let nom_bond = &mut self.nominations.0[loc];
				nom_bond.amount = nom_bond.amount.saturating_add(more);
				self.total = self.total.saturating_add(more);
				self.active_bond = self.active_bond.saturating_add(more);
				if unfreeze_bond {
					self.frozen_bond = Zero::zero();
				}
				Some(nom_bond.amount)
			}
			Err(_) => None,
		}
	}
	pub fn dec_nomination(&mut self, validator: AccountId, less: Balance) -> Result<Balance, &str> {
		match self.nominations.0.binary_search(&Bond::from_owner(validator)) {
			Ok(loc) => {
				let nom_bond = &mut self.nominations.0[loc];
				if nom_bond.amount > less {
					nom_bond.amount = nom_bond.amount.saturating_sub(less);
					self.active_bond = self.active_bond.saturating_sub(less);
					Ok(nom_bond.amount)
				} else {
					Err("Underflow")
				}
			}
			Err(_) => Err("NominationDNE"),
		}
	}
}

impl<AccountId, Balance> Nominator<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: AtLeast32BitUnsigned + Saturating + Copy + sp_std::ops::AddAssign + sp_std::ops::SubAssign + Default,
{
	/// Slash the validator for a given amount of balance. This can grow the value
	/// of the slash in the case that the validator has less than `minimum_balance`
	/// active funds. Returns the amount of funds actually slashed.
	///
	/// Slashes from `active` funds first, and then `unlocking`, starting with the
	/// chunks that are closest to unlocking.
	pub(crate) fn slash_nomination(
		&mut self,
		validator: AccountId,
		mut value: Balance,
		minimum_balance: Balance,
	) -> Balance {
		let pre_total = self.total;
		let total = &mut self.total;
		let pre_active_bond = self.active_bond;
		let active_bond = &mut self.active_bond;

		let slash_out_of = |total_remaining: &mut Balance, target: &mut Balance, value: &mut Balance| {
			let mut slash_from_target = (*value).min(*target);

			if !slash_from_target.is_zero() {
				*target = target.saturating_sub(slash_from_target);
				*total_remaining = total_remaining.saturating_sub(slash_from_target);

				// Make sure not drop below ED
				if *total_remaining <= minimum_balance {
					let diff_val = minimum_balance.saturating_sub(*total_remaining);
					*target = target.saturating_add(diff_val);
					*total_remaining = total_remaining.saturating_add(diff_val);
					slash_from_target = slash_from_target.saturating_sub(diff_val);
				}
				*value = value.saturating_sub(slash_from_target);
			}
		};

		if let Ok(loc) = self.nominations.0.binary_search(&Bond::from_owner(validator)) {
			let nom_bond = &mut self.nominations.0[loc];
			slash_out_of(active_bond, &mut nom_bond.amount, &mut value);
		};

		*total = total.saturating_sub(pre_active_bond.saturating_sub(*active_bond));

		let i = self
			.unlocking
			.iter_mut()
			.map(|chunk| {
				slash_out_of(total, &mut chunk.value, &mut value);
				chunk.value
			})
			.take_while(|value| value.is_zero()) // take all fully-consumed chunks out.
			.count();

		// kill all drained chunks.
		let _ = self.unlocking.drain(..i);
		pre_total.saturating_sub(*total)
	}
	/// Remove entries from `unlocking` that are sufficiently old and reduce the
	/// total by the sum of their balances.
	pub fn consolidate_unlocked(&mut self, current_session: SessionIndex) -> Balance {
		let mut total = self.total;
		self.unlocking.retain(|&chunk| {
			if chunk.session_idx > current_session {
				true
			} else {
				total = total.saturating_sub(chunk.value);
				false
			}
		});
		let unlocked_val = self.total.saturating_sub(total);
		self.total = total;
		unlocked_val
	}
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, RuntimeDebug, Clone, scale_info::TypeInfo)]
pub struct UnappliedSlash<AccountId, Balance: HasCompact> {
	/// The stash ID of the offending validator.
	pub(crate) validator: AccountId,
	/// The validator's own slash.
	pub(crate) own: Balance,
	/// All other slashed stakers and amounts.
	pub(crate) others: Vec<(AccountId, Balance)>,
	/// Reporters of the offence; bounty payout recipients.
	pub(crate) reporters: Vec<AccountId>,
	/// The amount of payout.
	pub(crate) payout: Balance,
}

#[allow(dead_code)]
impl<AccountId, Balance: Default + HasCompact> UnappliedSlash<AccountId, Balance> {
	pub(crate) fn from_default(validator: AccountId) -> Self {
		Self {
			validator,
			own: Default::default(),
			others: vec![],
			reporters: vec![],
			payout: Default::default(),
		}
	}
}
