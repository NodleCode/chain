/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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

//! This module implements a Token Curated Registry where members (represented by their
//! `AccountId`) are accepted based on the number of tokens staked in support to their
//! application.
mod benchmarking;

#[cfg(test)]
mod tests;

use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::{ChangeMembers, Currency, Get, Imbalance, ReservableCurrency},
};
use frame_system::{self as system};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{CheckedAdd, CheckedDiv, CheckedSub, Saturating, Zero},
    Perbill,
};
use sp_std::prelude::Vec;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

type BalanceOf<T, I> =
    <<T as Config<I>>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T, I> =
    <<T as Config<I>>::Currency as Currency<<T as system::Config>::AccountId>>::NegativeImbalance;
type PositiveImbalanceOf<T, I> =
    <<T as Config<I>>::Currency as Currency<<T as system::Config>::AccountId>>::PositiveImbalance;

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
pub struct Application<AccountId, Balance, BlockNumber> {
    candidate: AccountId,
    candidate_deposit: Balance,
    metadata: Vec<u8>, // For instance, a link or a name...

    challenger: Option<AccountId>,
    challenger_deposit: Balance,

    votes_for: Balance,
    voters_for: Vec<(AccountId, Balance)>,
    votes_against: Balance,
    voters_against: Vec<(AccountId, Balance)>,

    created_block: BlockNumber,
    challenged_block: BlockNumber,
}

impl<AccountId, Balance, BlockNumber> Application<AccountId, Balance, BlockNumber> {
    fn voters_for_and_candidate(self) -> Vec<(AccountId, Balance)> {
        let mut voters = self.voters_for;
        voters.push((self.candidate, self.candidate_deposit));

        voters
    }

