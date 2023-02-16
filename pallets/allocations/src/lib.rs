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
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::weights::Weight;
use frame_support::{
	ensure,
	pallet_prelude::MaxEncodedLen,
	traits::{tokens::ExistenceRequirement, Contains, Currency, Get},
	BoundedVec, PalletId,
};

use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_arithmetic::traits::UniqueSaturatedInto;
use sp_runtime::{
	traits::{AccountIdConversion, BlockNumberProvider, Bounded, CheckedAdd, CheckedDiv, One, Saturating, Zero},
	DispatchResult, Perbill, RuntimeDebug,
};
use sp_std::prelude::*;
use support::WithAccountId;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// A value placed in storage that represents the current version of the Allocations storage.
// This value is used by the `on_runtime_upgrade` logic to determine whether we run storage
// migration logic. This should match directly with the semantic versions of the Rust crate.
#[cfg(not(tarpaulin))]
#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
enum Releases {
	V0, // Legacy version
	V1, // Adds storage info
}

#[cfg(not(tarpaulin))]
impl Default for Releases {
	fn default() -> Self {
		Releases::V0
	}
}

#[derive(Default, TypeInfo)]
pub struct MintCurve<T: Config> {
	session_period: T::BlockNumber,
	fiscal_period: T::BlockNumber,
	inflation_steps: Vec<Perbill>,
	maximum_supply: BalanceOf<T>,
}

impl<T: Config> MintCurve<T> {
	pub fn new(
		session_period: T::BlockNumber,
		fiscal_period: T::BlockNumber,
		inflation_steps: &[Perbill],
		maximum_supply: BalanceOf<T>,
	) -> Self {
		let valid_session_period = session_period.max(One::one());
		let valid_fiscal_period = fiscal_period.max(valid_session_period);
		Self {
			session_period: valid_session_period,
			fiscal_period: valid_fiscal_period,
			inflation_steps: inflation_steps.to_vec(),
			maximum_supply,
		}
	}

	pub fn calc_session_quota(
		&self,
		n: T::BlockNumber,
		curve_start: T::BlockNumber,
		current_supply: BalanceOf<T>,
	) -> BalanceOf<T> {
		let step: usize = n
			.saturating_sub(curve_start)
			.checked_div(&self.fiscal_period)
			.unwrap_or_else(Bounded::max_value)
			.unique_saturated_into();
		let max_inflation_rate = *self
			.inflation_steps
			.get(step)
			.or_else(|| self.inflation_steps.last())
			.unwrap_or(&Zero::zero());
		let target_increase =
			(self.maximum_supply.saturating_sub(current_supply)).min(max_inflation_rate * current_supply);
		Perbill::from_rational(self.session_period, self.fiscal_period) * target_increase
	}

	pub fn next_quota_renew_schedule(&self, n: T::BlockNumber, curve_start: T::BlockNumber) -> T::BlockNumber {
		Self::next_schedule(n, curve_start, self.session_period)
	}

	pub fn next_quota_calc_schedule(&self, n: T::BlockNumber, curve_start: T::BlockNumber) -> T::BlockNumber {
		Self::next_schedule(n, curve_start, self.fiscal_period)
	}

	#[inline(always)]
	pub fn session_period(&self) -> T::BlockNumber {
		self.session_period
	}

	#[inline(always)]
	pub fn fiscal_period(&self) -> T::BlockNumber {
		self.fiscal_period
	}

	#[inline(always)]
	pub fn maximum_supply(&self) -> BalanceOf<T> {
		self.maximum_supply
	}

