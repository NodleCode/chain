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
	ensure,
	traits::{tokens::ExistenceRequirement, ChangeMembers, Currency, Get, InitializeMembers},
	transactional, PalletId,
};
use frame_system::ensure_signed;
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;
use support::WithAccountId;

use sp_runtime::{
	traits::{CheckedAdd, Saturating, Zero},
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
	pub trait Config: frame_system::Config + pallet_emergency_shutdown::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type ProtocolFee: Get<Perbill>;
		type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;

		#[pallet::constant]
		type MaximumCoinsEverAllocated: Get<BalanceOf<Self>>;

		/// Runtime existential deposit
		#[pallet::constant]
		type ExistentialDeposit: Get<BalanceOf<Self>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Can only be called by an oracle, trigger a coin creation and an event
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::allocate(proof.len() as u32)
		)]
		// we add the `transactional` modifier here in the event that one of the
		// transfers fail. the code itself should already prevent this but we add
		// this as an additional guarantee.
		#[transactional]
		pub fn allocate(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_oracle(origin)?;
			ensure!(
				!pallet_emergency_shutdown::Pallet::<T>::shutdown(),
				Error::<T>::UnderShutdown
			);
			ensure!(
				amount >= T::ExistentialDeposit::get().saturating_mul(2u32.into()),
				Error::<T>::DoesNotSatisfyExistentialDeposit,
			);

			if amount == Zero::zero() {
				return Ok(Pays::No.into());
			}

			let coins_already_allocated = Self::coins_consumed();
			let coins_that_will_be_consumed = coins_already_allocated
				.checked_add(&amount)
				.ok_or("Overflow computing coins consumed")?;

			ensure!(
				coins_that_will_be_consumed <= T::MaximumCoinsEverAllocated::get(),
				Error::<T>::TooManyCoinsToAllocate
			);

			// When using a Perbill type as T::ProtocolFee::get() returns the default way to go is to used the
			// standard mathematic operands. The risk of {over, under}flow is void as this operation will
			// effectively take a part of `amount` and thus always produce a lower number. (We use Perbill to
			// represent percentages)
			let amount_for_protocol = T::ProtocolFee::get() * amount;
			let amount_for_grantee = amount.saturating_sub(amount_for_protocol);

			<CoinsConsumed<T>>::put(coins_that_will_be_consumed);

			T::Currency::resolve_creating(&T::PalletId::get().into_account(), T::Currency::issue(amount));
			T::Currency::transfer(
				&T::PalletId::get().into_account(),
				&T::ProtocolFeeReceiver::account_id(),
				amount_for_protocol,
				// we use `KeepAlive` here because we want the guarantee that the funds left
				// won't be considered dust, which would prevent us from sending the rest to
				// the grantee.
				ExistenceRequirement::KeepAlive,
			)?;
			T::Currency::transfer(
				&T::PalletId::get().into_account(),
				&to,
				amount_for_grantee,
				ExistenceRequirement::AllowDeath,
			)?;

			Self::deposit_event(Event::NewAllocation(to, amount_for_grantee, amount_for_protocol, proof));
			Ok(Pays::No.into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An allocation was triggered \[who, value, fee, proof\]
		NewAllocation(T::AccountId, BalanceOf<T>, BalanceOf<T>, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Function is restricted to oracles only
		OracleAccessDenied,
		/// We are trying to allocate more coins than we can
		TooManyCoinsToAllocate,
		/// Emergency shutdown is active, operations suspended
		UnderShutdown,
		/// Amount is too low and will conflict with the ExistentialDeposit parameter
		DoesNotSatisfyExistentialDeposit,
	}

	#[pallet::storage]
	#[pallet::getter(fn oracles)]
	pub type Oracles<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

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
		<Oracles<T>>::put(new);
	}
}

impl<T: Config> InitializeMembers<T::AccountId> for Pallet<T> {
	fn initialize_members(init: &[T::AccountId]) {
		<Oracles<T>>::put(init);
	}
}
