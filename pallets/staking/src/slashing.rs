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

use super::{BalanceOf, Config, Event, NegativeImbalanceOf, Pallet, Store};
use crate::hooks::SessionInterface;
use crate::types::{SpanIndex, UnappliedSlash, ValidatorSnapshot};
use frame_support::traits::{
    Currency, Get, Imbalance, LockableCurrency, OnUnbalanced, WithdrawReasons,
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{
    traits::{Saturating, Zero},
    DispatchResult, Perbill, RuntimeDebug,
};
use sp_staking::{offence::DisableStrategy, SessionIndex};
use sp_std::vec::Vec;

// A range of start..end eras for a slashing span.
#[derive(Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct SlashingSpan {
    pub(crate) index: SpanIndex,
    pub(crate) start: SessionIndex,
    pub(crate) length: Option<SessionIndex>, // the ongoing slashing span has indeterminate length.
}

impl SlashingSpan {
    fn contains_era(&self, era: SessionIndex) -> bool {
        self.start <= era
            && self
                .length
                .map_or(true, |l| self.start.saturating_add(l) > era)
    }
}

/// An encoding of all of a nominator's slashing spans.
#[derive(Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct SlashingSpans {
    // the index of the current slashing span of the nominator. different for
    // every controller, resets when the account hits free balance 0.
    span_index: SpanIndex,
    // the start era of the most recent (ongoing) slashing span.
    last_start: SessionIndex,
    // the last era at which a non-zero slash occurred.
    last_nonzero_slash: SessionIndex,
    // all prior slashing spans' start indices, in reverse order (most recent first)
    // encoded as offsets relative to the slashing span after it.
    prior: Vec<SessionIndex>,
}

impl SlashingSpans {
    // creates a new record of slashing spans for a controller, starting at the beginning
    // of the bonding period, relative to now.
    pub(crate) fn new(window_start: SessionIndex) -> Self {
        SlashingSpans {
            span_index: 0,
            last_start: window_start,
            // initialize to zero, as this structure is lazily created until
            // the first slash is applied. setting equal to `window_start` would
            // put a time limit on nominations.
            last_nonzero_slash: 0,
            prior: Vec::new(),
        }
    }

    // update the slashing spans to reflect the start of a new span at the era after `now`
    // returns `true` if a new span was started, `false` otherwise. `false` indicates
    // that internal state is unchanged.
    pub(crate) fn end_span(&mut self, now: SessionIndex) -> bool {
        let next_start = now.saturating_add(1);
        if next_start <= self.last_start {
            return false;
        }
        let last_length = next_start.saturating_sub(self.last_start);
        self.prior.insert(0, last_length);
        self.last_start = next_start;
        self.span_index = self.span_index.saturating_add(1);
        true
    }

