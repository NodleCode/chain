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

mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::{
	ensure,
	traits::{Currency, ExistenceRequirement, LockIdentifier, LockableCurrency, WithdrawReasons},
};
use sp_runtime::{
	traits::{AtLeast32Bit, BlockNumberProvider, CheckedAdd, Saturating, StaticLookup, Zero},
	DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	vec::Vec,
};

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type VestingScheduleOf<T> = VestingSchedule<<T as frame_system::Config>::BlockNumber, BalanceOf<T>>;
pub type ListVestingScheduleOf<T> = Vec<VestingScheduleOf<T>>;
pub type ScheduledGrant<T> = (
	<T as frame_system::Config>::BlockNumber,
	<T as frame_system::Config>::BlockNumber,
	u32,
	BalanceOf<T>,
);
pub type ScheduledItem<T> = (<T as frame_system::Config>::AccountId, Vec<ScheduledGrant<T>>);

/// The vesting schedule.
///
/// Benefits would be granted gradually, `per_period` amount every `period` of blocks
/// after `start`.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct VestingSchedule<BlockNumber, Balance> {
	pub start: BlockNumber,
	pub period: BlockNumber,
	pub period_count: u32,
	pub per_period: Balance,
}

impl<BlockNumber: AtLeast32Bit + Copy, Balance: AtLeast32Bit + Copy> VestingSchedule<BlockNumber, Balance> {
	/// Returns the end of all periods, `None` if calculation overflows.
	pub fn end(&self) -> Option<BlockNumber> {
		self.period
			.checked_mul(&self.period_count.into())?
			.checked_add(&self.start)
	}

	/// Returns all locked amount, `None` if calculation overflows.
	pub fn total_amount(&self) -> Option<Balance> {
		self.per_period.checked_mul(&self.period_count.into())
	}

