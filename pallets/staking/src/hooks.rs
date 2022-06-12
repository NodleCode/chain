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

use super::{
	ActiveSession, BalanceOf, BondedSessions, Config, Event, NegativeImbalanceOf, Pallet, SessionAccumulatedBalance,
	SessionValidatorReward, SlashRewardProportion, Staked, Store, Total,
};
// use crate::slashing;
use crate::types::{ValidatorSnapshot, ValidatorSnapshotOf};
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, Get, Imbalance, OnUnbalanced},
	BoundedVec,
};
use frame_system::{self as system};
use pallet_session::historical;
use sp_runtime::{
	traits::{AccountIdConversion, Convert, Saturating},
	Perbill,
};
use sp_staking::{
	offence::{DisableStrategy, OffenceDetails, OnOffenceHandler},
	SessionIndex,
};
use sp_std::prelude::*;

/// A `Convert` implementation that finds the stash of the given controller account,
/// if any.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<T::AccountId>> for StashOf<T> {
	fn convert(validator: T::AccountId) -> Option<T::AccountId> {
		<Pallet<T>>::validator_state(&validator).map(|l| l.id)
	}
}

impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Pallet<T> {
	fn on_nonzero_unbalanced(imbalance: NegativeImbalanceOf<T>) {
		let now = <ActiveSession<T>>::get();
		<SessionAccumulatedBalance<T>>::mutate(now, |v: &mut BalanceOf<T>| *v = v.saturating_add(imbalance.peek()));
		T::Currency::resolve_creating(&T::PalletId::get().into_account(), imbalance);
	}
}

/// Means for interacting with a specialized version of the `session` trait.
///
/// This is needed because `Staking` sets the `ValidatorIdOf` of the `pallet_session::Config`
pub trait SessionInterface<AccountId>: frame_system::Config {
	/// Disable a given validator by stash ID.
	///
	/// Returns `true` if new era should be forced at the end of this session.
	/// This allows preventing a situation where there is too many validators
	/// disabled and block production stalls.
	fn disable_validator(validator: &AccountId);
	/// Get the validators from session.
	fn validators() -> Vec<AccountId>;
	/// Prune historical session tries up to but not including the given index.
	fn prune_historical_up_to(up_to: SessionIndex);
}

impl<T: Config> SessionInterface<<T as frame_system::Config>::AccountId> for T
where
	T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
	T: pallet_session::historical::Config<
		FullIdentification = ValidatorSnapshot<T, T::MaxNominatorsPerValidator>,
		FullIdentificationOf = ValidatorSnapshotOf<T, T::MaxNominatorsPerValidator>,
	>,
	T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
	T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
	T::ValidatorIdOf: Convert<<T as frame_system::Config>::AccountId, Option<<T as frame_system::Config>::AccountId>>,
{
	fn disable_validator(validator: &<T as frame_system::Config>::AccountId) {
		// function Returns `false` either if the validator could not be
		// found or it was already disabled,
		// which is not used in this context.
		let _ = <pallet_session::Pallet<T>>::disable(validator);
	}

	fn validators() -> Vec<<T as frame_system::Config>::AccountId> {
		<pallet_session::Pallet<T>>::validators()
	}

	fn prune_historical_up_to(up_to: SessionIndex) {
		<pallet_session::historical::Pallet<T>>::prune_up_to(up_to);
	}
}