    // an iterator over all slashing spans in _reverse_ order - most recent first.
    pub(crate) fn iter(&'_ self) -> impl Iterator<Item = SlashingSpan> + '_ {
        let mut last_start = self.last_start;
        let mut index = self.span_index;
        let last = SlashingSpan {
            index,
            start: last_start,
            length: None,
        };
        let prior = self.prior.iter().cloned().map(move |length| {
            let start = last_start - length;
            last_start = start;
            index = index.saturating_sub(1);

            SlashingSpan {
                index,
                start,
                length: Some(length),
            }
        });

        sp_std::iter::once(last).chain(prior)
    }

    /// Yields the era index where the most recent non-zero slash occurred.
    pub fn last_nonzero_slash(&self) -> SessionIndex {
        self.last_nonzero_slash
    }

    // prune the slashing spans against a window, whose start era index is given.
    //
    // If this returns `Some`, then it includes a range start..end of all the span
    // indices which were pruned.
    fn prune(&mut self, window_start: SessionIndex) -> Option<(SpanIndex, SpanIndex)> {
        let old_idx = self
            .iter()
            .skip(1) // skip ongoing span.
            .position(|span| {
                span.length
                    .map_or(false, |len| span.start + len <= window_start)
            });

        let earliest_span_index = self.span_index - self.prior.len() as SpanIndex;
        let pruned = match old_idx {
            Some(o) => {
                self.prior.truncate(o);
                let new_earliest = self.span_index - self.prior.len() as SpanIndex;
                Some((earliest_span_index, new_earliest))
            }
            None => None,
        };

        // readjust the ongoing span, if it started before the beginning of the window.
        self.last_start = sp_std::cmp::max(self.last_start, window_start);
        pruned
    }
}

/// A slashing-span record for a particular controller.
#[derive(Encode, Decode, Default, scale_info::TypeInfo)]
pub struct SpanRecord<Balance> {
    pub slashed: Balance,
    pub paid_out: Balance,
}

impl<Balance> SpanRecord<Balance> {
    /// The value of controller balance slashed in this span.
    #[cfg(test)]
    pub(crate) fn amount_slashed(&self) -> &Balance {
        &self.slashed
    }
}

/// Parameters for performing a slash.
#[derive(Clone)]
pub(crate) struct SlashParams<'a, T: 'a + Config> {
    /// The controller account being slashed.
    pub(crate) controller: &'a T::AccountId,
    /// The proportion of the slash.
    pub(crate) slash: Perbill,
    /// The exposure of the controller and all nominators.
    pub(crate) exposure: &'a ValidatorSnapshot<T::AccountId, BalanceOf<T>>,
    /// The session where the offence occurred.
    pub(crate) slash_session: SessionIndex,
    /// The first era in the current bonding period.
    pub(crate) window_start: SessionIndex,
    /// The current era.
    pub(crate) now: SessionIndex,
    /// The maximum percentage of a slash that ever gets paid out.
    /// This is f_inf in the paper.
    pub(crate) reward_proportion: Perbill,
    /// When to disable offenders.
    pub(crate) disable_strategy: DisableStrategy,
}

/// Computes a slash of a validator and nominators. It returns an unapplied
/// record to be applied at some later point. Slashing metadata is updated in storage,
/// since unapplied records are only rarely intended to be dropped.
///
/// The pending slash record returned does not have initialized reporters. Those have
/// to be set at a higher level, if any.
pub(crate) fn compute_slash<T: Config>(
    params: SlashParams<T>,
) -> Option<UnappliedSlash<T::AccountId, BalanceOf<T>>> {
    let SlashParams {
        controller,
        slash,
        exposure,
        slash_session,
        window_start,
        now,
        reward_proportion,
        disable_strategy: _,
    } = params.clone();

    log::trace!("compute_slash:[{:#?}] - Slash-[{:#?}]", line!(), slash);

    // is the slash amount here a maximum for the era?
    let own_slash = slash * exposure.bond;
    if slash * exposure.total == Zero::zero() {
        log::trace!(
            "compute_slash:[{:#?}] - ValidatorSS[ B-[{:#?}] | T-[{:#?}]] - Nop",
            line!(),
            exposure.bond,
            exposure.total,
        );
        // kick out the validator even if they won't be slashed,
        // as long as the misbehavior is from their most recent slashing span.
        kick_out_if_recent::<T>(params);
        return None;
    }

    let (prior_slash_p, _era_slash) =
        <Pallet<T> as Store>::ValidatorSlashInSession::get(&slash_session, controller)
            .unwrap_or((Perbill::zero(), Zero::zero()));

    // compare slash proportions rather than slash values to avoid issues due to rounding
    // error.
    if slash.deconstruct() > prior_slash_p.deconstruct() {
        <Pallet<T> as Store>::ValidatorSlashInSession::insert(
            &slash_session,
            controller,
            &(slash, own_slash),
        );
    } else {
        // we slash based on the max in era - this new event is not the max,
        // so neither the validator or any nominators will need an update.
        //
        // this does lead to a divergence of our system from the paper, which
        // pays out some reward even if the latest report is not max-in-era.
        // we opt to avoid the nominator lookups and edits and leave more rewards
        // for more drastic misbehavior.
        return None;
    }

    // apply slash to validator.
    let mut spans = fetch_spans::<T>(controller, window_start, reward_proportion);

    let target_span = spans.compare_and_update_span_slash(slash_session, own_slash);

    if target_span == Some(spans.span_index()) {
        // misbehavior occurred within the current slashing span - take appropriate
        // actions.

        // chill the validator - it misbehaved in the current span and should
        // not continue in the next election. also end the slashing span.
        spans.end_span(now);
        log::trace!(
            "compute_slash:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
            line!(),
            spans.span_index(),
            now
        );
        let _ = <Pallet<T>>::validator_deactivate(params.controller);

        // make sure to disable validator till the end of this session
        let _ = T::SessionInterface::disable_validator(params.controller);
    }

    // apply slash to Nominator.
    let mut nominators_slashed = Vec::new();
    spans.paid_out = spans.paid_out.saturating_add(slash_nominators::<T>(
        params,
        prior_slash_p,
        &mut nominators_slashed,
    ));

    Some(UnappliedSlash {
        validator: controller.clone(),
        own: spans.slash_of,
        others: nominators_slashed,
        reporters: Vec::new(),
        payout: spans.paid_out,
    })
}

/// doesn't apply any slash, but kicks out the validator if the misbehavior is from
/// the most recent slashing span.
fn kick_out_if_recent<T: Config>(params: SlashParams<T>) {
    log::trace!("kick_out_if_recent:[{:#?}]", line!());

    let mut spans = fetch_spans::<T>(
        params.controller,
        params.window_start,
        params.reward_proportion,
    );

    if spans.era_span(params.slash_session).map(|s| s.index) == Some(spans.span_index()) {
        spans.end_span(params.now);
        log::trace!(
            "kick_out_if_recent:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
            line!(),
            spans.span_index(),
            params.now,
        );
        let _ = <Pallet<T>>::validator_deactivate(params.controller);

        // make sure to disable validator till the end of this session
        let _ = T::SessionInterface::disable_validator(params.controller);
    }
}

/// Slash nominators. Accepts general parameters and the prior slash percentage of the validator.
///
/// Returns the amount of reward to pay out.
fn slash_nominators<T: Config>(
    params: SlashParams<T>,
    prior_slash_p: Perbill,
    nominators_slashed: &mut Vec<(T::AccountId, BalanceOf<T>)>,
) -> BalanceOf<T> {
    let SlashParams {
        controller: _,
        slash,
        exposure,
        slash_session,
        window_start,
        now,
        reward_proportion,
        disable_strategy: _,
    } = params;

    let mut reward_payout: BalanceOf<T> = Zero::zero();
    nominators_slashed.reserve(exposure.nominators.len());

    for nominator in &exposure.nominators {
        let controller = &nominator.owner;

        // the era slash of a nominator always grows, if the validator
        // had a new max slash for the era.
        let era_slash = {
            let own_slash_prior = prior_slash_p * nominator.amount;
            let own_slash_by_validator = slash * nominator.amount;
            let own_slash_difference = own_slash_by_validator.saturating_sub(own_slash_prior);

            let mut era_slash =
                <Pallet<T> as Store>::NominatorSlashInSession::get(&slash_session, controller)
                    .unwrap_or_else(|| Zero::zero());

            era_slash = era_slash.saturating_add(own_slash_difference);

            <Pallet<T> as Store>::NominatorSlashInSession::insert(
                &slash_session,
                controller,
                &era_slash,
            );

            era_slash
        };

        // compare the era slash against other eras in the same span.
        let mut spans = fetch_spans::<T>(controller, window_start, reward_proportion);

        let target_span = spans.compare_and_update_span_slash(slash_session, era_slash);

        if target_span == Some(spans.span_index()) {
            // End the span, but don't chill the nominator. its nomination
            // on this validator will be ignored in the future.
            spans.end_span(now);
            log::trace!(
                "slash_nominators:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
                line!(),
                spans.span_index(),
                now
            );
        }

        reward_payout = reward_payout.saturating_add(spans.paid_out);
        nominators_slashed.push((controller.clone(), spans.slash_of));
    }

    reward_payout
}

/// helper struct for managing a set of spans we are currently inspecting.
/// writes alterations to disk on drop, but only if a slash has been carried out.
///
/// NOTE: alterations to slashing metadata should not be done after this is dropped.
/// dropping this struct applies any necessary slashes, which can lead to free balance
/// being 0, and the account being garbage-collected -- a dead account should get no new
/// metadata.
struct InspectingSpans<'a, T: Config + 'a> {
    dirty: bool,
    window_start: SessionIndex,
    controller: &'a T::AccountId,
    spans: SlashingSpans,
    paid_out: BalanceOf<T>,
    slash_of: BalanceOf<T>,
    reward_proportion: Perbill,
    _marker: sp_std::marker::PhantomData<T>,
}

/// fetches the slashing spans record for a controller account, initializing it if necessary.
fn fetch_spans<'a, T: Config + 'a>(
    controller: &'a T::AccountId,
    window_start: SessionIndex,
    reward_proportion: Perbill,
) -> InspectingSpans<'a, T> {
    let spans = <Pallet<T> as Store>::SlashingSpans::get(controller).unwrap_or_else(|| {
        let spans = SlashingSpans::new(window_start);
        <Pallet<T> as Store>::SlashingSpans::insert(controller, &spans);
        spans
    });

