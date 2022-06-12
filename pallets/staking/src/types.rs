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

use super::{ActiveSession, AtStake, BalanceOf, Config, Error, Pallet};
use crate::set::OrderedSet;
use codec::{Codec, Decode, Encode, HasCompact};
use derivative::Derivative;
use frame_support::{bounded_vec, pallet_prelude::MaxEncodedLen, traits::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Convert, Saturating, Zero},
	RuntimeDebug,
};
use sp_staking::SessionIndex;
use sp_std::marker::PhantomData;
use sp_std::{
	cmp::{max, Ordering},
	convert::From,
	fmt::Debug,
	prelude::*,
};

/// The index of a slashing span - unique to each controller.
pub(crate) type SpanIndex = u32;

/// The type define for validators reward
pub(crate) type RewardPoint = u32;

#[derive(Clone, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Bond<AccountId, Balance> {
	pub owner: AccountId,
	pub amount: Balance,
}

impl<AccountId, Balance> Bond<AccountId, Balance>
where
	Balance: Default,
{
	pub(crate) fn from_owner(owner: AccountId) -> Self {
		Bond {
			owner,
			amount: Balance::default(),
		}
	}
}

impl<AccountId, Balance> Eq for Bond<AccountId, Balance> where AccountId: Ord {}

impl<AccountId, Balance> PartialEq for Bond<AccountId, Balance>
where
	AccountId: Ord,
{
	fn eq(&self, other: &Self) -> bool {
		self.owner == other.owner
	}
}

impl<AccountId, Balance> Ord for Bond<AccountId, Balance>
where
	AccountId: Ord,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.owner.cmp(&other.owner)
	}
}

impl<AccountId, Balance> PartialOrd for Bond<AccountId, Balance>
where
	AccountId: Ord,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, Copy, MaxEncodedLen, TypeInfo)]
pub struct UnlockChunk<Balance> {
	/// Amount of funds to be unlocked.
	pub(crate) value: Balance,
	/// Session number at which point it'll be unlocked.
	pub(crate) session_idx: SessionIndex,
}

pub(crate) type StakeReward<Balance> = UnlockChunk<Balance>;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
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

/// Global validator state with commission fee, bonded stake, and nominations
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(T, MaxNominators, MaxUnlock))]
pub struct Validator<T: Config, MaxNominators: Get<u32>, MaxUnlock: Get<u32>> {
	pub id: <T as frame_system::Config>::AccountId,
	pub bond: BalanceOf<T>,
	pub nomi_bond_total: BalanceOf<T>,
	pub nominators: OrderedSet<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>, MaxNominators>,
	pub total: BalanceOf<T>,
	pub state: ValidatorStatus,
	pub unlocking: BoundedVec<UnlockChunk<BalanceOf<T>>, MaxUnlock>,
}

impl<T, MaxNominators, MaxUnlock> MaxEncodedLen for Validator<T, MaxNominators, MaxUnlock>
where
	T: Config,
	MaxNominators: Get<u32>,
	MaxUnlock: Get<u32>,
{
	fn max_encoded_len() -> usize {
		max(MaxNominators::get(), MaxUnlock::get()) as usize
	}
}