    fn voters_against_and_challenger(self) -> Vec<(AccountId, Balance)> {
        let mut voters = self.voters_against;
        if self.challenger.is_some() {
            voters.push((
                self.challenger
                    .expect("we just checked that challenger was not none; qed"),
                self.challenger_deposit,
            ));
        };

        voters
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// The currency used to represent the voting power
        type Currency: ReservableCurrency<Self::AccountId>;
        /// Minimum amount of tokens required to apply
        #[pallet::constant]
        type MinimumApplicationAmount: Get<BalanceOf<Self, I>>;
        /// Minimum amount of tokens required to counter an application
        #[pallet::constant]
        type MinimumCounterAmount: Get<BalanceOf<Self, I>>;
        /// Minimum amount of tokens required to challenge a member's application
        #[pallet::constant]
        type MinimumChallengeAmount: Get<BalanceOf<Self, I>>;
        /// How many blocks we need to wait for before validating an application
        #[pallet::constant]
        type FinalizeApplicationPeriod: Get<Self::BlockNumber>;
        /// How many blocks we need to wait for before finalizing a challenge
        #[pallet::constant]
        type FinalizeChallengePeriod: Get<Self::BlockNumber>;
        /// How do we slash loosing parties when challenges are finalized, application's
        /// member will be slashed at the same value
        #[pallet::constant]
        type LoosersSlash: Get<Perbill>;
        /// Hook that we call whenever some members are added or removed from the TCR
        type ChangeMembers: ChangeMembers<Self::AccountId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
        /// At the end of each blocks, commit applications or challenges as needed
        fn on_finalize(block: T::BlockNumber) {
            let (mut new_1, _) =
                Self::commit_applications(block).unwrap_or((Vec::new(), Vec::new()));
            let (new_2, mut old) =
                Self::resolve_challenges(block).unwrap_or((Vec::new(), Vec::new()));

            // Should never be the same, so should not need some uniq checks
            new_1.extend(new_2);
            new_1.sort();
            old.sort();

            Self::notify_members_change(new_1, old);
        }
    }

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// Apply to join the TCR, `metadata` can be used to add something like a URL or ID
        #[pallet::weight(
			T::WeightInfo::apply(metadata.len() as u32)
		)]
        pub fn apply(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
            deposit: BalanceOf<T, I>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                deposit >= T::MinimumApplicationAmount::get(),
                Error::<T, I>::DepositTooSmall
            );
            ensure!(
                !<Applications<T, I>>::contains_key(sender.clone()),
                Error::<T, I>::ApplicationPending
            );
            ensure!(
                !<Challenges<T, I>>::contains_key(sender.clone()),
                Error::<T, I>::ApplicationChallenged
            );
            ensure!(
                !<Members<T, I>>::contains_key(sender.clone()),
                Error::<T, I>::AlreadyMember
            );

            Self::reserve_for(sender.clone(), deposit)?;

            <Applications<T, I>>::insert(
                sender.clone(),
                Application {
                    candidate: sender.clone(),
                    candidate_deposit: deposit,
                    metadata,

                    challenger: None,
                    challenger_deposit: Zero::zero(),

                    votes_for: Zero::zero(),
                    voters_for: Vec::new(),
                    votes_against: Zero::zero(),
                    voters_against: Vec::new(),

                    created_block: <system::Pallet<T>>::block_number(),
                    challenged_block: Zero::zero(),
                },
            );

            Self::deposit_event(Event::NewApplication(sender, deposit));
            Ok(().into())
        }

        /// Counter a pending application, this will initiate a challenge
        #[pallet::weight(T::WeightInfo::counter())]
        pub fn counter(
            origin: OriginFor<T>,
            member: T::AccountId,
            deposit: BalanceOf<T, I>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                deposit >= T::MinimumCounterAmount::get(),
                Error::<T, I>::DepositTooSmall
            );
            ensure!(
                <Applications<T, I>>::contains_key(member.clone()),
                Error::<T, I>::ApplicationNotFound
            );

            Self::reserve_for(sender.clone(), deposit)?;

            // Note the use of `take` instead of `get` that will effectively remove
            // the application from Applications::<T, I>.
            // We actually move the application out and convert it to a challenge.
            let mut application = <Applications<T, I>>::take(member.clone());
            application.challenger = Some(sender.clone());
            application.challenger_deposit = deposit;
            application.challenged_block = <system::Pallet<T>>::block_number();

            <Challenges<T, I>>::insert(member.clone(), application);

            Self::deposit_event(Event::ApplicationCountered(member, sender, deposit));
            Ok(().into())
        }

        /// Vote in support or opposition of a given challenge
        #[pallet::weight(T::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            member: T::AccountId,
            supporting: bool,
            deposit: BalanceOf<T, I>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                <Challenges<T, I>>::contains_key(member.clone()),
                Error::<T, I>::ChallengeNotFound
            );

            let mut application = <Challenges<T, I>>::get(member.clone());

            if supporting {
                let new_votes = application
                    .votes_for
                    .checked_add(&deposit)
                    .ok_or_else(|| Error::<T, I>::DepositOverflow)?;
                application.votes_for = new_votes;
                application.voters_for.push((sender.clone(), deposit));
            } else {
                let new_votes = application
                    .votes_against
                    .checked_add(&deposit)
                    .ok_or_else(|| Error::<T, I>::DepositOverflow)?;
                application.votes_against = new_votes;
                application.voters_against.push((sender.clone(), deposit));
            }

            Self::reserve_for(sender.clone(), deposit)?;
            <Challenges<T, I>>::insert(member.clone(), application);

            Self::deposit_event(Event::VoteRecorded(member, sender, deposit, supporting));
            Ok(().into())
        }

        /// Trigger a new challenge to remove an existing member
        #[pallet::weight(T::WeightInfo::challenge())]
        pub fn challenge(
            origin: OriginFor<T>,
            member: T::AccountId,
            deposit: BalanceOf<T, I>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                deposit >= T::MinimumChallengeAmount::get(),
                Error::<T, I>::DepositTooSmall
            );
            ensure!(
                <Members<T, I>>::contains_key(member.clone()),
                Error::<T, I>::MemberNotFound
            );
            ensure!(
                !<Challenges<T, I>>::contains_key(member.clone()),
                Error::<T, I>::ApplicationAlreadyChallenged
            );

            Self::reserve_for(sender.clone(), deposit)?;

            let mut application = <Members<T, I>>::get(member.clone());
            application.challenger = Some(sender.clone());
            application.challenger_deposit = deposit;
            application.challenged_block = <system::Pallet<T>>::block_number();
            application.votes_for = Zero::zero();
            application.voters_for = Vec::new();
            application.votes_against = Zero::zero();
            application.voters_against = Vec::new();

            <Challenges<T, I>>::insert(member.clone(), application);

            Self::deposit_event(Event::ApplicationChallenged(member, sender, deposit));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// Someone applied to join the registry
        NewApplication(T::AccountId, BalanceOf<T, I>),
        /// Someone countered an application
        ApplicationCountered(T::AccountId, T::AccountId, BalanceOf<T, I>),
        /// A new vote for an application has been recorded
        VoteRecorded(T::AccountId, T::AccountId, BalanceOf<T, I>, bool),
        /// An application passed without being countered
        ApplicationPassed(T::AccountId),
        /// A member's application is being challenged
        ApplicationChallenged(T::AccountId, T::AccountId, BalanceOf<T, I>),
        /// A challenge killed the given application
        ChallengeRefusedApplication(T::AccountId),
        /// A challenge accepted the application
        ChallengeAcceptedApplication(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// An application for this Origin is already pending
        ApplicationPending,
        /// A similar application is being challenged
        ApplicationChallenged,
        /// Not enough funds to pay the deposit
        NotEnoughFunds,
        /// The deposit is too small
        DepositTooSmall,
        /// The application linked to the member was not found
        ApplicationNotFound,
        /// The challenge linked ot the member was not found
        ChallengeNotFound,
        /// The account id is not a member
        MemberNotFound,
        /// Application was already challenged by someone else
        ApplicationAlreadyChallenged,
        /// Deposit value overflows votes
        DepositOverflow,
        /// No need to apply as this account is already a member
        AlreadyMember,

        ReserveOverflow,
        UnreserveOverflow,
    }

    /// Proposals that have been made.
    #[pallet::storage]
    #[pallet::getter(fn applications)]
    pub type Applications<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
        ValueQuery,
    >;

    /// This keeps track of the member status / applications being challenged for removal
    #[pallet::storage]
    #[pallet::getter(fn challenges)]
    pub type Challenges<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
        ValueQuery,
    >;

    /// This keeps track of all the registry's members
    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
        ValueQuery,
    >;
}