    InspectingSpans {
        dirty: false,
        window_start,
        controller,
        spans,
        slash_of: Zero::zero(),
        paid_out: Zero::zero(),
        reward_proportion,
        _marker: sp_std::marker::PhantomData,
    }
}

impl<'a, T: 'a + Config> InspectingSpans<'a, T> {
    fn span_index(&self) -> SpanIndex {
        self.spans.span_index
    }

    fn end_span(&mut self, now: SessionIndex) {
        self.dirty = self.spans.end_span(now) || self.dirty;
    }

    /// add some value to the slash of the staker.
    /// invariant: the staker is being slashed for non-zero value here
    /// although `amount` may be zero, as it is only a difference.
    fn add_slash(&mut self, amount: BalanceOf<T>, slash_session: SessionIndex) {
        self.slash_of = self.slash_of.saturating_add(amount);
        self.spans.last_nonzero_slash =
            sp_std::cmp::max(self.spans.last_nonzero_slash, slash_session);
    }

    /// find the span index of the given era, if covered.
    fn era_span(&self, era: SessionIndex) -> Option<SlashingSpan> {
        self.spans.iter().find(|span| span.contains_era(era))
    }

    /// compares the slash in an era to the overall current span slash.
    /// if it's higher, applies the difference of the slashes and then updates the span on disk.
    ///
    /// returns the span index of the era where the slash occurred, if any.
    fn compare_and_update_span_slash(
        &mut self,
        slash_session: SessionIndex,
        slash: BalanceOf<T>,
    ) -> Option<SpanIndex> {
        let target_span = self.era_span(slash_session)?;
        let span_slash_key = (self.controller.clone(), target_span.index);
        let mut span_record = <Pallet<T> as Store>::SpanSlash::get(&span_slash_key);
        let mut changed = false;

        let reward = if span_record.slashed < slash {
            // new maximum span slash. apply the difference.
            let difference = slash - span_record.slashed;
            span_record.slashed = slash;

            // compute reward.
            let reward = T::DefaultSlashRewardFraction::get()
                * (self.reward_proportion * slash).saturating_sub(span_record.paid_out);

            log::trace!(
                "compare_and_update_span_slash>[{:#?}] | Reward[{:#?}] | Perc[{:#?}]",
                line!(),
                reward,
                T::DefaultSlashRewardFraction::get(),
            );

            self.add_slash(difference, slash_session);
            changed = true;

            reward
        } else if span_record.slashed == slash {
            // compute reward. no slash difference to apply.
            T::DefaultSlashRewardFraction::get()
                * (self.reward_proportion * slash).saturating_sub(span_record.paid_out)
        } else {
            Zero::zero()
        };

        if !reward.is_zero() {
            changed = true;
            span_record.paid_out = span_record.paid_out.saturating_add(reward);
            self.paid_out = self.paid_out.saturating_add(reward);
        }

        if changed {
            self.dirty = true;
            <Pallet<T> as Store>::SpanSlash::insert(&span_slash_key, &span_record);
        }

        Some(target_span.index)
    }
}

