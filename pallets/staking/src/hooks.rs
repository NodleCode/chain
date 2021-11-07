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

use super::{
    ActiveSession, BalanceOf, BondedSessions, Config, Event, NegativeImbalanceOf, Pallet,
    SessionAccumulatedBalance, SessionValidatorReward, SlashRewardProportion, Staked, Store, Total,
};
use crate::slashing;
use crate::types::{ValidatorSnapshot, ValidatorSnapshotOf};
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, Get, Imbalance, OnUnbalanced},
};
use frame_system::{self as system};
use pallet_session::historical;
use sp_runtime::{
    traits::{AccountIdConversion, Convert, Saturating},
    Perbill,
};
use sp_staking::{
    offence::{OffenceDetails, OnOffenceHandler},
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
        <SessionAccumulatedBalance<T>>::mutate(now, |v: &mut BalanceOf<T>| {
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
        log::trace!("note_author:[{:#?}] - Author[{:#?}]", line!(), author);
        Self::reward_by_ids(vec![(author, 20)])
    }
    fn note_uncle(author: T::AccountId, _age: T::BlockNumber) {
        log::trace!("note_uncle:[{:#?}] - Author[{:#?}]", line!(), author);
        Self::reward_by_ids(vec![
            (<pallet_authorship::Pallet<T>>::author(), 2),
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
        log::trace!("new_session:[{:#?}] - Sess-idx[{:#?}]", line!(), new_index);

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

        log::debug!(
            "new_session:[{:#?}] - Event::NewSession(SI[{}],VC[{}],TS[{:#?}])",
            line!(),
            new_index,
            validator_count,
            total_staked,
        );

        Some(Self::selected_validators())
    }
    fn start_session(start_index: SessionIndex) {
        log::trace!(
            "start_session:[{:#?}] - Sess-idx[{:#?}]",
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

        log::trace!(
            "start_session:[{:#?}] - Exit!!! Sess-idx[{:#?}]",
            line!(),
            start_index
        );
    }
    fn end_session(end_index: SessionIndex) {
        log::trace!("end_session:[{:#?}] - Sess-idx[{:#?}]", line!(), end_index);

        if Self::active_session() == end_index {
            let payout = Self::session_accumulated_balance(end_index);

            // Set ending session reward.
            <SessionValidatorReward<T>>::insert(&end_index, payout);

            // pay all stakers for T::BondedDuration rounds ago
            Self::pay_stakers(end_index);

        // // Clear the DB cached state of last session
        // Self::clear_session_information(Self::active_session());
        } else {
            log::error!(
                "end_session:[{:#?}] - Something wrong (CSI[{}], ESI[{}])",
                line!(),
                Self::active_session(),
                end_index,
            );
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
    fn disable_validator(validator: &AccountId) -> bool;
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
    fn disable_validator(validator: &<T as frame_system::Config>::AccountId) -> bool {
        <pallet_session::Pallet<T>>::disable(validator)
    }

    fn validators() -> Vec<<T as frame_system::Config>::AccountId> {
        <pallet_session::Pallet<T>>::validators()
    }

    fn prune_historical_up_to(up_to: SessionIndex) {
        <pallet_session::historical::Pallet<T>>::prune_up_to(up_to);
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
    ) -> Weight {
        log::trace!(
            "on_offence:[{:#?}] - Sess-idx [{:#?}] | Slash-Frac [{:#?}]",
            line!(),
            slash_session,
            slash_fraction,
        );

        let reward_proportion = <SlashRewardProportion<T>>::get();
        let mut consumed_weight: Weight = 0;
        let mut add_db_reads_writes = |reads, writes| {
            consumed_weight =
                consumed_weight.saturating_add(T::DbWeight::get().reads_writes(reads, writes));
        };

        let active_session = Self::active_session();
        add_db_reads_writes(1, 0);

        let window_start = active_session.saturating_sub(T::BondedDuration::get());
        let slash_defer_duration = T::SlashDeferDuration::get();

        let invulnerables = Self::invulnerables();
        add_db_reads_writes(1, 0);

        log::trace!(
            "on_offence:[{:#?}] - Invulnerables[{:#?}]",
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

                    let slash_cost = (6, 5);
                    let reward_cost = (2, 2);
                    add_db_reads_writes(
                        (1 + nominators_len) * slash_cost.0 + reward_cost.0 * reporters_len,
                        (1 + nominators_len) * slash_cost.1 + reward_cost.1 * reporters_len,
                    );
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
                log::trace!("on_offence:[{:#?}] - NOP", line!(),);
                add_db_reads_writes(4 /* fetch_spans */, 5 /* kick_out_if_recent */);
            }
        }
        consumed_weight
    }
}