	/// Helper function to calculate the very next schedule based on the current block number.
	fn next_schedule(n: T::BlockNumber, curve_start: T::BlockNumber, period: T::BlockNumber) -> T::BlockNumber {
		if n >= curve_start {
			n.saturating_sub(curve_start)
				.checked_div(&period)
				.unwrap_or_else(Bounded::max_value)
				.saturating_add(One::one())
				.saturating_mul(period)
				.saturating_add(curve_start)
		} else {
			let schedule = curve_start.saturating_sub(
				curve_start
					.saturating_sub(n)
					.checked_div(&period)
					.unwrap_or_else(Bounded::max_value)
					.saturating_mul(period),
			);
			if n == schedule {
				n.saturating_add(period)
			} else {
				schedule
			}
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::dispatch::PostDispatchInfo;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type ProtocolFee: Get<Perbill>;
		type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;

		/// Runtime existential deposit
		#[pallet::constant]
		type ExistentialDeposit: Get<BalanceOf<Self>>;

		/// How big a batch can be
		#[pallet::constant]
		type MaxAllocs: Get<u32>;

		type OracleMembers: Contains<Self::AccountId>;

		/// MintCurve acts as an upper bound limiting how much the total token issuance can inflate
		/// over a configured session
		type MintCurve: Get<&'static MintCurve<Self>>;

		/// Provide access to the block number that should be used in mint curve calculations. For
		/// example those who use this pallet for a parachain may decide to use the block creation
		/// pace of the relay chain for timing.
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = Self::BlockNumber>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Optimized allocation call, which will batch allocations of various amounts
		/// and destinations and together. This allow us to be much more efficient and thus
		/// increase our chain's capacity in handling these transactions.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::allocate(batch.len().try_into().unwrap_or_else(|_| T::MaxAllocs::get())).saturating_add(T::WeightInfo::checked_update_session_quota()))]
		pub fn batch(
			origin: OriginFor<T>,
			batch: BoundedVec<(T::AccountId, BalanceOf<T>), T::MaxAllocs>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_oracle(origin)?;
			let update_weight = Self::checked_update_session_quota();
			let rewards_len = batch.len().try_into().unwrap_or_else(|_| T::MaxAllocs::get());
			Self::allocate(batch)?;
			let dispatch_info = PostDispatchInfo::from((
				Some(update_weight.saturating_add(T::WeightInfo::allocate(rewards_len))),
				Pays::No,
			));
			Ok(dispatch_info)
		}
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_curve_starting_block())]
		pub fn set_curve_starting_block(
			origin: OriginFor<T>,
			curve_start: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<MintCurveStartingBlock<T>>::put(curve_start);
			Self::update_session_quota_schedules(curve_start);
			Ok(Pays::No.into())
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Function is restricted to oracles only
		OracleAccessDenied,
		/// We are exceeding the session's limit for rewards
		AllocationExceedsSessionQuota,
		/// Amount is too low and will conflict with the ExistentialDeposit parameter
		DoesNotSatisfyExistentialDeposit,
		/// Batch is empty or no issuance is necessary
		BatchEmpty,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Session quota is renewed at the beginning of a new session
		SessionQuotaRenewed,
		/// Session quota is calculated and this new value will be used from the next session
		SessionQuotaCalculated(BalanceOf<T>),
	}

	#[cfg(not(tarpaulin))]
	#[pallet::storage]
	pub(crate) type StorageVersion<T: Config> = StorageValue<_, Releases, ValueQuery>;

	#[cfg(feature = "runtime-benchmarks")]
	#[pallet::storage]
	#[pallet::getter(fn benchmark_oracles)]
	pub type BenchmarkOracles<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, benchmarking::MaxMembers>, ValueQuery>;

	/// The transitional allocation quota that is left for the current session.
	///
	/// This will be renewed on a new allocation session
	#[pallet::storage]
	#[pallet::getter(fn session_quota)]
	pub(crate) type SessionQuota<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// The next session's allocation quota, in other words, the top up that is coming for
	/// `SessionQuota`.
	///
	/// The next session quota is calculated from the targeted max inflation rates for the current
	/// fiscal period and is renewed on a new fiscal period.
	#[pallet::storage]
	#[pallet::getter(fn next_session_quota)]
	pub(crate) type NextSessionQuota<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// The block in or after which the the session quota should be renewed
	#[pallet::storage]
	#[pallet::getter(fn quota_renew_schedule)]
	pub(crate) type SessionQuotaRenewSchedule<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	/// The block in or after which the the session quota should be calculated
	#[pallet::storage]
	#[pallet::getter(fn quota_calc_schedule)]
	pub(crate) type SessionQuotaCalculationSchedule<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	/// The block from which the mint curve should be considered starting its first inflation step
	#[pallet::storage]
	#[pallet::getter(fn mint_curve_starting_block)]
	pub(crate) type MintCurveStartingBlock<T: Config> = StorageValue<_, T::BlockNumber, OptionQuery>;
}

impl<T: Config> Pallet<T> {
	pub fn is_oracle(who: T::AccountId) -> bool {
		#[cfg(feature = "runtime-benchmarks")]
		return T::OracleMembers::contains(&who) || Self::benchmark_oracles().contains(&who);
		#[cfg(not(feature = "runtime-benchmarks"))]
		return T::OracleMembers::contains(&who);
	}