impl<'a, T: 'a + Config> Drop for InspectingSpans<'a, T> {
    fn drop(&mut self) {
        // only update on disk if we slashed this account.
        if !self.dirty {
            return;
        }

        if let Some((start, end)) = self.spans.prune(self.window_start) {
            for span_index in start..end {
                <Pallet<T> as Store>::SpanSlash::remove(&(self.controller.clone(), span_index));
            }
        }

        <Pallet<T> as Store>::SlashingSpans::insert(self.controller, &self.spans);
    }
}

/// Clear slashing metadata for an obsolete session.
pub(crate) fn clear_session_metadata<T: Config>(obsolete_session: SessionIndex) {
    <Pallet<T> as Store>::ValidatorSlashInSession::remove_prefix(&obsolete_session, None);
    <Pallet<T> as Store>::NominatorSlashInSession::remove_prefix(&obsolete_session, None);
}

/// Clear slashing metadata for a dead account.
pub(crate) fn clear_slash_metadata<T: Config>(controller: &T::AccountId) -> DispatchResult {
    let spans = match <Pallet<T>>::slashing_spans(controller) {
        None => return Ok(()),
        Some(s) => s,
    };

    <Pallet<T> as Store>::SlashingSpans::remove(controller);

    // kill slashing-span metadata for account.
    //
    // this can only happen while the account is staked _if_ they are completely slashed.
    // in that case, they may re-bond, but it would count again as span 0. Further ancient
    // slashes would slash into this new bond, since metadata has now been cleared.
    for span in spans.iter() {
        <Pallet<T> as Store>::SpanSlash::remove(&(controller.clone(), span.index));
    }

    Ok(())
}

