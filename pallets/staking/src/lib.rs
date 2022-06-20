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

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::pallet;
mod hooks;
mod set;
mod slashing;
mod types;
mod weights;

pub use pallet::*;

#[pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			Currency, ExistenceRequirement, Get, LockIdentifier, LockableCurrency, OnUnbalanced, ValidatorRegistration,
			WithdrawReasons,
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

	use hooks::SessionInterface;
	use types::{Bond, Nominator, RewardPoint, SpanIndex, StakeReward, UnlockChunk, Validator, ValidatorSnapshot};
	use weights::WeightInfo;

	use set::OrderedSet;

	pub(crate) type StakingInvulnerables<T> =
		BoundedVec<<T as frame_system::Config>::AccountId, <T as Config>::MaxInvulnerableStakers>;

	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub(crate) type NegativeImbalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The staking balance.
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		/// Handler for the unbalanced reduction when slashing a staker.
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
		/// Number of sessions that staked fund remain bonded for
		type BondedDuration: Get<SessionIndex>;
		/// Number of sessions that slashes are deferred by, after computation.
		type SlashDeferDuration: Get<SessionIndex>;
		/// Minimum number of selected validators every round
		#[pallet::constant]
		type MinSelectedValidators: Get<u32>;
		/// Maximum invulnerable Stakers
		#[pallet::constant]
		type MaxInvulnerableStakers: Get<u32>;
		/// Maximum nominators per validator
		#[pallet::constant]
		type MaxNominatorsPerValidator: Get<u32>;
		/// Maximum validators per nominator
		#[pallet::constant]
		type MaxValidatorPerNominator: Get<u32>;
		/// Maximum Slash Reporters
		#[pallet::constant]
		type MaxSlashReporters: Get<u32>;
		/// Fee due to validators, set at genesis
		#[pallet::constant]
		type DefaultValidatorFee: Get<Perbill>;
		/// Default Slash reward propostion, set at genesis
		#[pallet::constant]
		type DefaultSlashRewardProportion: Get<Perbill>;
		/// The proportion of the slashing reward to be paid out on the first slashing detection.
		#[pallet::constant]
		type DefaultSlashRewardFraction: Get<Perbill>;
		/// Maximum validators allowed to join the pool.
		#[pallet::constant]
		type DefaultStakingMaxValidators: Get<u32>;
		/// Minimum stake required for any account to be in `SelectedCandidates` for the session
		#[pallet::constant]
		type DefaultStakingMinStakeSessionSelection: Get<BalanceOf<Self>>;
		/// Minimum stake required for any account to be a validator candidate
		#[pallet::constant]
		type DefaultStakingMinValidatorBond: Get<BalanceOf<Self>>;
		/// Minimum stake for any registered on-chain account to nominate
		#[pallet::constant]
		type DefaultStakingMinNominationChillThreshold: Get<BalanceOf<Self>>;
		/// Minimum stake for any registered on-chain account to become a nominator
		#[pallet::constant]
		type DefaultStakingMinNominatorTotalBond: Get<BalanceOf<Self>>;
		/// Tokens have been minted and are unused for validator-reward.
		/// See [Era payout](./index.html#era-payout).
		type RewardRemainder: OnUnbalanced<NegativeImbalanceOf<Self>>;
		/// Interface for interacting with a session module.
		type SessionInterface: SessionInterface<Self::AccountId>;
		/// Validate a user is registered
		type ValidatorRegistration: ValidatorRegistration<Self::AccountId>;
		/// This pallet's module id. Used to derivate a dedicated account id to store session
		/// rewards for validators and nominators in.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// staking pallet Lock Identifier used for set_lock()
		#[pallet::constant]
		type StakingLockId: Get<LockIdentifier>;
		/// Max number of unbond request supported by queue
		#[pallet::constant]
		type MaxChunkUnlock: Get<u32>;
		/// Max number of Unapplied Slash
		#[pallet::constant]
		type MaxUnAppliedSlash: Get<u32>;
		/// Max number of Slash Spans
		#[pallet::constant]
		type MaxSlashSpan: Get<u32>;
		/// The origin which can cancel a deferred slash. Root can always do this.
		type CancelOrigin: EnsureOrigin<Self::Origin>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			// migrations::v1::PoAToStaking::<T>::pre_upgrade()
			Ok(())
		}

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			// migrations::v1::PoAToStaking::<T>::on_runtime_upgrade()
			0u64
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			// migrations::v1::PoAToStaking::<T>::post_upgrade()
			Ok(())
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the validators who cannot be slashed (if any).
		///
		/// The dispatch origin must be Root.
		#[pallet::weight(T::WeightInfo::set_invulnerables(invulnerables.len() as u32))]
		pub fn set_invulnerables(origin: OriginFor<T>, invulnerables: Vec<T::AccountId>) -> DispatchResultWithPostInfo {
			T::CancelOrigin::try_origin(origin).map(|_| ()).or_else(ensure_root)?;

			let bounded_invulnerables: StakingInvulnerables<T> =
				BoundedVec::try_from(invulnerables).map_err(|_| <Error<T>>::InvulnerablesOverflow)?;

			<Invulnerables<T>>::put(&bounded_invulnerables);
			Self::deposit_event(Event::NewInvulnerables(bounded_invulnerables));
			Ok(().into())
		}
		/// Set the total number of validator selected per round
		/// - changes are not applied until the start of the next round
		#[pallet::weight(T::WeightInfo::set_total_validator_per_round(*new))]
		pub fn set_total_validator_per_round(origin: OriginFor<T>, new: u32) -> DispatchResultWithPostInfo {
			T::CancelOrigin::try_origin(origin).map(|_| ()).or_else(ensure_root)?;
			ensure!(new >= T::MinSelectedValidators::get(), <Error<T>>::CannotSetBelowMin);
			let old = <TotalSelected<T>>::get();
			<TotalSelected<T>>::put(new);
			Self::deposit_event(Event::TotalSelectedSet(old, new));
			Ok(().into())
		}
		#[pallet::weight(T::WeightInfo::set_staking_limits())]
		pub fn set_staking_limits(
			origin: OriginFor<T>,
			max_stake_validators: u32,
			min_stake_session_selection: BalanceOf<T>,
			min_validator_bond: BalanceOf<T>,
			min_nominator_total_bond: BalanceOf<T>,
			min_nominator_chill_threshold: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::CancelOrigin::try_origin(origin).map(|_| ()).or_else(ensure_root)?;

			ensure!(max_stake_validators > 0, <Error<T>>::InvalidArguments);
			ensure!(min_stake_session_selection > Zero::zero(), <Error<T>>::InvalidArguments);
			ensure!(min_validator_bond > Zero::zero(), <Error<T>>::InvalidArguments);
			ensure!(min_nominator_total_bond > Zero::zero(), <Error<T>>::InvalidArguments);
			ensure!(
				min_nominator_chill_threshold > Zero::zero(),
				<Error<T>>::InvalidArguments
			);

			let old_max_stake_validators = <StakingMaxValidators<T>>::get();
			<StakingMaxValidators<T>>::set(max_stake_validators);

			let old_min_stake_session_selection = <StakingMinStakeSessionSelection<T>>::get();
			<StakingMinStakeSessionSelection<T>>::set(min_stake_session_selection);

			let old_min_validator_bond = <StakingMinValidatorBond<T>>::get();
			<StakingMinValidatorBond<T>>::set(min_validator_bond);

			let old_min_nominator_total_bond = <StakingMinNominatorTotalBond<T>>::get();
			<StakingMinNominatorTotalBond<T>>::set(min_nominator_total_bond);

			let old_min_nominator_chill_threshold = <StakingMinNominationChillThreshold<T>>::get();
			<StakingMinNominationChillThreshold<T>>::set(min_nominator_chill_threshold);

			Self::deposit_event(Event::NewStakingLimits(
				old_max_stake_validators,
				max_stake_validators,
				old_min_stake_session_selection,
				min_stake_session_selection,
				old_min_validator_bond,
				min_validator_bond,
				old_min_nominator_total_bond,
				min_nominator_total_bond,
				old_min_nominator_chill_threshold,
				min_nominator_chill_threshold,
			));

			Self::active_stake_reconciliation();

			Ok(().into())
		}
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

			<ExitQueue<T>>::try_mutate(|exits| -> DispatchResultWithPostInfo {
				exits
					.insert(Bond {
						owner: validator.clone(),
						amount: when,
					})
					.map_err(|_| <Error<T>>::ExitQueueOverflow)?;
				Ok(().into())
			})?;

			<ValidatorState<T>>::mutate(&validator, |maybe_validator| {
				if let Some(state) = maybe_validator {
					state.leave_validators_pool(when);
				}
			});

			<ValidatorPool<T>>::try_mutate(|validators| -> DispatchResultWithPostInfo {
				validators.remove(&Bond::from_owner(validator.clone())).map_err(|_| {
					log::error!("validator_exit_pool:[{:#?}] - ValidatorPool Update Failure", line!(),);
					<Error<T>>::OrderedSetFailure
				})?;
				Ok(().into())
			})?;

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

			<ValidatorState<T>>::try_mutate(&validator, |maybe_validator| -> DispatchResultWithPostInfo {
				let state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;
				let before = state.bond;
				state.bond_more(more);
				T::Currency::set_lock(T::StakingLockId::get(), &validator, state.bond, WithdrawReasons::all());
				let after = state.bond;
				state.go_online();
				if state.is_active() {
					Self::update_validators_pool(&validator, state.bond.saturating_add(state.nomi_bond_total))
						.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
				}
				<Total<T>>::mutate(|x| *x = x.saturating_add(more));
				Self::deposit_event(Event::ValidatorBondedMore(validator.clone(), before, after));
				Ok(().into())
			})?;

			Self::validator_stake_reconciliation(&validator)?;

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
					Self::update_validators_pool(&validator, state.bond.saturating_add(state.nomi_bond_total))
						.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
				}

				// Update the overall total, since there is change in
				// active total.
				<Total<T>>::mutate(|x| *x = x.saturating_sub(less));

				state
					.unlocking
					.try_push(UnlockChunk {
						value: less,
						session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
					})
					.map_err(|_| <Error<T>>::UnlockOverFlowError)?;

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
				Self::update_validators_pool(&validator, validator_state.total)
					.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
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

			Self::nominator_revokes_validator(&acc, &validator, do_force)
		}
		/// Quit the set of nominators and, by implication, revoke all ongoing nominations
		// #[pallet::weight(T::WeightInfo::nominator_denominate_all())]
		#[pallet::weight(1000)]
		pub fn nominator_denominate_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let acc = ensure_signed(origin)?;

			let nominator_state = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

			let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
				.nominations
				.get_inner()
				.map_err(|_| <Error<T>>::OrderedSetFailure)?;

			for bond in nominations_inner {
				Self::nominator_revokes_validator(&acc, &bond.owner, true)?;
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
			validator_state.inc_nominator(nominator.clone(), more)?;
			let after = validator_state.total;
			<Total<T>>::mutate(|x| *x = x.saturating_add(more));
			if validator_state.is_active() {
				Self::update_validators_pool(&validator, validator_state.total)
					.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
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

			nominations
				.unlocking
				.try_push(UnlockChunk {
					value: less,
					session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
				})
				.map_err(|_| <Error<T>>::UnlockOverFlowError)?;

			let before = validator_state.bond.saturating_add(validator_state.nomi_bond_total);
			validator_state.dec_nominator(&nominator, less)?;
			let after = validator_state.bond.saturating_add(validator_state.nomi_bond_total);
			<Total<T>>::mutate(|x| *x = x.saturating_sub(less));
			if validator_state.is_active() {
				Self::update_validators_pool(&validator, after).map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
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

			<NominatorState<T>>::try_mutate(nominator_acc.clone(), |maybe_nominator| -> DispatchResultWithPostInfo {
				let mut nominator_state = maybe_nominator.as_mut().ok_or(<Error<T>>::NominatorDNE)?;

				let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominator_state
					.nominations
					.get_inner()
					.map_err(|_| <Error<T>>::OrderedSetFailure)?;

				ensure!(
					(nominations_inner.len() as u32) <= T::MaxValidatorPerNominator::get(),
					<Error<T>>::ExceedMaxValidatorPerNom,
				);

				let mut to_validator_state = <ValidatorState<T>>::get(&to_validator).ok_or(<Error<T>>::ValidatorDNE)?;

				let valid_nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = to_validator_state
					.nominators
					.get_inner()
					.map_err(|_| <Error<T>>::OrderedSetFailure)?;

				ensure!(
					(valid_nominations_inner.len() as u32) < T::MaxNominatorsPerValidator::get(),
					<Error<T>>::TooManyNominators,
				);

				let old_active_bond = nominator_state.active_bond;
				let remaining = nominator_state.rm_nomination(&from_validator, false)?;

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
					to_validator_state
						.nominators
						.insert(nomination)
						.map_err(|_| <Error<T>>::NominationOverflow)?;
					to_validator_state.inc_nominator(nominator_acc.clone(), total_nomination_amount)?;
				} else {
					// Validator already exist in nomination pool
					let _ =
						nominator_state.inc_nomination(to_validator.clone(), total_nomination_amount, unfreeze_bond)?;
					to_validator_state.inc_nominator(nominator_acc.clone(), total_nomination_amount)?;
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
					Self::update_validators_pool(&to_validator, to_validator_state.total)
						.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
				}
				// Already ensured nominator is part of validator state.
				// So ignoring the return value.
				let _ = Self::nominator_leaves_validator(&nominator_acc, &from_validator)?;
				<Total<T>>::mutate(|x| *x = x.saturating_add(amount));
				<ValidatorState<T>>::insert(&to_validator, to_validator_state.clone());

				// Get the latest source validator info here
				// Error should not occur here.
				let from_validator_state = <ValidatorState<T>>::get(&from_validator).ok_or(<Error<T>>::ValidatorDNE)?;

				Self::deposit_event(Event::NominationMoved(
					nominator_acc,
					nominator_state.total,
					from_validator,
					from_validator_state.total,
					to_validator,
					to_validator_state.total,
				));

				Ok(().into())
			})?;

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated InVulnerable validator list \[validator_list\],
		NewInvulnerables(StakingInvulnerables<T>),
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
		/// Invulnerables Overflow.
		InvulnerablesOverflow,
		/// Exit Queue Overflow.
		ExitQueueOverflow,
		/// Validator Pool Overflow.
		ValidatorPoolOverflow,
		/// Nomination Invalid
		InvalidNomination,
		/// Slash Reporters Overflow
		ReportersOverflow,
		/// Stake Reward Overflow
		StakeRewardOverflow,
		/// Stake Reward Storage Error
		StakeRewardDNE,
	}

	/// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
	/// easy to initialize and the performance hit is minimal (we expect no more than four
	/// invulnerables) and restricted to testnets.
	#[pallet::storage]
	#[pallet::getter(fn invulnerables)]
	pub(crate) type Invulnerables<T: Config> = StorageValue<_, StakingInvulnerables<T>, ValueQuery>;

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

	/// The total validators selected every round
	#[pallet::storage]
	#[pallet::getter(fn total_selected)]
	pub(crate) type TotalSelected<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The validators selected for the current round
	#[pallet::storage]
	#[pallet::getter(fn selected_validators)]
	pub(crate) type SelectedValidators<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::DefaultStakingMaxValidators>, ValueQuery>;

	/// The pool of validator validators, each with their total backing stake
	#[pallet::storage]
	#[pallet::getter(fn validator_pool)]
	pub(crate) type ValidatorPool<T: Config> =
		StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>, T::DefaultStakingMaxValidators>, ValueQuery>;

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

	/// Snapshot of validator slash state
	#[pallet::storage]
	#[pallet::getter(fn slashing_spans)]
	pub(crate) type SlashingSpans<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, slashing::SlashingSpans<T, T::MaxSlashSpan>, OptionQuery>;

	/// Snapshot of validator slash state
	#[pallet::storage]
	#[pallet::getter(fn span_slash)]
	pub(crate) type SpanSlash<T: Config> =
		StorageMap<_, Twox64Concat, (T::AccountId, SpanIndex), slashing::SpanRecord<BalanceOf<T>>, ValueQuery>;

	/// All slashing events on validators, mapped by session to the highest
	/// slash proportion and slash value of the session.
	#[pallet::storage]
	#[pallet::getter(fn validator_slash_in_session)]
	pub(crate) type ValidatorSlashInSession<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		SessionIndex,
		Twox64Concat,
		T::AccountId,
		(Perbill, BalanceOf<T>),
		OptionQuery,
	>;

	/// All slashing events on nominators,
	/// mapped by session to the highest slash value of the session.
	#[pallet::storage]
	#[pallet::getter(fn nominator_slash_in_session)]
	pub(crate) type NominatorSlashInSession<T: Config> =
		StorageDoubleMap<_, Twox64Concat, SessionIndex, Twox64Concat, T::AccountId, BalanceOf<T>, OptionQuery>;

	/// All unapplied slashes that are queued for later.
	#[pallet::storage]
	#[pallet::getter(fn unapplied_slashes)]
	pub(crate) type UnappliedSlashes<T: Config> = StorageMap<
		_,
		Twox64Concat,
		SessionIndex,
		BoundedVec<
			slashing::UnappliedSlash<T, T::MaxNominatorsPerValidator, T::MaxSlashReporters>,
			T::MaxUnAppliedSlash,
		>,
		ValueQuery,
	>;

	/// A mapping of still-bonded sessions
	#[pallet::storage]
	#[pallet::getter(fn bonded_sessions)]
	pub(crate) type BondedSessions<T: Config> =
		StorageValue<_, BoundedVec<SessionIndex, T::MaxChunkUnlock>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub stakers: Vec<(T::AccountId, Option<T::AccountId>, BalanceOf<T>)>,
		pub invulnerables: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				stakers: Default::default(),
				invulnerables: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			log::trace!("GenesisBuild:[{:#?}] - Entry!!!", line!());

			let duplicate_invulnerables = self.invulnerables.iter().collect::<std::collections::BTreeSet<_>>();
			assert!(
				duplicate_invulnerables.len() == self.invulnerables.len(),
				"duplicate invulnerables in genesis."
			);

			let invulnerables: StakingInvulnerables<T> =
				self.invulnerables.clone().try_into().expect("Too long invulnerables");

			<Invulnerables<T>>::put(invulnerables);

			// Ensure balance is >= ED
			let imbalance = T::Currency::issue(T::Currency::minimum_balance());
			T::Currency::resolve_creating(&T::PalletId::get().into_account(), imbalance);

			// Set collator commission to default config
			<ValidatorFee<T>>::put(T::DefaultValidatorFee::get());
			// Set total selected validators to minimum config
			<TotalSelected<T>>::put(T::MinSelectedValidators::get());
			// Set default slash reward fraction
			<SlashRewardProportion<T>>::put(T::DefaultSlashRewardProportion::get());
			// Maximum Validators allowed to join the validators pool
			<StakingMaxValidators<T>>::put(T::DefaultStakingMaxValidators::get());
			// Minimum stake required for any account to be in `SelectedCandidates` for the session
			<StakingMinStakeSessionSelection<T>>::put(T::DefaultStakingMinStakeSessionSelection::get());
			// Minimum stake required for any account to be a validator candidate
			<StakingMinValidatorBond<T>>::put(T::DefaultStakingMinValidatorBond::get());
			// Set default min nomination stake value
			<StakingMinNominationChillThreshold<T>>::put(T::DefaultStakingMinNominationChillThreshold::get());
			// Staking config minimum nominator total bond
			<StakingMinNominatorTotalBond<T>>::put(T::DefaultStakingMinNominatorTotalBond::get());

			log::trace!(
				"GenesisBuild:[{:#?}] - Staking Cfg ([{:#?}],[{:#?}],[{:#?}],[{:#?}],[{:#?}])",
				line!(),
				<StakingMaxValidators<T>>::get(),
				<StakingMinStakeSessionSelection<T>>::get(),
				<StakingMinValidatorBond<T>>::get(),
				<StakingMinNominationChillThreshold<T>>::get(),
				<StakingMinNominatorTotalBond<T>>::get(),
			);

			for &(ref actor, ref opt_val, balance) in &self.stakers {
				assert!(
					T::Currency::free_balance(actor) >= balance,
					"Account does not have enough balance to bond."
				);

				let _ = if let Some(nominated_val) = opt_val {
					<Pallet<T>>::nominator_nominate(
						T::Origin::from(Some(actor.clone()).into()),
						nominated_val.clone(),
						balance,
						false,
					)
				} else {
					<Pallet<T>>::validator_join_pool(T::Origin::from(Some(actor.clone()).into()), balance)
				};
			}

			let genesis_session_idx = 0u32;

			// Choose top TotalSelected validators
			let (v_count, total_staked) = match <Pallet<T>>::select_session_validators(genesis_session_idx) {
				Ok((v_count, total_staked)) => (v_count, total_staked),
				Err(_) => {
					log::error!("GenesisBuild:[{:#?}] - Select Session Validator Failure", line!(),);
					(Zero::zero(), Zero::zero())
				}
			};

			// Start Session 1
			<ActiveSession<T>>::put(genesis_session_idx);
			// Snapshot total stake
			<Staked<T>>::insert(genesis_session_idx, <Total<T>>::get());

			log::trace!(
				"GenesisBuild:[{:#?}] - (SI[{}],VC[{}],TS[{:#?}])",
				line!(),
				genesis_session_idx,
				v_count,
				total_staked,
			);

			<Pallet<T>>::deposit_event(Event::NewSession(
				T::BlockNumber::zero(),
				genesis_session_idx,
				v_count,
				total_staked,
			));
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn is_validator(acc: &T::AccountId) -> bool {
			<ValidatorState<T>>::get(acc).is_some()
		}

		pub(crate) fn is_nominator(acc: &T::AccountId) -> bool {
			<NominatorState<T>>::get(acc).is_some()
		}

		// ensure validator is active before calling
		pub fn update_validators_pool(validator: &T::AccountId, total: BalanceOf<T>) -> Result<(), ()> {
			log::trace!(
				"update_validators_pool:[{:#?}] | Own[{:#?}] | Tot[{:#?}]",
				line!(),
				validator,
				total,
			);
			<ValidatorPool<T>>::try_mutate(|validators| -> Result<(), ()> {
				validators.remove(&Bond::from_owner(validator.clone()))?;
				validators.insert(Bond {
					owner: validator.clone(),
					amount: total,
				})?;
				Ok(())
			})
		}

		// ensure validator is active before calling
		pub fn remove_from_validators_pool(validator: &T::AccountId) -> Result<(), ()> {
			log::trace!("remove_from_validators_pool:[{:#?}] | Own[{:#?}]", line!(), validator);
			<ValidatorPool<T>>::try_mutate(|validators| -> Result<(), ()> {
				validators.remove(&Bond::from_owner(validator.clone()))?;
				Ok(())
			})
		}

		pub(crate) fn validator_deactivate(controller: &T::AccountId) -> Result<(), Error<T>> {
			log::trace!("validator_deactivate:[{:#?}] - Acc[{:#?}]", line!(), controller);
			<ValidatorState<T>>::try_mutate(&controller, |maybe_validator| -> Result<(), Error<T>> {
				let valid_state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;
				valid_state.go_offline();
				Self::remove_from_validators_pool(&controller).map_err(|_| <Error<T>>::ValidatorDNE)?;
				Ok(().into())
			})
		}

		fn nominator_leaves_validator(
			nominator: &T::AccountId,
			validator: &T::AccountId,
		) -> DispatchResultWithPostInfo {
			<ValidatorState<T>>::try_mutate(validator, |maybe_validator| -> DispatchResultWithPostInfo {
				let mut state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;
				let mut exists: Option<BalanceOf<T>> = None;

				let nominations_inner: Vec<Bond<T::AccountId, BalanceOf<T>>> = state
					.nominators
					.get_inner()
					.map_err(|_| <Error<T>>::OrderedSetFailure)?;

				let noms: Vec<Bond<T::AccountId, BalanceOf<T>>> = nominations_inner
					.into_iter()
					.filter_map(|nom| {
						if nom.owner != *nominator {
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
					Self::update_validators_pool(&validator, state.total)
						.map_err(|_| <Error<T>>::ValidatorPoolOverflow)?;
				}
				Self::deposit_event(Event::NominatorLeftValidator(
					nominator.clone(),
					validator.clone(),
					nominator_stake,
					state.total,
				));
				Ok(().into())
			})?;
			Ok(().into())
		}

		fn nominator_revokes_validator(
			acc: &T::AccountId,
			validator: &T::AccountId,
			do_force: bool,
		) -> DispatchResultWithPostInfo {
			let mut nominator_state = <NominatorState<T>>::get(acc).ok_or(<Error<T>>::NominatorDNE)?;

			ensure!(
				(nominator_state.unlocking.len() as u32) < T::MaxChunkUnlock::get(),
				<Error<T>>::NoMoreChunks,
			);

			let old_active_bond = nominator_state.active_bond;

			let remaining = nominator_state.rm_nomination(validator, false)?;

			if !do_force {
				ensure!(
					remaining >= <StakingMinNominatorTotalBond<T>>::get(),
					<Error<T>>::NominatorBondBelowMin
				);
			}

			Self::nominator_leaves_validator(&acc, validator)?;

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

		fn validator_revokes_nomination(
			nominator_acc: &T::AccountId,
			validator: &T::AccountId,
		) -> DispatchResultWithPostInfo {
			<NominatorState<T>>::try_mutate(nominator_acc, |maybe_nominator_state| -> DispatchResultWithPostInfo {
				let nominator_state = maybe_nominator_state.as_mut().ok_or(<Error<T>>::NominatorDNE)?;

				let old_active_bond = nominator_state.active_bond;

				let _remaining = nominator_state.rm_nomination(validator, false)?;

				nominator_state
					.unlocking
					.try_push(UnlockChunk {
						value: old_active_bond.saturating_sub(nominator_state.active_bond),
						session_idx: Self::active_session(),
					})
					.map_err(|_| <Error<T>>::UnlockOverFlowError)?;

				Self::deposit_event(Event::NominatorLeftValidator(
					nominator_acc.clone(),
					validator.clone(),
					old_active_bond.saturating_sub(nominator_state.active_bond),
					Zero::zero(),
				));

				Ok(().into())
			})?;
			Ok(().into())
		}

		fn validator_freeze_nomination(
			nominator_acc: &T::AccountId,
			validator: &T::AccountId,
		) -> DispatchResultWithPostInfo {
			<NominatorState<T>>::try_mutate(&nominator_acc, |maybe_nominator_state| -> DispatchResultWithPostInfo {
				if let Some(nominator_state) = maybe_nominator_state.as_mut() {
					let old_active_bond = nominator_state.active_bond;

					let _remaining = nominator_state.rm_nomination(&validator, true)?;
					Self::deposit_event(Event::NominationBelowThreashold(
						nominator_acc.clone(),
						validator.clone(),
						old_active_bond.saturating_sub(nominator_state.active_bond),
						nominator_state.frozen_bond,
						nominator_state.active_bond,
					));
				}
				Ok(().into())
			})
		}

		pub(crate) fn pay_stakers(next: SessionIndex) {
			log::trace!("pay_stakers:[{:#?}] - Sess-idx[{:#?}]", line!(), next);

			let mint = |amt: BalanceOf<T>, to: T::AccountId| -> DispatchResultWithPostInfo {
				if amt > T::Currency::minimum_balance() {
					<StakeRewards<T>>::try_mutate(&to, |rewards| -> DispatchResultWithPostInfo {
						rewards
							.try_push(StakeReward {
								session_idx: next,
								value: amt,
							})
							.map_err(|_| {
								log::error!("pay_stakers:[{:#?}] - StakeRewards Overflow Error", line!(),);
								<Error<T>>::StakeRewardOverflow
							})?;
						Self::deposit_event(Event::StakeReward(to.clone(), amt));
						Ok(().into())
					})?;
				}
				Ok(().into())
			};

			let validator_fee = <ValidatorFee<T>>::get();
			let total = <Points<T>>::get(next);
			// let total_staked = <Staked<T>>::get(next);
			// let issuance = Self::compute_issuance(total_staked);
			let issuance = Self::session_validator_reward(next);
			for (val, pts) in <AwardedPts<T>>::iter_prefix(next) {
				let pct_due = Perbill::from_rational(pts, total);
				let mut amt_due = pct_due * issuance;

				log::trace!(
					"pay_stakers:[{:#?}] - L1 [{:#?}] | [{:#?}] | [{:#?}]",
					line!(),
					total,
					issuance,
					pct_due
				);

				log::trace!(
					"pay_stakers:[{:#?}] - L2 [{:#?}] | [{:#?}] | [{:#?}]",
					line!(),
					val,
					pts,
					amt_due
				);

				if amt_due <= T::Currency::minimum_balance() {
					continue;
				}
				// Take the snapshot of block author and nominations
				// let state = <AtStake<T>>::take(next, &val);
				let state = Self::at_stake(next, &val);

				if state.nominators.is_empty() {
					// solo collator with no nominators
					match mint(amt_due, val.clone()) {
						Ok(_) => {
							log::trace!("pay_stakers:[{:#?}] - L3 Solo Mode", line!());
						}
						Err(err) => {
							log::error!("pay_stakers:[{:#?}] - Mint Failure:[{:#?}]", line!(), err);
						}
					}
				} else {
					let val_pct = Perbill::from_rational(state.bond, state.total);
					let commission = validator_fee * amt_due;
					let val_due = if commission > T::Currency::minimum_balance() {
						amt_due = amt_due.saturating_sub(commission);
						(val_pct * amt_due).saturating_add(commission)
					} else {
						// commission is negligible so not applied
						val_pct * amt_due
					};

					log::trace!(
						"pay_stakers:[{:#?}] - L4 [{:#?}] | [{:#?}] | [{:#?}]",
						line!(),
						validator_fee,
						val_due,
						amt_due,
					);

					match mint(val_due, val.clone()) {
						Ok(_) => {
							log::trace!("pay_stakers:[{:#?}] - L3 Solo Mode", line!());
						}
						Err(err) => {
							log::error!("pay_stakers:[{:#?}] - Mint Failure:[{:#?}]", line!(), err);
						}
					}

					// pay nominators due portion
					for Bond { owner, amount } in state.nominators {
						let percent = Perbill::from_rational(amount, state.total);
						let due = percent * amt_due;
						match mint(due, owner) {
							Ok(_) => (),
							Err(err) => {
								log::error!("pay_stakers:[{:#?}] - Mint Failure:[{:#?}]", line!(), err);
							}
						}
					}
				}
			}
		}

		pub(crate) fn execute_delayed_validator_exits(next: SessionIndex) {
			let remain_exits = <ExitQueue<T>>::get()
				.0
				.into_iter()
				.filter_map(|x| {
					if x.amount > next {
						Some(x)
					} else {
						if let Some(state) = <ValidatorState<T>>::get(&x.owner) {
							// revoke all nominations
							let nominators: Vec<Bond<T::AccountId, BalanceOf<T>>> = match state.nominators.get_inner() {
								Ok(nominators) => nominators,
								Err(_) => {
									log::error!(
										"execute_delayed_validator_exits:[{:#?}] - OrderedSet Failure",
										line!(),
									);
									return None;
								}
							};

							for bond in nominators {
								match Self::validator_revokes_nomination(&bond.owner, &x.owner) {
									Ok(_) => (),
									Err(_) => {
										log::error!(
											"execute_delayed_validator_exits:[{:#?}] - validator_revokes_nomination Failure",
											line!(),
										);
									}
								}
							}
							// return stake to validator
							let mut unlock_chunk_total: BalanceOf<T> = Zero::zero();
							let _ = state.unlocking.clone().to_vec().iter().map(|chunk| {
								unlock_chunk_total = unlock_chunk_total.saturating_add(chunk.value);
							});

							let new_total =
								<Total<T>>::get().saturating_sub(state.total.saturating_sub(unlock_chunk_total));
							<Total<T>>::put(new_total);

							T::Currency::remove_lock(T::StakingLockId::get(), &x.owner);

							let _ = Self::kill_state_info(&x.owner);

							Self::deposit_event(Event::ValidatorLeft(x.owner, state.total, new_total));
						}
						None
					}
				})
				.collect::<Vec<Bond<T::AccountId, SessionIndex>>>();

			let updated_exist_list: OrderedSet<Bond<T::AccountId, SessionIndex>, T::DefaultStakingMaxValidators> =
				match OrderedSet::try_from(remain_exits) {
					Ok(updated_exist_list) => updated_exist_list,
					Err(_) => {
						log::error!(
							"execute_delayed_validator_exits:[{:#?}] - OrderedSet Failure, Could be Overflow",
							line!(),
						);
						return ();
					}
				};

			<ExitQueue<T>>::put(updated_exist_list);
		}

		fn active_stake_reconciliation() {
			let validators: Vec<Bond<T::AccountId, BalanceOf<T>>> = match <ValidatorPool<T>>::get().get_inner() {
				Ok(validators) => validators,
				Err(_) => {
					log::error!("active_stake_reconciliation:[{:#?}] - OrderedSet Failure", line!(),);
					return ();
				}
			};

			let reconciled_list = validators
				.into_iter()
				.filter_map(|x| {
					let bond_status = if let Some(valid_state) = <ValidatorState<T>>::get(&x.owner) {
						if valid_state.is_active() && valid_state.bond > Self::staking_min_validator_bond() {
							Some(Bond {
								owner: x.owner.clone(),
								amount: valid_state.total,
							})
						} else {
							None
						}
					} else {
						None
					};

					match Self::validator_stake_reconciliation(&x.owner) {
						Err(_) => {
							log::error!(
								"active_stake_reconciliation:[{:#?}] - Reconciliation failure for Validator[{:#?}]",
								line!(),
								&x.owner
							);
							None
						}
						Ok(_) => bond_status,
					}
				})
				.collect::<Vec<Bond<T::AccountId, BalanceOf<T>>>>();

			let reconciled_updated = match OrderedSet::try_from(reconciled_list) {
				Ok(reconciled_updated) => reconciled_updated,
				Err(_) => {
					log::error!(
						"active_stake_reconciliation:[{:#?}] - OrderedSet Failure Could be Overflow",
						line!(),
					);
					return ();
				}
			};

			<ValidatorPool<T>>::put(reconciled_updated);
		}

		pub(crate) fn validator_stake_reconciliation(controller: &T::AccountId) -> Result<(), Error<T>> {
			<ValidatorState<T>>::try_mutate(&controller, |maybe_validator| -> Result<(), Error<T>> {
				let mut valid_state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;

				let nomination = valid_state
					.clone()
					.nominators
					.get_inner()
					.map_err(|_| <Error<T>>::OrderedSetFailure)?;

				let noms: Vec<Bond<T::AccountId, BalanceOf<T>>> = nomination
					.into_iter()
					.filter_map(|nom| {
						if nom.amount < Self::staking_min_nomination_chill_threshold() {
							match Self::validator_freeze_nomination(&nom.owner, &controller) {
								Ok(_) => (),
								Err(_) => {
									log::error!(
										"validator_stake_reconciliation:[{:#?}] - validator_freeze_nomination Failure",
										line!(),
									);
								}
							}

							match valid_state.dec_nominator(&nom.owner, nom.amount) {
								Ok(_) => (),
								Err(_) => {
									log::error!(
										"validator_stake_reconciliation:[{:#?}] - dec_nominator Failure",
										line!(),
									);
								}
							}

							None
						} else {
							Some(nom)
						}
					})
					.collect();

				let nominators = OrderedSet::try_from(noms).map_err(|_| <Error<T>>::OrderedSetFailure)?;
				valid_state.nominators = nominators;

				if valid_state.bond < Self::staking_min_validator_bond() && valid_state.is_active() {
					valid_state.go_offline();
					Self::deposit_event(Event::ValidatorBondBelowThreashold(
						controller.clone(),
						valid_state.bond,
						valid_state.total,
					));
				}

				Ok(().into())
			})
		}

		/// Best as in most cumulatively supported in terms of stake
		pub(crate) fn select_session_validators(next: SessionIndex) -> Result<(u32, BalanceOf<T>), ()> {
			let (mut validators_count, mut total) = (0u32, <BalanceOf<T>>::zero());

			let mut validators = match <ValidatorPool<T>>::get().get_inner() {
				Ok(validators) => validators,
				Err(_) => {
					log::error!("select_session_validators:[{:#?}] - OrderedSet Failure", line!(),);
					return Err(());
				}
			};

			// order validators pool by stake (least to greatest so requires `rev()`)
			validators.sort_unstable_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
			let top_n = <TotalSelected<T>>::get() as usize;
			// choose the top TotalSelected qualified validators, ordered by stake
			let mut top_validators = validators
				.into_iter()
				.rev()
				.take(top_n)
				.filter(|x| x.amount >= <StakingMinStakeSessionSelection<T>>::get())
				.filter(|x| T::ValidatorRegistration::is_registered(&x.owner))
				.map(|x| x.owner)
				.collect::<Vec<T::AccountId>>();

			if !<Invulnerables<T>>::get().to_vec().is_empty() {
				top_validators = Self::invulnerables()
					.to_vec()
					.iter()
					.chain(top_validators.iter())
					.cloned()
					.collect::<Vec<T::AccountId>>();
			}

			top_validators.sort();
			top_validators.dedup();

			// Overflow should not occur, since size is reserved for DefaultStakingMaxValidators
			let selected_validators: BoundedVec<T::AccountId, T::DefaultStakingMaxValidators> =
				top_validators.clone().try_into().expect("Selected Valiadtors Overflow");

			// snapshot exposure for round for weighting reward distribution
			for account in top_validators.iter() {
				let state = <ValidatorState<T>>::get(&account).expect("all members of ValidatorQ must be validators");
				let amount = state.bond.saturating_add(state.nomi_bond_total);
				let exposure: ValidatorSnapshot<T, T::MaxNominatorsPerValidator> =
					state.build_snapshot().expect("Validator Snapshot Build Failure");
				<AtStake<T>>::insert(next, account, exposure);
				validators_count = validators_count.saturating_add(1u32);
				total = total.saturating_add(amount);
				Self::deposit_event(Event::ValidatorChosen(next, account.clone(), amount));
			}

			// top_validators.sort();
			// insert canonical collator set
			<SelectedValidators<T>>::put(selected_validators);
			Ok((validators_count, total))
		}
		/// Add reward points to validators using their account ID.
		///
		/// Validators are keyed by stash account ID and must be in the current elected set.
		///
		/// For each element in the iterator the given number of points in u32 is added to the
		/// validator, thus duplicates are handled.
		///
		/// At the end of the era each the total payout will be distributed among validator
		/// relatively to their points.
		///
		/// COMPLEXITY: Complexity is `number_of_validator_to_reward x current_elected_len`.
		pub(crate) fn reward_by_ids(validators_points: impl IntoIterator<Item = (T::AccountId, u32)>) {
			let now = Self::active_session();
			for (validator, points) in validators_points.into_iter() {
				if Self::is_validator(&validator) {
					let score_points = <AwardedPts<T>>::get(now, &validator).saturating_add(points);
					<AwardedPts<T>>::insert(now, validator, score_points);
					<Points<T>>::mutate(now, |x| *x = x.saturating_add(points));
				}
			}
		}
		/// Remove all associated data of a stash account from the staking system.
		///
		/// Assumes storage is upgraded before calling.
		///
		/// This is called:
		/// - after a `withdraw_unbonded()` call that frees all of a stash's bonded balance.
		/// - through `reap_stash()` if the balance has fallen to zero (through slashing).
		fn kill_state_info(controller: &T::AccountId) -> DispatchResult {
			slashing::clear_slash_metadata::<T>(controller)?;
			if Self::is_validator(controller) {
				<ValidatorState<T>>::remove(controller);
			} else if Self::is_nominator(controller) {
				<NominatorState<T>>::remove(controller);
			}
			Ok(())
		}
		/// Clear session information for given session index
		pub(crate) fn clear_session_information(session_idx: SessionIndex) {
			log::trace!(
				"clear_session_information:[{:#?}] - AccuBal[{:#?}]",
				line!(),
				<SessionAccumulatedBalance<T>>::get(session_idx),
			);

			<Staked<T>>::remove(session_idx);
			<AtStake<T>>::remove_prefix(session_idx, None);
			<Points<T>>::remove(session_idx);
			<AwardedPts<T>>::remove_prefix(session_idx, None);
			<SessionValidatorReward<T>>::remove(session_idx);
			<UnappliedSlashes<T>>::remove(session_idx);
			slashing::clear_session_metadata::<T>(session_idx);

			// withdraw rewards
			match T::Currency::withdraw(
				&T::PalletId::get().into_account(),
				<SessionAccumulatedBalance<T>>::take(session_idx),
				WithdrawReasons::all(),
				ExistenceRequirement::KeepAlive,
			) {
				Ok(imbalance) => T::RewardRemainder::on_unbalanced(imbalance),
				Err(err) => {
					log::error!(
						"clear_session_information:[{:#?}] - [{:#?}] | [{:#?}]",
						line!(),
						err,
						"Warning: an error happened when trying to handle active session rewards \
						 remainder",
					);
				}
			};
		}
		/// Apply previously-unapplied slashes on the beginning of a new session, after a delay.
		pub(crate) fn apply_unapplied_slashes(active_session: SessionIndex) {
			if <UnappliedSlashes<T>>::contains_key(active_session) {
				let session_slashes = <UnappliedSlashes<T>>::take(&active_session);
				for mut unapplied_slash in session_slashes {
					unapplied_slash.apply_slash();
				}
			}
		}
	}
}