	fn ensure_oracle(origin: T::RuntimeOrigin) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), Error::<T>::OracleAccessDenied);
		Ok(())
	}

	fn allocate(batch: BoundedVec<(T::AccountId, BalanceOf<T>), T::MaxAllocs>) -> DispatchResult {
		ensure!(batch.len() > Zero::zero(), Error::<T>::BatchEmpty);

		// sanity checks
		let min_alloc = T::ExistentialDeposit::get().saturating_mul(2u32.into());
		let mut full_issuance: BalanceOf<T> = Zero::zero();
		for (_account, amount) in batch.iter() {
			ensure!(amount >= &min_alloc, Error::<T>::DoesNotSatisfyExistentialDeposit,);

			// overflow, so too many coins to allocate
			full_issuance = full_issuance
				.checked_add(amount)
				.ok_or(Error::<T>::AllocationExceedsSessionQuota)?;
		}

		let session_quota = <SessionQuota<T>>::get();
		ensure!(
			full_issuance <= session_quota,
			Error::<T>::AllocationExceedsSessionQuota
		);

		<SessionQuota<T>>::put(session_quota.saturating_sub(full_issuance));

		// allocate the coins to the proxy account
		T::Currency::resolve_creating(
			&T::PalletId::get().into_account_truncating(),
			T::Currency::issue(full_issuance),
		);

		// send to accounts, unfortunately we need to loop again
		let mut full_protocol: BalanceOf<T> = Zero::zero();
		for (account, amount) in batch.iter().cloned() {
			let amount_for_protocol = T::ProtocolFee::get() * amount;
			let amount_for_grantee = amount.saturating_sub(amount_for_protocol);
			T::Currency::transfer(
				&T::PalletId::get().into_account_truncating(),
				&account,
				amount_for_grantee,
				ExistenceRequirement::KeepAlive,
			)?;
			full_protocol = full_protocol.saturating_add(amount_for_protocol);
		}

		// send protocol fees
		T::Currency::transfer(
			&T::PalletId::get().into_account_truncating(),
			&T::ProtocolFeeReceiver::account_id(),
			full_protocol,
			ExistenceRequirement::AllowDeath,
		)?;

		Ok(())
	}

	/// Use the block number provider and recalculate and/or renew the session quota if it's time
	/// for doing that based on the configured schedules for these actions.
	/// Return the weight of the call.
	fn checked_update_session_quota() -> Weight {
		let n = T::BlockNumberProvider::current_block_number();
		// Storage: ParachainSystem ValidationData (r:1 w:0)
		let read_block_number_weight = T::DbWeight::get().reads(1);

		let calc_quota_weight = Self::checked_calc_session_quota(n);
		let renew_quota_weight = Self::checked_renew_session_quota(n);

		read_block_number_weight
			.saturating_add(calc_quota_weight)
			.saturating_add(renew_quota_weight)
	}

	/// Update both the quota renewal and re-calculation schedules based on the given starting block
	/// for the curve.
	fn update_session_quota_schedules(curve_start: T::BlockNumber) {
		let n = T::BlockNumberProvider::current_block_number();
		Self::update_session_quota_calculation_schedule(n, curve_start);
		Self::update_session_quota_renew_schedule(n, curve_start);
	}

	/// Calculate the session quota and update the corresponding storage only once during a fiscal
	/// period.
	/// Return the weight of the call.
	fn checked_calc_session_quota(n: T::BlockNumber) -> Weight {
		if n >= <SessionQuotaCalculationSchedule<T>>::get() {
			let curve_start = Self::curve_start_or(n);
			Self::update_session_quota_calculation_schedule(n, curve_start);
			let session_quota = T::MintCurve::get().calc_session_quota(n, curve_start, T::Currency::total_issuance());
			<NextSessionQuota<T>>::put(session_quota);
			Self::deposit_event(Event::SessionQuotaCalculated(session_quota));
			T::WeightInfo::calc_quota()
		} else {
			// Storage: Allocations SessionQuotaCalculationSchedule (r:1 w:0)
			T::DbWeight::get().reads(1)
		}
	}

	/// Renew the session quota and update the corresponding storage only once during a session
	/// period.
	/// Return the weight of the call.
	fn checked_renew_session_quota(n: T::BlockNumber) -> Weight {
		if n >= <SessionQuotaRenewSchedule<T>>::get() {
			let curve_start = Self::curve_start_or(n);
			Self::update_session_quota_renew_schedule(n, curve_start);
			<SessionQuota<T>>::put(<NextSessionQuota<T>>::get());
			Self::deposit_event(Event::SessionQuotaRenewed);
			T::WeightInfo::renew_quota()
		} else {
			// Storage: Allocations SessionQuotaRenewSchedule (r:1 w:0)
			T::DbWeight::get().reads(1)
		}
	}

	/// Update the schedule for calculating the session quota.
	fn update_session_quota_calculation_schedule(n: T::BlockNumber, curve_start: T::BlockNumber) {
		let next_schedule = T::MintCurve::get().next_quota_calc_schedule(n, curve_start);
		<SessionQuotaCalculationSchedule<T>>::put(next_schedule);
	}

	/// Update the schedule for renewing (refilling the bucket) for the session quota.
	fn update_session_quota_renew_schedule(n: T::BlockNumber, curve_start: T::BlockNumber) {
		let next_schedule = T::MintCurve::get().next_quota_renew_schedule(n, curve_start);
		<SessionQuotaRenewSchedule<T>>::put(next_schedule);
	}

	/// Return the mint curve starting block number or if it's not set before return `n` itself
	/// while setting the mint curve starting block to n.
	fn curve_start_or(n: T::BlockNumber) -> T::BlockNumber {
		<MintCurveStartingBlock<T>>::get().unwrap_or_else(|| {
			<MintCurveStartingBlock<T>>::put(n);
			n
		})
	}
}
