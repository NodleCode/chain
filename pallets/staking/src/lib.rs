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

//! Nodle Chain - custom tailored, staking pallet.
//! Use a non inflationary reward system.

#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod mock;

use frame_support::pallet;
mod hooks;
mod set;
mod types;

pub use pallet::*;

#[pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		bounded_vec,
		pallet_prelude::*,
		traits::{
			Currency, ExistenceRequirement, Get, Imbalance, LockIdentifier, LockableCurrency, OnUnbalanced, Polling,
			ValidatorRegistration, WithdrawReasons,
		},
		BoundedVec, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{AccountIdConversion, Saturating, Zero},
		DispatchResult, Perbill,
	};
	use sp_staking::SessionIndex;
	use sp_std::{convert::From, prelude::*};

	use set::OrderedSet;
	use types::{Bond, Nominator, RewardPoint, StakeReward, UnlockChunk, Validator, ValidatorSnapshot};

	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub(crate) type NegativeImbalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The staking balance.
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		/// Number of sessions that staked fund remain bonded for
		type BondedDuration: Get<SessionIndex>;
		/// Maximum validators allowed to join the pool.
		#[pallet::constant]
		type DefaultStakingMaxValidators: Get<u32>;
		/// Maximum nominators per validator
		type MaxNominatorsPerValidator: Get<u32>;
		/// Maximum validators per nominator
		#[pallet::constant]
		type MaxValidatorPerNominator: Get<u32>;
		/// staking pallet Lock Identifier used for set_lock()
		#[pallet::constant]
		type StakingLockId: Get<LockIdentifier>;
		/// This pallet's module id. Used to derivate a dedicated account id to store session
		/// rewards for validators and nominators in.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// Max number of unbond request supported by queue
		#[pallet::constant]
		type MaxChunkUnlock: Get<u32> + MaxEncodedLen + Clone;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Join the set of validators pool
		#[pallet::weight(1000)]
		pub fn validator_join_pool(origin: OriginFor<T>, bond: BalanceOf<T>) -> DispatchResultWithPostInfo {
			log::debug!("validator_join_pool:[{:#?}] - Entry!!!", line!(),);

			let acc = ensure_signed(origin)?;

			ensure!(!Self::is_validator(&acc), <Error<T>>::ValidatorExists);

			ensure!(
				bond >= <StakingMinValidatorBond<T>>::get(),
				<Error<T>>::ValidatorBondBelowMin
			);

			log::debug!(
				"validator_join_pool:[{:#?}] | Cfg StakingMinValidatorBond::[{:#?}]",
				line!(),
				<StakingMinValidatorBond<T>>::get(),
			);

			log::debug!("validator_join_pool:[{:#?}]", line!(),);

			let mut validators = <ValidatorPool<T>>::get();
			ensure!(
				validators.len().map_err(|_| <Error<T>>::OrderedSetFailure)? < Self::staking_max_validators() as usize,
				<Error<T>>::ValidatorPoolFull
			);

			let status = validators
				.insert(Bond {
					owner: acc.clone(),
					amount: bond,
				})
				.map_err(|_| <Error<T>>::OrderedSetFailure)?;

			ensure!(status, <Error<T>>::ValidatorExists);

			log::debug!("validator_join_pool:[{:#?}]", line!());

			let validator_free_balance = T::Currency::free_balance(&acc);
			log::debug!(
				"validator_join_pool:[{:#?}] | acc::[{:#?}] | bond::[{:#?}] | free_bal:[{:#?}]",
				line!(),
				acc,
				bond,
				validator_free_balance
			);
			ensure!(validator_free_balance >= bond, <Error<T>>::InsufficientBalance);

			log::debug!("validator_join_pool:[{:#?}]", line!(),);

			T::Currency::set_lock(T::StakingLockId::get(), &acc, bond, WithdrawReasons::all());

			let validator = Validator::new(acc.clone(), bond);

			<Total<T>>::mutate(|x| *x = x.saturating_add(bond));
			<ValidatorState<T>>::insert(&acc, validator);
			<ValidatorPool<T>>::put(validators);
			Self::deposit_event(Event::JoinedValidatorPool(acc, bond, Self::total()));
			log::debug!("validator_join_pool:[{:#?}] - Exit!!!", line!(),);
			Ok(().into())
		}
		/// Request to exit the validators pool. If successful,
		/// the account is immediately removed from the validator pool
		/// to prevent selection as a validator, but unbonding
		/// is executed with a delay of `BondedDuration` rounds.
		#[pallet::weight(1000)]
		pub fn validator_exit_pool(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let validator = ensure_signed(origin)?;

			ensure!(Self::is_validator(&validator), <Error<T>>::ValidatorDNE);

			// Since we ensure validator exist, no harm in operating on `unwrap` object directly.
			ensure!(
				!Self::validator_state(&validator).unwrap().is_leaving(),
				<Error<T>>::AlreadyLeaving,
			);

			let now = Self::active_session();
			let when = now.saturating_add(T::BondedDuration::get());

			<ExitQueue<T>>::mutate(|exits| {
				exits.insert(Bond {
					owner: validator.clone(),
					amount: when,
				});
			});

			<ValidatorState<T>>::mutate(&validator, |maybe_validator| {
				if let Some(state) = maybe_validator {
					state.leave_validators_pool(when);
				}
			});

			<ValidatorPool<T>>::mutate(|validators| {
				validators.remove(&Bond::from_owner(validator.clone()));
			});

			Self::deposit_event(Event::ValidatorScheduledExit(now, validator, when));
			Ok(().into())
		}
		/// Bond more for validator
		// #[pallet::weight(T::WeightInfo::validator_bond_more())]
		#[pallet::weight(1000)]
		pub fn validator_bond_more(origin: OriginFor<T>, more: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let validator = ensure_signed(origin)?;

			ensure!(Self::is_validator(&validator), <Error<T>>::ValidatorDNE);

			let valid_state = Self::validator_state(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

			ensure!(!valid_state.is_leaving(), <Error<T>>::CannotActivateIfLeaving,);

			let validator_free_balance = T::Currency::free_balance(&validator);
			ensure!(
				validator_free_balance >= valid_state.bond.saturating_add(more),
				<Error<T>>::InsufficientBalance,
			);

			ensure!(
				valid_state.bond.saturating_add(more) > Self::staking_min_validator_bond(),
				<Error<T>>::ValidatorBondBelowMin,
			);

			<ValidatorState<T>>::mutate(validator.clone(), |maybe_validator| {
				if let Some(state) = maybe_validator {
					let before = state.bond;
					state.bond_more(more);
					T::Currency::set_lock(T::StakingLockId::get(), &validator, state.bond, WithdrawReasons::all());
					let after = state.bond;
					state.go_online();
					if state.is_active() {
						Self::update_validators_pool(
							validator.clone(),
							state.bond.saturating_add(state.nomi_bond_total),
						);
					}
					<Total<T>>::mutate(|x| *x = x.saturating_add(more));
					Self::deposit_event(Event::ValidatorBondedMore(validator.clone(), before, after));
				}
			});

			// Self::validator_stake_reconciliation(&validator);

			Ok(().into())
		}
		/// Bond less for validator
		// #[pallet::weight(T::WeightInfo::validator_bond_less())]
		#[pallet::weight(1000)]
		pub fn validator_bond_less(origin: OriginFor<T>, less: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let validator = ensure_signed(origin)?;

			ensure!(Self::is_validator(&validator), <Error<T>>::ValidatorDNE);

			if let Some(mut state) = Self::validator_state(&validator) {
				ensure!(!state.is_leaving(), <Error<T>>::CannotActivateIfLeaving);

				let before = state.bond;
				let after = state.bond_less(less).ok_or(<Error<T>>::Underflow)?;
				ensure!(
					after >= <StakingMinValidatorBond<T>>::get(),
					<Error<T>>::ValidatorBondBelowMin
				);
				ensure!(
					(state.unlocking.len() as u32) < T::MaxChunkUnlock::get(),
					<Error<T>>::NoMoreChunks,
				);

				// `state.total` is locked for bonding duration to handle the slash,
				//  But for upcoming validator selection,
				//  State in validator pool is updated with ( validator bond + nominator bond).
				if state.is_active() {
					Self::update_validators_pool(validator.clone(), state.bond.saturating_add(state.nomi_bond_total));
				}

				// Update the overall total, since there is change in
				// active total.
				<Total<T>>::mutate(|x| *x = x.saturating_sub(less));

				// T::Currency::unreserve(&validator, less);
				state.unlocking.try_push(UnlockChunk {
					value: less,
					session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
				});

				<ValidatorState<T>>::insert(&validator, state);
				Self::deposit_event(Event::ValidatorBondedLess(validator, before, after));
			}
			Ok(().into())
		}

		/// If caller is not a nominator, then join the set of nominators
		/// If caller is a nominator, then makes nomination to change their nomination state
		// #[pallet::weight(T::WeightInfo::nominator_nominate())]
		#[pallet::weight(1000)]
		pub fn nominator_nominate(
			origin: OriginFor<T>,
			validator: T::AccountId,
			amount: BalanceOf<T>,
			unfreeze_bond: bool,
		) -> DispatchResultWithPostInfo {
			let nominator_acc = ensure_signed(origin)?;

			// cannot be a validator candidate and nominator with same AccountId
			ensure!(!Self::is_validator(&nominator_acc), <Error<T>>::ValidatorExists);

			log::trace!("nominator_nominate:[{:#?}] - Entry!!!", line!());

			let mut validator_state = <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

			let mut do_add_nomination = true;
			let mut nominator_state = if Self::is_nominator(&nominator_acc) {
				Self::nominator_state(&nominator_acc).ok_or(<Error<T>>::NominatorDNE)?
			} else {
				do_add_nomination = false;
				Nominator::new(validator.clone(), amount)?
			};

			let amount = if unfreeze_bond {
				amount.saturating_add(nominator_state.frozen_bond)
			} else {
				amount
			};

			ensure!(
				amount >= <StakingMinNominationChillThreshold<T>>::get(),
				<Error<T>>::NominationBelowMin
			);

			let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
				.nominations
				.get_inner()
				.map_err(|_| <Error<T>>::OrderedSetFailure)?;

			ensure!(
				(nominations_inner.len() as u32) < T::MaxValidatorPerNominator::get(),
				<Error<T>>::ExceedMaxValidatorPerNom,
			);

			ensure!(
				(nominations_inner.len() as u32) < T::MaxNominatorsPerValidator::get(),
				<Error<T>>::TooManyNominators,
			);

			let nominator_free_balance = T::Currency::free_balance(&nominator_acc);

			if do_add_nomination {
				ensure!(
					nominator_free_balance >= nominator_state.total.saturating_add(amount),
					<Error<T>>::InsufficientBalance
				);

				nominator_state.add_nomination(
					Bond {
						owner: validator.clone(),
						amount,
					},
					unfreeze_bond,
				)?;
			} else {
				ensure!(nominator_free_balance >= amount, <Error<T>>::InsufficientBalance);
			}

			ensure!(
				nominator_state.total.saturating_add(amount) >= <StakingMinNominatorTotalBond<T>>::get(),
				<Error<T>>::NominatorBondBelowMin,
			);

			let nomination = Bond {
				owner: nominator_acc.clone(),
				amount,
			};

			let insert_status = validator_state
				.nominators
				.insert(nomination)
				.map_err(|_| <Error<T>>::NominationOverflow)?;

			ensure!(insert_status, <Error<T>>::NominatorExists);

			T::Currency::set_lock(
				T::StakingLockId::get(),
				&nominator_acc,
				nominator_state.total,
				WithdrawReasons::all(),
			);

			validator_state.nomi_bond_total = validator_state.nomi_bond_total.saturating_add(amount);
			validator_state.total = validator_state.total.saturating_add(amount);
			let validator_new_total = validator_state.total;
			if validator_state.is_active() {
				Self::update_validators_pool(validator.clone(), validator_state.total);
			}

			<Total<T>>::mutate(|x| *x = x.saturating_add(amount));
			<ValidatorState<T>>::insert(&validator, validator_state);
			<NominatorState<T>>::insert(&nominator_acc, nominator_state);

			Self::deposit_event(Event::Nomination(nominator_acc, amount, validator, validator_new_total));

			log::trace!("nominator_nominate:[{:#?}] - Exit!!!", line!());

			Ok(().into())
		}
		/// Revoke an existing nomination
		// #[pallet::weight(T::WeightInfo::nominator_denominate())]
		#[pallet::weight(1000)]
		pub fn nominator_denominate(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResultWithPostInfo {
			let acc = ensure_signed(origin)?;

			let nominator_state = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

			let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
				.nominations
				.get_inner()
				.map_err(|_| <Error<T>>::OrderedSetFailure)?;

			let do_force = nominations_inner.len() == 1;

			Self::nominator_revokes_validator(acc, validator, do_force)
		}
		/// Quit the set of nominators and, by implication, revoke all ongoing nominations
		// #[pallet::weight(T::WeightInfo::nominator_denominate_all())]
		#[pallet::weight(1000)]
		pub fn nominator_denominate_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let acc = ensure_signed(origin)?;

			let nominator = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

			let nominator_state = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

			let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
				.nominations
				.get_inner()
				.map_err(|_| <Error<T>>::OrderedSetFailure)?;

			for bond in nominations_inner {
				Self::nominator_revokes_validator(acc.clone(), bond.owner.clone(), true)?;
			}

			Ok(().into())
		}
		/// Bond more for nominators with respect to a specific validator
		// #[pallet::weight(T::WeightInfo::nominator_bond_more())]
		#[pallet::weight(1000)]
		pub fn nominator_bond_more(
			origin: OriginFor<T>,
			validator: T::AccountId,
			more: BalanceOf<T>,
			unfreeze_bond: bool,
		) -> DispatchResultWithPostInfo {
			let nominator = ensure_signed(origin)?;
			let mut nominations = <NominatorState<T>>::get(&nominator).ok_or(<Error<T>>::NominatorDNE)?;
			let mut validator_state = <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

			let more = if unfreeze_bond {
				more.saturating_add(nominations.frozen_bond)
			} else {
				more
			};

			let new_nomination_bond = nominations.inc_nomination(validator.clone(), more, unfreeze_bond)?;

			ensure!(
				new_nomination_bond >= <StakingMinNominationChillThreshold<T>>::get(),
				<Error<T>>::NominationBelowMin
			);

			let nominator_free_balance = T::Currency::free_balance(&nominator);
			ensure!(
				nominator_free_balance >= nominations.total,
				<Error<T>>::InsufficientBalance
			);

			T::Currency::set_lock(
				T::StakingLockId::get(),
				&nominator,
				nominations.total,
				WithdrawReasons::all(),
			);

			let before = validator_state.total;
			validator_state.inc_nominator(nominator.clone(), more);
			let after = validator_state.total;
			<Total<T>>::mutate(|x| *x = x.saturating_add(more));
			if validator_state.is_active() {
				Self::update_validators_pool(validator.clone(), validator_state.total);
			}
			<ValidatorState<T>>::insert(&validator, validator_state);
			<NominatorState<T>>::insert(&nominator, nominations);
			Self::deposit_event(Event::NominationIncreased(
				nominator,
				new_nomination_bond,
				validator,
				before,
				after,
			));
			Ok(().into())
		}
		/// Bond less for nominators with respect to a specific nominated validator
		// #[pallet::weight(T::WeightInfo::nominator_bond_less())]
		#[pallet::weight(1000)]
		pub fn nominator_bond_less(
			origin: OriginFor<T>,
			validator: T::AccountId,
			less: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let nominator = ensure_signed(origin)?;
			let mut nominations = <NominatorState<T>>::get(&nominator).ok_or(<Error<T>>::NominatorDNE)?;
			let remaining = nominations.dec_nomination(validator.clone(), less)?;

			ensure!(
				remaining >= <StakingMinNominationChillThreshold<T>>::get(),
				<Error<T>>::NominationBelowMin
			);
			ensure!(
				nominations.active_bond >= <StakingMinNominatorTotalBond<T>>::get(),
				<Error<T>>::NominatorBondBelowMin
			);

			ensure!(
				(nominations.unlocking.len() as u32) < T::MaxChunkUnlock::get(),
				<Error<T>>::NoMoreChunks,
			);

			let mut validator_state = <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

			nominations.unlocking.try_push(UnlockChunk {
				value: less,
				session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
			});

			let before = validator_state.bond.saturating_add(validator_state.nomi_bond_total);
			validator_state.dec_nominator(nominator.clone(), less);
			let after = validator_state.bond.saturating_add(validator_state.nomi_bond_total);
			<Total<T>>::mutate(|x| *x = x.saturating_sub(less));
			if validator_state.is_active() {
				Self::update_validators_pool(validator.clone(), after);
			}

			<ValidatorState<T>>::insert(&validator, validator_state);
			<NominatorState<T>>::insert(&nominator, nominations);

			Self::deposit_event(Event::NominationDecreased(nominator, validator, before, after));

			Ok(().into())
		}
		// #[pallet::weight(T::WeightInfo::nominator_move_nomination())]
		#[pallet::weight(1000)]
		pub fn nominator_move_nomination(
			origin: OriginFor<T>,
			from_validator: T::AccountId,
			to_validator: T::AccountId,
			amount: BalanceOf<T>,
			unfreeze_bond: bool,
		) -> DispatchResultWithPostInfo {
			let nominator_acc = ensure_signed(origin)?;

			// ensure validity of the args
			ensure!(Self::is_validator(&from_validator), <Error<T>>::ValidatorDNE);

			// ensure validity of the args
			ensure!(Self::is_validator(&to_validator), <Error<T>>::ValidatorDNE);

			ensure!(from_validator != to_validator, <Error<T>>::ValidatorDNE);

			<NominatorState<T>>::try_mutate_exists(
				nominator_acc.clone(),
				|maybe_nominator| -> DispatchResultWithPostInfo {
					let mut nominator_state = maybe_nominator.as_mut().ok_or(<Error<T>>::NominatorDNE)?;

					let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
						.nominations
						.get_inner()
						.map_err(|_| <Error<T>>::OrderedSetFailure)?;

					ensure!(
						(nominations_inner.len() as u32) <= T::MaxValidatorPerNominator::get(),
						<Error<T>>::ExceedMaxValidatorPerNom,
					);

					let mut to_validator_state =
						<ValidatorState<T>>::get(&to_validator).ok_or(<Error<T>>::ValidatorDNE)?;

					let valid_nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = to_validator_state
						.nominators
						.get_inner()
						.map_err(|_| <Error<T>>::OrderedSetFailure)?;

					ensure!(
						(valid_nominations_inner.len() as u32) < T::MaxNominatorsPerValidator::get(),
						<Error<T>>::TooManyNominators,
					);

					let old_active_bond = nominator_state.active_bond;
					let remaining = nominator_state.rm_nomination(from_validator.clone(), false)?;

					let mut total_nomination_amount = old_active_bond.saturating_sub(remaining);
					nominator_state.total = nominator_state.total.saturating_sub(total_nomination_amount);

					total_nomination_amount = total_nomination_amount.saturating_add(amount);

					if unfreeze_bond {
						total_nomination_amount = total_nomination_amount.saturating_add(nominator_state.frozen_bond);
					}

					ensure!(
						total_nomination_amount >= <StakingMinNominationChillThreshold<T>>::get(),
						<Error<T>>::NominationBelowMin
					);

					let nominator_free_balance = T::Currency::free_balance(&nominator_acc);

					ensure!(
						nominator_free_balance >= nominator_state.total.saturating_add(total_nomination_amount),
						<Error<T>>::InsufficientBalance
					);

					let add_nomination_status = nominator_state.add_nomination(
						Bond {
							owner: to_validator.clone(),
							amount: total_nomination_amount,
						},
						unfreeze_bond,
					)?;

					if add_nomination_status {
						// Validator is new to the nomination pool
						let nomination = Bond {
							owner: nominator_acc.clone(),
							amount: total_nomination_amount,
						};
						to_validator_state.nominators.insert(nomination);
						to_validator_state.inc_nominator(nominator_acc.clone(), total_nomination_amount);
					} else {
						// Validator already exist in nomination pool
						let _ = nominator_state.inc_nomination(
							to_validator.clone(),
							total_nomination_amount,
							unfreeze_bond,
						)?;
						to_validator_state.inc_nominator(nominator_acc.clone(), total_nomination_amount);
					}

					ensure!(
						nominator_state.total >= <StakingMinNominatorTotalBond<T>>::get(),
						<Error<T>>::NominatorBondBelowMin,
					);

					T::Currency::set_lock(
						T::StakingLockId::get(),
						&nominator_acc,
						nominator_state.total,
						WithdrawReasons::all(),
					);

					if to_validator_state.is_active() {
						Self::update_validators_pool(to_validator.clone(), to_validator_state.total);
					}
					// Already ensured nominator is part of validator state.
					// So ignoring the return value.
					let _ = Self::nominator_leaves_validator(nominator_acc.clone(), from_validator.clone())?;
					<Total<T>>::mutate(|x| *x = x.saturating_add(amount));
					<ValidatorState<T>>::insert(&to_validator, to_validator_state.clone());

					// Get the latest source validator info here
					// Error should not occur here.
					let from_validator_state =
						<ValidatorState<T>>::get(&from_validator).ok_or(<Error<T>>::ValidatorDNE)?;

					Self::deposit_event(Event::NominationMoved(
						nominator_acc,
						nominator_state.total,
						from_validator,
						from_validator_state.total,
						to_validator,
						to_validator_state.total,
					));

					Ok(().into())
				},
			)?;

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated total validators per session \[old, new\],
		TotalSelectedSet(u32, u32),
		/// Updated staking config, maximum Validators allowed to join the validators pool
		StakingMaxValidators(u32, u32),
		/// Updated staking config,
		/// minimum validator bond to join the pool \[old, new\],
		/// minimum stake requirement for session  validator selection \[old, new\],
		/// minimum total nominator bond \[old, new\],
		/// minimum nomination bond threshold \[old, new\],
		NewStakingLimits(
			u32,
			u32,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
		),
		/// Prep task done for next new session,
		/// \[current_block_index, new_session_index, number_of_validator_selected,
		/// balance_staked_for_session\],
		NewSession(T::BlockNumber, SessionIndex, u32, BalanceOf<T>),
		/// New validator joined the validators pools
		/// \[account, bond_value, total_staked_value\],
		JoinedValidatorPool(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Validator selected for session validation
		/// \[session_index, account, validator_total_stake\]
		ValidatorChosen(SessionIndex, T::AccountId, BalanceOf<T>),
		/// Validator increased the bond value
		/// \[account, old_bond_value, new_bond_value,\],
		ValidatorBondedMore(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Validator request to unlock the bond value
		/// \[account, old_bond_value, new_bond_value,\],
		ValidatorBondedLess(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Validator request to exit the validator pool
		/// \[current_session_index, account, future_unbonding_session_index\]
		ValidatorScheduledExit(SessionIndex, T::AccountId, SessionIndex),
		/// Validator left the pool after unbonding duration
		/// \[account, validator_unbonded_stake, new_total_stake\]
		ValidatorLeft(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Validator bond drops below threshold
		/// \[account, validator_bond, validator_total_stake\]
		ValidatorBondBelowThreashold(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Nominator nominated the validator
		/// \[account, nomination_value, validator_account, new_validator_total_stake\]
		Nomination(T::AccountId, BalanceOf<T>, T::AccountId, BalanceOf<T>),
		/// Nominator increased the nomination bond value
		/// \[account, new_nomination_amount, validator_account, before_validator_total_stake,
		/// after_validator_total_stake\]
		NominationIncreased(T::AccountId, BalanceOf<T>, T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Nominator decreased the nomination bond value
		/// \[account, validator_account, before_validator_total_stake,
		/// after_validator_total_stake\]
		NominationDecreased(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Nominator switched nomination bond value from old to new validator
		/// \[account, nominator_total_bond, old_validator_account, old_validator_total_stake,
		/// new_validator_account, new_validator_total_stake\]
		NominationMoved(
			T::AccountId,
			BalanceOf<T>,
			T::AccountId,
			BalanceOf<T>,
			T::AccountId,
			BalanceOf<T>,
		),
		/// Nominator denominate validator
		/// \[account, validator_account, nominated_value, new_validator_total_stake\]
		NominatorLeftValidator(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Nominator withdraw all nominations
		/// \[account, nominator_total_stake_unlocked\]
		NominatorLeft(T::AccountId, BalanceOf<T>),
		/// Nomination bond drops below threshold
		/// \[account, validator_account, frozen_bond, total_frozen_bond, nominator_active_bond\]
		NominationBelowThreashold(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
		/// Nomination unbond frozen balance
		/// \[account, unfrozen_bond, new_active_bond, new_total_bond\]
		NominationUnbondFrozen(T::AccountId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
		/// Reward gained by stakers.
		/// \[account, reward_value\]
		StakeReward(T::AccountId, BalanceOf<T>),
		/// Reward payout to stakers.
		/// \[account, reward_value\]
		Rewarded(T::AccountId, BalanceOf<T>),
		/// Violated stakers slashed.
		/// \[account, slashed_value\]
		Slash(T::AccountId, BalanceOf<T>),
		/// Reward payout to offence reporter.
		/// \[account, reward_value\]
		PayReporterReward(T::AccountId, BalanceOf<T>),
		/// Staker slashing is deferred
		/// \[current_session_index, slashed_validator_account\]
		DeferredUnappliedSlash(SessionIndex, T::AccountId),
		/// Staked value unlocked or withdrawn from unlocking queue.
		/// \[controller_account, amount\].
		Withdrawn(T::AccountId, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Not a validator account.
		ValidatorDNE,
		/// Not a nominator account.
		NominatorDNE,
		/// Validator pool full.
		ValidatorPoolFull,
		/// Validator account already part of validator pool.
		ValidatorExists,
		/// Nominator account already part of nominator pool.
		NominatorExists,
		/// Low free balance in caller account.
		InsufficientBalance,
		/// Rewards don't exist.
		RewardsDNE,
		/// Validator bond is less than `MinValidatorPoolStake` value.
		ValidatorBondBelowMin,
		/// Nominator bond is less than `MinNominatorStake` value.
		NominatorBondBelowMin,
		/// Nominator nomination amount is less tha `StakingMinNomination`.
		NominationBelowMin,
		/// Validator account exit pool request is in progress.
		AlreadyLeaving,
		/// Cannot activate, validator account exit pool request is in progress.
		CannotActivateIfLeaving,
		/// Validator is already nominated by `MaxNominatorsPerValidator` nominators.
		TooManyNominators,
		/// Nominator already nominated `MaxValidatorPerNominator` validators.
		ExceedMaxValidatorPerNom,
		/// Trying for duplicate nomination.
		AlreadyNominatedValidator,
		/// Selected validator is not nominated by nominator.
		NominationDNE,
		/// Underflow in bonded value.
		Underflow,
		/// Selected number of validators per session is below `MinSelectedValidators`.
		CannotSetBelowMin,
		/// Invalid argument to `slash_cancel_deferred()`, arg `controllers` is empty.
		EmptyTargets,
		/// Invalid argument to `slash_cancel_deferred()`, arg `session_idx` is invalid.
		InvalidSessionIndex,
		/// Unbonding request exceeds `MaxChunkUnlock`.
		NoMoreChunks,
		/// Error in Increment the reference counter on an account.
		BadState,
		/// Error Invalid arguments
		InvalidArguments,
		/// Unlock Request OverFlowError
		UnlockOverFlowError,
		/// Unable to retrive content from OrderSet.
		OrderedSetFailure,
		/// Nomination Overflow.
		NominationOverflow,
		/// Nomination Invalid
		InvalidNomination,
	}

	/// Maximum Validators allowed to join the validators pool
	#[pallet::storage]
	#[pallet::getter(fn staking_max_validators)]
	pub(crate) type StakingMaxValidators<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Minimum stake required for any account to be in `SelectedCandidates` for the session
	#[pallet::storage]
	#[pallet::getter(fn staking_min_stake_session_selection)]
	pub(crate) type StakingMinStakeSessionSelection<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Minimum stake required for any account to be a validator candidate
	#[pallet::storage]
	#[pallet::getter(fn staking_min_validator_bond)]
	pub(crate) type StakingMinValidatorBond<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Staking config minimum nomination staking per validator
	#[pallet::storage]
	#[pallet::getter(fn staking_min_nomination_chill_threshold)]
	pub(crate) type StakingMinNominationChillThreshold<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Staking config minimum nominator total bond
	#[pallet::storage]
	#[pallet::getter(fn staking_min_nominator_total_bond)]
	pub(crate) type StakingMinNominatorTotalBond<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Total capital locked by this staking pallet
	#[pallet::storage]
	#[pallet::getter(fn total)]
	pub(crate) type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Current session index
	#[pallet::storage]
	#[pallet::getter(fn active_session)]
	pub(crate) type ActiveSession<T: Config> = StorageValue<_, SessionIndex, ValueQuery>;

	/// Commission percent taken off of rewards for all validators
	#[pallet::storage]
	#[pallet::getter(fn validator_fee)]
	pub(crate) type ValidatorFee<T: Config> = StorageValue<_, Perbill, ValueQuery>;

	/// A queue of validators awaiting exit `BondedDuration` delay after request
	#[pallet::storage]
	#[pallet::getter(fn exit_queue)]
	pub(crate) type ExitQueue<T: Config> =
		StorageValue<_, OrderedSet<Bond<T::AccountId, SessionIndex>, T::DefaultStakingMaxValidators>, ValueQuery>;

	/// Snapshot of validator nomination stake at the start of the round
	#[pallet::storage]
	#[pallet::getter(fn at_stake)]
	pub(crate) type AtStake<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		SessionIndex,
		Twox64Concat,
		T::AccountId,
		ValidatorSnapshot<T, T::MaxNominatorsPerValidator>,
		ValueQuery,
	>;

	/// Total backing stake for selected validators in the round
	#[pallet::storage]
	#[pallet::getter(fn staked)]
	pub(crate) type Staked<T: Config> = StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

	/// Accumulated balances for the last Session Round
	#[pallet::storage]
	#[pallet::getter(fn session_accumulated_balance)]
	pub(crate) type SessionAccumulatedBalance<T: Config> =
		StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

	/// Validator reward for the Session
	#[pallet::storage]
	#[pallet::getter(fn session_validator_reward)]
	pub(crate) type SessionValidatorReward<T: Config> =
		StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

	/// Total points awarded to validator for block production in the round
	#[pallet::storage]
	#[pallet::getter(fn points)]
	pub(crate) type Points<T: Config> = StorageMap<_, Twox64Concat, SessionIndex, RewardPoint, ValueQuery>;

	/// Points for each validator per round
	#[pallet::storage]
	#[pallet::getter(fn awarded_pts)]
	pub(crate) type AwardedPts<T: Config> =
		StorageDoubleMap<_, Twox64Concat, SessionIndex, Twox64Concat, T::AccountId, RewardPoint, ValueQuery>;

	/// stakers nodle rewards per session
	#[pallet::storage]
	#[pallet::getter(fn stake_rewards)]
	pub(crate) type StakeRewards<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<StakeReward<BalanceOf<T>>, T::MaxChunkUnlock>, ValueQuery>;

	/// The percentage of the slash that is distributed to reporters.
	///
	/// The rest of the slashed value is handled by the `Slash`.
	#[pallet::storage]
	#[pallet::getter(fn slash_reward_proportion)]
	pub(crate) type SlashRewardProportion<T: Config> = StorageValue<_, Perbill, ValueQuery>;

	/// Get validator state associated with an account if account is collating else None
	#[pallet::storage]
	#[pallet::getter(fn validator_state)]
	pub(crate) type ValidatorState<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		Validator<T, T::MaxNominatorsPerValidator, T::MaxChunkUnlock>,
		OptionQuery,
	>;

	/// Get nominator state associated with an account if account is nominating else None
	#[pallet::storage]
	#[pallet::getter(fn nominator_state)]
	pub(crate) type NominatorState<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		Nominator<T, T::MaxValidatorPerNominator, T::MaxChunkUnlock>,
		OptionQuery,
	>;

	/// The pool of validator validators, each with their total backing stake
	#[pallet::storage]
	#[pallet::getter(fn validator_pool)]
	pub(crate) type ValidatorPool<T: Config> =
		StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>, T::DefaultStakingMaxValidators>, ValueQuery>;

	/// A mapping of still-bonded sessions
	#[pallet::storage]
	#[pallet::getter(fn bonded_sessions)]
	pub(crate) type BondedSessions<T: Config> =
		StorageValue<_, BoundedVec<SessionIndex, T::MaxChunkUnlock>, ValueQuery>;

	impl<T: Config> Pallet<T> {
		pub(crate) fn is_validator(acc: &T::AccountId) -> bool {
			<ValidatorState<T>>::get(acc).is_some()
		}
		pub(crate) fn is_nominator(acc: &T::AccountId) -> bool {
			<NominatorState<T>>::get(acc).is_some()
		}
		// ensure validator is active before calling
		pub fn update_validators_pool(validator: T::AccountId, total: BalanceOf<T>) {
			log::trace!(
				"update_validators_pool:[{:#?}] | Own[{:#?}] | Tot[{:#?}]",
				line!(),
				validator,
				total,
			);
			<ValidatorPool<T>>::mutate(|validators| {
				validators.remove(&Bond::from_owner(validator.clone()));
				validators.insert(Bond {
					owner: validator,
					amount: total,
				});
			});
		}
		fn nominator_revokes_validator(
			acc: T::AccountId,
			validator: T::AccountId,
			do_force: bool,
		) -> DispatchResultWithPostInfo {
			let mut nominator_state = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

			ensure!(
				(nominator_state.unlocking.len() as u32) < T::MaxChunkUnlock::get(),
				<Error<T>>::NoMoreChunks,
			);

			let old_active_bond = nominator_state.active_bond;

			let remaining = nominator_state.rm_nomination(validator.clone(), false)?;

			if !do_force {
				ensure!(
					remaining >= <StakingMinNominatorTotalBond<T>>::get(),
					<Error<T>>::NominatorBondBelowMin
				);
			}

			Self::nominator_leaves_validator(acc.clone(), validator)?;

			<Total<T>>::mutate(|x| *x = x.saturating_sub(old_active_bond.saturating_sub(remaining)));

			nominator_state
				.unlocking
				.try_push(UnlockChunk {
					value: old_active_bond.saturating_sub(nominator_state.active_bond),
					session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
				})
				.map_err(|_| <Error<T>>::UnlockOverFlowError)?;

			<NominatorState<T>>::insert(acc, nominator_state);

			Ok(().into())
		}
		fn nominator_leaves_validator(nominator: T::AccountId, validator: T::AccountId) -> DispatchResultWithPostInfo {
			<ValidatorState<T>>::try_mutate_exists(
				validator.clone(),
				|maybe_validator| -> DispatchResultWithPostInfo {
					let mut state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;
					let mut exists: Option<BalanceOf<T>> = None;

					let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = state
						.nominators
						.get_inner()
						.map_err(|_| <Error<T>>::OrderedSetFailure)?;

					let noms: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominations_inner
						.into_iter()
						.filter_map(|nom| {
							if nom.owner != nominator {
								Some(nom)
							} else {
								exists = Some(nom.amount);
								None
							}
						})
						.collect();
					let nominator_stake = exists.ok_or(<Error<T>>::ValidatorDNE)?;
					let nominators = OrderedSet::try_from(noms).map_err(|_| <Error<T>>::OrderedSetFailure)?;

					state.nominators = nominators;
					state.nomi_bond_total = state.nomi_bond_total.saturating_sub(nominator_stake);
					state.total = state.total.saturating_sub(nominator_stake);
					if state.is_active() {
						Self::update_validators_pool(validator.clone(), state.total);
					}
					Self::deposit_event(Event::NominatorLeftValidator(
						nominator,
						validator,
						nominator_stake,
						state.total,
					));
					Ok(().into())
				},
			)?;
			Ok(().into())
		}
	}
}
