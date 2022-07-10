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

mod migrations;

use codec::{Decode, Encode};
use frame_support::{
	ensure,
	pallet_prelude::MaxEncodedLen,
	traits::{tokens::ExistenceRequirement, Contains, Currency, Get},
	transactional, PalletId,
};

use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{
	traits::{CheckedAdd, Saturating, Zero},
	DispatchResult, Perbill, RuntimeDebug,
};
use sp_std::prelude::*;
use support::WithAccountId;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

type BalanceOf<T, I> = <<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// A value placed in storage that represents the current version of the Allocations storage.
// This value is used by the `on_runtime_upgrade` logic to determine whether we run storage
// migration logic. This should match directly with the semantic versions of the Rust crate.
#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
enum Releases {
	V0_0_0Legacy, // To handle Legacy version
	V2_0_21,
}

impl Default for Releases {
	fn default() -> Self {
		Releases::V0_0_0Legacy
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::OnRuntimeUpgrade;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config<I: 'static = ()>:
		frame_system::Config + pallet_emergency_shutdown::Config + pallet_membership::Config<I>
	{
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type ProtocolFee: Get<Perbill>;
		type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;

		#[pallet::constant]
		type MaximumCoinsEverAllocated: Get<BalanceOf<Self, I>>;

		/// Runtime existential deposit
		#[pallet::constant]
		type ExistentialDeposit: Get<BalanceOf<Self, I>>;

		type OracleMembers: Contains<Self::AccountId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedOracles::<T, I>::pre_upgrade()
		}

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migrations::v1::MigrateToBoundedOracles::<T, I>::on_runtime_upgrade()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedOracles::<T, I>::post_upgrade()
		}
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Can only be called by an oracle, trigger a coin creation and an event
		#[pallet::weight(
			<T as pallet::Config<I>>::WeightInfo::allocate(proof.len() as u32)
		)]
		// we add the `transactional` modifier here in the event that one of the
		// transfers fail. the code itself should already prevent this but we add
		// this as an additional guarantee.
		#[transactional]
		pub fn allocate(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T, I>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = Self::ensure_oracle(origin.clone()).map(|_| true)? || ensure_root(origin).map(|_| true)?;

			ensure!(
				!pallet_emergency_shutdown::Pallet::<T>::shutdown(),
				Error::<T, I>::UnderShutdown
			);
			ensure!(
				amount >= T::ExistentialDeposit::get().saturating_mul(2u32.into()),
				Error::<T, I>::DoesNotSatisfyExistentialDeposit,
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
				Error::<T, I>::TooManyCoinsToAllocate
			);

			// When using a Perbill type as T::ProtocolFee::get() returns the default way to go is to used the
			// standard mathematic operands. The risk of {over, under}flow is void as this operation will
			// effectively take a part of `amount` and thus always produce a lower number. (We use Perbill to
			// represent percentages)
			let amount_for_protocol = T::ProtocolFee::get() * amount;
			let amount_for_grantee = amount.saturating_sub(amount_for_protocol);

			<CoinsConsumed<T, I>>::put(coins_that_will_be_consumed);

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
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// An allocation was triggered \[who, value, fee, proof\]
		NewAllocation(T::AccountId, BalanceOf<T, I>, BalanceOf<T, I>, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
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
	#[pallet::getter(fn coins_consumed)]
	pub type CoinsConsumed<T: Config<I>, I: 'static = ()> = StorageValue<_, BalanceOf<T, I>, ValueQuery>;

	#[pallet::storage]
	pub(crate) type StorageVersion<T: Config<I>, I: 'static = ()> = StorageValue<_, Releases, ValueQuery>;
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn is_oracle(who: T::AccountId) -> bool {
		T::OracleMembers::contains(&who)
	}

	fn ensure_oracle(origin: T::Origin) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), Error::<T, I>::OracleAccessDenied);
		Ok(())
	}
}