impl<T, MaxNominators, MaxUnlock> Validator<T, MaxNominators, MaxUnlock>
where
	T: Config,
	MaxNominators: Get<u32>,
	MaxUnlock: Get<u32>,
{
	pub fn new(id: <T as frame_system::Config>::AccountId, bond: BalanceOf<T>) -> Self {
		let total = bond;
		let unlocking: BoundedVec<UnlockChunk<BalanceOf<T>>, MaxUnlock> = bounded_vec![];
		Validator {
			id,
			bond,
			nomi_bond_total: Zero::zero(),
			nominators: OrderedSet::<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>, MaxNominators>::new(),
			total,
			state: ValidatorStatus::default(), // default active
			unlocking: unlocking,
		}
	}

	pub fn is_active(&self) -> bool {
		self.state == ValidatorStatus::Active
	}

	pub fn is_leaving(&self) -> bool {
		matches!(self.state, ValidatorStatus::Leaving(_))
	}

	pub fn bond_more(&mut self, more: BalanceOf<T>) {
		self.bond = self.bond.saturating_add(more);
		self.total = self.total.saturating_add(more);
	}

	// Returns None if underflow or less == self.bond (in which case validator should leave)
	pub fn bond_less(&mut self, less: BalanceOf<T>) -> Option<BalanceOf<T>> {
		if self.bond > less {
			self.bond = self.bond.saturating_sub(less);
			Some(self.bond)
		} else {
			None
		}
	}

	pub fn inc_nominator(
		&mut self,
		nominator: <T as frame_system::Config>::AccountId,
		more: BalanceOf<T>,
	) -> Result<(), Error<T>> {
		let mut nominators: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>> =
			self.nominators.get_inner().map_err(|_| <Error<T>>::OrderedSetFailure)?;

		if let Ok(loc) = nominators.binary_search(&Bond::from_owner(nominator)) {
			let nom_bond = &mut nominators[loc];
			nom_bond.amount = nom_bond.amount.saturating_add(more);
			self.nominators
				.update_inner(nominators)
				.map_err(|_| <Error<T>>::NominationOverflow)?;
			self.nomi_bond_total = self.nomi_bond_total.saturating_add(more);
			self.total = self.total.saturating_add(more);
		} else {
			return Err(<Error<T>>::InvalidNomination);
		}

		Ok(())
	}

	pub fn dec_nominator(
		&mut self,
		nominator: <T as frame_system::Config>::AccountId,
		less: BalanceOf<T>,
	) -> Result<(), Error<T>> {
		let mut nominators: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>> =
			self.nominators.get_inner().map_err(|_| <Error<T>>::OrderedSetFailure)?;

		if let Ok(loc) = nominators.binary_search(&Bond::from_owner(nominator)) {
			let nom_bond = &mut nominators[loc];
			nom_bond.amount = nom_bond.amount.saturating_sub(less);
			self.nomi_bond_total = self.nomi_bond_total.saturating_sub(less);
			self.total = self.total.saturating_sub(less);
		} else {
			return Err(<Error<T>>::InvalidNomination);
		}
		Ok(())
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

	pub fn build_snapshot(&self) -> Result<ValidatorSnapshot<T, MaxNominators>, Error<T>> {
		let nominators: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>> =
			self.nominators.get_inner().map_err(|_| <Error<T>>::OrderedSetFailure)?;

		// let nominators_snapshot = BoundedVec::try_from(nominators).map_err(|_|
		// <Error<T>>::OrderedSetFailure)?;

		Ok(ValidatorSnapshot {
			bond: self.bond,
			nominators: nominators,
			total: self.bond + self.nomi_bond_total,
			stub: <PhantomData<MaxNominators>>::default(),
		})
	}
}

/// Snapshot of validator state at the start of the round for which they are selected
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(T, MaxNominators))]
pub struct ValidatorSnapshot<T: Config, MaxNominators: Get<u32>> {
	pub bond: BalanceOf<T>,
	pub nominators: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>>,
	pub total: BalanceOf<T>,
	pub stub: PhantomData<MaxNominators>,
}

impl<T, MaxNominators> MaxEncodedLen for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
	fn max_encoded_len() -> usize {
		MaxNominators::get() as usize
	}
}

impl<T, MaxNominators> Default for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
	fn default() -> Self {
		let inner: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>> =
			Vec::with_capacity(MaxNominators::get() as usize);

		// let nominators: BoundedVec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>,
		// MaxNominators> = 	BoundedVec::try_from(inner).expect("ValidatorSnapshot Failed To Create
		// Default");

		Self {
			bond: <BalanceOf<T>>::default(),
			nominators: inner,
			total: <BalanceOf<T>>::default(),
			stub: <PhantomData<MaxNominators>>::default(),
		}
	}
}

impl<T, MaxNominators> Eq for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
}

impl<T, MaxNominators> PartialEq for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
	fn eq(&self, other: &Self) -> bool {
		self.nominators == other.nominators
	}
}

impl<T, MaxNominators> Ord for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.total.cmp(&other.total)
	}
}

impl<T, MaxNominators> PartialOrd for ValidatorSnapshot<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
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
pub struct ValidatorSnapshotOf<T, MaxNominators: Get<u32>>(PhantomData<T>, PhantomData<MaxNominators>);

impl<T: Config, MaxNominators: Get<u32>>
	Convert<T::AccountId, Option<ValidatorSnapshot<T, T::MaxNominatorsPerValidator>>>
	for ValidatorSnapshotOf<T, MaxNominators>
{
	fn convert(validator: T::AccountId) -> Option<ValidatorSnapshot<T, T::MaxNominatorsPerValidator>> {
		let now = <ActiveSession<T>>::get();
		if <AtStake<T>>::contains_key(now, &validator) {
			Some(<Pallet<T>>::at_stake(now, &validator))
		} else {
			None
		}
	}
}

#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(T, MaxNomination, MaxUnlock))]
pub struct Nominator<T: Config, MaxNomination: Get<u32>, MaxUnlock: Get<u32>> {
	pub nominations: OrderedSet<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>, MaxNomination>,
	pub total: BalanceOf<T>,
	pub active_bond: BalanceOf<T>,
	pub frozen_bond: BalanceOf<T>,
	pub unlocking: BoundedVec<UnlockChunk<BalanceOf<T>>, MaxUnlock>,
}