/// apply the slash to a validator controller account, deducting any missing funds from the reward
/// payout, saturating at 0. this is mildly unfair but also an edge-case that
/// can only occur when overlapping locked funds have been slashed.
fn do_slash_validator<T: Config>(
    controller: &T::AccountId,
    value: BalanceOf<T>,
    reward_payout: &mut BalanceOf<T>,
    slashed_imbalance: &mut NegativeImbalanceOf<T>,
) {
    <Pallet<T> as Store>::ValidatorState::mutate(&controller, |validator_state| {
        if let Some(validator_state) = validator_state {
            let old_active_bond = validator_state.bond;
            let valid_pre_total = validator_state
                .total
                .saturating_sub(validator_state.nomi_bond_total);
            let slashed_value = validator_state.slash(value, T::Currency::minimum_balance());

            log::trace!(
                "do_slash_validator:[{:#?}] - [{:#?}] | [{:#?}] | Min [{:#?}]",
                line!(),
                value,
                slashed_value,
                T::Currency::minimum_balance()
            );

            if !slashed_value.is_zero() {
                let pre_balance_stat = T::Currency::free_balance(controller);

                // let (imbalance, missing) = T::Currency::slash_reserved(controller, slashed_value);
                let (imbalance, missing) = T::Currency::slash(controller, slashed_value);
                slashed_imbalance.subsume(imbalance);

                T::Currency::set_lock(
                    T::StakingLockId::get(),
                    &controller,
                    valid_pre_total.saturating_sub(slashed_value),
                    WithdrawReasons::all(),
                );

                // Consider only the value slashed on active bond.
                <Pallet<T> as Store>::Total::mutate(|x| {
                    *x = x.saturating_sub(old_active_bond.saturating_sub(validator_state.bond))
                });

                let cur_balance_stat = T::Currency::free_balance(controller);

                log::trace!(
                    "do_slash_validator:[{:#?}] - [{:#?}] | [{:#?}]",
                    line!(),
                    pre_balance_stat,
                    cur_balance_stat,
                );

                if !missing.is_zero() {
                    // deduct overslash from the reward payout
                    *reward_payout = reward_payout.saturating_sub(missing);
                }

                if validator_state.is_active() {
                    <Pallet<T>>::update_validators_pool(controller.clone(), validator_state.total);
                }

                // trigger the event
                <Pallet<T>>::deposit_event(Event::Slash(controller.clone(), slashed_value));
            }
        }
    });
}