	/// Returns locked amount for a given `time`.
	///
	/// Note this func assumes schedule is a valid one(non-zero period and non-overflow total
	/// amount), and it should be guaranteed by callers.
	pub fn locked_amount(&self, time: BlockNumber) -> Balance {
		let full = time
			.saturating_sub(self.start)
			.checked_div(&self.period)
			.expect("ensured non-zero period; qed");
		let unrealized = self.period_count.saturating_sub(full.unique_saturated_into());
		self.per_period
			.checked_mul(&unrealized.into())
			.expect("ensured non-overflow total amount; qed")
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		type CancelOrigin: EnsureOrigin<Self::Origin>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		// The block number provider
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claim funds that have been vested so far
		#[pallet::weight(T::WeightInfo::claim())]
		pub fn claim(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let locked_amount = Self::do_claim(&who);

			if locked_amount.is_zero() {
				// No more claimable, clear
				VestingSchedules::<T>::remove(who.clone());
			}

			Self::deposit_event(Event::Claimed(who, locked_amount));
			Ok(().into())
		}

		/// Wire funds to be vested by the receiver
		#[pallet::weight(T::WeightInfo::add_vesting_schedule())]
		pub fn add_vesting_schedule(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			schedule: VestingScheduleOf<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let to = T::Lookup::lookup(dest)?;
			Self::do_add_vesting_schedule(&from, &to, schedule.clone())?;

			Self::deposit_event(Event::VestingScheduleAdded(from, to, schedule));
			Ok(().into())
		}

		/// Cancel all vested schedules for the given user. If there are coins to be
		/// claimed they will be auto claimed for the given user. If `limit_to_free_balance`
		/// is set to true we will not error if the free balance of `who` has less coins
		/// than what was granted and is being revoked (useful if the state was corrupted).
		#[pallet::weight(T::WeightInfo::cancel_all_vesting_schedules())]
		pub fn cancel_all_vesting_schedules(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			funds_collector: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			T::CancelOrigin::try_origin(origin).map(|_| ()).or_else(ensure_root)?;

			let account_with_schedule = T::Lookup::lookup(who)?;
			let account_collector = T::Lookup::lookup(funds_collector)?;

			let locked_amount_left = Self::do_claim(&account_with_schedule);
			let free_balance = T::Currency::free_balance(&account_with_schedule);
			let collectable_funds = locked_amount_left.min(free_balance);

			// we need to remove the lock before doing the transfer to avoid
			// liquidity restrictions
			T::Currency::remove_lock(VESTING_LOCK_ID, &account_with_schedule);
			T::Currency::transfer(
				&account_with_schedule,
				&account_collector,
				collectable_funds,
				ExistenceRequirement::AllowDeath,
			)?;
			VestingSchedules::<T>::remove(account_with_schedule.clone());

			Self::deposit_event(Event::VestingSchedulesCanceled(account_with_schedule));

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added new vesting schedule \[from, to, vesting_schedule\]
		VestingScheduleAdded(T::AccountId, T::AccountId, VestingScheduleOf<T>),
		/// Claimed vesting \[who, locked_amount\]
		Claimed(T::AccountId, BalanceOf<T>),
		/// Canceled all vesting schedules \[who\]
		VestingSchedulesCanceled(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		ZeroVestingPeriod,
		ZeroVestingPeriodCount,
		NumOverflow,
		InsufficientBalanceToLock,
		EmptySchedules,
		VestingToSelf,
	}

	#[pallet::storage]
	#[pallet::getter(fn vesting_schedules)]
	pub type VestingSchedules<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<VestingScheduleOf<T>>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vesting: Vec<ScheduledItem<T>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				vesting: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let grants = self
				.vesting
				.iter()
				.map(|(ref who, schedules)| {
					(
						who.clone(),
						schedules
							.iter()
							.map(|&(start, period, period_count, per_period)| VestingSchedule {
								start,
								period,
								period_count,
								per_period,
							})
							.collect::<Vec<_>>(),
					)
				})
				.collect::<Vec<_>>();

			// Create the required coins at genesis and add to storage
			grants.iter().for_each(|(ref who, schedules)| {
				let total_grants = schedules.iter().fold(Zero::zero(), |acc: BalanceOf<T>, s| {
					acc.saturating_add(s.locked_amount(Zero::zero()))
				});

				T::Currency::resolve_creating(who, T::Currency::issue(total_grants));
				T::Currency::set_lock(VESTING_LOCK_ID, who, total_grants, WithdrawReasons::all());
				<VestingSchedules<T>>::insert(who, schedules);
			});
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

pub const VESTING_LOCK_ID: LockIdentifier = *b"nvesting";

impl<T: Config> Pallet<T> {
	fn do_claim(who: &T::AccountId) -> BalanceOf<T> {
		let locked = Self::locked_balance(who);
		if locked.is_zero() {
			T::Currency::remove_lock(VESTING_LOCK_ID, who);
		} else {
			T::Currency::set_lock(VESTING_LOCK_ID, who, locked, WithdrawReasons::all());
		}
		locked
	}

	/// Returns locked balance based on current block number.
	fn locked_balance(who: &T::AccountId) -> BalanceOf<T> {
		let now = T::BlockNumberProvider::current_block_number();
		Self::vesting_schedules(who)
            .iter()
            .fold(Zero::zero(), |acc, s| {
                acc.checked_add(&s.locked_amount(now)).expect(
                    "locked amount is a balance and can't be higher than the total balance stored inside the same integer type; qed",
                )
            })
	}

	fn do_add_vesting_schedule(
		from: &T::AccountId,
		to: &T::AccountId,
		schedule: VestingScheduleOf<T>,
	) -> DispatchResult {
		ensure!(from != to, Error::<T>::VestingToSelf);

		let schedule_amount = Self::ensure_valid_vesting_schedule(&schedule)?;
		let total_amount = Self::locked_balance(to)
			.checked_add(&schedule_amount)
			.ok_or(Error::<T>::NumOverflow)?;

		T::Currency::transfer(from, to, schedule_amount, ExistenceRequirement::AllowDeath)?;
		T::Currency::set_lock(VESTING_LOCK_ID, to, total_amount, WithdrawReasons::all());
		<VestingSchedules<T>>::mutate(to, |v| (*v).push(schedule));

		Ok(())
	}

	/// Returns `Ok(amount)` if valid schedule, or error.
	fn ensure_valid_vesting_schedule(schedule: &VestingScheduleOf<T>) -> Result<BalanceOf<T>, Error<T>> {
		ensure!(!schedule.period.is_zero(), Error::<T>::ZeroVestingPeriod);
		ensure!(!schedule.period_count.is_zero(), Error::<T>::ZeroVestingPeriodCount);
		ensure!(schedule.end().is_some(), Error::<T>::NumOverflow);

		schedule.total_amount().ok_or(Error::<T>::NumOverflow)
	}
}
