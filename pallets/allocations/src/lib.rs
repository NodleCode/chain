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

use frame_support::{
	dispatch::Weight,
	ensure,
	migration::remove_storage_prefix,
	traits::{tokens::ExistenceRequirement, ChangeMembers, Currency, Get, InitializeMembers},
	transactional, BoundedVec, PalletId,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use support::WithAccountId;

use sp_runtime::{
	traits::{AccountIdConversion, CheckedAdd, Saturating, Zero},
	DispatchResult, Perbill,
};

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;

		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type ProtocolFee: Get<Perbill>;
		type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;

		#[pallet::constant]
		type MaximumSupply: Get<BalanceOf<Self>>;

		/// Runtime existential deposit
		#[pallet::constant]
		type ExistentialDeposit: Get<BalanceOf<Self>>;

		/// The maximum number of oracle members.
		#[pallet::constant]
		type MaxOracles: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			remove_storage_prefix(<Pallet<T>>::name().as_bytes(), b"CoinsConsumed", b"");
			T::DbWeight::get().writes(1)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Optimized allocation call, which will batch allocations of various amounts
		/// and destinations and together. This allow us to be much more efficient and thus
		/// increase our chain's capacity in handling these transactions.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::batch(batch.len().try_into().unwrap_or_else(|_| T::MaxAllocs::get())))]
		#[transactional]
		pub fn batch(
			origin: OriginFor<T>,
			batch: BoundedVec<(T::AccountId, BalanceOf<T>), T::MaxAllocs>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_oracle(origin)?;

			ensure!(batch.len() > Zero::zero(), Error::<T>::BatchEmpty);

			// sanity checks
			let min_alloc = T::ExistentialDeposit::get().saturating_mul(2u32.into());
			let mut full_issuance: BalanceOf<T> = Zero::zero();
			for (_account, amount) in batch.iter() {
				ensure!(amount >= &min_alloc, Error::<T>::DoesNotSatisfyExistentialDeposit,);

				// overflow, so too many coins to allocate
				full_issuance = full_issuance
					.checked_add(amount)
					.ok_or(Error::<T>::TooManyCoinsToAllocate)?;
			}

			let current_supply = T::Currency::total_issuance();
			ensure!(
				current_supply.saturating_add(full_issuance) <= T::MaximumSupply::get(),
				Error::<T>::TooManyCoinsToAllocate
			);

			// allocate the coins to the proxy account
			T::Currency::resolve_creating(&T::PalletId::get().into_account(), T::Currency::issue(full_issuance));

			// send to accounts, unfortunately we need to loop again
			let mut full_protocol: BalanceOf<T> = Zero::zero();
			for (account, amount) in batch.iter().cloned() {
				let amount_for_protocol = T::ProtocolFee::get() * amount;
				let amount_for_grantee = amount.saturating_sub(amount_for_protocol);
				T::Currency::transfer(
					&T::PalletId::get().into_account(),
					&account,
					amount_for_grantee,
					ExistenceRequirement::KeepAlive,
				)?;
				full_protocol = full_protocol.saturating_add(amount_for_protocol);
			}

			// send protocol fees
			T::Currency::transfer(
				&T::PalletId::get().into_account(),
				&T::ProtocolFeeReceiver::account_id(),
				full_protocol,
				ExistenceRequirement::AllowDeath,
			)?;

			Ok(Pays::No.into())
		}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An allocation was triggered \[who, value, fee, proof\]
		NewAllocation(T::AccountId, BalanceOf<T>, BalanceOf<T>, Vec<u8>),
		OracleMembersOverFlow(u32, u32),
		OracleMembersUpdated(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Function is restricted to oracles only
		OracleAccessDenied,
		/// We are trying to allocate more coins than we can
		TooManyCoinsToAllocate,
		/// Amount is too low and will conflict with the ExistentialDeposit parameter
		DoesNotSatisfyExistentialDeposit,
		/// Batch is empty or no issuance is necessary
		BatchEmpty,
	}

	#[pallet::storage]
	#[pallet::getter(fn oracles)]
	pub type Oracles<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxOracles>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn coins_consumed)]
	pub type CoinsConsumed<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;
}

impl<T: Config> Pallet<T> {
	pub fn is_oracle(who: T::AccountId) -> bool {
		Self::oracles().contains(&who)
	}

	fn ensure_oracle(origin: T::Origin) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), Error::<T>::OracleAccessDenied);
		Ok(())
	}
}

impl<T: Config> ChangeMembers<T::AccountId> for Pallet<T> {
	fn change_members_sorted(_incoming: &[T::AccountId], _outgoing: &[T::AccountId], new: &[T::AccountId]) {
		let new_members_length: u32 = new.len() as u32;
		if new_members_length > T::MaxOracles::get() {
			Self::deposit_event(Event::OracleMembersOverFlow(T::MaxOracles::get(), new_members_length));
		} else {
			// <Oracles<T>>::put(init);
			<Oracles<T>>::mutate(|maybe_oracles| {
				let new_clone: Vec<T::AccountId> = new.iter().map(|x| x.clone()).collect();

				match <BoundedVec<T::AccountId, T::MaxOracles>>::try_from(new_clone) {
					Ok(oracles) => {
						*maybe_oracles = oracles;
						Self::deposit_event(Event::OracleMembersUpdated(new_members_length));
					}
					Err(_) => {
						Self::deposit_event(Event::OracleMembersOverFlow(T::MaxOracles::get(), new_members_length));
					}
				};
			})
		}
	}
}

impl<T: Config> InitializeMembers<T::AccountId> for Pallet<T> {
	fn initialize_members(init: &[T::AccountId]) {
		let init_members_length = init.len() as u32;

		if init_members_length > T::MaxOracles::get() {
			Self::deposit_event(Event::OracleMembersOverFlow(T::MaxOracles::get(), init_members_length));
		} else {
			// <Oracles<T>>::put(init);
			<Oracles<T>>::mutate(|maybe_oracles| {
				let init_clone: Vec<T::AccountId> = init.iter().map(|x| x.clone()).collect();
				match <BoundedVec<T::AccountId, T::MaxOracles>>::try_from(init_clone) {
					Ok(oracles) => {
						*maybe_oracles = oracles;
						Self::deposit_event(Event::OracleMembersUpdated(init_members_length));
					}
					Err(_) => {
						Self::deposit_event(Event::OracleMembersOverFlow(T::MaxOracles::get(), init_members_length));
					}
				};
			})
		}
	}
}
