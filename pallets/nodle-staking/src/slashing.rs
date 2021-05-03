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

// use super::{
// 	EraIndex, Config, Module, Store, BalanceOf, Exposure, Perbill, SessionInterface,
// 	NegativeImbalanceOf, Error, SessionIndex, UnappliedSlash,
// };
use super::{
	Config, Pallet, BalanceOf, SessionInterface,
	NegativeImbalanceOf, Error, ValidatorSnapshot,
	UnappliedSlash,
};
use sp_runtime::{
	traits::{Zero, Saturating},
	Perbill, RuntimeDebug, DispatchResult
};
use frame_support::{
	StorageMap, StorageDoubleMap, ensure,
	traits::{Currency, OnUnbalanced, Imbalance},
};
use sp_std::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use sp_staking::SessionIndex;

/// The proportion of the slashing reward to be paid out on the first slashing detection.
/// This is f_1 in the paper.
const REWARD_F1: Perbill = Perbill::from_percent(50);

/// The index of a slashing span - unique to each stash.
pub type SpanIndex = u32;

// A range of start..end eras for a slashing span.
#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct SlashingSpan {
	pub(crate) index: SpanIndex,
	pub(crate) start: SessionIndex,
	pub(crate) length: Option<SessionIndex>, // the ongoing slashing span has indeterminate length.
}

impl SlashingSpan {
	fn contains_era(&self, era: SessionIndex) -> bool {
		self.start <= era && self.length.map_or(true, |l| self.start + l > era)
	}
}

/// An encoding of all of a nominator's slashing spans.
#[derive(Encode, Decode, RuntimeDebug)]
pub struct SlashingSpans {
	// the index of the current slashing span of the nominator. different for
	// every stash, resets when the account hits free balance 0.
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
	// creates a new record of slashing spans for a stash, starting at the beginning
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
		let next_start = now + 1;
		if next_start <= self.last_start { return false }

		let last_length = next_start - self.last_start;
		self.prior.insert(0, last_length);
		self.last_start = next_start;
		self.span_index += 1;
		true
	}

	// an iterator over all slashing spans in _reverse_ order - most recent first.
	pub(crate) fn iter(&'_ self) -> impl Iterator<Item = SlashingSpan> + '_ {
		let mut last_start = self.last_start;
		let mut index = self.span_index;
		let last = SlashingSpan { index, start: last_start, length: None };
		let prior = self.prior.iter().cloned().map(move |length| {
			let start = last_start - length;
			last_start = start;
			index -= 1;

			SlashingSpan { index, start, length: Some(length) }
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
		let old_idx = self.iter()
			.skip(1) // skip ongoing span.
			.position(|span| span.length.map_or(false, |len| span.start + len <= window_start));

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

/// A slashing-span record for a particular stash.
#[derive(Encode, Decode, Default)]
pub struct SpanRecord<Balance> {
	pub slashed: Balance,
	pub paid_out: Balance,
}

impl<Balance> SpanRecord<Balance> {
	/// The value of stash balance slashed in this span.
	#[cfg(test)]
	pub(crate) fn amount_slashed(&self) -> &Balance {
		&self.slashed
	}
}

/// Parameters for performing a slash.
#[derive(Clone)]
pub(crate) struct SlashParams<'a, T: 'a + Config> {
	/// The stash account being slashed.
	pub(crate) stash: &'a T::AccountId,
	/// The proportion of the slash.
	pub(crate) slash: Perbill,
	/// The exposure of the stash and all nominators.
	pub(crate) exposure: &'a ValidatorSnapshot<T::AccountId, BalanceOf<T>>,
	/// The era where the offence occurred.
	pub(crate) slash_era: SessionIndex,
	/// The first era in the current bonding period.
	pub(crate) window_start: SessionIndex,
	/// The current era.
	pub(crate) now: SessionIndex,
	/// The maximum percentage of a slash that ever gets paid out.
	/// This is f_inf in the paper.
	pub(crate) reward_proportion: Perbill,
}

/// Clear slashing metadata for a dead account.
pub(crate) fn clear_stash_metadata<T: Config>(
	controller: &T::AccountId,
	num_slashing_spans: u32,
) -> DispatchResult {

	let spans = match <Pallet<T>>::slashing_spans(controller) {
		None => return Ok(()),
		Some(s) => s,
	};

	ensure!(num_slashing_spans as usize >= spans.iter().count(), Error::<T>::IncorrectSlashingSpans);

	// Pallet::<SlashingSpans<T>>::remove(controller);

	// kill slashing-span metadata for account.
	//
	// this can only happen while the account is staked _if_ they are completely slashed.
	// in that case, they may re-bond, but it would count again as span 0. Further ancient
	// slashes would slash into this new bond, since metadata has now been cleared.
	for span in spans.iter() {
		// <Pallet<T>>::SpanSlash::remove(&(controller.clone(), span.index));
		// super::pallet::<SpanSlash<T>>::remove(&(controller.clone(), span.index));
	}

	Ok(())
}

// apply the slash to a stash account, deducting any missing funds from the reward
// payout, saturating at 0. this is mildly unfair but also an edge-case that
// can only occur when overlapping locked funds have been slashed.
pub fn do_slash<T: Config>(
	controller: &T::AccountId,
	value: BalanceOf<T>,
	reward_payout: &mut BalanceOf<T>,
	slashed_imbalance: &mut NegativeImbalanceOf<T>,
) {
	let mut ledger = match <Pallet<T>>::ledger(&controller) {
		Some(ledger) => ledger,
		None => return, // nothing to do.
	};

	let value = ledger.slash(value, T::Currency::minimum_balance());

	if !value.is_zero() {
		let (imbalance, missing) = T::Currency::slash(controller, value);
		slashed_imbalance.subsume(imbalance);

		if !missing.is_zero() {
			// deduct overslash from the reward payout
			*reward_payout = reward_payout.saturating_sub(missing);
		}

		<Pallet<T>>::update_ledger(&controller, &ledger);

		// trigger the event
		<Pallet<T>>::deposit_event(
			super::Event::Slash(controller.clone(), value)
		);
	}
}

/// Apply a previously-unapplied slash.
pub(crate) fn apply_slash<T: Config>(unapplied_slash: UnappliedSlash<T::AccountId, BalanceOf<T>>) {
	let mut slashed_imbalance = NegativeImbalanceOf::<T>::zero();
	let mut reward_payout = unapplied_slash.payout;

	do_slash::<T>(
		&unapplied_slash.validator,
		unapplied_slash.own,
		&mut reward_payout,
		&mut slashed_imbalance,
	);

	for &(ref nominator, nominator_slash) in &unapplied_slash.others {
		do_slash::<T>(
			&nominator,
			nominator_slash,
			&mut reward_payout,
			&mut slashed_imbalance,
		);
	}

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
		return
	}

	// take rewards out of the slashed imbalance.
	let reward_payout = reward_payout.min(slashed_imbalance.peek());
	let (mut reward_payout, mut value_slashed) = slashed_imbalance.split(reward_payout);

	let per_reporter = reward_payout.peek() / (reporters.len() as u32).into();
	for reporter in reporters {
		let (reporter_reward, rest) = reward_payout.split(per_reporter);
		reward_payout = rest;

		// this cancels out the reporter reward imbalance internally, leading
		// to no change in total issuance.
		T::Currency::resolve_creating(reporter, reporter_reward);
	}

	// the rest goes to the on-slash imbalance handler (e.g. treasury)
	value_slashed.subsume(reward_payout); // remainder of reward division remains.
	T::Slash::on_unbalanced(value_slashed);
}
