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

mod set;
use frame_support::pallet;

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;

pub use pallet::*;

#[pallet]
pub mod pallet {
    // use super::*;
    use crate::set::OrderedSet;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, Get, Imbalance, ReservableCurrency};
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::{Decode, Encode};
    use sp_runtime::{
        traits::{AtLeast32BitUnsigned, Zero},
        Perbill, RuntimeDebug,
    };
    use sp_std::cmp::Ordering;

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

    impl<AccountId: Ord, Balance> PartialEq for Bond<AccountId, Balance> {
        fn eq(&self, other: &Self) -> bool {
            self.owner == other.owner
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
    /// The activity status of the validator
    pub enum ValidatorStatus {
        /// Committed to be online and producing valid blocks
        Active,
        /// Temporarily inactive and excused for inactivity
        Idle,
        /// Bonded until the inner round
        Leaving(RoundIndex),
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
        pub fn leave_validators_pool(&mut self, round: RoundIndex) {
            self.state = ValidatorStatus::Leaving(round);
        }
    }

    #[derive(Default, Encode, Decode, RuntimeDebug)]
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

    #[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
    /// The current round index and transition information
    pub struct RoundInfo<BlockNumber> {
        /// Current round index
        pub current: RoundIndex,
        /// The first block of the current round
        pub first: BlockNumber,
        /// The length of the current round in number of blocks
        pub length: u32,
    }
    impl<
            B: Copy
                + sp_std::ops::Add<Output = B>
                + sp_std::ops::Sub<Output = B>
                + From<u32>
                + PartialOrd,
        > RoundInfo<B>
    {
        pub fn new(current: RoundIndex, first: B, length: u32) -> RoundInfo<B> {
            RoundInfo {
                current,
                first,
                length,
            }
        }
        /// Check if the round should be updated
        pub fn should_update(&self, now: B) -> bool {
            now - self.first >= self.length.into()
        }
        /// New round
        pub fn update(&mut self, now: B) {
            self.current += 1u32;
            self.first = now;
        }
    }
    impl<
            B: Copy
                + sp_std::ops::Add<Output = B>
                + sp_std::ops::Sub<Output = B>
                + From<u32>
                + PartialOrd,
        > Default for RoundInfo<B>
    {
        fn default() -> RoundInfo<B> {
            RoundInfo::new(1u32, 1u32.into(), 20u32.into())
        }
    }

    type RoundIndex = u32;
    type RewardPoint = u32;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The currency type
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// Minimum number of blocks per round
        type MinBlocksPerRound: Get<u32>;
        /// Default number of blocks per round at genesis
        type DefaultBlocksPerRound: Get<u32>;
        /// Number of rounds that validators remain bonded before exit request is executed
        type BondDuration: Get<RoundIndex>;
        /// Minimum number of selected validators every round
        type MinSelectedValidators: Get<u32>;
        /// Maximum nominators per validator
        type MaxNominatorsPerValidator: Get<u32>;
        /// Maximum validators per nominator
        type MaxValidatorPerNominator: Get<u32>;
        /// Commission due to validators, set at genesis
        type DefaultValidatorCommission: Get<Perbill>;
        /// Minimum stake required for any account to be in `SelectedCandidates` for the round
        type MinValidatorStake: Get<BalanceOf<Self>>;
        /// Minimum stake required for any account to be a validator candidate
        type MinValidatorPoolStake: Get<BalanceOf<Self>>;
        /// Minimum stake for any registered on-chain account to nominate
        type MinNomination: Get<BalanceOf<Self>>;
        /// Minimum stake for any registered on-chain account to become a nominator
        type MinNominatorStake: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(n: T::BlockNumber) {
            let mut round = <Round<T>>::get();
            if round.should_update(n) {
                // mutate round
                round.update(n);
                // pay all stakers for T::BondDuration rounds ago
                Self::pay_stakers(round.current);
                // start next round
                <Round<T>>::put(round);
                // snapshot total stake
                <Staked<T>>::insert(round.current, <Total<T>>::get());
                // Self::deposit_event(Event::NewRound(
                // 	round.first,
                // 	round.current,
                // 	collator_count,
                // 	total_staked,
                // ));
            }
        }
    }

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
        /// Set the total number of validator candidates selected per round
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
        #[pallet::weight(10_000)]
        /// Set blocks per round
        /// - if called with `new` less than length of current round, will transition immediately
        /// in the next block
        pub fn set_blocks_per_round(origin: OriginFor<T>, new: u32) -> DispatchResultWithPostInfo {
            frame_system::ensure_root(origin)?;
            ensure!(
                new >= T::MinBlocksPerRound::get(),
                Error::<T>::CannotSetBelowMin,
            );
            let mut round = <Round<T>>::get();
            let (now, first, old) = (round.current, round.first, round.length);
            round.length = new;
            <Round<T>>::put(round);
            Self::deposit_event(Event::BlocksPerRoundSet(now, first, old, new));
            Ok(().into())
        }
        /// Join the set of validators pool
        #[pallet::weight(10_000)]
        pub fn join_validator_pool(
            origin: OriginFor<T>,
            bond: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;
            ensure!(!Self::is_validator(&acc), Error::<T>::ValidatorExists);
            ensure!(!Self::is_nominator(&acc), Error::<T>::NominatorExists);
            ensure!(
                bond >= T::MinValidatorPoolStake::get(),
                Error::<T>::ValidatorBondBelowMin,
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
            let new_total = <Total<T>>::get() + bond;
            <Total<T>>::put(new_total);
            <ValidatorState<T>>::insert(&acc, validator);
            <ValidatorPool<T>>::put(validators);
            Self::deposit_event(Event::JoinedValidatorPool(acc, bond, new_total));
            Ok(().into())
        }
        /// Request to exit the validators pool. If successful,
        /// the account is immediately removed from the validator pool
        /// to prevent selection as a validator, but unbonding
        /// is executed with a delay of `BondDuration` rounds.
        #[pallet::weight(10_000)]
        pub fn exit_validators_pool(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;
            let mut state = <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            ensure!(!state.is_leaving(), Error::<T>::AlreadyLeaving);
            let mut exits = <ExitQueue<T>>::get();
            let now = <Round<T>>::get().current;
            let when = now + T::BondDuration::get();
            ensure!(
                exits.insert(Bond {
                    owner: validator.clone(),
                    amount: when,
                }),
                Error::<T>::AlreadyLeaving,
            );
            state.leave_validators_pool(when);
            let mut candidates = <ValidatorPool<T>>::get();
            if candidates.remove(&Bond::from_owner(validator.clone())) {
                <ValidatorPool<T>>::put(candidates);
            }
            <ExitQueue<T>>::put(exits);
            <ValidatorState<T>>::insert(&validator, state);
            Self::deposit_event(Event::ValidatorScheduledExit(now, validator, when));
            Ok(().into())
        }
        /// Rejoin the set of validators pool
        /// if previously had called `go_validator_offline`
        #[pallet::weight(10_000)]
        pub fn go_validator_online(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;
            let mut state = <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            ensure!(!state.is_active(), Error::<T>::AlreadyActive);
            ensure!(!state.is_leaving(), Error::<T>::CannotActivateIfLeaving);
            state.go_online();
            let mut validators = <ValidatorPool<T>>::get();
            ensure!(
                validators.insert(Bond {
                    owner: validator.clone(),
                    amount: state.total
                }),
                Error::<T>::AlreadyActive
            );
            <ValidatorPool<T>>::put(validators);
            <ValidatorState<T>>::insert(&validator, state);
            Self::deposit_event(Event::ValidatorBackOnline(
                <Round<T>>::get().current,
                validator,
            ));
            Ok(().into())
        }
        /// Bond more for validator
        #[pallet::weight(10_000)]
        pub fn validator_bond_more(
            origin: OriginFor<T>,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;
            let mut state = <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
            ensure!(!state.is_leaving(), Error::<T>::CannotActivateIfLeaving);
            T::Currency::reserve(&validator, more)?;
            let before = state.bond;
            state.bond_more(more);
            let after = state.bond;
            if state.is_active() {
                Self::update_validators_pool(validator.clone(), state.total);
            }
            <ValidatorState<T>>::insert(&validator, state);
            Self::deposit_event(Event::ValidatorBondedMore(validator, before, after));
            Ok(().into())
        }
        /// Bond less for validator
        #[pallet::weight(0)]
        pub fn validator_bond_less(
            origin: OriginFor<T>,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let validator = ensure_signed(origin)?;
            let mut state = <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;

            ensure!(!state.is_leaving(), Error::<T>::CannotActivateIfLeaving,);
            let before = state.bond;
            let after = state.bond_less(less).ok_or(Error::<T>::Underflow)?;

            ensure!(
                after >= T::MinValidatorPoolStake::get(),
                Error::<T>::ValidatorBondBelowMin,
            );
            T::Currency::unreserve(&validator, less);
            if state.is_active() {
                Self::update_validators_pool(validator.clone(), state.total);
            }
            <ValidatorState<T>>::insert(&validator, state);
            Self::deposit_event(Event::ValidatorBondedLess(validator, before, after));
            Ok(().into())
        }
        /// If caller is not a nominator, then join the set of nominators
        /// If caller is a nominator, then makes nomination to change their nomination state
        #[pallet::weight(10_000)]
        pub fn nominate(
            origin: OriginFor<T>,
            validator: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let acc = ensure_signed(origin)?;
            if let Some(mut nominator) = <NominatorState<T>>::get(&acc) {
                // nomination after first
                ensure!(
                    amount >= T::MinNomination::get(),
                    Error::<T>::NominationBelowMin
                );
                ensure!(
                    (nominator.nominations.0.len() as u32) < T::MaxValidatorPerNominator::get(),
                    Error::<T>::ExceedMaxValidatorPerNom,
                );
                let mut state =
                    <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;
                ensure!(
                    (state.nominators.0.len() as u32) < T::MaxNominatorsPerValidator::get(),
                    Error::<T>::TooManyNominators
                );

                ensure!(
                    nominator.add_nomination(Bond {
                        owner: validator.clone(),
                        amount
                    }),
                    Error::<T>::AlreadyNominatedValidator,
                );

                let nomination = Bond {
                    owner: acc.clone(),
                    amount,
                };
                ensure!(
                    state.nominators.insert(nomination),
                    Error::<T>::NominatorExists
                );

                T::Currency::reserve(&acc, amount)?;
                let new_total = state.total + amount;
                if state.is_active() {
                    Self::update_validators_pool(validator.clone(), new_total);
                }
                let new_total_locked = <Total<T>>::get() + amount;
                <Total<T>>::put(new_total_locked);
                state.total = new_total;
                <ValidatorState<T>>::insert(&validator, state);
                <NominatorState<T>>::insert(&acc, nominator);
                Self::deposit_event(Event::Nomination(acc, amount, validator, new_total));
            } else {
                // first nomination
                ensure!(
                    amount >= T::MinNominatorStake::get(),
                    Error::<T>::NominatorBondBelowMin
                );
                // cannot be a validator candidate and nominator with same AccountId
                ensure!(!Self::is_validator(&acc), Error::<T>::ValidatorExists);
                let mut state =
                    <ValidatorState<T>>::get(&validator).ok_or(Error::<T>::ValidatorDNE)?;

                ensure!(
                    (state.nominators.0.len() as u32) <= T::MaxNominatorsPerValidator::get(),
                    Error::<T>::TooManyNominators
                );

                let nomination = Bond {
                    owner: acc.clone(),
                    amount,
                };
                ensure!(
                    state.nominators.insert(nomination),
                    Error::<T>::NominatorExists
                );

                T::Currency::reserve(&acc, amount)?;
                let new_total = state.total + amount;
                if state.is_active() {
                    Self::update_validators_pool(validator.clone(), new_total);
                }
                let new_total_locked = <Total<T>>::get() + amount;
                <Total<T>>::put(new_total_locked);
                state.total = new_total;
                <ValidatorState<T>>::insert(&validator, state);
                <NominatorState<T>>::insert(&acc, Nominator::new(validator.clone(), amount));
                Self::deposit_event(Event::Nomination(acc, amount, validator, new_total));
            }
            Ok(().into())
        }
        /// Revoke an existing nomination
        #[pallet::weight(10_000)]
        pub fn revoke_nomination(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            Self::nominator_revokes_validator(ensure_signed(origin)?, validator)
        }
        /// Quit the set of nominators and, by implication, revoke all ongoing nominations
        #[pallet::weight(0)]
        pub fn quit_nominators(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    pub enum Event<T: Config> {
        NewInvulnerables(Vec<T::AccountId>),
        /// Starting Block, Round, Number of Validators Selected, Total Balance
        NewRound(T::BlockNumber, RoundIndex, u32, BalanceOf<T>),
        /// Account, Amount Locked, New Total Amt Locked
        JoinedValidatorPool(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Round, Validator Account, Total Exposed Amount (includes all nominations)
        ValidatorChosen(RoundIndex, T::AccountId, BalanceOf<T>),
        /// Validator Account, Old Bond, New Bond
        ValidatorBondedMore(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        /// Validator Account, Old Bond, New Bond
        ValidatorBondedLess(T::AccountId, BalanceOf<T>, BalanceOf<T>),
        ValidatorWentOffline(RoundIndex, T::AccountId),
        ValidatorBackOnline(RoundIndex, T::AccountId),
        /// Round, Validator Account, Scheduled Exit
        ValidatorScheduledExit(RoundIndex, T::AccountId, RoundIndex),
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
        BlocksPerRoundSet(RoundIndex, T::BlockNumber, u32, u32),
    }

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total)]
    /// Total capital locked by this staking pallet
    type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn round)]
    /// Current round index and next round scheduled transition
    type Round<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_commission)]
    /// Commission percent taken off of rewards for all validators
    type ValidatorCommission<T: Config> = StorageValue<_, Perbill, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_state)]
    /// Get validator state associated with an account if account is collating else None
    type ValidatorState<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Validator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn nominator_state)]
    /// Get nominator state associated with an account if account is nominating else None
    type NominatorState<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Nominator<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn total_selected)]
    /// The total validators selected every round
    type TotalSelected<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn selected_validators)]
    /// The validators selected for the current round
    type SelectedValidators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_pool)]
    /// The pool of validator validators, each with their total backing stake
    type ValidatorPool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn exit_queue)]
    /// A queue of validators awaiting exit `BondDuration` delay after request
    type ExitQueue<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, RoundIndex>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn at_stake)]
    /// Snapshot of validator nomination stake at the start of the round
    pub type AtStake<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        RoundIndex,
        Twox64Concat,
        T::AccountId,
        ValidatorSnapshot<T::AccountId, BalanceOf<T>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn staked)]
    /// Total backing stake for selected validators in the round
    pub type Staked<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn points)]
    /// Total points awarded to validator for block production in the round
    pub type Points<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, RewardPoint, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn awarded_pts)]
    /// Points for each validator per round
    pub type AwardedPts<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        RoundIndex,
        Twox64Concat,
        T::AccountId,
        RewardPoint,
        ValueQuery,
    >;

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
        }
    }

    impl<T: Config> Pallet<T> {
        fn is_validator(acc: &T::AccountId) -> bool {
            <ValidatorState<T>>::get(acc).is_some()
        }

        fn is_selected_validator(acc: &T::AccountId) -> bool {
            <SelectedValidators<T>>::get().binary_search(acc).is_ok()
        }

        fn is_nominator(acc: &T::AccountId) -> bool {
            <NominatorState<T>>::get(acc).is_some()
        }
        // ensure validator is active before calling
        fn update_validators_pool(validator: T::AccountId, total: BalanceOf<T>) {
            let mut validators = <ValidatorPool<T>>::get();
            validators.remove(&Bond::from_owner(validator.clone()));
            validators.insert(Bond {
                owner: validator,
                amount: total,
            });
            <ValidatorPool<T>>::put(validators);
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
            let mut nominator = <NominatorState<T>>::get(&acc).ok_or(Error::<T>::ValidatorDNE)?;
            let old_total = nominator.total;
            let remaining = nominator
                .rm_nomination(validator.clone())
                .ok_or(Error::<T>::NominationDNE)?;
            // edge case; if no nominations remaining, leave set of nominators
            if nominator.nominations.0.len().is_zero() {
                // leave the set of nominators because no nominations left
                Self::nominator_leaves_validator(acc.clone(), validator)?;
                <NominatorState<T>>::remove(&acc);
                Self::deposit_event(Event::NominatorLeft(acc, old_total));
                return Ok(().into());
            }
            ensure!(
                remaining >= T::MinNominatorStake::get(),
                Error::<T>::NominatorBondBelowMin
            );
            Self::nominator_leaves_validator(acc.clone(), validator)?;
            <NominatorState<T>>::insert(&acc, nominator);
            Ok(().into())
        }

        // Calculate round issuance based on total staked for the given round
        fn compute_issuance(staked: BalanceOf<T>) -> BalanceOf<T> {
            // TODO :: issuance model ideal for nodle
            return staked;
        }

        fn pay_stakers(next: RoundIndex) {
            let mint = |amt: BalanceOf<T>, to: T::AccountId| {
                if amt > T::Currency::minimum_balance() {
                    if let Ok(imb) = T::Currency::deposit_into_existing(&to, amt) {
                        Self::deposit_event(Event::Rewarded(to.clone(), imb.peek()));
                    }
                }
            };
            let duration = T::BondDuration::get();
            let validator_fee = <ValidatorCommission<T>>::get();
            if next > duration {
                let round_to_payout = next - duration;
                let total = <Points<T>>::get(round_to_payout);
                let total_staked = <Staked<T>>::get(round_to_payout);
                let issuance = Self::compute_issuance(total_staked);
                for (val, pts) in <AwardedPts<T>>::drain_prefix(round_to_payout) {
                    let pct_due = Perbill::from_rational_approximation(pts, total);
                    let mut amt_due = pct_due * issuance;
                    if amt_due <= T::Currency::minimum_balance() {
                        continue;
                    }
                    // Take the snapshot of block author and nominations
                    let state = <AtStake<T>>::take(round_to_payout, &val);
                    if state.nominators.is_empty() {
                        // solo collator with no nominators
                        mint(amt_due, val.clone());
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
        }
    }

    /// Add reward points to block authors:
    /// * 20 points to the block producer for producing a block in the chain
    impl<T: Config> author_inherent::EventHandler<T::AccountId> for Pallet<T> {
        fn note_author(author: T::AccountId) {
            let now = <Round<T>>::get().current;
            let score_plus_20 = <AwardedPts<T>>::get(now, &author) + 20;
            <AwardedPts<T>>::insert(now, author, score_plus_20);
            <Points<T>>::mutate(now, |x| *x += 20);
        }
    }

    impl<T: Config> author_inherent::CanAuthor<T::AccountId> for Pallet<T> {
        fn can_author(account: &T::AccountId) -> bool {
            Self::is_selected_validator(account)
        }
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
