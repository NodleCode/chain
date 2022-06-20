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

use super::{BalanceOf, Config, Error, Event, NegativeImbalanceOf, Pallet, Store};
use crate::hooks::SessionInterface;
use crate::types::{SpanIndex, ValidatorSnapshot};
use codec::{Decode, Encode};
use derivative::Derivative;
use frame_support::{
	bounded_vec,
	pallet_prelude::MaxEncodedLen,
	traits::{Currency, Get, Imbalance, LockableCurrency, OnUnbalanced, WithdrawReasons},
	BoundedVec,
};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Saturating, Zero},
	DispatchResult, Perbill,
};
use sp_staking::{offence::DisableStrategy, SessionIndex};
use sp_std::{
	cmp::{max, Ordering},
	fmt::Debug,
	marker::PhantomData,
	vec::Vec,
};

// A range of start..end eras for a slashing span.
#[derive(Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct SlashingSpan {
	index: SpanIndex,
	start: SessionIndex,
	length: Option<SessionIndex>, // the ongoing slashing span has indeterminate length.
}

impl SlashingSpan {
	fn contains_era(&self, era: SessionIndex) -> bool {
		self.start <= era && self.length.map_or(true, |l| self.start.saturating_add(l) > era)
	}
}

/// An encoding of all of a nominator's slashing spans.
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(T, S))]
pub struct SlashingSpans<T: Config, S: Get<u32>> {
	// the index of the current slashing span of the nominator. different for
	// every controller, resets when the account hits free balance 0.
	span_index: SpanIndex,
	// the start era of the most recent (ongoing) slashing span.
	last_start: SessionIndex,
	// the last era at which a non-zero slash occurred.
	last_nonzero_slash: SessionIndex,
	// all prior slashing spans' start indices, in reverse order (most recent first)
	// encoded as offsets relative to the slashing span after it.
	prior: BoundedVec<SessionIndex, S>,
	// dummy marker field
	_marker: PhantomData<T>,
}

impl<T, S> MaxEncodedLen for SlashingSpans<T, S>
where
	T: Config,
	S: Get<u32>,
{
	fn max_encoded_len() -> usize {
		S::get() as usize
	}
}