fn do_slash_nominator<T: Config>(
    controller: &T::AccountId,
    validator: &T::AccountId,
    value: BalanceOf<T>,
    reward_payout: &mut BalanceOf<T>,
    slashed_imbalance: &mut NegativeImbalanceOf<T>,
) {
    <Pallet<T> as Store>::NominatorState::mutate(&controller, |nominator_state| {
        if let Some(nominator_state) = nominator_state {
            let old_active_bond = nominator_state.active_bond;

            let slashed_value = nominator_state.slash_nomination(
                validator.clone(),
                value,
                T::Currency::minimum_balance(),
            );

            log::trace!(
                "do_slash_nominator:[{:#?}] - [{:#?}] | [{:#?}] | Min [{:#?}]",
                line!(),
                value,
                slashed_value,
                T::Currency::minimum_balance(),
            );

            if !slashed_value.is_zero() {
                let pre_balance_stat = T::Currency::free_balance(controller);

                <Pallet<T> as Store>::ValidatorState::mutate(&validator, |validator_state| {
                    if let Some(validator_state) = validator_state {
                        validator_state.dec_nominator(controller.clone(), slashed_value);
                        if validator_state.is_active() {
                            <Pallet<T>>::update_validators_pool(
                                validator.clone(),
                                validator_state.total,
                            );
                        }
                    }
                });

                // let (imbalance, missing) = T::Currency::slash_reserved(controller, slashed_value);
                let (imbalance, missing) = T::Currency::slash(controller, slashed_value);
                slashed_imbalance.subsume(imbalance);

                T::Currency::set_lock(
                    T::StakingLockId::get(),
                    &controller,
                    nominator_state.total,
                    WithdrawReasons::all(),
                );

                // Consider only the value slashed on active bond.
                <Pallet<T> as Store>::Total::mutate(|x| {
                    *x = x
                        .saturating_sub(old_active_bond.saturating_sub(nominator_state.active_bond))
                });

                let cur_balance_stat = T::Currency::free_balance(controller);

                log::trace!(
                    "do_slash_nominator:[{:#?}] - [{:#?}] | [{:#?}]",
                    line!(),
                    pre_balance_stat,
                    cur_balance_stat,
                );

                if !missing.is_zero() {
                    // deduct overslash from the reward payout
                    *reward_payout = reward_payout.saturating_sub(missing);
                }

                // trigger the event
                <Pallet<T>>::deposit_event(Event::Slash(controller.clone(), slashed_value));
            }
        }
    });
}

/// Apply a previously-unapplied slash.
pub(crate) fn apply_slash<T: Config>(unapplied_slash: UnappliedSlash<T::AccountId, BalanceOf<T>>) {
    let mut slashed_imbalance = NegativeImbalanceOf::<T>::zero();
    let mut reward_payout = unapplied_slash.payout;

    do_slash_validator::<T>(
        &unapplied_slash.validator,
        unapplied_slash.own,
        &mut reward_payout,
        &mut slashed_imbalance,
    );

    for &(ref nominator, nominator_slash) in &unapplied_slash.others {
        do_slash_nominator::<T>(
            &nominator,
            &unapplied_slash.validator,
            nominator_slash,
            &mut reward_payout,
            &mut slashed_imbalance,
        );
    }

    <Pallet<T>>::validator_stake_reconciliation(&unapplied_slash.validator);

    pay_reporters::<T>(reward_payout, slashed_imbalance, &unapplied_slash.reporters);
}

/// Apply a reward payout to some reporters, paying the rewards out of the slashed imbalance.
fn pay_reporters<T: Config>(
    reward_payout: BalanceOf<T>,
    slashed_imbalance: NegativeImbalanceOf<T>,
    reporters: &[T::AccountId],
) {
    if reward_payout.is_zero() || reporters.is_empty() {
        // nobody to pay out to or nothing to pay;
        // just treat the whole value as slashed.
        T::Slash::on_unbalanced(slashed_imbalance);
        return;
    }

    // take rewards out of the slashed imbalance.
    let reward_payout = reward_payout.min(slashed_imbalance.peek());
    let (mut reward_payout, mut value_slashed) = slashed_imbalance.split(reward_payout);

    let per_reporter = reward_payout.peek() / (reporters.len() as u32).into();
    for reporter in reporters {
        let (reporter_reward, rest) = reward_payout.split(per_reporter);
        reward_payout = rest;
        let reporter_reward_peek = reporter_reward.peek();

        // this cancels out the reporter reward imbalance internally, leading
        // to no change in total issuance.
        T::Currency::resolve_creating(reporter, reporter_reward);

        <Pallet<T>>::deposit_event(Event::PayReporterReward(
            reporter.clone(),
            reporter_reward_peek,
        ));
    }

    // the rest goes to the on-slash imbalance handler (e.g. treasury)
    value_slashed.subsume(reward_payout); // remainder of reward division remains.
    T::Slash::on_unbalanced(value_slashed);
}