impl<T, MaxNomination, MaxUnlock> MaxEncodedLen for Nominator<T, MaxNomination, MaxUnlock>
where
	T: Config,
	MaxNomination: Get<u32>,
	MaxUnlock: Get<u32>,
{
	fn max_encoded_len() -> usize {
		max(MaxNomination::get(), MaxUnlock::get()) as usize
	}
}

impl<T, MaxNomination, MaxUnlock> Nominator<T, MaxNomination, MaxUnlock>
where
	T: Config,
	MaxNomination: Get<u32>,
	MaxUnlock: Get<u32>,
{
	pub fn new(validator: <T as frame_system::Config>::AccountId, amount: BalanceOf<T>) -> Result<Self, Error<T>> {
		let unlocking: BoundedVec<UnlockChunk<BalanceOf<T>>, MaxUnlock> = bounded_vec![];

		let nominations = OrderedSet::try_from(vec![Bond {
			owner: validator,
			amount,
		}])
		.map_err(|_| <Error<T>>::OrderedSetFailure)?;

		Ok(Nominator {
			nominations: nominations,
			total: amount,
			active_bond: amount,
			frozen_bond: Zero::zero(),
			unlocking: unlocking,
		})
	}
	pub fn add_nomination(
		&mut self,
		bond: Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>,
		unfreeze_bond: bool,
	) -> Result<bool, Error<T>> {
		let amt = bond.amount;
		let status = self
			.nominations
			.insert(bond)
			.map_err(|_| <Error<T>>::NominationOverflow)?;
		if status {
			if unfreeze_bond {
				self.frozen_bond = Zero::zero();
			}
			self.total = self.total.saturating_add(amt);
			self.active_bond = self.active_bond.saturating_add(amt);
			Ok(true)
		} else {
			return Err(<Error<T>>::InvalidNomination);
		}
	}
	// Returns Some(remaining balance), must be more than MinNominatorStake
	// Returns None if nomination not found
	pub fn rm_nomination(
		&mut self,
		validator: <T as frame_system::Config>::AccountId,
		freeze_bond: bool,
	) -> Result<BalanceOf<T>, Error<T>> {
		let mut amt: Option<BalanceOf<T>> = None;

		let nominations_inner = self
			.nominations
			.get_inner()
			.map_err(|_| <Error<T>>::OrderedSetFailure)?;

		let nominations: Vec<Bond<<T as frame_system::Config>::AccountId, BalanceOf<T>>> = nominations_inner
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
			self.nominations = OrderedSet::try_from(nominations).map_err(|_| <Error<T>>::OrderedSetFailure)?;
			self.active_bond = self.active_bond.saturating_sub(balance);
			if freeze_bond {
				self.frozen_bond = self.frozen_bond.saturating_add(balance);
			}
			Ok(self.active_bond)
		} else {
			return Err(<Error<T>>::NominationDNE);
		}
	}
	pub fn unbond_frozen(&mut self) -> Option<BalanceOf<T>> {
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
	pub fn inc_nomination(
		&mut self,
		validator: <T as frame_system::Config>::AccountId,
		more: BalanceOf<T>,
		unfreeze_bond: bool,
	) -> Result<BalanceOf<T>, Error<T>> {
		let mut nominations_inner = self
			.nominations
			.get_inner()
			.map_err(|_| <Error<T>>::OrderedSetFailure)?;

		if let Ok(loc) = nominations_inner.binary_search(&Bond::from_owner(validator)) {
			nominations_inner[loc].amount = nominations_inner[loc].amount.saturating_add(more);
			let nom_bond_amount = nominations_inner[loc].amount;

			self.nominations
				.update_inner(nominations_inner)
				.map_err(|_| <Error<T>>::NominationOverflow)?;
			self.total = self.total.saturating_add(more);
			self.active_bond = self.active_bond.saturating_add(more);
			if unfreeze_bond {
				self.frozen_bond = Zero::zero();
			}
			Ok(nom_bond_amount)
		} else {
			return Err(<Error<T>>::NominationDNE);
		}
	}
	pub fn dec_nomination(
		&mut self,
		validator: <T as frame_system::Config>::AccountId,
		less: BalanceOf<T>,
	) -> Result<BalanceOf<T>, Error<T>> {
		let mut nominations_inner = self
			.nominations
			.get_inner()
			.map_err(|_| <Error<T>>::OrderedSetFailure)?;

		if let Ok(loc) = nominations_inner.binary_search(&Bond::from_owner(validator)) {
			if nominations_inner[loc].amount > less {
				nominations_inner[loc].amount = nominations_inner[loc].amount.saturating_sub(less);
				let nom_bond_amount = nominations_inner[loc].amount;
				self.active_bond = self.active_bond.saturating_sub(less);
				Ok(nom_bond_amount)
			} else {
				return Err(<Error<T>>::Underflow);
			}
		} else {
			return Err(<Error<T>>::NominationDNE);
		}
	}
}