impl<T, S> SlashingSpans<T, S>
where
	T: Config,
	S: Get<u32>,
{
	// creates a new record of slashing spans for a controller, starting at the beginning
	// of the bonding period, relative to now.
	pub(crate) fn new(window_start: SessionIndex) -> Self {
		let inner: Vec<SessionIndex> = Vec::with_capacity(S::get() as usize);
		let prior_default = BoundedVec::try_from(inner).expect("SlashingSpans Failed To Create Default");
		SlashingSpans {
			span_index: 0,
			last_start: window_start,
			// initialize to zero, as this structure is lazily created until
			// the first slash is applied. setting equal to `window_start` would
			// put a time limit on nominations.
			last_nonzero_slash: 0,
			prior: prior_default,
			_marker: <PhantomData<T>>::default(),
		}
	}

	// update the slashing spans to reflect the start of a new span at the era after `now`
	// returns `true` if a new span was started, `false` otherwise. `false` indicates
	// that internal state is unchanged.
	pub(crate) fn end_span(&mut self, now: SessionIndex) -> Result<bool, ()> {
		let next_start = now.saturating_add(1);
		if next_start <= self.last_start {
			return Ok(false);
		}
		let last_length = next_start.saturating_sub(self.last_start);
		self.prior.try_insert(0, last_length).map_err(|_| {
			log::error!("Slashing Spans Overflow Error");
			()
		})?;
		self.last_start = next_start;
		self.span_index = self.span_index.saturating_add(1);
		Ok(true)
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

		let span_prior: Vec<SessionIndex> = self.prior.clone().to_vec();

		let prior: Vec<SlashingSpan> = span_prior
			.iter()
			.map(move |length| {
				let start = last_start - length;
				last_start = start;
				index = index.saturating_sub(1);

				SlashingSpan {
					index,
					start,
					length: Some(*length),
				}
			})
			.collect();

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
	fn prune(&mut self, window_start: SessionIndex) -> Result<Option<(SpanIndex, SpanIndex)>, ()> {
		let old_idx = self
			.iter()
			.skip(1) // skip ongoing span.
			.position(|span| span.length.map_or(false, |len| span.start + len <= window_start));

		let mut prior_span = self.prior.to_vec();

		let earliest_span_index = self.span_index - prior_span.len() as SpanIndex;
		let pruned = match old_idx {
			Some(o) => {
				prior_span.truncate(o);
				let new_earliest = self.span_index - self.prior.len() as SpanIndex;
				Some((earliest_span_index, new_earliest))
			}
			None => None,
		};

		self.prior = BoundedVec::try_from(prior_span).map_err(|_| {
			log::error!("Slasing Span prune failure");
			()
		})?;

		// readjust the ongoing span, if it started before the beginning of the window.
		self.last_start = sp_std::cmp::max(self.last_start, window_start);
		Ok(pruned)
	}
}

/// A slashing-span record for a particular controller.
#[derive(Encode, Decode, Default, MaxEncodedLen, TypeInfo)]
pub struct SpanRecord<Balance> {
	slashed: Balance,
	paid_out: Balance,
}

impl<Balance> SpanRecord<Balance> {
	/// The value of controller balance slashed in this span.
	#[cfg(test)]
	pub(crate) fn amount_slashed(&self) -> &Balance {
		&self.slashed
	}
}

/// helper struct for managing a set of spans we are currently inspecting.
/// writes alterations to disk on drop, but only if a slash has been carried out.
///
/// NOTE: alterations to slashing metadata should not be done after this is dropped.
/// dropping this struct applies any necessary slashes, which can lead to free balance
/// being 0, and the account being garbage-collected -- a dead account should get no new
/// metadata.
struct InspectingSpans<T: Config> {
	dirty: bool,
	window_start: SessionIndex,
	controller: T::AccountId,
	spans: SlashingSpans<T, T::MaxSlashSpan>,
	paid_out: BalanceOf<T>,
	slash_of: BalanceOf<T>,
	reward_proportion: Perbill,
	_marker: sp_std::marker::PhantomData<T>,
}

impl<T: Config> InspectingSpans<T> {
	fn span_index(&self) -> SpanIndex {
		self.spans.span_index
	}

	fn end_span(&mut self, now: SessionIndex) {
		self.dirty = match self.spans.end_span(now) {
			Ok(state) => state,
			_ => self.dirty,
		};
	}

	/// add some value to the slash of the staker.
	/// invariant: the staker is being slashed for non-zero value here
	/// although `amount` may be zero, as it is only a difference.
	fn add_slash(&mut self, amount: BalanceOf<T>, slash_session: SessionIndex) {
		self.slash_of = self.slash_of.saturating_add(amount);
		self.spans.last_nonzero_slash = sp_std::cmp::max(self.spans.last_nonzero_slash, slash_session);
	}

	/// find the span index of the given era, if covered.
	fn era_span(&self, era: SessionIndex) -> Option<SlashingSpan> {
		self.spans.iter().find(|span| span.contains_era(era))
	}

	/// compares the slash in an era to the overall current span slash.
	/// if it's higher, applies the difference of the slashes and then updates the span on disk.
	///
	/// returns the span index of the era where the slash occurred, if any.
	fn compare_and_update_span_slash(&mut self, slash_session: SessionIndex, slash: BalanceOf<T>) -> Option<SpanIndex> {
		let target_span = self.era_span(slash_session)?;
		let span_slash_key = (self.controller.clone(), target_span.index);
		let mut span_record = <Pallet<T> as Store>::SpanSlash::get(&span_slash_key);
		let mut changed = false;

		let reward = match span_record.slashed.cmp(&slash) {
			Ordering::Less => {
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
			}
			Ordering::Equal => {
				// compute reward. no slash difference to apply.
				T::DefaultSlashRewardFraction::get()
					* (self.reward_proportion * slash).saturating_sub(span_record.paid_out)
			}
			_ => Zero::zero(),
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

impl<T: Config> Drop for InspectingSpans<T> {
	fn drop(&mut self) {
		// only update on disk if we slashed this account.
		if !self.dirty {
			return;
		}

		if let Ok(Some((start, end))) = self.spans.prune(self.window_start) {
			for span_index in start..end {
				<Pallet<T> as Store>::SpanSlash::remove(&(self.controller.clone(), span_index));
			}
		}

		<Pallet<T> as Store>::SlashingSpans::insert(&self.controller, self.spans.clone());
	}
}

/// Parameters for performing a slash.
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
pub(crate) struct SlashParams<T: Config, MaxNominators: Get<u32>> {
	/// The controller account being slashed.
	pub(crate) controller: T::AccountId,
	/// The proportion of the slash.
	pub(crate) slash: Perbill,
	/// The exposure of the controller and all nominators.
	pub(crate) exposure: ValidatorSnapshot<T, MaxNominators>,
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

impl<T, MaxNominators> SlashParams<T, MaxNominators>
where
	T: Config,
	MaxNominators: Get<u32>,
{
	/// fetches the slashing spans record for a controller account, initializing it if necessary.
	fn fetch_spans(&self) -> InspectingSpans<T> {
		let spans = <Pallet<T> as Store>::SlashingSpans::get(&self.controller).unwrap_or_else(|| {
			let spans = SlashingSpans::new(self.window_start);
			<Pallet<T> as Store>::SlashingSpans::insert(&self.controller, &spans);
			spans
		});

		InspectingSpans {
			dirty: false,
			window_start: self.window_start,
			controller: self.controller.clone(),
			spans: spans,
			slash_of: Zero::zero(),
			paid_out: Zero::zero(),
			reward_proportion: self.reward_proportion,
			_marker: sp_std::marker::PhantomData,
		}
	}

	/// doesn't apply any slash, but kicks out the validator if the misbehavior is from
	/// the most recent slashing span.
	fn kick_out_if_recent(&self) {
		log::trace!("kick_out_if_recent:[{:#?}]", line!());

		let mut inspect_spans = self.fetch_spans();

		if inspect_spans.era_span(self.slash_session).map(|s| s.index) == Some(inspect_spans.span_index()) {
			inspect_spans.end_span(self.now);
			log::trace!(
				"kick_out_if_recent:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
				line!(),
				inspect_spans.span_index(),
				self.now,
			);

			if self.disable_strategy == DisableStrategy::Always {
				match <Pallet<T>>::validator_deactivate(&self.controller) {
					Err(_) => {
						log::error!("kick_out_if_recent:[{:#?}] - validator_deactivate failure", line!());
					}
					Ok(_) => (),
				}

				// make sure to disable validator till the end of this session
				T::SessionInterface::disable_validator(&self.controller);
			}
		}
	}

	/// Computes a slash of a validator and nominators. It returns an unapplied
	/// record to be applied at some later point. Slashing metadata is updated in storage,
	/// since unapplied records are only rarely intended to be dropped.
	///
	/// The pending slash record returned does not have initialized reporters. Those have
	/// to be set at a higher level, if any.
	pub(crate) fn compute_slash(
		&self,
	) -> Result<Option<UnappliedSlash<T, T::MaxNominatorsPerValidator, T::MaxSlashReporters>>, Error<T>> {
		log::trace!("compute_slash:[{:#?}] - Slash-[{:#?}]", line!(), self.slash);

		// is the slash amount here a maximum for the era?
		let own_slash = self.slash * self.exposure.bond;
		if self.slash * self.exposure.total == Zero::zero() {
			log::trace!(
				"compute_slash:[{:#?}] - ValidatorSS[ B-[{:#?}] | T-[{:#?}]] - Nop",
				line!(),
				self.exposure.bond,
				self.exposure.total,
			);
			// kick out the validator even if they won't be slashed,
			// as long as the misbehavior is from their most recent slashing span.
			self.kick_out_if_recent();
			return Ok(None);
		}

		let (prior_slash_p, _era_slash) =
			<Pallet<T> as Store>::ValidatorSlashInSession::get(&self.slash_session, &self.controller)
				.unwrap_or((Perbill::zero(), Zero::zero()));

		// compare slash proportions rather than slash values to avoid issues due to rounding
		// error.
		if self.slash.deconstruct() > prior_slash_p.deconstruct() {
			<Pallet<T> as Store>::ValidatorSlashInSession::insert(
				&self.slash_session,
				&self.controller,
				&(self.slash, own_slash),
			);
		} else {
			// we slash based on the max in era - this new event is not the max,
			// so neither the validator or any nominators will need an update.
			//
			// this does lead to a divergence of our system from the paper, which
			// pays out some reward even if the latest report is not max-in-era.
			// we opt to avoid the nominator lookups and edits and leave more rewards
			// for more drastic misbehavior.
			return Ok(None);
		}

		// apply slash to validator.
		let mut inspect_spans = self.fetch_spans();

		let target_span = inspect_spans.compare_and_update_span_slash(self.slash_session, own_slash);

		if target_span == Some(inspect_spans.span_index()) {
			// misbehavior occurred within the current slashing span - take appropriate
			// actions.

			// chill the validator - it misbehaved in the current span and should
			// not continue in the next election. also end the slashing span.
			inspect_spans.end_span(self.now);
			log::trace!(
				"compute_slash:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
				line!(),
				inspect_spans.span_index(),
				self.now
			);

			if self.disable_strategy != DisableStrategy::Never {
				match <Pallet<T>>::validator_deactivate(&self.controller) {
					Err(_) => {
						log::error!("kick_out_if_recent:[{:#?}] - validator_deactivate failure", line!());
					}
					Ok(_) => (),
				}

				// make sure to disable validator till the end of this session
				T::SessionInterface::disable_validator(&self.controller);
			}
		}

		// apply slash to Nominator.
		let mut nominators_slashed = Vec::new();
		inspect_spans.paid_out = inspect_spans
			.paid_out
			.saturating_add(self.slash_nominators(prior_slash_p, &mut nominators_slashed));

		let mut unapplied_slash: UnappliedSlash<T, T::MaxNominatorsPerValidator, T::MaxSlashReporters> =
			UnappliedSlash::from_default(self.controller.clone(), inspect_spans.slash_of, inspect_spans.paid_out);

		let _ = unapplied_slash.try_update_slashed_nominators(nominators_slashed.as_slice())?;

		Ok(Some(unapplied_slash))
	}

	/// Slash nominators. Accepts general parameters and the prior slash percentage of the
	/// validator.
	///
	/// Returns the amount of reward to pay out.
	fn slash_nominators(
		&self,
		prior_slash_p: Perbill,
		nominators_slashed: &mut Vec<(T::AccountId, BalanceOf<T>)>,
	) -> BalanceOf<T> {
		let mut reward_payout: BalanceOf<T> = Zero::zero();
		nominators_slashed.reserve(self.exposure.nominators.len());

		for nominator in &self.exposure.nominators {
			let controller = &nominator.owner;

			// the era slash of a nominator always grows, if the validator
			// had a new max slash for the era.
			let era_slash = {
				let own_slash_prior = prior_slash_p * nominator.amount;
				let own_slash_by_validator = self.slash * nominator.amount;
				let own_slash_difference = own_slash_by_validator.saturating_sub(own_slash_prior);

				let mut era_slash =
					<Pallet<T> as Store>::NominatorSlashInSession::get(&self.slash_session, self.controller.clone())
						.unwrap_or_else(Zero::zero);

				era_slash = era_slash.saturating_add(own_slash_difference);

				<Pallet<T> as Store>::NominatorSlashInSession::insert(
					&self.slash_session,
					self.controller.clone(),
					&era_slash,
				);

				era_slash
			};

			// compare the era slash against other eras in the same span.
			let mut spans = self.fetch_spans();

			let target_span = spans.compare_and_update_span_slash(self.slash_session, era_slash);

			if target_span == Some(spans.span_index()) {
				// End the span, but don't chill the nominator. its nomination
				// on this validator will be ignored in the future.
				spans.end_span(self.now);
				log::trace!(
					"slash_nominators:[{:#?}] - Call end_span() | SI[{:#?}] | @[{:#?}]",
					line!(),
					spans.span_index(),
					self.now
				);
			}

			reward_payout = reward_payout.saturating_add(spans.paid_out);
			nominators_slashed.push((controller.clone(), spans.slash_of));
		}

		reward_payout
	}
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Clone(bound = "T: Clone"))]
#[derive(Decode, Encode, TypeInfo)]
#[scale_info(skip_type_params(T, MaxNominators, MaxReporters))]
pub struct UnappliedSlash<T: Config, MaxNominators: Get<u32>, MaxReporters: Get<u32>> {
	/// The stash ID of the offending validator.
	pub(crate) validator: T::AccountId,
	/// The validator's own slash.
	pub(crate) own: BalanceOf<T>,
	/// All other slashed stakers and amounts.
	pub(crate) others: BoundedVec<(T::AccountId, BalanceOf<T>), MaxNominators>,
	/// Reporters of the offence; bounty payout recipients.
	pub(crate) reporters: BoundedVec<T::AccountId, MaxReporters>,
	/// The amount of payout.
	pub(crate) payout: BalanceOf<T>,
}

impl<T, MaxNominators, MaxReporters> MaxEncodedLen for UnappliedSlash<T, MaxNominators, MaxReporters>
where
	T: Config,
	MaxNominators: Get<u32>,
	MaxReporters: Get<u32>,
{
	fn max_encoded_len() -> usize {
		max(MaxNominators::get(), MaxReporters::get()) as usize
	}
}

impl<T, MaxNominators, MaxReporters> UnappliedSlash<T, MaxNominators, MaxReporters>
where
	T: Config,
	MaxNominators: Get<u32>,
	MaxReporters: Get<u32>,
{
	pub(crate) fn from_default(validator: T::AccountId, own_slash: BalanceOf<T>, payout: BalanceOf<T>) -> Self {
		let others: BoundedVec<(T::AccountId, BalanceOf<T>), MaxNominators> = bounded_vec![];
		let reporters: BoundedVec<T::AccountId, MaxReporters> = bounded_vec![];
		Self {
			validator,
			own: own_slash,
			others: others,
			reporters: reporters,
			payout: payout,
		}
	}

	pub(crate) fn try_update_slashed_nominators(
		&mut self,
		nominators_slashed: &[(T::AccountId, BalanceOf<T>)],
	) -> Result<(), Error<T>> {
		self.others = BoundedVec::try_from(nominators_slashed.to_vec()).map_err(|_| <Error<T>>::NominationOverflow)?;
		Ok(())
	}

	/// Apply a previously-unapplied slash.
	pub(crate) fn apply_slash(&mut self) {
		let mut slashed_imbalance = NegativeImbalanceOf::<T>::zero();

		self.do_slash_validator(&mut slashed_imbalance);

		for &(ref nominator, nominator_slash) in &self.others.to_vec() {
			self.do_slash_nominator(nominator, nominator_slash, &mut slashed_imbalance);
		}

		match <Pallet<T>>::validator_stake_reconciliation(&self.validator) {
			Err(_) => {
				log::error!(
					"apply_slash:[{:#?}] - Reconciliation failure for Validator[{:#?}]",
					line!(),
					&self.validator
				);
			}
			Ok(_) => (),
		}

		self.pay_reporters(slashed_imbalance);
	}

	/// apply the slash to a validator controller account, deducting any missing funds from the
	/// reward payout, saturating at 0. this is mildly unfair but also an edge-case that
	/// can only occur when overlapping locked funds have been slashed.
	fn do_slash_validator(&mut self, slashed_imbalance: &mut NegativeImbalanceOf<T>) {
		<Pallet<T> as Store>::ValidatorState::mutate(&self.validator, |validator_state| {
			if let Some(validator_state) = validator_state {
				let old_active_bond = validator_state.bond;
				let valid_pre_total = validator_state.total.saturating_sub(validator_state.nomi_bond_total);
				let slashed_value = validator_state.slash(self.own, T::Currency::minimum_balance());

				log::trace!(
					"do_slash_validator:[{:#?}] - [{:#?}] | [{:#?}] | Min [{:#?}]",
					line!(),
					self.own,
					slashed_value,
					T::Currency::minimum_balance()
				);

				if !slashed_value.is_zero() {
					let pre_balance_stat = T::Currency::free_balance(&self.validator);

					let (imbalance, missing) = T::Currency::slash(&self.validator, slashed_value);
					slashed_imbalance.subsume(imbalance);

					T::Currency::set_lock(
						T::StakingLockId::get(),
						&self.validator,
						valid_pre_total.saturating_sub(slashed_value),
						WithdrawReasons::all(),
					);

					// Consider only the value slashed on active bond.
					<Pallet<T> as Store>::Total::mutate(|x| {
						*x = x.saturating_sub(old_active_bond.saturating_sub(validator_state.bond))
					});

					let cur_balance_stat = T::Currency::free_balance(&self.validator);

					log::trace!(
						"do_slash_validator:[{:#?}] - [{:#?}] | [{:#?}]",
						line!(),
						pre_balance_stat,
						cur_balance_stat,
					);

					if !missing.is_zero() {
						// deduct overslash from the reward payout
						self.payout = self.payout.saturating_sub(missing);
					}

					if validator_state.is_active() {
						match <Pallet<T>>::update_validators_pool(&self.validator, validator_state.total) {
							Err(_) => {
								log::error!(
									"do_slash_validator:[{:#?}] - update_validators_pool Overflow Error",
									line!()
								);
							}
							Ok(_) => (),
						}
					}

					// trigger the event
					<Pallet<T>>::deposit_event(Event::Slash(self.validator.clone(), slashed_value));
				}
			}
		});
	}

	fn do_slash_nominator(
		&mut self,
		controller: &T::AccountId,
		value: BalanceOf<T>,
		slashed_imbalance: &mut NegativeImbalanceOf<T>,
	) {
		<Pallet<T> as Store>::NominatorState::mutate(&controller, |nominator_state| {
			if let Some(nominator_state) = nominator_state {
				let old_active_bond = nominator_state.active_bond;

				let slashed_value =
					nominator_state.slash_nomination(&self.validator, value, T::Currency::minimum_balance());

				log::trace!(
					"do_slash_nominator:[{:#?}] - [{:#?}] | [{:#?}] | Min [{:#?}]",
					line!(),
					value,
					slashed_value,
					T::Currency::minimum_balance(),
				);

				if !slashed_value.is_zero() {
					let pre_balance_stat = T::Currency::free_balance(controller);

					<Pallet<T> as Store>::ValidatorState::mutate(&self.validator, |validator_state| {
						if let Some(validator_state) = validator_state {
							match validator_state.dec_nominator(controller, slashed_value) {
								Err(err) => {
									log::error!(
										"do_slash_nominator:[{:#?}] - dec_nominator failure[{:#?}]",
										line!(),
										err,
									);
								}
								Ok(_) => (),
							}

							if validator_state.is_active() {
								match <Pallet<T>>::update_validators_pool(&self.validator, validator_state.total) {
									Err(_) => {
										log::error!(
											"do_slash_nominator:[{:#?}] - update_validators_pool Overflow Error",
											line!()
										);
									}
									Ok(_) => (),
								}
							}
						}
					});

					// let (imbalance, missing) = T::Currency::slash_reserved(controller, slashed_value);
					let (imbalance, missing) = T::Currency::slash(controller, slashed_value);
					slashed_imbalance.subsume(imbalance);

					T::Currency::set_lock(
						T::StakingLockId::get(),
						controller,
						nominator_state.total,
						WithdrawReasons::all(),
					);

					// Consider only the value slashed on active bond.
					<Pallet<T> as Store>::Total::mutate(|x| {
						*x = x.saturating_sub(old_active_bond.saturating_sub(nominator_state.active_bond))
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
						self.payout = self.payout.saturating_sub(missing);
					}

					// trigger the event
					<Pallet<T>>::deposit_event(Event::Slash(controller.clone(), slashed_value));
				}
			}
		});
	}

	/// Apply a reward payout to some reporters, paying the rewards out of the slashed imbalance.
	fn pay_reporters(&mut self, slashed_imbalance: NegativeImbalanceOf<T>) {
		let reporters: Vec<T::AccountId> = self.reporters.clone().to_vec();

		if self.payout.is_zero() || reporters.is_empty() {
			// nobody to pay out to or nothing to pay;
			// just treat the whole value as slashed.
			T::Slash::on_unbalanced(slashed_imbalance);
			return;
		}

		// take rewards out of the slashed imbalance.
		let reward_payout = self.payout.min(slashed_imbalance.peek());
		let (mut reward_payout, mut value_slashed) = slashed_imbalance.split(reward_payout);

		let per_reporter = reward_payout.peek() / (reporters.len() as u32).into();
		for reporter in reporters {
			let (reporter_reward, rest) = reward_payout.split(per_reporter);
			reward_payout = rest;
			let reporter_reward_peek = reporter_reward.peek();

			// this cancels out the reporter reward imbalance internally, leading
			// to no change in total issuance.
			T::Currency::resolve_creating(&reporter, reporter_reward);

			<Pallet<T>>::deposit_event(Event::PayReporterReward(reporter.clone(), reporter_reward_peek));
		}

		// the rest goes to the on-slash imbalance handler (e.g. treasury)
		value_slashed.subsume(reward_payout); // remainder of reward division remains.
		T::Slash::on_unbalanced(value_slashed);
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
