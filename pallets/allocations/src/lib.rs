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
	transactional, BoundedVec, PalletId,
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

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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

		/// How big a batch can be
		#[pallet::constant]
		type MaxAllocs: Get<u32>;

		type OracleMembers: Contains<Self::AccountId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedOracles::<T>::pre_upgrade()
		}

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migrations::v1::MigrateToBoundedOracles::<T>::on_runtime_upgrade()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			migrations::v1::MigrateToBoundedOracles::<T>::post_upgrade()
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

		/// Can only be called by an oracle, trigger a token mint and dispatch to
		/// `amount`, minus protocol fees
		#[pallet::weight(
			 <T as pallet::Config>::WeightInfo::allocate()
		 )]
		// we add the `transactional` modifier here in the event that one of the
		// transfers fail. the code itself should already prevent this but we add
		// this as an additional guarantee.
		#[transactional]
		#[deprecated(note = "allocate is sub-optimized and chain heavy")]
		pub fn allocate(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			_proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let mut vec = BoundedVec::with_max_capacity();
			vec.try_push((to, amount))
				.expect("shouldn't panic because we have enough capacity");
			Pallet::<T>::batch(origin, vec)
		}
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
	pub(crate) type StorageVersion<T: Config> = StorageValue<_, Releases, ValueQuery>;

	#[cfg(feature = "runtime-benchmarks")]
	#[pallet::storage]
	#[pallet::getter(fn benchmark_oracles)]
	pub type BenchmarkOracles<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, benchmarking::MaxMembers>, ValueQuery>;
}

impl<T: Config> Pallet<T> {
	pub fn is_oracle(who: T::AccountId) -> bool {
		#[cfg(feature = "runtime-benchmarks")]
		return T::OracleMembers::contains(&who) || Self::benchmark_oracles().contains(&who);
		#[cfg(not(feature = "runtime-benchmarks"))]
		return T::OracleMembers::contains(&who);
	}

	fn ensure_oracle(origin: T::Origin) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		ensure!(Self::is_oracle(sender), Error::<T>::OracleAccessDenied);
		Ok(())
	}
}
