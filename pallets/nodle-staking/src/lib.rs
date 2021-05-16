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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[macro_use]
pub mod helpers;

mod set;

use frame_support::pallet;
pub mod slashing;

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;

pub use pallet::*;

pub(crate) const LOG_TARGET: &'static str = "runtime::staking";

#[cfg(test)]
pub(crate) const TST_LOG_TARGET: &'static str = "runtime::tststaking";

#[pallet]
pub mod pallet {
    use super::*;
    use crate::set::OrderedSet;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            Currency, ExistenceRequirement, Get, Imbalance, OnUnbalanced, ReservableCurrency,
            WithdrawReasons,
        },
    };
    use frame_system::pallet_prelude::*;
    use frame_system::{self as system};
    use pallet_session::historical;
    use parity_scale_codec::{Decode, Encode, HasCompact};
    use sp_runtime::{
        traits::{AccountIdConversion, AtLeast32BitUnsigned, Convert, Saturating, Zero},
        ModuleId, Perbill, RuntimeDebug,
    };
    use sp_staking::{
        offence::{Offence, OffenceDetails, OffenceError, OnOffenceHandler, ReportOffence},
        SessionIndex,
    };
    use sp_std::{cmp::Ordering, convert::From, prelude::*};

    #[derive(Default, Clone, Encode, Decode, RuntimeDebug)]
    pub struct Bond<AccountId, Balance> {
        pub owner: AccountId,
        pub amount: Balance,
    }

    impl<A, B: Default> Bond<A, B> {
        fn from_owner(owner: A) -> Self {
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

    #[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
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

    #[derive(Encode, Decode, RuntimeDebug)]
    /// Global validator state with commission fee, bonded stake, and nominations
    pub struct Validator<AccountId, Balance> {
        pub id: AccountId,
        pub bond: Balance,
        pub nominators: OrderedSet<Bond<AccountId, Balance>>,
        pub total: Balance,
        pub state: ValidatorStatus,
    }

    impl<
            A: Ord + Clone,
            B: AtLeast32BitUnsigned + Ord + Copy + sp_std::ops::AddAssign + sp_std::ops::SubAssign,
        > Validator<A, B>
    {
        pub fn new(id: A, bond: B) -> Self {
            let total = bond;
            Validator {
                id,
                bond,
                nominators: OrderedSet::new(),
                total,
                state: ValidatorStatus::default(), // default active
            }
        }
        pub fn is_active(&self) -> bool {
            self.state == ValidatorStatus::Active
        }
        pub fn is_leaving(&self) -> bool {
            matches!(self.state, ValidatorStatus::Leaving(_))
        }
        pub fn bond_more(&mut self, more: B) {
            self.bond += more;
            self.total += more;
        }
        // Returns None if underflow or less == self.bond (in which case validator should leave)
        pub fn bond_less(&mut self, less: B) -> Option<B> {
            if self.bond > less {
                self.bond -= less;
                self.total -= less;
                Some(self.bond)
            } else {
                None
            }
        }
        pub fn inc_nominator(&mut self, nominator: A, more: B) {
            for x in &mut self.nominators.0 {
                if x.owner == nominator {
                    x.amount += more;
                    self.total += more;
                    return;
                }
            }
        }
        pub fn dec_nominator(&mut self, nominator: A, less: B) {
            for x in &mut self.nominators.0 {
                if x.owner == nominator {
                    x.amount -= less;
                    self.total -= less;
                    return;
                }
            }
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
        Balance: AtLeast32BitUnsigned + Saturating + Copy,
    {
        /// Slash the validator for a given amount of balance. This can grow the value
        /// of the slash in the case that the validator has less than `minimum_balance`
        /// active funds. Returns the amount of funds actually slashed.
        ///
        /// Slashes from `active` funds first, and then `unlocking`, starting with the
        /// chunks that are closest to unlocking.
        pub(crate) fn slash(&mut self, value: Balance, minimum_balance: Balance) -> Balance {
            let pre_total = self.total;
            let total = &mut self.total;
            let active = &mut self.bond;

            let slash_out_of =
                |total_remaining: &mut Balance, target: &mut Balance, value: &Balance| {
                    let mut slash_from_target = (*value).min(*target);

                    if !slash_from_target.is_zero() {
                        *target -= slash_from_target;

                        // Make sure not drop below ED
                        if *target <= minimum_balance {
                            let diff_val = minimum_balance.saturating_sub(*target);
                            *target += diff_val;
                            slash_from_target -= diff_val;
                        }

                        *total_remaining = total_remaining.saturating_sub(slash_from_target);
                    }
                };

            slash_out_of(total, active, &value);

            pre_total.saturating_sub(*total)
        }
    }

    #[derive(Clone, Default, Encode, Decode, RuntimeDebug)]
    /// Snapshot of validator state at the start of the round for which they are selected
    pub struct ValidatorSnapshot<AccountId, Balance> {
        pub bond: Balance,
        pub nominators: Vec<Bond<AccountId, Balance>>,
        pub total: Balance,
    }

    impl<A: Clone, B: Copy> From<Validator<A, B>> for ValidatorSnapshot<A, B> {
        fn from(other: Validator<A, B>) -> ValidatorSnapshot<A, B> {
            ValidatorSnapshot {
                bond: other.bond,
                nominators: other.nominators.0,
                total: other.total,
            }
        }
    }

    impl<AccountId: Ord, Balance> Eq for ValidatorSnapshot<AccountId, Balance> {}

    impl<AccountId: Ord, Balance> PartialEq for ValidatorSnapshot<AccountId, Balance> {
        fn eq(&self, other: &Self) -> bool {
            self.nominators == other.nominators
        }
    }

    // impl<AccountId: Ord, Balance> PartialEq for ValidatorSnapshot<AccountId, Balance> {
    // 	fn eq(&self, other: &Self) -> bool {
    // 		self.nominators == other.nominators
    // 	}
    // }

    // impl<AccountId: Ord, Balance> Ord for ValidatorSnapshot<AccountId, Balance> {
    // 	fn cmp(&self, other: &Self) -> Ordering {
    // 		self.owner.cmp(&other.owner)
    // 	}
    // }

    // impl<AccountId: Ord, Balance> PartialOrd for ValidatorSnapshot<AccountId, Balance> {
    // 	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // 		Some(self.cmp(other))
    // 	}
    // }

    #[derive(Encode, Decode, RuntimeDebug)]
    pub struct Nominator<AccountId, Balance> {
        pub nominations: OrderedSet<Bond<AccountId, Balance>>,
        pub total: Balance,
    }

    impl<
            AccountId: Ord + Clone,
            Balance: Copy
                + sp_std::ops::AddAssign
                + sp_std::ops::Add<Output = Balance>
                + sp_std::ops::SubAssign
                + PartialOrd,
        > Nominator<AccountId, Balance>
    {
        pub fn new(validator: AccountId, amount: Balance) -> Self {
            Nominator {
                nominations: OrderedSet::from(vec![Bond {
                    owner: validator,
                    amount,
                }]),
                total: amount,
            }
        }
        pub fn add_nomination(&mut self, bond: Bond<AccountId, Balance>) -> bool {
            let amt = bond.amount;
            if self.nominations.insert(bond) {
                self.total += amt;
                true
            } else {
                false
            }
        }
        // Returns Some(remaining balance), must be more than MinNominatorStake
        // Returns None if nomination not found
        pub fn rm_nomination(&mut self, validator: AccountId) -> Option<Balance> {
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
                self.total -= balance;
                Some(self.total)
            } else {
                None
            }
        }
        // Returns None if nomination not found
        pub fn inc_nomination(&mut self, validator: AccountId, more: Balance) -> Option<Balance> {
            for x in &mut self.nominations.0 {
                if x.owner == validator {
                    x.amount += more;
                    self.total += more;
                    return Some(x.amount);
                }
            }
            None
        }
        // Returns Some(Some(balance)) if successful
        // None if nomination not found
        // Some(None) if underflow
        pub fn dec_nomination(
            &mut self,
            validator: AccountId,
            less: Balance,
        ) -> Option<Option<Balance>> {
            for x in &mut self.nominations.0 {
                if x.owner == validator {
                    if x.amount > less {
                        x.amount -= less;
                        self.total -= less;
                        return Some(Some(x.amount));
                    } else {
                        // underflow error; should rm entire nomination if x.amount == validator
                        return Some(None);
                    }
                }
            }
            None
        }
    }

    impl<AccountId, Balance> Nominator<AccountId, Balance>
    where
        AccountId: Ord + Clone,
        Balance: AtLeast32BitUnsigned + Saturating + Copy,
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
            value: Balance,
            minimum_balance: Balance,
        ) -> Balance {
            let pre_total = self.total;
            let total = &mut self.total;

            let slash_out_of =
                |total_remaining: &mut Balance, target: &mut Balance, value: &Balance| {
                    let mut slash_from_target = (*value).min(*target);

                    if !slash_from_target.is_zero() {
                        *target -= slash_from_target;

                        // Make sure not drop below ED
                        if *target <= minimum_balance {
                            let diff_val = minimum_balance.saturating_sub(*target);
                            *target += diff_val;
                            slash_from_target -= diff_val;
                        }
                        *total_remaining = total_remaining.saturating_sub(slash_from_target);
                    }
                };

            for x in &mut self.nominations.0 {
                if x.owner == validator {
                    slash_out_of(total, &mut x.amount, &value);
                }
            }
            pre_total.saturating_sub(*total)
        }
    }

    /// A pending slash record. The value of the slash has been computed but not applied yet,
    /// rather deferred for several eras.
    #[derive(Encode, Decode, Default, RuntimeDebug, Clone)]
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

    type RewardPoint = u32;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::PositiveImbalance;

    pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The currency type
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// Handler for the unbalanced reduction when slashing a staker.
        type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
        /// Handler for the unbalanced increment when rewarding a staker.
        type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
        /// Number of sessions that staked fund remain bonded for
        type BondedDuration: Get<SessionIndex>;
        /// Number of sessions that slashes are deferred by, after computation.
        type SlashDeferDuration: Get<SessionIndex>;
        /// The origin which can cancel a deferred slash. Root can always do this.
        type SlashCancelOrigin: EnsureOrigin<Self::Origin>;
        /// Minimum number of selected validators every round
        type MinSelectedValidators: Get<u32>;
        /// Maximum nominators per validator
        type MaxNominatorsPerValidator: Get<u32>;
        /// Maximum validators per nominator
        type MaxValidatorPerNominator: Get<u32>;
        /// Commission due to validators, set at genesis
        type DefaultValidatorCommission: Get<Perbill>;
        /// Default Slash reward, set at genesis
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
        type SessionInterface: self::SessionInterface<Self::AccountId>;
        /// This pallet's module id. Used to derivate a dedicated account id to store session
        /// rewards for validators and nominators in.
        type PalletId: Get<ModuleId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(crate) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the validators who cannot be slashed (if any).
        ///
        /// The dispatch origin must be Root.
        #[pallet::weight(10_000)]
        pub fn set_invulnerables(
            origin: OriginFor<T>,
            invulnerables: Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <Invulnerables<T>>::put(&invulnerables);
            Self::deposit_event(Event::NewInvulnerables(invulnerables));
            Ok(().into())
        }
        #[pallet::weight(10_000)]
        /// Set the total number of validator selected per round
        /// - changes are not applied until the start of the next round
        pub fn set_total_validator_per_round(
            origin: OriginFor<T>,
            new: u32,
        ) -> DispatchResultWithPostInfo {
            frame_system::ensure_root(origin)?;
            ensure!(
                new >= T::MinSelectedValidators::get(),
                Error::<T>::CannotSetBelowMin
            );
            let old = <TotalSelected<T>>::get();
            <TotalSelected<T>>::put(new);
            Self::deposit_event(Event::TotalSelectedSet(old, new));
            Ok(().into())
        }
        #[pallet::weight(10_000)]
        /// Set the commission for all validators
        pub fn set_validator_commission(
            origin: OriginFor<T>,
            pct: Perbill,
        ) -> DispatchResultWithPostInfo {
            frame_system::ensure_root(origin)?;
            let old = <ValidatorCommission<T>>::get();
            <ValidatorCommission<T>>::put(pct);
            Self::deposit_event(Event::ValidatorCommissionSet(old, pct));
            Ok(().into())
        }
        /// Join the set of validators pool
        #[pallet::weight(10_000)]
        pub fn validator_join_pool(
            origin: OriginFor<T>,
            bond: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;
            ensure!(!Self::is_validator(&acc), Error::<T>::ValidatorExists);
            ensure!(!Self::is_nominator(&acc), Error::<T>::NominatorExists);
            ensure!(
                bond >= T::MinValidatorPoolStake::get(),
                Error::<T>::ValidatorBondBelowMin
            );
            let mut validators = <ValidatorPool<T>>::get();
            ensure!(
                validators.insert(Bond {
                    owner: acc.clone(),
                    amount: bond
                }),
                Error::<T>::ValidatorExists
            );
            T::Currency::reserve(&acc, bond)?;
            let validator = Validator::new(acc.clone(), bond);

            <Total<T>>::mutate(|x| *x += bond);
            <ValidatorState<T>>::insert(&acc, validator);
            <ValidatorPool<T>>::put(validators);
            Self::deposit_event(Event::JoinedValidatorPool(acc, bond, Self::total()));
            Ok(().into())
        }
        /// Request to exit the validators pool. If successful,
        /// the account is immediately removed from the validator pool
        /// to prevent selection as a validator, but unbonding
        /// is executed with a delay of `BondedDuration` rounds.
        #[pallet::weight(10_000)]
        pub fn exit_validators_pool(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;

            ensure!(Self::is_validator(&validator), Error::<T>::ValidatorDNE);

            ensure!(
                !Self::validator_state(&validator).unwrap().is_leaving(),
                Error::<T>::AlreadyLeaving,
            );

            let now = Self::active_session();
            let when = now + T::BondedDuration::get();

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
        #[pallet::weight(10_000)]
        pub fn validator_bond_more(
            origin: OriginFor<T>,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;

            ensure!(Self::is_validator(&validator), Error::<T>::ValidatorDNE);

            ensure!(
                !Self::validator_state(&validator).unwrap().is_leaving(),
                Error::<T>::CannotActivateIfLeaving,
            );

            T::Currency::reserve(&validator, more)?;

            <ValidatorState<T>>::mutate(validator.clone(), |maybe_validator| {
                if let Some(state) = maybe_validator {
                    let before = state.bond;
                    state.bond_more(more);
                    let after = state.bond;
                    state.go_online();
                    if state.is_active() {
                        Self::update_validators_pool(validator.clone(), state.total);
                    }
                    <Total<T>>::mutate(|x| *x += more);
                    Self::deposit_event(Event::ValidatorBondedMore(validator, before, after));
                }
            });

            Ok(().into())
        }
        /// Bond less for validator
        #[pallet::weight(0)]
        pub fn validator_bond_less(
            origin: OriginFor<T>,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;

            ensure!(Self::is_validator(&validator), Error::<T>::ValidatorDNE);

            if let Some(mut state) = Self::validator_state(&validator) {
                ensure!(!state.is_leaving(), Error::<T>::CannotActivateIfLeaving);

                let before = state.bond;
                let after = state.bond_less(less).ok_or(Error::<T>::Underflow)?;
                ensure!(
                    after >= T::MinValidatorPoolStake::get(),
                    Error::<T>::ValidatorBondBelowMin
                );

                T::Currency::unreserve(&validator, less);

                if state.is_active() {
                    Self::update_validators_pool(validator.clone(), state.total);
                }
                <Total<T>>::mutate(|x| *x -= less);
                <ValidatorState<T>>::insert(&validator, state);
                Self::deposit_event(Event::ValidatorBondedLess(validator, before, after));
            }
            Ok(().into())
        }
        /// If caller is not a nominator, then join the set of nominators
        /// If caller is a nominator, then makes nomination to change their nomination state
        #[pallet::weight(10_000)]
        pub fn nominator_nominate(
            origin: OriginFor<T>,
            validator: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator_acc = ensure_signed(origin)?;

            // cannot be a validator candidate and nominator with same AccountId
            ensure!(
                !Self::is_validator(&nominator_acc),
                Error::<T>::ValidatorExists
            );

            ensure!(
                amount >= T::MinNomination::get(),
                Error::<T>::NominationBelowMin
            );

            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;

            let mut do_add_nomination = true;
            let mut nominator_state = if Self::is_nominator(&nominator_acc) {
                Self::nominator_state(&nominator_acc).ok_or(Error::<T>::NominatorDNE)?
            } else {
                do_add_nomination = false;
                Nominator::new(validator.clone(), amount)
            };

            ensure!(
                (nominator_state.nominations.0.len() as u32) < T::MaxValidatorPerNominator::get(),
                Error::<T>::ExceedMaxValidatorPerNom,
            );

            ensure!(
                (validator_state.nominators.0.len() as u32) < T::MaxNominatorsPerValidator::get(),
                Error::<T>::TooManyNominators,
            );

            if do_add_nomination {
                ensure!(
                    nominator_state.add_nomination(Bond {
                        owner: validator.clone(),
                        amount
                    }),
                    Error::<T>::AlreadyNominatedValidator,
                );
            }

            let nomination = Bond {
                owner: nominator_acc.clone(),
                amount,
            };
            ensure!(
                validator_state.nominators.insert(nomination),
                Error::<T>::NominatorExists,
            );

            T::Currency::reserve(&nominator_acc, amount)?;
            validator_state.total += amount;
            let validator_new_total = validator_state.total;
            if validator_state.is_active() {
                Self::update_validators_pool(validator.clone(), validator_state.total);
            }

            <Total<T>>::mutate(|x| *x += amount);
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
        #[pallet::weight(10_000)]
        pub fn nominator_denominate(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            Self::nominator_revokes_validator(ensure_signed(origin)?, validator)
        }
        /// Quit the set of nominators and, by implication, revoke all ongoing nominations
        #[pallet::weight(0)]
        pub fn nominator_denominate_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;

            let nominator = <NominatorState<T>>::get(&acc).ok_or(Error::<T>::NominatorDNE)?;
            for bond in nominator.nominations.0 {
                Self::nominator_leaves_validator(acc.clone(), bond.owner.clone())?;
            }
            <NominatorState<T>>::remove(&acc);
            Self::deposit_event(Event::NominatorLeft(acc, nominator.total));
            Ok(().into())
        }
        /// Bond more for nominators with respect to a specific validator
        #[pallet::weight(10_000)]
        pub fn nominator_bond_more(
            origin: OriginFor<T>,
            validator: T::AccountId,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator = ensure_signed(origin)?;
            let mut nominations =
                <NominatorState<T>>::get(&nominator).ok_or(Error::<T>::NominatorDNE)?;
            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            let _ = nominations
                .inc_nomination(validator.clone(), more)
                .ok_or(Error::<T>::NominationDNE)?;
            T::Currency::reserve(&nominator, more)?;
            let before = validator_state.total;
            validator_state.inc_nominator(nominator.clone(), more);
            let after = validator_state.total;
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
        #[pallet::weight(10_000)]
        pub fn nominator_bond_less(
            origin: OriginFor<T>,
            validator: T::AccountId,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator = ensure_signed(origin)?;
            let mut nominations =
                <NominatorState<T>>::get(&nominator).ok_or(Error::<T>::NominatorDNE)?;
            let remaining = nominations
                .dec_nomination(validator.clone(), less)
                .ok_or(Error::<T>::NominationDNE)?
                .ok_or(Error::<T>::Underflow)?;
            ensure!(
                remaining >= T::MinNomination::get(),
                Error::<T>::NominationBelowMin
            );
            ensure!(
                nominations.total >= T::MinNominatorStake::get(),
                Error::<T>::NominatorBondBelowMin
            );
            let mut validator_state =
                <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            T::Currency::unreserve(&nominator, less);
            let before = validator_state.total;
            validator_state.dec_nominator(nominator.clone(), less);
            let after = validator_state.total;
            if validator_state.is_active() {
                Self::update_validators_pool(validator.clone(), validator_state.total);
            }
            <ValidatorState<T>>::insert(&validator, validator_state);
            <NominatorState<T>>::insert(&nominator, nominations);
            Self::deposit_event(Event::NominationDecreased(
                nominator, validator, before, after,
            ));
            Ok(().into())
        }
        /// Cancel enactment of a deferred slash.
        ///
        /// Can be called by the `T::SlashCancelOrigin`.
        ///
        /// Parameters: session index and validator list of the slashes for that session to kill.
        #[pallet::weight(10_000)]
        pub fn slash_cancel_deferred(
            origin: OriginFor<T>,
            session_idx: SessionIndex,
            controllers: Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            T::SlashCancelOrigin::ensure_origin(origin)?;

            let apply_at = session_idx.saturating_add(T::SlashDeferDuration::get());

            ensure!(!controllers.is_empty(), Error::<T>::EmptyTargets);
            ensure!(
                <UnappliedSlashes<T>>::contains_key(apply_at),
                Error::<T>::InvalidSessionIndex
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
        ValidatorDNE,
        NominatorDNE,
        NominatorExists,
        ValidatorExists,
        ValidatorBondBelowMin,
        NominationBelowMin,
        NominatorBondBelowMin,
        AlreadyOffline,
        AlreadyActive,
        AlreadyLeaving,
        CannotActivateIfLeaving,
        TooManyNominators,
        ExceedMaxValidatorPerNom,
        AlreadyNominatedValidator,
        NominationDNE,
        Underflow,
        CannotSetBelowMin,
        IncorrectSlashingSpans,
        EmptyTargets,
        InvalidSessionIndex,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    pub enum Event<T: Config> {
        NewInvulnerables(Vec<T::AccountId>),
        /// Starting Block, Round, Number of Validators Selected, Total Balance
        NewSession(T::BlockNumber, SessionIndex, u32, BalanceOf<T>),
        /// Account, Amount Locked, New Total Amt Locked
        JoinedValidatorPool(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Round, Validator Account, Total Exposed Amount (includes all nominations)
        ValidatorChosen(SessionIndex, T::AccountId, BalanceOf<T>),
        /// Validator Account, Old Bond, New Bond
        ValidatorBondedMore(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Validator Account, Old Bond, New Bond
        ValidatorBondedLess(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        ValidatorDeActive(SessionIndex, T::AccountId),
        ValidatorActive(SessionIndex, T::AccountId),
        /// Round, Validator Account, Scheduled Exit
        ValidatorScheduledExit(SessionIndex, T::AccountId, SessionIndex),
        /// Account, Amount Unlocked, New Total Amt Locked
        ValidatorLeft(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        // Nominator, Validator, Old Nomination, New Nomination
        NominationIncreased(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        // Nominator, Validator, Old Nomination, New Nomination
        NominationDecreased(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Nominator, Amount Unstaked
        NominatorLeft(T::AccountId, BalanceOf<T>),
        /// Nominator, Amount Locked, Validator, New Total Amt backing Validator
        Nomination(T::AccountId, BalanceOf<T>, T::AccountId, BalanceOf<T>),
        /// Nominator, Validator, Amount Unstaked, New Total Amt Staked for Validator
        NominatorLeftValidator(T::AccountId, T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Paid the account (nominator or validator) the balance as liquid rewards
        Rewarded(T::AccountId, BalanceOf<T>),
        /// Round inflation range set with the provided annual inflation range
        RoundInflationSet(Perbill, Perbill, Perbill),
        /// Staking expectations set
        StakeExpectationsSet(BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
        /// Set total selected validators to this value [old, new]
        TotalSelectedSet(u32, u32),
        /// Set validator commission to this value [old, new]
        ValidatorCommissionSet(Perbill, Perbill),
        /// Set blocks per round [current_round, first_block, old, new]
        BlocksPerRoundSet(SessionIndex, T::BlockNumber, u32, u32),
        /// One validator (and its nominators) has been slashed by the given amount.
        /// \[validator, amount\]
        Slash(T::AccountId, BalanceOf<T>),
        /// An old slashing report was discarded because it could
        /// not be processed. \[session_index\]
        OldSlashingReportDiscarded(SessionIndex),
        PayReporterReward(T::AccountId, BalanceOf<T>),
        DeferredUnappliedSlash(SessionIndex, T::AccountId),
    }

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    pub(crate) type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total)]
    /// Total capital locked by this staking pallet
    pub(crate) type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_session)]
    /// Current session index
    pub(crate) type ActiveSession<T: Config> = StorageValue<_, SessionIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_commission)]
    /// Commission percent taken off of rewards for all validators
    pub(crate) type ValidatorCommission<T: Config> = StorageValue<_, Perbill, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_state)]
    /// Get validator state associated with an account if account is collating else None
    pub(crate) type ValidatorState<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Validator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn nominator_state)]
    /// Get nominator state associated with an account if account is nominating else None
    pub(crate) type NominatorState<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Nominator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn total_selected)]
    /// The total validators selected every round
    pub(crate) type TotalSelected<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn selected_validators)]
    /// The validators selected for the current round
    pub(crate) type SelectedValidators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_pool)]
    /// The pool of validator validators, each with their total backing stake
    pub(crate) type ValidatorPool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn exit_queue)]
    /// A queue of validators awaiting exit `BondedDuration` delay after request
    pub(crate) type ExitQueue<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, SessionIndex>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn at_stake)]
    /// Snapshot of validator nomination stake at the start of the round
    pub(crate) type AtStake<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::AccountId,
        ValidatorSnapshot<T::AccountId, BalanceOf<T>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn staked)]
    /// Total backing stake for selected validators in the round
    pub(crate) type Staked<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn session_accumulated_balance)]
    /// Accumulated balances for the last Session Round
    pub(crate) type SessionAccumulatedBalance<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn session_validator_reward)]
    /// Validator reward for the Session
    pub(crate) type SessionValidatorReward<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn points)]
    /// Total points awarded to validator for block production in the round
    pub(crate) type Points<T: Config> =
        StorageMap<_, Twox64Concat, SessionIndex, RewardPoint, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn awarded_pts)]
    /// Points for each validator per round
    pub(crate) type AwardedPts<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SessionIndex,
        Twox64Concat,
        T::AccountId,
        RewardPoint,
        ValueQuery,
    >;

    /// The percentage of the slash that is distributed to reporters.
    ///
    /// The rest of the slashed value is handled by the `Slash`.
    #[pallet::storage]
    #[pallet::getter(fn slash_reward_fraction)]
    /// Commission percent taken off of rewards for all validators
    pub(crate) type SlashRewardFraction<T: Config> = StorageValue<_, Perbill, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn slash_state)]
    // /// Snapshot of validator slash state
    // pub(crate) type SlashState<T: Config> = StorageDoubleMap<
    // 	_,
    // 	Twox64Concat,
    // 	SessionIndex,
    // 	Twox64Concat,
    // 	T::AccountId,
    // 	slashing::SpanRecord<BalanceOf<T>>,
    // 	OptionQuery,
    // >;

    #[pallet::storage]
    #[pallet::getter(fn slashing_spans)]
    /// Snapshot of validator slash state
    pub(crate) type SlashingSpans<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, slashing::SlashingSpans, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn span_slash)]
    /// Snapshot of validator slash state
    pub(crate) type SpanSlash<T: Config> = StorageMap<
        _,
        Twox64Concat,
        (T::AccountId, slashing::SpanIndex),
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
    /// Total backing stake for selected validators in the round
    pub(crate) type UnappliedSlashes<T: Config> = StorageMap<
        _,
        Twox64Concat,
        SessionIndex,
        Vec<UnappliedSlash<T::AccountId, BalanceOf<T>>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn bonded_sessions)]
    /// A mapping of still-bonded sessions
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

            // TODO :: Have to recheck validity of this fix.
            let imbalance = T::Currency::issue(T::Currency::minimum_balance());
            T::Currency::resolve_creating(&T::PalletId::get().into_account(), imbalance);

            // Set collator commission to default config
            <ValidatorCommission<T>>::put(T::DefaultValidatorCommission::get());
            // Set total selected validators to minimum config
            <TotalSelected<T>>::put(T::MinSelectedValidators::get());
            // Set default slash reward fraction
            <SlashRewardFraction<T>>::put(T::DefaultSlashRewardFraction::get());
            // Choose top TotalSelected validators
            let (v_count, total_staked) = <Pallet<T>>::select_session_validators(1u32);
            // Start Session 1
            <ActiveSession<T>>::put(1u32);
            // Snapshot total stake
            <Staked<T>>::insert(1u32, <Total<T>>::get());
            <Pallet<T>>::deposit_event(Event::NewSession(
                T::BlockNumber::zero(),
                1u32,
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
        #[allow(dead_code)]
        pub(crate) fn is_validator(acc: &T::AccountId) -> bool {
            <ValidatorState<T>>::get(acc).is_some()
        }

        #[allow(dead_code)]
        pub(crate) fn is_selected_validator(acc: &T::AccountId) -> bool {
            <SelectedValidators<T>>::get().binary_search(acc).is_ok()
        }

        pub(crate) fn is_nominator(acc: &T::AccountId) -> bool {
            <NominatorState<T>>::get(acc).is_some()
        }
        // ensure validator is active before calling
        pub fn update_validators_pool(validator: T::AccountId, total: BalanceOf<T>) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] | Own[{:#?}]",
                function!(),
                line!(),
                validator
            );
            let mut validators = <ValidatorPool<T>>::get();
            validators.remove(&Bond::from_owner(validator.clone()));
            validators.insert(Bond {
                owner: validator,
                amount: total,
            });
            <ValidatorPool<T>>::put(validators);
        }
        // ensure validator is active before calling
        pub fn remove_from_validators_pool(validator: T::AccountId) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] | Own[{:#?}]",
                function!(),
                line!(),
                validator
            );
            let mut validators = <ValidatorPool<T>>::get();
            validators.remove(&Bond::from_owner(validator.clone()));
            <ValidatorPool<T>>::put(validators);
        }

        pub(crate) fn validator_deactivate(controller: &T::AccountId) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Acc[{:#?}]",
                function!(),
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
            let mut state = <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            let mut exists: Option<BalanceOf<T>> = None;
            let noms = state
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
            let nominator_stake = exists.ok_or(Error::<T>::ValidatorDNE)?;
            let nominators = OrderedSet::from(noms);
            T::Currency::unreserve(&nominator, nominator_stake);
            state.nominators = nominators;
            state.total -= nominator_stake;
            if state.is_active() {
                Self::update_validators_pool(validator.clone(), state.total);
            }
            let new_total_locked = <Total<T>>::get() - nominator_stake;
            <Total<T>>::put(new_total_locked);
            let new_total = state.total;
            <ValidatorState<T>>::insert(&validator, state);
            Self::deposit_event(Event::NominatorLeftValidator(
                nominator,
                validator,
                nominator_stake,
                new_total,
            ));
            Ok(().into())
        }

        fn nominator_revokes_validator(
            acc: T::AccountId,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let mut nominator = <NominatorState<T>>::get(&acc).ok_or(Error::<T>::NominatorDNE)?;
            let old_total = nominator.total;
            let remaining = nominator
                .rm_nomination(validator.clone())
                .ok_or(Error::<T>::NominationDNE)?;

            ensure!(
                remaining >= T::MinNominatorStake::get(),
                Error::<T>::NominatorBondBelowMin
            );
            Self::nominator_leaves_validator(acc.clone(), validator)?;

            // edge case; if no nominations remaining, leave set of nominators
            if nominator.nominations.0.len().is_zero() {
                <NominatorState<T>>::remove(&acc);
                Self::deposit_event(Event::NominatorLeft(acc, old_total));
            } else {
                <NominatorState<T>>::insert(&acc, nominator);
            }

            Ok(().into())
        }

        // // Calculate round issuance based on total staked for the given round
        // fn compute_issuance(staked: BalanceOf<T>) -> BalanceOf<T> {
        //     // TODO :: issuance model ideal for nodle
        //     return staked;
        // }

        fn pay_stakers(next: SessionIndex) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Sess-idx[{:#?}]",
                function!(),
                line!(),
                next
            );

            let mint = |amt: BalanceOf<T>, to: T::AccountId| {
                if amt > T::Currency::minimum_balance() {
                    if let Ok(imb) = T::Currency::deposit_into_existing(&to, amt) {
                        Self::deposit_event(Event::Rewarded(to.clone(), imb.peek()));
                    }
                }
            };

            let validator_fee = <ValidatorCommission<T>>::get();
            let total = <Points<T>>::get(next);
            // let total_staked = <Staked<T>>::get(next);
            // let issuance = Self::compute_issuance(total_staked);
            let issuance = Self::session_validator_reward(next);
            for (val, pts) in <AwardedPts<T>>::drain_prefix(next) {
                let pct_due = Perbill::from_rational_approximation(pts, total);
                let mut amt_due = pct_due * issuance;

                log!(
                    trace,
                    "[{:#?}]:[{:#?}] - L1 [{:#?}] | [{:#?}] | [{:#?}]",
                    function!(),
                    line!(),
                    total,
                    issuance,
                    pct_due
                );

                log!(
                    trace,
                    "[{:#?}]:[{:#?}] - L2 [{:#?}] | [{:#?}] | [{:#?}]",
                    function!(),
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
                    log!(
                        trace,
                        "[{:#?}]:[{:#?}] - L3 Solo Mode",
                        function!(),
                        line!()
                    );
                } else {
                    // pay collator first; commission + due_portion
                    let val_pct = Perbill::from_rational_approximation(state.bond, state.total);
                    let commission = validator_fee * amt_due;
                    let val_due = if commission > T::Currency::minimum_balance() {
                        amt_due -= commission;
                        (val_pct * amt_due) + commission
                    } else {
                        // commission is negligible so not applied
                        val_pct * amt_due
                    };

                    log!(
                        trace,
                        "[{:#?}]:[{:#?}] - L4 [{:#?}] | [{:#?}] | [{:#?}]",
                        function!(),
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
        fn execute_delayed_validator_exits(next: SessionIndex) {
            let remain_exits = <ExitQueue<T>>::get()
                .0
                .into_iter()
                .filter_map(|x| {
                    if x.amount > next {
                        Some(x)
                    } else {
                        if let Some(state) = <ValidatorState<T>>::get(&x.owner) {
                            for bond in state.nominators.0 {
                                // return stake to nominator
                                T::Currency::unreserve(&bond.owner, bond.amount);
                                // remove nomination from nominator state
                                if let Some(mut nominator) = <NominatorState<T>>::get(&bond.owner) {
                                    if let Some(remaining) =
                                        nominator.rm_nomination(x.owner.clone())
                                    {
                                        if remaining.is_zero() {
                                            <NominatorState<T>>::remove(&bond.owner);
                                        } else {
                                            <NominatorState<T>>::insert(&bond.owner, nominator);
                                        }
                                    }
                                }
                            }
                            // return stake to collator
                            T::Currency::unreserve(&state.id, state.bond);
                            let new_total = <Total<T>>::get() - state.total;
                            <Total<T>>::put(new_total);
                            <ValidatorState<T>>::remove(&x.owner);
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
        fn select_session_validators(next: SessionIndex) -> (u32, BalanceOf<T>) {
            let (mut validators_count, mut total) = (0u32, BalanceOf::<T>::zero());
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
            // snapshot exposure for round for weighting reward distribution
            for account in top_validators.iter() {
                let state = <ValidatorState<T>>::get(&account)
                    .expect("all members of ValidatorQ must be validators");
                let amount = state.total;
                let exposure: ValidatorSnapshot<T::AccountId, BalanceOf<T>> = state.into();
                <AtStake<T>>::insert(next, account, exposure);
                validators_count += 1u32;
                total += amount;
                Self::deposit_event(Event::ValidatorChosen(next, account.clone(), amount));
            }
            top_validators.sort();
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
                let score_points = <AwardedPts<T>>::get(now, &validator) + points;
                <AwardedPts<T>>::insert(now, validator, score_points);
                <Points<T>>::mutate(now, |x| *x += points);
            }
        }
        /// Clear session information for given session index
        fn clear_session_information(session_idx: SessionIndex) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - AccuBal[{:#?}]",
                function!(),
                line!(),
                <SessionAccumulatedBalance<T>>::get(session_idx),
            );

            <Staked<T>>::remove(session_idx);
            <Points<T>>::remove(session_idx);
            <AtStake<T>>::remove_prefix(session_idx);
            <AwardedPts<T>>::remove_prefix(session_idx);

            // withdraw rewards
            match T::Currency::withdraw(
                &T::PalletId::get().into_account(),
                <SessionAccumulatedBalance<T>>::take(session_idx),
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            ) {
                Ok(imbalance) => T::RewardRemainder::on_unbalanced(imbalance),
                Err(err) => {
                    log!(
                        error,
                        "[{:#?}]:[{:#?}] - [{:#?}] | [{:#?}]",
                        function!(),
                        line!(),
                        err,
                        "Warning: an error happened when trying to handle active session rewards \
						 remainder",
                    );
                }
            };
        }
        /// Apply previously-unapplied slashes on the beginning of a new session, after a delay.
        fn apply_unapplied_slashes(active_session: SessionIndex) {
            if <UnappliedSlashes<T>>::contains_key(active_session) {
                let session_slashes = <UnappliedSlashes<T>>::take(&active_session);
                for unapplied_slash in session_slashes {
                    slashing::apply_slash::<T>(unapplied_slash);
                }
            }
        }
    }

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
            SessionAccumulatedBalance::<T>::mutate(now, |v: &mut BalanceOf<T>| {
                *v = v.saturating_add(imbalance.peek())
            });
            T::Currency::resolve_creating(&T::PalletId::get().into_account(), imbalance);
        }
    }

    /// Add reward points to block authors:
    /// * 20 points to the block producer for producing a (non-uncle) block in the relay chain,
    /// * 2 points to the block producer for each reference to a previously unreferenced uncle, and
    /// * 1 point to the producer of each referenced uncle block.
    impl<T> pallet_authorship::EventHandler<T::AccountId, T::BlockNumber> for Pallet<T>
    where
        T: Config + pallet_authorship::Config + pallet_session::Config,
    {
        fn note_author(author: T::AccountId) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Author[{:#?}]",
                function!(),
                line!(),
                author
            );
            Self::reward_by_ids(vec![(author, 20)])
        }
        fn note_uncle(author: T::AccountId, _age: T::BlockNumber) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Author[{:#?}]",
                function!(),
                line!(),
                author
            );
            Self::reward_by_ids(vec![
                (<pallet_authorship::Module<T>>::author(), 2),
                (author, 1),
            ])
        }
    }
    /// In this implementation `new_session(session)` must be called before `end_session(session-1)`
    /// i.e. the new session must be planned before the ending of the previous session.
    ///
    /// Once the first new_session is planned, all session must start and then end in order.
    impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
        fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Sess-idx[{:#?}]",
                function!(),
                line!(),
                new_index
            );

            let current_block_number = system::Pallet::<T>::block_number();

            // select top collator validators for next round
            let (validator_count, total_staked) = Self::select_session_validators(new_index);

            // snapshot total stake
            <Staked<T>>::insert(new_index, <Total<T>>::get());

            Self::deposit_event(Event::NewSession(
                current_block_number,
                new_index,
                validator_count,
                total_staked,
            ));

            log!(
                debug,
                "[{:#?}]:[{:#?}] - Event::NewSession(B[{}],SI[{}],VC[{}],TS[{:#?}])",
                function!(),
                line!(),
                current_block_number,
                new_index,
                validator_count,
                total_staked,
            );

            Some(Self::selected_validators())
        }
        fn start_session(start_index: SessionIndex) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Sess-idx[{:#?}]",
                function!(),
                line!(),
                start_index
            );

            <ActiveSession<T>>::put(start_index);

            let bonding_duration = T::BondedDuration::get();

            <BondedSessions<T>>::mutate(|bonded| {
                bonded.push(start_index);

                if start_index > bonding_duration {
                    let first_kept = start_index - bonding_duration;

                    // prune out everything that's from before the first-kept index.
                    let n_to_prune = bonded
                        .iter()
                        .take_while(|&&session_idx| session_idx < first_kept)
                        .count();

                    for prune_session in bonded.drain(..n_to_prune) {
                        // Clear the DB cached state of last session
                        Self::clear_session_information(prune_session);

                        // TODO :: kill slashing metadata.
                        // slashing::clear_era_metadata::<T>(pruned_era);
                    }

                    if let Some(&first_session) = bonded.first() {
                        T::SessionInterface::prune_historical_up_to(first_session);
                    }
                }
            });

            // execute all delayed validator exits
            Self::execute_delayed_validator_exits(start_index);

            // Handle the unapplied deferd slashes
            Self::apply_unapplied_slashes(start_index);
        }
        fn end_session(end_index: SessionIndex) {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Sess-idx[{:#?}]",
                function!(),
                line!(),
                end_index
            );

            if Self::active_session() == end_index {
                let payout = Self::session_accumulated_balance(end_index);

                // Set ending session reward.
                <SessionValidatorReward<T>>::insert(&end_index, payout);

                // pay all stakers for T::BondedDuration rounds ago
                Self::pay_stakers(end_index);

            // // Clear the DB cached state of last session
            // Self::clear_session_information(Self::active_session());
            } else {
                log!(
                    error,
                    "[{:#?}]:[{:#?}] - Something wrong (CSI[{}], ESI[{}])",
                    function!(),
                    line!(),
                    Self::active_session(),
                    end_index,
                );
            }
        }
    }

    impl<T: Config>
        historical::SessionManager<T::AccountId, ValidatorSnapshot<T::AccountId, BalanceOf<T>>>
        for Pallet<T>
    {
        fn new_session(
            new_index: SessionIndex,
        ) -> Option<Vec<(T::AccountId, ValidatorSnapshot<T::AccountId, BalanceOf<T>>)>> {
            <Self as pallet_session::SessionManager<_>>::new_session(new_index).map(|validators| {
                validators
                    .into_iter()
                    .map(|v| {
                        let validator_inst = Self::at_stake(new_index, &v);
                        (v, validator_inst)
                    })
                    .collect()
            })
        }
        fn start_session(start_index: SessionIndex) {
            <Self as pallet_session::SessionManager<_>>::start_session(start_index)
        }
        fn end_session(end_index: SessionIndex) {
            <Self as pallet_session::SessionManager<_>>::end_session(end_index)
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
        fn convert(
            validator: T::AccountId,
        ) -> Option<ValidatorSnapshot<T::AccountId, BalanceOf<T>>> {
            let now = <ActiveSession<T>>::get();
            if <AtStake<T>>::contains_key(now, &validator) {
                Some(<Pallet<T>>::at_stake(now, &validator))
            } else {
                None
            }
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
        fn disable_validator(validator: &AccountId) -> Result<bool, ()>;
        /// Get the validators from session.
        fn validators() -> Vec<AccountId>;
        /// Prune historical session tries up to but not including the given index.
        fn prune_historical_up_to(up_to: SessionIndex);
    }

    impl<T: Config> SessionInterface<<T as frame_system::Config>::AccountId> for T
    where
        T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
        T: pallet_session::historical::Config<
            FullIdentification = ValidatorSnapshot<
                <T as frame_system::Config>::AccountId,
                BalanceOf<T>,
            >,
            FullIdentificationOf = ValidatorSnapshotOf<T>,
        >,
        T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
        T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
        T::ValidatorIdOf: Convert<
            <T as frame_system::Config>::AccountId,
            Option<<T as frame_system::Config>::AccountId>,
        >,
    {
        fn disable_validator(
            validator: &<T as frame_system::Config>::AccountId,
        ) -> Result<bool, ()> {
            <pallet_session::Module<T>>::disable(validator)
        }

        fn validators() -> Vec<<T as frame_system::Config>::AccountId> {
            <pallet_session::Module<T>>::validators()
        }

        fn prune_historical_up_to(up_to: SessionIndex) {
            <pallet_session::historical::Module<T>>::prune_up_to(up_to);
        }
    }

    /// This is intended to be used with `FilterHistoricalOffences`.
    impl<T: Config>
        OnOffenceHandler<T::AccountId, pallet_session::historical::IdentificationTuple<T>, Weight>
        for Pallet<T>
    where
        T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
        T: pallet_session::historical::Config<
            FullIdentification = ValidatorSnapshot<
                <T as frame_system::Config>::AccountId,
                BalanceOf<T>,
            >,
            FullIdentificationOf = ValidatorSnapshotOf<T>,
        >,
        T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
        T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
        T::ValidatorIdOf: Convert<
            <T as frame_system::Config>::AccountId,
            Option<<T as frame_system::Config>::AccountId>,
        >,
    {
        fn on_offence(
            offenders: &[OffenceDetails<
                T::AccountId,
                pallet_session::historical::IdentificationTuple<T>,
            >],
            slash_fraction: &[Perbill],
            slash_session: SessionIndex,
        ) -> Result<Weight, ()> {
            log!(
                trace,
                "[{:#?}]:[{:#?}] - Sess-idx [{:#?}] | Slash-Frac [{:#?}]",
                function!(),
                line!(),
                slash_session,
                slash_fraction,
            );

            let reward_proportion = <SlashRewardFraction<T>>::get();
            // let reward_proportion: Perbill = Perbill::from_percent(10);
            let mut consumed_weight: Weight = 0;
            let mut add_db_reads_writes = |reads, writes| {
                consumed_weight += T::DbWeight::get().reads_writes(reads, writes);
            };

            let active_session = Self::active_session();
            add_db_reads_writes(1, 0);

            let window_start = active_session.saturating_sub(T::BondedDuration::get());
            let slash_defer_duration = T::SlashDeferDuration::get();

            let invulnerables = Self::invulnerables();
            add_db_reads_writes(1, 0);

            log!(
                trace,
                "[{:#?}]:[{:#?}] - Invulnerables[{:#?}]",
                function!(),
                line!(),
                invulnerables,
            );

            for (details, slash_fraction) in offenders.iter().zip(slash_fraction) {
                let (controller, exposure) = &details.offender;

                // Skip if the validator is invulnerable.
                if invulnerables.contains(controller) {
                    continue;
                }

                let unapplied = slashing::compute_slash::<T>(slashing::SlashParams {
                    controller,
                    slash: *slash_fraction,
                    exposure,
                    slash_session,
                    window_start,
                    now: active_session,
                    reward_proportion,
                });

                if let Some(mut unapplied) = unapplied {
                    let nominators_len = unapplied.others.len() as u64;
                    let reporters_len = details.reporters.len() as u64;

                    {
                        let upper_bound = 1 /* Validator/NominatorSlashInEra */ + 2 /* fetch_spans */;
                        let rw = upper_bound + nominators_len * upper_bound;
                        add_db_reads_writes(rw, rw);
                    }
                    unapplied.reporters = details.reporters.clone();
                    if slash_defer_duration == 0 {
                        // apply right away.
                        slashing::apply_slash::<T>(unapplied);
                        {
                            let slash_cost = (6, 5);
                            let reward_cost = (2, 2);
                            add_db_reads_writes(
                                (1 + nominators_len) * slash_cost.0 + reward_cost.0 * reporters_len,
                                (1 + nominators_len) * slash_cost.1 + reward_cost.1 * reporters_len,
                            );
                        }
                    } else {
                        // defer to end of some `slash_defer_duration` from now.
                        let apply_at = active_session.saturating_add(slash_defer_duration);

                        <Self as Store>::UnappliedSlashes::mutate(apply_at, |for_later| {
                            for_later.push(unapplied.clone())
                        });

                        <Pallet<T>>::deposit_event(Event::DeferredUnappliedSlash(
                            active_session,
                            unapplied.validator,
                        ));

                        add_db_reads_writes(1, 1);
                    }
                } else {
                    log!(trace, "[{:#?}]:[{:#?}] - NOP", function!(), line!(),);
                    add_db_reads_writes(4 /* fetch_spans */, 5 /* kick_out_if_recent */);
                }
            }
            Ok(consumed_weight)
        }

        fn can_report() -> bool {
            true
        }
    }

    /// Filter historical offences out and only allow those from the bonding period.
    pub struct FilterHistoricalOffences<T, R> {
        _inner: sp_std::marker::PhantomData<(T, R)>,
    }

    impl<T, Reporter, Offender, R, O> ReportOffence<Reporter, Offender, O>
        for FilterHistoricalOffences<Module<T>, R>
    where
        T: Config,
        R: ReportOffence<Reporter, Offender, O>,
        O: Offence<Offender>,
    {
        fn report_offence(reporters: Vec<Reporter>, offence: O) -> Result<(), OffenceError> {
            // disallow any slashing from before the current bonding period.
            let offence_session = offence.session_index();
            let bonded_sessions = <BondedSessions<T>>::get();

            if bonded_sessions
                .first()
                .filter(|start| offence_session >= **start)
                .is_some()
            {
                R::report_offence(reporters, offence)
            } else {
                <Pallet<T>>::deposit_event(Event::OldSlashingReportDiscarded(offence_session));
                Ok(())
            }
        }

        fn is_known_offence(offenders: &[Offender], time_slot: &O::TimeSlot) -> bool {
            R::is_known_offence(offenders, time_slot)
        }
    }
}
