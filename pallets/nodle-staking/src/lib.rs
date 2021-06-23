/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod set;
pub mod weights;

use frame_support::pallet;
pub(crate) mod hooks;
mod migrations;
pub(crate) mod slashing;
pub(crate) mod types;

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;

pub use pallet::*;

#[pallet]
pub mod pallet {
    use super::*;
    use crate::set::OrderedSet;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            Currency, ExistenceRequirement, Get, GetPalletVersion, Imbalance, LockIdentifier,
            LockableCurrency, OnUnbalanced, PalletVersion, WithdrawReasons,
        },
    };
    use frame_system::pallet_prelude::*;
    use frame_system::{self as system};
    use sp_runtime::{
        traits::{AccountIdConversion, Saturating, Zero},
        DispatchResult, ModuleId, Perbill,
    };
    use sp_staking::SessionIndex;
    use sp_std::{convert::From, prelude::*};

    pub use weights::WeightInfo;

    use types::{
        Bond, Nominator, RewardPoint, SpanIndex, StakeReward, UnappliedSlash, UnlockChunk,
        Validator,
    };

    pub use types::{ValidatorSnapshot, ValidatorSnapshotOf};

    pub use hooks::{SessionInterface, StashOf};

    pub(crate) type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

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
        type MinSelectedValidators: Get<u32>;
        /// Maximum nominators per validator
        type MaxNominatorsPerValidator: Get<u32>;
        /// Maximum validators per nominator
        type MaxValidatorPerNominator: Get<u32>;
        /// Fee due to validators, set at genesis
        type DefaultValidatorFee: Get<Perbill>;
        /// Default Slash reward propostion, set at genesis
        type DefaultSlashRewardProportion: Get<Perbill>;
        /// The proportion of the slashing reward to be paid out on the first slashing detection.
        type DefaultSlashRewardFraction: Get<Perbill>;
        /// Minimum stake required for any account to be in `SelectedCandidates` for the round
        type MinValidatorStake: Get<BalanceOf<Self>>;
        /// Minimum stake required for any account to be a validator candidate
        type MinValidatorPoolStake: Get<BalanceOf<Self>>;
        /// Minimum stake for any registered on-chain account to nominate
        type MinNomination: Get<BalanceOf<Self>>;
        /// Minimum stake for any registered on-chain account to become a nominator
        type MinNominatorStake: Get<BalanceOf<Self>>;
        /// Tokens have been minted and are unused for validator-reward.
        /// See [Era payout](./index.html#era-payout).
        type RewardRemainder: OnUnbalanced<NegativeImbalanceOf<Self>>;
        /// Interface for interacting with a session module.
        type SessionInterface: SessionInterface<Self::AccountId>;
        /// This pallet's module id. Used to derivate a dedicated account id to store session
        /// rewards for validators and nominators in.
        type PalletId: Get<ModuleId>;
        /// staking pallet Lock Identifier used for set_lock()
        type StakingLockId: Get<LockIdentifier>;
        /// Max number of unbond request supported by queue
        type MaxChunkUnlock: Get<usize>;
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
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let mut weight = Weight::from(0u64);

            // migrates our storage to pallet version 2.0.6
            let version: PalletVersion =
                <Pallet<T>>::storage_version().unwrap_or(<Pallet<T>>::current_version());

            log::info!(
                "on_runtime_upgrade>[{:#?}]=> - Current Pallet Version-[{:#?}]",
                line!(),
                version,
            );

            if version.major == 2 && version.minor == 0 {
                weight = migrations::poa_validators_migration::<T>();
            }

            weight
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the validators who cannot be slashed (if any).
        ///
        /// The dispatch origin must be Root.
        #[pallet::weight(T::WeightInfo::set_invulnerables(invulnerables.len() as u32))]
        pub fn set_invulnerables(
            origin: OriginFor<T>,
            invulnerables: Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            T::CancelOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;
            <Invulnerables<T>>::put(&invulnerables);
            Self::deposit_event(Event::NewInvulnerables(invulnerables));
            Ok(().into())
        }
        /// Set the total number of validator selected per round
        /// - changes are not applied until the start of the next round
        #[pallet::weight(T::WeightInfo::set_total_validator_per_round(*new))]
        pub fn set_total_validator_per_round(
            origin: OriginFor<T>,
            new: u32,
        ) -> DispatchResultWithPostInfo {
            T::CancelOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;
            ensure!(
                new >= T::MinSelectedValidators::get(),
                <Error<T>>::CannotSetBelowMin
            );
            let old = <TotalSelected<T>>::get();
            <TotalSelected<T>>::put(new);
            Self::deposit_event(Event::TotalSelectedSet(old, new));
            Ok(().into())
        }
        /// Join the set of validators pool
        #[pallet::weight(T::WeightInfo::validator_join_pool())]
        pub fn validator_join_pool(
            origin: OriginFor<T>,
            bond: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            log::debug!("validator_join_pool:[{:#?}] - Entry!!!", line!(),);

            let acc = ensure_signed(origin)?;

            ensure!(!Self::is_validator(&acc), <Error<T>>::ValidatorExists);

            ensure!(
                bond >= T::MinValidatorPoolStake::get(),
                <Error<T>>::ValidatorBondBelowMin
            );

            log::debug!("validator_join_pool:[{:#?}]", line!(),);
            let mut validators = <ValidatorPool<T>>::get();
            ensure!(
                validators.insert(Bond {
                    owner: acc.clone(),
                    amount: bond
                }),
                <Error<T>>::ValidatorExists
            );
            log::debug!("validator_join_pool:[{:#?}]", line!(),);

            let validator_free_balance = T::Currency::free_balance(&acc);
            log::debug!(
                "validator_join_pool:[{:#?}] | free_bal:[{:#?}]",
                line!(),
                validator_free_balance
            );
            ensure!(
                validator_free_balance >= bond,
                <Error<T>>::InsufficientBalance
            );

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
        #[pallet::weight(T::WeightInfo::validator_exit_pool())]
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
        #[pallet::weight(T::WeightInfo::validator_bond_more())]
        pub fn validator_bond_more(
            origin: OriginFor<T>,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;

            ensure!(Self::is_validator(&validator), <Error<T>>::ValidatorDNE);

            let valid_state = Self::validator_state(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

            ensure!(
                !valid_state.is_leaving(),
                <Error<T>>::CannotActivateIfLeaving,
            );

            let validator_free_balance = T::Currency::free_balance(&validator);
            ensure!(
                validator_free_balance >= valid_state.bond.saturating_add(more),
                <Error<T>>::InsufficientBalance,
            );

            <ValidatorState<T>>::mutate(validator.clone(), |maybe_validator| {
                if let Some(state) = maybe_validator {
                    let before = state.bond;
                    state.bond_more(more);
                    T::Currency::set_lock(
                        T::StakingLockId::get(),
                        &validator,
                        state.bond,
                        WithdrawReasons::all(),
                    );
                    let after = state.bond;
                    state.go_online();
                    if state.is_active() {
                        Self::update_validators_pool(
                            validator.clone(),
                            state.bond.saturating_add(state.nomi_bond_total),
                        );
                    }
                    <Total<T>>::mutate(|x| *x = x.saturating_add(more));
                    Self::deposit_event(Event::ValidatorBondedMore(validator, before, after));
                }
            });

            Ok(().into())
        }
        /// Bond less for validator
        #[pallet::weight(T::WeightInfo::validator_bond_less())]
        pub fn validator_bond_less(
            origin: OriginFor<T>,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;

            ensure!(Self::is_validator(&validator), <Error<T>>::ValidatorDNE);

            if let Some(mut state) = Self::validator_state(&validator) {
                ensure!(!state.is_leaving(), <Error<T>>::CannotActivateIfLeaving);

                let before = state.bond;
                let after = state.bond_less(less).ok_or(<Error<T>>::Underflow)?;
                ensure!(
                    after >= T::MinValidatorPoolStake::get(),
                    <Error<T>>::ValidatorBondBelowMin
                );
                ensure!(
                    state.unlocking.len() < T::MaxChunkUnlock::get(),
                    <Error<T>>::NoMoreChunks,
                );

                // `state.total` is locked for bonding duration to handle the slash,
                //  But for upcoming validator selection,
                //  State in validator pool is updated with ( validator bond + nominator bond).
                if state.is_active() {
                    Self::update_validators_pool(
                        validator.clone(),
                        state.bond.saturating_add(state.nomi_bond_total),
                    );
                }

                // Update the overall total, since there is change in
                // active total.
                <Total<T>>::mutate(|x| *x = x.saturating_sub(less));

                // T::Currency::unreserve(&validator, less);
                state.unlocking.push(UnlockChunk {
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
        #[pallet::weight(T::WeightInfo::nominator_nominate())]
        pub fn nominator_nominate(
            origin: OriginFor<T>,
            validator: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator_acc = ensure_signed(origin)?;

            // cannot be a validator candidate and nominator with same AccountId
            ensure!(
                !Self::is_validator(&nominator_acc),
                <Error<T>>::ValidatorExists
            );

            ensure!(
                amount >= T::MinNomination::get(),
                <Error<T>>::NominationBelowMin
            );

            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

            let mut do_add_nomination = true;
            let mut nominator_state = if Self::is_nominator(&nominator_acc) {
                Self::nominator_state(&nominator_acc).ok_or(<Error<T>>::NominatorDNE)?
            } else {
                do_add_nomination = false;
                Nominator::new(validator.clone(), amount)
            };

            ensure!(
                (nominator_state.nominations.0.len() as u32) < T::MaxValidatorPerNominator::get(),
                <Error<T>>::ExceedMaxValidatorPerNom,
            );

            ensure!(
                (validator_state.nominators.0.len() as u32) < T::MaxNominatorsPerValidator::get(),
                <Error<T>>::TooManyNominators,
            );

            let nominator_free_balance = T::Currency::free_balance(&nominator_acc);

            if do_add_nomination {
                ensure!(
                    nominator_free_balance >= nominator_state.total.saturating_add(amount),
                    <Error<T>>::InsufficientBalance
                );
                ensure!(
                    nominator_state.add_nomination(Bond {
                        owner: validator.clone(),
                        amount
                    }),
                    <Error<T>>::AlreadyNominatedValidator,
                );
            } else {
                ensure!(
                    nominator_free_balance >= amount,
                    <Error<T>>::InsufficientBalance
                );
            }

            ensure!(
                nominator_state.total.saturating_add(amount) >= T::MinNominatorStake::get(),
                <Error<T>>::NominatorBondBelowMin,
            );

            let nomination = Bond {
                owner: nominator_acc.clone(),
                amount,
            };
            ensure!(
                validator_state.nominators.insert(nomination),
                <Error<T>>::NominatorExists,
            );

            T::Currency::set_lock(
                T::StakingLockId::get(),
                &nominator_acc,
                nominator_state.total,
                WithdrawReasons::all(),
            );

            validator_state.nomi_bond_total =
                validator_state.nomi_bond_total.saturating_add(amount);
            validator_state.total = validator_state.total.saturating_add(amount);
            let validator_new_total = validator_state.total;
            if validator_state.is_active() {
                Self::update_validators_pool(validator.clone(), validator_state.total);
            }

            <Total<T>>::mutate(|x| *x = x.saturating_add(amount));
            <ValidatorState<T>>::insert(&validator, validator_state);
            <NominatorState<T>>::insert(&nominator_acc, nominator_state);

            Self::deposit_event(Event::Nomination(
                nominator_acc,
                amount,
                validator,
                validator_new_total,
            ));

            Ok(().into())
        }
        /// Revoke an existing nomination
        #[pallet::weight(T::WeightInfo::nominator_denominate())]
        pub fn nominator_denominate(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            Self::nominator_revokes_validator(ensure_signed(origin)?, validator, false)
        }
        /// Quit the set of nominators and, by implication, revoke all ongoing nominations
        #[pallet::weight(T::WeightInfo::nominator_denominate_all())]
        pub fn nominator_denominate_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;

            let nominator = <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

            for bond in nominator.nominations.0 {
                Self::nominator_revokes_validator(acc.clone(), bond.owner.clone(), true)?;
            }

            Ok(().into())
        }
        /// Bond more for nominators with respect to a specific validator
        #[pallet::weight(T::WeightInfo::nominator_bond_more())]
        pub fn nominator_bond_more(
            origin: OriginFor<T>,
            validator: T::AccountId,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator = ensure_signed(origin)?;
            let mut nominations =
                <NominatorState<T>>::get(&nominator).ok_or(<Error<T>>::NominatorDNE)?;
            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;
            let _ = nominations
                .inc_nomination(validator.clone(), more)
                .ok_or(<Error<T>>::NominationDNE)?;

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
                nominator, validator, before, after,
            ));
            Ok(().into())
        }
        /// Bond less for nominators with respect to a specific nominated validator
        #[pallet::weight(T::WeightInfo::nominator_bond_less())]
        pub fn nominator_bond_less(
            origin: OriginFor<T>,
            validator: T::AccountId,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator = ensure_signed(origin)?;
            let mut nominations =
                <NominatorState<T>>::get(&nominator).ok_or(<Error<T>>::NominatorDNE)?;
            let remaining = nominations
                .dec_nomination(validator.clone(), less)
                .map(|bal| bal)
                .map_err(|err_str| {
                    if err_str == "Underflow" {
                        <Error<T>>::Underflow
                    } else {
                        <Error<T>>::NominationDNE
                    }
                })?;

            ensure!(
                remaining >= T::MinNomination::get(),
                <Error<T>>::NominationBelowMin
            );
            ensure!(
                nominations.active_bond >= T::MinNominatorStake::get(),
                <Error<T>>::NominatorBondBelowMin
            );

            ensure!(
                nominations.unlocking.len() < T::MaxChunkUnlock::get(),
                <Error<T>>::NoMoreChunks,
            );

            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(<Error<T>>::ValidatorDNE)?;

            nominations.unlocking.push(UnlockChunk {
                value: less,
                session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
            });

            let before = validator_state
                .bond
                .saturating_add(validator_state.nomi_bond_total);
            validator_state.dec_nominator(nominator.clone(), less);
            let after = validator_state
                .bond
                .saturating_add(validator_state.nomi_bond_total);
            <Total<T>>::mutate(|x| *x = x.saturating_sub(less));
            if validator_state.is_active() {
                Self::update_validators_pool(validator.clone(), after);
            }

            <ValidatorState<T>>::insert(&validator, validator_state);
            <NominatorState<T>>::insert(&nominator, nominations);

            Self::deposit_event(Event::NominationDecreased(
                nominator, validator, before, after,
            ));

            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::withdraw_unbonded())]
        pub fn withdraw_unbonded(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;

            let curr_session = Self::active_session();
            let mut unlocked_bal = Zero::zero();
            let mut new_total = Zero::zero();

            if Self::is_validator(&acc) {
                <ValidatorState<T>>::mutate(&acc, |maybe_validator| {
                    if let Some(state) = maybe_validator {
                        let pre_total = state.total.saturating_sub(state.nomi_bond_total);
                        unlocked_bal = state.consolidate_unlocked(curr_session);
                        new_total = pre_total.saturating_sub(unlocked_bal);
                    }
                });

                if unlocked_bal > Zero::zero() {
                    T::Currency::set_lock(
                        T::StakingLockId::get(),
                        &acc,
                        new_total,
                        WithdrawReasons::all(),
                    );
                }

                Self::deposit_event(Event::Withdrawn(acc, unlocked_bal));
            } else if Self::is_nominator(&acc) {
                if let Some(mut nominator_state) = Self::nominator_state(&acc) {
                    let old_total = nominator_state.total;
                    unlocked_bal = nominator_state.consolidate_unlocked(curr_session);
                    new_total = nominator_state.total;

                    if unlocked_bal > Zero::zero() {
                        T::Currency::set_lock(
                            T::StakingLockId::get(),
                            &acc,
                            new_total,
                            WithdrawReasons::all(),
                        );
                    }

                    Self::deposit_event(Event::Withdrawn(acc.clone(), unlocked_bal));

                    if nominator_state.nominations.0.len().is_zero() {
                        T::Currency::remove_lock(T::StakingLockId::get(), &acc);
                        let _ = Self::kill_state_info(&acc);
                        Self::deposit_event(Event::NominatorLeft(acc.clone(), old_total));
                    } else {
                        <NominatorState<T>>::insert(acc.clone(), nominator_state);
                    }
                }
            }

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::withdraw_staking_rewards())]
        pub fn withdraw_staking_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;

            if <StakeRewards<T>>::contains_key(&acc) {
                let mut total_rewards: BalanceOf<T> = Zero::zero();

                let rewards = <StakeRewards<T>>::take(&acc);

                // Iterate over the reward gains for the given account.
                for reward in rewards.iter() {
                    total_rewards = total_rewards.saturating_add(reward.value);
                }

                // deposit the reward gain to
                if total_rewards > T::Currency::minimum_balance() {
                    if let Ok(imb) = T::Currency::deposit_into_existing(&acc, total_rewards) {
                        Self::deposit_event(Event::Rewarded(acc.clone(), imb.peek()));
                    }
                } else {
                    // staking rewards are below ED
                    Self::deposit_event(Event::Rewarded(acc.clone(), Zero::zero()));
                }
            } else {
                // Not a validator or Nominator
                Self::deposit_event(Event::Rewarded(acc.clone(), Zero::zero()));
            }

            Ok(().into())
        }

        /// Cancel enactment of a deferred slash.
        ///
        /// Can be called by the `T::SlashCancelOrigin`.
        ///
        /// Parameters: session index and validator list of the slashes for that session to kill.
        #[pallet::weight(T::WeightInfo::slash_cancel_deferred(*session_idx as u32, controllers.len() as u32))]
        pub fn slash_cancel_deferred(
            origin: OriginFor<T>,
            session_idx: SessionIndex,
            controllers: Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            T::CancelOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            let apply_at = session_idx.saturating_add(T::SlashDeferDuration::get());

            ensure!(!controllers.is_empty(), <Error<T>>::EmptyTargets);
            ensure!(
                <UnappliedSlashes<T>>::contains_key(apply_at),
                <Error<T>>::InvalidSessionIndex
            );

            <UnappliedSlashes<T>>::mutate(&apply_at, |unapplied| {
                for controller_acc in controllers {
                    unapplied.retain(|ustat| {
                        if ustat.validator == controller_acc {
                            false
                        } else {
                            true
                        }
                    });
                }
            });
            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not a validator account.
        ValidatorDNE,
        /// Not a nominator account.
        NominatorDNE,
        /// Validator account already part of validator pool.
        ValidatorExists,
        /// Nominator account already part of nominator pool.
        NominatorExists,
        /// Low free balance in caller account.
        InsufficientBalance,
        /// Validator bond is less than `MinValidatorPoolStake` value.
        ValidatorBondBelowMin,
        /// Nominator bond is less than `MinNominatorStake` value.
        NominatorBondBelowMin,
        /// Nominator nomination amount is less tha `MinNomination`.
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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    pub enum Event<T: Config> {
        /// Updated InVulnerable validator list \[validator_list\],
        NewInvulnerables(Vec<T::AccountId>),
        /// Updated total validators per session \[old, new\],
        TotalSelectedSet(u32, u32),
        /// Prep task done for next new session,
        /// \[current_block_index, new_session_index, number_of_validator_selected, balance_staked_for_session\],
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
        /// Nominator nominated the validator
        /// \[account, nomination_value, validator_account, new_validator_total_stake\]
        Nomination(T::AccountId, BalanceOf<T>, T::AccountId, BalanceOf<T>),
        /// Nominator increased the nomination bond value
        /// \[account, validator_account, before_validator_total_stake, after_validator_total_stake\]
        NominationIncreased(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Nominator decreased the nomination bond value
        /// \[account, validator_account, before_validator_total_stake, after_validator_total_stake\]
        NominationDecreased(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Nominator denominate validator
        /// \[account, validator_account, nominated_value, new_validator_total_stake\]
        NominatorLeftValidator(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Nominator withdraw all nominations
        /// \[account, nominator_total_stake_unlocked\]
        NominatorLeft(T::AccountId, BalanceOf<T>),
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

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    pub(crate) type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

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
        Validator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// Get nominator state associated with an account if account is nominating else None
    #[pallet::storage]
    #[pallet::getter(fn nominator_state)]
    pub(crate) type NominatorState<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Nominator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// The total validators selected every round
    #[pallet::storage]
    #[pallet::getter(fn total_selected)]
    pub(crate) type TotalSelected<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The validators selected for the current round
    #[pallet::storage]
    #[pallet::getter(fn selected_validators)]
    pub(crate) type SelectedValidators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// The pool of validator validators, each with their total backing stake
    #[pallet::storage]
    #[pallet::getter(fn validator_pool)]
    pub(crate) type ValidatorPool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    /// A queue of validators awaiting exit `BondedDuration` delay after request
    #[pallet::storage]
    #[pallet::getter(fn exit_queue)]
    pub(crate) type ExitQueue<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, SessionIndex>>, ValueQuery>;

    /// Snapshot of validator nomination stake at the start of the round
    #[pallet::storage]
    #[pallet::getter(fn at_stake)]
    pub(crate) type AtStake<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::AccountId,
        ValidatorSnapshot<T::AccountId, BalanceOf<T>>,
        ValueQuery,
    >;

    /// Total backing stake for selected validators in the round
    #[pallet::storage]
    #[pallet::getter(fn staked)]
    pub(crate) type Staked<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

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
    pub(crate) type Points<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, RewardPoint, ValueQuery>;

    /// Points for each validator per round
    #[pallet::storage]
    #[pallet::getter(fn awarded_pts)]
    pub(crate) type AwardedPts<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::AccountId,
        RewardPoint,
        ValueQuery,
    >;

    /// stakers nodle rewards per session
    #[pallet::storage]
    #[pallet::getter(fn stake_rewards)]
    pub(crate) type StakeRewards<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<StakeReward<BalanceOf<T>>>, ValueQuery>;

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
        StorageMap<_, Twox64Concat, T::AccountId, slashing::SlashingSpans, OptionQuery>;

    /// Snapshot of validator slash state
    #[pallet::storage]
    #[pallet::getter(fn span_slash)]
    pub(crate) type SpanSlash<T: Config> = StorageMap<
        _,
        Twox64Concat,
        (T::AccountId, SpanIndex),
        slashing::SpanRecord<BalanceOf<T>>,
        ValueQuery,
    >;

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
    pub(crate) type NominatorSlashInSession<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::AccountId,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// All unapplied slashes that are queued for later.
    #[pallet::storage]
    #[pallet::getter(fn unapplied_slashes)]
    pub(crate) type UnappliedSlashes<T: Config> = StorageMap<
        _,
        Twox64Concat,
        SessionIndex,
        Vec<UnappliedSlash<T::AccountId, BalanceOf<T>>>,
        ValueQuery,
    >;

    /// A mapping of still-bonded sessions
    #[pallet::storage]
    #[pallet::getter(fn bonded_sessions)]
    pub(crate) type BondedSessions<T: Config> = StorageValue<_, Vec<SessionIndex>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub stakers: Vec<(T::AccountId, Option<T::AccountId>, BalanceOf<T>)>,
        pub invulnerables: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                stakers: vec![],
                invulnerables: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            log::trace!("GenesisBuild:[{:#?}] - Entry!!!", line!(),);

            let duplicate_invulnerables = self
                .invulnerables
                .iter()
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                duplicate_invulnerables.len() == self.invulnerables.len(),
                "duplicate invulnerables in genesis."
            );
            <Invulnerables<T>>::put(&self.invulnerables);

            for &(ref actor, ref opt_val, balance) in &self.stakers {
                assert!(
                    T::Currency::free_balance(&actor) >= balance,
                    "Account does not have enough balance to bond."
                );

                let _ = if let Some(nominated_val) = opt_val {
                    <Pallet<T>>::nominator_nominate(
                        T::Origin::from(Some(actor.clone()).into()),
                        nominated_val.clone(),
                        balance,
                    )
                } else {
                    <Pallet<T>>::validator_join_pool(
                        T::Origin::from(Some(actor.clone()).into()),
                        balance,
                    )
                };
            }

            // Ensure balance is >= ED
            let imbalance = T::Currency::issue(T::Currency::minimum_balance());
            T::Currency::resolve_creating(&T::PalletId::get().into_account(), imbalance);

            // Set collator commission to default config
            <ValidatorFee<T>>::put(T::DefaultValidatorFee::get());
            // Set total selected validators to minimum config
            <TotalSelected<T>>::put(T::MinSelectedValidators::get());
            // Set default slash reward fraction
            <SlashRewardProportion<T>>::put(T::DefaultSlashRewardProportion::get());
            let genesis_session_idx = 0u32;
            // Choose top TotalSelected validators
            let (v_count, total_staked) =
                <Pallet<T>>::select_session_validators(genesis_session_idx);
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

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }

        /// Direct implementation of `GenesisBuild::assimilate_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
            <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
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
        // ensure validator is active before calling
        pub fn remove_from_validators_pool(validator: T::AccountId) {
            log::trace!(
                "remove_from_validators_pool:[{:#?}] | Own[{:#?}]",
                line!(),
                validator
            );
            <ValidatorPool<T>>::mutate(|validators| {
                validators.remove(&Bond::from_owner(validator.clone()));
            });
        }
        pub(crate) fn validator_deactivate(controller: &T::AccountId) {
            log::trace!(
                "validator_deactivate:[{:#?}] - Acc[{:#?}]",
                line!(),
                controller
            );
            <ValidatorState<T>>::mutate(&controller, |maybe_validator| {
                if let Some(valid_state) = maybe_validator {
                    valid_state.go_offline();
                    Self::remove_from_validators_pool(controller.clone());
                }
            });
        }
        fn nominator_leaves_validator(
            nominator: T::AccountId,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            <ValidatorState<T>>::try_mutate_exists(
                validator.clone(),
                |maybe_validator| -> DispatchResultWithPostInfo {
                    let mut state = maybe_validator.as_mut().ok_or(<Error<T>>::ValidatorDNE)?;
                    let mut exists: Option<BalanceOf<T>> = None;
                    let noms = state
                        .clone()
                        .nominators
                        .0
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
                    let nominators = OrderedSet::from(noms);

                    state.nominators = nominators;
                    state.nomi_bond_total = state.nomi_bond_total.saturating_sub(nominator_stake);
                    state.total = state.total.saturating_sub(nominator_stake);
                    if state.is_active() {
                        Self::update_validators_pool(validator.clone(), state.total);
                    }
                    <Total<T>>::mutate(|x| *x = x.saturating_sub(nominator_stake));
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

        fn nominator_revokes_validator(
            acc: T::AccountId,
            validator: T::AccountId,
            do_force: bool,
        ) -> DispatchResultWithPostInfo {
            let mut nominator_state =
                <NominatorState<T>>::get(&acc).ok_or(<Error<T>>::NominatorDNE)?;

            ensure!(
                nominator_state.unlocking.len() < T::MaxChunkUnlock::get(),
                <Error<T>>::NoMoreChunks,
            );

            let old_active_bond = nominator_state.active_bond;
            let remaining = nominator_state
                .rm_nomination(validator.clone())
                .ok_or(<Error<T>>::NominationDNE)?;

            if !do_force {
                ensure!(
                    remaining >= T::MinNominatorStake::get(),
                    <Error<T>>::NominatorBondBelowMin
                );
            }

            Self::nominator_leaves_validator(acc.clone(), validator)?;

            nominator_state.unlocking.push(UnlockChunk {
                value: old_active_bond.saturating_sub(nominator_state.active_bond),
                session_idx: Self::active_session().saturating_add(T::BondedDuration::get()),
            });

            <NominatorState<T>>::insert(acc.clone(), nominator_state);

            Ok(().into())
        }

        fn validator_revokes_nomination(nominator_acc: T::AccountId, validator: T::AccountId) {
            <NominatorState<T>>::mutate(&nominator_acc, |maybe_nominator_state| {
                if let Some(nominator_state) = maybe_nominator_state.as_mut() {
                    let old_active_bond = nominator_state.active_bond;

                    if let Some(_remaining) = nominator_state.rm_nomination(validator.clone()) {
                        nominator_state.unlocking.push(UnlockChunk {
                            value: old_active_bond.saturating_sub(nominator_state.active_bond),
                            session_idx: Self::active_session(),
                        });

                        Self::deposit_event(Event::NominatorLeftValidator(
                            nominator_acc.clone(),
                            validator,
                            old_active_bond.saturating_sub(nominator_state.active_bond),
                            Zero::zero(),
                        ));
                    }
                }
            });
        }

        pub(crate) fn pay_stakers(next: SessionIndex) {
            log::trace!("pay_stakers:[{:#?}] - Sess-idx[{:#?}]", line!(), next);

            let mint = |amt: BalanceOf<T>, to: T::AccountId| {
                if amt > T::Currency::minimum_balance() {
                    <StakeRewards<T>>::mutate(&to, |rewards| {
                        rewards.push(StakeReward {
                            session_idx: next,
                            value: amt,
                        });
                        // *rewards = rewards;
                        Self::deposit_event(Event::StakeReward(to.clone(), amt));
                    });
                }
            };

            let validator_fee = <ValidatorFee<T>>::get();
            let total = <Points<T>>::get(next);
            // let total_staked = <Staked<T>>::get(next);
            // let issuance = Self::compute_issuance(total_staked);
            let issuance = Self::session_validator_reward(next);
            for (val, pts) in <AwardedPts<T>>::iter_prefix(next) {
                let pct_due = Perbill::from_rational_approximation(pts, total);
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
                    mint(amt_due, val.clone());
                    log::trace!("pay_stakers:[{:#?}] - L3 Solo Mode", line!());
                } else {
                    let val_pct = Perbill::from_rational_approximation(state.bond, state.total);
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

                    mint(val_due, val.clone());
                    // pay nominators due portion
                    for Bond { owner, amount } in state.nominators {
                        let percent = Perbill::from_rational_approximation(amount, state.total);
                        let due = percent * amt_due;
                        mint(due, owner);
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
                            for bond in state.nominators.0 {
                                Self::validator_revokes_nomination(
                                    bond.owner.clone(),
                                    x.owner.clone(),
                                );
                            }
                            // return stake to validator
                            let mut unlock_chunk_total: BalanceOf<T> = Zero::zero();
                            let _ = state.unlocking.iter().map(|chunk| {
                                unlock_chunk_total = unlock_chunk_total.saturating_add(chunk.value);
                            });

                            let new_total = <Total<T>>::get()
                                .saturating_sub(state.total.saturating_sub(unlock_chunk_total));
                            <Total<T>>::put(new_total);

                            T::Currency::remove_lock(T::StakingLockId::get(), &x.owner);

                            let _ = Self::kill_state_info(&x.owner);

                            Self::deposit_event(Event::ValidatorLeft(
                                x.owner,
                                state.total,
                                new_total,
                            ));
                        }
                        None
                    }
                })
                .collect::<Vec<Bond<T::AccountId, SessionIndex>>>();
            <ExitQueue<T>>::put(OrderedSet::from(remain_exits));
        }
        /// Best as in most cumulatively supported in terms of stake
        pub(crate) fn select_session_validators(next: SessionIndex) -> (u32, BalanceOf<T>) {
            let (mut validators_count, mut total) = (0u32, <BalanceOf<T>>::zero());
            let mut validators = <ValidatorPool<T>>::get().0;
            // order validators pool by stake (least to greatest so requires `rev()`)
            validators.sort_unstable_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
            let top_n = <TotalSelected<T>>::get() as usize;
            // choose the top TotalSelected qualified validators, ordered by stake
            let mut top_validators = validators
                .into_iter()
                .rev()
                .take(top_n)
                .filter(|x| x.amount >= T::MinValidatorStake::get())
                .map(|x| x.owner)
                .collect::<Vec<T::AccountId>>();

            if <Invulnerables<T>>::get().len() > 0 {
                top_validators = Self::invulnerables()
                    .iter()
                    .chain(top_validators.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<T::AccountId>>();
            }

            top_validators.sort();
            top_validators.dedup();

            // snapshot exposure for round for weighting reward distribution
            for account in top_validators.iter() {
                let state = <ValidatorState<T>>::get(&account)
                    .expect("all members of ValidatorQ must be validators");
                let amount = state.bond.saturating_add(state.nomi_bond_total);
                let exposure: ValidatorSnapshot<T::AccountId, BalanceOf<T>> = state.into();
                <AtStake<T>>::insert(next, account, exposure);
                validators_count = validators_count.saturating_add(1u32);
                total = total.saturating_add(amount);
                Self::deposit_event(Event::ValidatorChosen(next, account.clone(), amount));
            }

            // top_validators.sort();
            // insert canonical collator set
            <SelectedValidators<T>>::put(top_validators);
            (validators_count, total)
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
        pub(crate) fn reward_by_ids(
            validators_points: impl IntoIterator<Item = (T::AccountId, u32)>,
        ) {
            let now = Self::active_session();
            for (validator, points) in validators_points.into_iter() {
                let score_points = <AwardedPts<T>>::get(now, &validator).saturating_add(points);
                <AwardedPts<T>>::insert(now, validator, score_points);
                <Points<T>>::mutate(now, |x| *x = x.saturating_add(points));
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

            if Self::is_validator(&controller) {
                <ValidatorState<T>>::remove(controller);
            } else if Self::is_nominator(&controller) {
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
            <AtStake<T>>::remove_prefix(session_idx);
            <Points<T>>::remove(session_idx);
            <AwardedPts<T>>::remove_prefix(session_idx);
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
                for unapplied_slash in session_slashes {
                    slashing::apply_slash::<T>(unapplied_slash);
                }
            }
        }
    }
}