type FinalizeHelperResultFrom<T> = Result<
    (
        Vec<<T as frame_system::Config>::AccountId>,
        Vec<<T as frame_system::Config>::AccountId>,
    ),
    DispatchError,
>;
type AccountsAndDepositsFromResolvedChallenge<T, I> =
    Vec<(<T as frame_system::Config>::AccountId, BalanceOf<T, I>)>;
type ResolvedChallengeResultFrom<T, I> = (
    AccountsAndDepositsFromResolvedChallenge<T, I>, // Winners -> will be compensated
    AccountsAndDepositsFromResolvedChallenge<T, I>, // Losers -> will be slashed
);

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    /// Do not just call `set_lock`, rather increase the locked amount
    fn reserve_for(who: T::AccountId, amount: BalanceOf<T, I>) -> DispatchResult {
        // Make sure we can lock has many funds
        if !T::Currency::can_reserve(&who, amount) {
            return Err(Error::<T, I>::NotEnoughFunds.into());
        }

        T::Currency::reserve(&who, amount)
    }

    /// Decrease the locked amount of tokens
    fn unreserve_for(who: T::AccountId, amount: BalanceOf<T, I>) {
        T::Currency::unreserve(&who, amount);
    }

    /// Takes some funds away from a loser, deposit in our own account
    fn slash_looser(who: T::AccountId, amount: BalanceOf<T, I>) -> NegativeImbalanceOf<T, I> {
        let to_be_slashed = T::LoosersSlash::get() * amount; // Sorry buddy...
        if T::Currency::can_slash(&who, to_be_slashed) {
            let (imbalance, _remaining) = T::Currency::slash(&who, to_be_slashed);
            imbalance
        } else {
            <NegativeImbalanceOf<T, I>>::zero()
        }
    }

    /// Number of tokens supporting a given application
    fn get_supporting(
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
    ) -> BalanceOf<T, I> {
        application
            .candidate_deposit
            .checked_add(&application.votes_for)
            .expect("coins can not exceed maximum supply which is not overflowing; qed")
    }

    /// Number of tokens opposing a given application
    fn get_opposing(
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
    ) -> BalanceOf<T, I> {
        application
            .challenger_deposit
            .checked_add(&application.votes_against)
            .expect("coins can not exceed maximum supply which is not overflowing; qed")
    }

    fn commit_applications(block: T::BlockNumber) -> FinalizeHelperResultFrom<T> {
        let new_members = <Applications<T, I>>::iter()
            .filter(|(_account_id, application)| {
                block
                    .checked_sub(&application.clone().created_block)
                    .expect("created_block should always be smaller than block; qed")
                    >= T::FinalizeApplicationPeriod::get()
            })
            .map(|(account_id, application)| {
                <Applications<T, I>>::remove(account_id.clone());
                <Members<T, I>>::insert(account_id.clone(), application.clone());
                Self::unreserve_for(account_id.clone(), application.candidate_deposit);
                Self::deposit_event(Event::ApplicationPassed(account_id.clone()));

                account_id
            })
            .collect::<Vec<T::AccountId>>();

        Ok((new_members, Vec::new()))
    }

    fn resolve_challenges(block: T::BlockNumber) -> FinalizeHelperResultFrom<T> {
        let all_members = <Challenges<T, I>>::iter()
            .filter(|(_account_id, application)| {
                Self::is_challenge_expired(block, application.clone())
            })
            .map(|(account_id, application)| {
                (
                    account_id.clone(),
                    application.clone(),
                    Self::is_challenge_passing(application),
                    <Members<T, I>>::contains_key(account_id),
                )
            })
            .map(|(account_id, application, challenge_passed, was_member)| {
                let (to_reward, to_slash) = match challenge_passed {
                    true => {
                        Self::resolve_challenge_application_passed(account_id.clone(), application)
                    }
                    false => Self::resolve_challenge_application_refused(
                        account_id.clone(),
                        application,
                        was_member,
                    ),
                };

                let total_winning_deposits: BalanceOf<T, I> =
                    to_reward.iter().fold(Zero::zero(), |acc, (_a, deposit)| {
                        acc.checked_add(deposit).expect(
                            "total deposits have already been checked for overflows before; qed",
                        )
                    });

                (
                    account_id,
                    challenge_passed,
                    was_member,
                    to_reward,
                    to_slash,
                    total_winning_deposits,
                )
            })
            .map(
                |(
                    account_id,
                    challenge_passed,
                    was_member,
                    to_reward,
                    to_slash,
                    total_winning_deposits,
                )| {
                    Self::resolve_challenge_execute_slashes_and_rewards(
                        to_reward,
                        to_slash,
                        total_winning_deposits,
                    );
                    <Challenges<T, I>>::remove(account_id.clone());

                    (account_id, challenge_passed, was_member)
                },
            )
            .collect::<Vec<(T::AccountId, bool, bool)>>();

        // note to tomorrow's eliott:
        // I need to undo the (account_id, challenge_passed) to (old, new). Maybe use options and results?
        // another option is to collect and iter two items after
        // also need to do more cleanup

        Ok((
            all_members
                .iter()
                .filter(|(_account_id, passed, _was_member)| *passed)
                .map(|(account_id, _passed, _was_member)| account_id.clone())
                .collect::<Vec<T::AccountId>>(), // new_members
            all_members
                .iter()
                .filter(|(_account_id, passed, was_member)| !*passed && *was_member)
                .map(|(account_id, _passed, _was_member)| account_id.clone())
                .collect::<Vec<T::AccountId>>(), // old_members
        ))
    }

    fn resolve_challenge_application_passed(
        account_id: T::AccountId,
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
    ) -> ResolvedChallengeResultFrom<T, I> {
        <Members<T, I>>::insert(account_id.clone(), application.clone());

        Self::deposit_event(Event::ChallengeAcceptedApplication(account_id));

        // The proposal passed, slash `challenger` and `voters_against`
        (
            application.clone().voters_for_and_candidate(),
            application.voters_against_and_challenger(),
        )
    }

    fn resolve_challenge_application_refused(
        account_id: T::AccountId,
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
        was_member: bool,
    ) -> ResolvedChallengeResultFrom<T, I> {
        // If it is a member, remove it
        if was_member {
            <Members<T, I>>::remove(application.clone().candidate);
        }

        Self::deposit_event(Event::ChallengeRefusedApplication(account_id));

        // The proposal did not pass, slash `candidate` and `voters_for`
        (
            application.clone().voters_against_and_challenger(),
            application.voters_for_and_candidate(),
        )
    }

    fn resolve_challenge_execute_slashes_and_rewards(
        to_reward: Vec<(T::AccountId, BalanceOf<T, I>)>,
        to_slash: Vec<(T::AccountId, BalanceOf<T, I>)>,
        total_winning_deposits: BalanceOf<T, I>,
    ) {
        let mut slashes_imbalance = <NegativeImbalanceOf<T, I>>::zero();
        to_slash.iter().cloned().for_each(|(account_id, deposit)| {
            Self::unreserve_for(account_id.clone(), deposit);
            let r = Self::slash_looser(account_id, deposit);
            slashes_imbalance.subsume(r);
        });

        // Execute rewards
        let mut rewards_imbalance = <PositiveImbalanceOf<T, I>>::zero();
        let rewards_pool = slashes_imbalance.peek();
        let mut allocated: BalanceOf<T, I> = Zero::zero();
        for (account_id, deposit) in to_reward.clone() {
            Self::unreserve_for(account_id.clone(), deposit);

            // deposit          deposit * pool
            // ------- * pool = --------------
            //  total               total
            let coins = deposit
                        .saturating_mul(rewards_pool)
                        .checked_div(&total_winning_deposits)
                        .expect("total should always be equal to the sum of all deposits and thus should never {over, under}flow; qed");

            if let Ok(r) = T::Currency::deposit_into_existing(&account_id, coins) {
                allocated = allocated
                            .checked_add(&r.peek())
                            .expect("a simple counters of coins that we already have and store in another variable; qed");
                rewards_imbalance.subsume(r);
            }
        }

        // Last element is `challenger` or `candidate`. They simply get whatever dust might be left
        let (dust_collector, _deposit) = &to_reward[to_reward.len() - 1];
        let remaining = rewards_pool.checked_sub(&allocated).expect("we do not expect to allocate more coins than in rewards pool, would this happen we'd have bigger problems somewhere else; qed");
        if let Ok(r) = T::Currency::deposit_into_existing(&dust_collector, remaining) {
            rewards_imbalance.subsume(r);
        }
    }

    fn is_challenge_expired(
        block: T::BlockNumber,
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
    ) -> bool {
        block - application.challenged_block >= T::FinalizeChallengePeriod::get()
    }

    fn is_challenge_passing(
        application: Application<T::AccountId, BalanceOf<T, I>, T::BlockNumber>,
    ) -> bool {
        Self::get_supporting(application.clone()) > Self::get_opposing(application)
    }

    fn notify_members_change(new_members: Vec<T::AccountId>, old_members: Vec<T::AccountId>) {
        if !new_members.is_empty() || !old_members.is_empty() {
            let mut sorted_members = <Members<T, I>>::iter()
                .map(|(a, _app)| a)
                .collect::<Vec<_>>();
            sorted_members.sort();
            T::ChangeMembers::change_members_sorted(
                &new_members,
                &old_members,
                &sorted_members[..],
            );
        }
    }
}
