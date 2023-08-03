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

use core::num::NonZeroU32;
use frame_support::pallet_prelude::{ensure, Decode, Encode, MaxEncodedLen, RuntimeDebug, TypeInfo};
use frame_support::{
	dispatch::{DispatchResult, Dispatchable, GetDispatchInfo},
	traits::{
		Currency,
		ExistenceRequirement::{AllowDeath, KeepAlive},
		InstanceFilter, IsSubType, OriginTrait, ReservableCurrency,
	},
};
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{TrailingZeroInput, Zero};
use sp_runtime::Saturating;
use support::LimitedBalance;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type PotDetailsOf<T> = PotDetails<<T as frame_system::Config>::AccountId, <T as Config>::SponsorshipType, BalanceOf<T>>;
type UserDetailsOf<T> = UserDetails<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

/// A pot details a sponsorship and its limits. The remained fee/reserve quota of a pot is not
/// withdrawn from the sponsor. So a valid pot does not guarantee that the sponsor has enough funds
/// to cover the fees/reserves of the sponsored transactions.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct PotDetails<AccountId, SponsorshipType, Balance> {
	/// The sponsor of the pot
	///
	/// The fees will be deducted from this account. The reserve funds will be taken from this.
	sponsor: AccountId,
	/// The category of the calls this pot will sponsor for its registered users.
	sponsorship_type: SponsorshipType,
	/// The remained allowance for covering fees of sponsored transactions.
	///
	/// When remained_fee_quota reaches zero, the pot is considered inactive. Any amount paid as fee
	/// is considered a permanent loss.
	remained_fee_quota: Balance,
	/// The remained allowance for covering reserves needed for some of sponsored transactions.
	///
	/// When remained_reserve_quota is zero, the pot could still be active but not suitable for some
	/// transactions. Any amount used as reserve may be returned to the sponsor when unreserved.
	remained_reserve_quota: Balance,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct UserDetails<AccountId, Balance: frame_support::traits::tokens::Balance> {
	/// The pure proxy account that is created for the user of a pot.
	///
	/// Same users would have different proxy accounts for different pots.
	proxy: AccountId,
	/// The remained allowance for covering fees of sponsored transactions for this user.
	///
	/// When remained_fee_quota reaches zero, the user is no longer sponsored by the pot.
	remained_fee_quota: Balance,
	/// The allowance for covering existential deposit needed to maintain proxy accounts plus the
	/// fund to be used as a reserve for some of the sponsored transactions of this user.
	///
	/// When remained_reserve_quota is zero, the use may still be sponsored for those transactions
	/// which do not require any reserve. Any amount used as reserve will be returned to the sponsor
	/// when unreserved. This is to prevent malicious users from draining sponsor funds.
	reserve: LimitedBalance<Balance>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::{boxed::Box, vec::Vec};
	use sp_runtime::Saturating;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The overarching call type.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>
			+ IsSubType<Call<Self>>
			+ IsType<<Self as frame_system::Config>::RuntimeCall>;
		/// The currency mechanism, used for paying for reserves.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Identifier for the pots.
		type PotId: Member + Parameter + MaxEncodedLen + Copy + From<u32>;
		/// The type for the categories of the calls that could be sponsored.
		/// The instance filter determines whether a given call may be sponsored under this type.
		///
		/// IMPORTANT 1: `Default` must be provided and MUST BE the the *most permissive* value.
		/// IMPORTANT 2: Never sponsor proxy calls or utility calls which allow other calls internally.
		/// This would allow a user to bypass the instance filter or alter the origin of the calls.
		type SponsorshipType: Parameter
			+ Member
			+ Ord
			+ PartialOrd
			+ InstanceFilter<<Self as Config>::RuntimeCall>
			+ MaxEncodedLen
			+ Default;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	/// Details of a pot.
	pub(super) type Pot<T: Config> = StorageMap<_, Blake2_128Concat, T::PotId, PotDetailsOf<T>, OptionQuery>;

	#[pallet::storage]
	/// User details of a pot.
	pub(super) type User<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::PotId, Blake2_128Concat, T::AccountId, UserDetailsOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a new pot is created.
		PotCreated(T::PotId),
		/// Event emitted when a pot is removed.
		PotRemoved(T::PotId),
		/// Event emitted when user/users are registered indicating the list of them
		UsersRegistered(T::PotId, Vec<T::AccountId>),
		/// Event emitted when user/users are removed indicating the list of them
		UsersRemoved(T::PotId, Vec<T::AccountId>),
		/// Event emitted when a sponsor_me call has been successful indicating the reserved amount
		Sponsored { top_up: BalanceOf<T>, refund: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The pot ID is already taken.
		InUse,
		/// The signing account has no permission to do the operation.
		NoPermission,
		/// The pot does not exist.
		PotNotExist,
		/// The user is not registered for the pot.
		UserNotRegistered,
		/// The user is already registered for the pot.
		UserAlreadyRegistered,
		/// The user is not removable due to holding some reserve.
		CannotRemoveProxy,
		/// Logic error: cannot create proxy account for user.
		/// This should never happen.
		CannotCreateProxy,
		/// Balance leak is not allowed during a sponsored call.
		BalanceLeak,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new sponsorship pot and set its limits.
		/// The pot id shouldn't be in use.
		///
		/// Emits `PotCreated(pot)` event when successful.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_pot())]
		pub fn create_pot(
			origin: OriginFor<T>,
			pot: T::PotId,
			sponsorship_type: T::SponsorshipType,
			fee_quota: BalanceOf<T>,
			reserve_quota: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Pot::<T>::contains_key(pot), Error::<T>::InUse);

			<Pot<T>>::insert(
				pot,
				PotDetailsOf::<T> {
					sponsor: who,
					sponsorship_type,
					remained_fee_quota: fee_quota,
					remained_reserve_quota: reserve_quota,
				},
			);

			Self::deposit_event(Event::PotCreated(pot));
			Ok(())
		}

		/// Allows the sponsor to remove the pot they have created themselves.
		/// The pot must not have any users. Users must have been removed prior to this call.
		///
		/// Emits `PotRemoved(pot)` when successful
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::remove_pot())]
		pub fn remove_pot(origin: OriginFor<T>, pot: T::PotId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Pot::<T>::try_mutate(pot, |maybe_pot_details| -> DispatchResult {
				let pot_details = maybe_pot_details.as_mut().ok_or(Error::<T>::InUse)?;
				ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
				let users = User::<T>::iter_prefix(pot).count();
				ensure!(users == 0, Error::<T>::InUse);
				*maybe_pot_details = None;
				Ok(())
			})?;
			Self::deposit_event(Event::PotRemoved(pot));
			Ok(())
		}

		/// Register users for a pot and set the same limit for the list of them.
		/// Only pot sponsor can do this.
		///
		/// Emits `UsersRegistered(pot, Vec<T::AccountId>)` with a list of registered when
		/// successful.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::register_users(users.len() as u32))]
		pub fn register_users(
			origin: OriginFor<T>,
			pot: T::PotId,
			users: Vec<T::AccountId>,
			common_fee_quota: BalanceOf<T>,
			common_reserve_quota: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
			for user in users.clone() {
				ensure!(!User::<T>::contains_key(pot, &user), Error::<T>::UserAlreadyRegistered);
				let proxy = Self::pure_account(&user, &pot).ok_or(Error::<T>::CannotCreateProxy)?;
				<User<T>>::insert(
					pot,
					user,
					UserDetailsOf::<T> {
						proxy,
						remained_fee_quota: common_fee_quota,
						reserve: LimitedBalance::with_limit(common_reserve_quota),
					},
				);
			}
			Self::deposit_event(Event::UsersRegistered(pot, users));
			Ok(())
		}

		/// Remove users from a pot.
		/// Only pot sponsor can do this.
		/// None of the specified users must have any reserved balance in their proxy accounts.
		/// User must be registered to be removable.
		/// Users receive the free balance in their proxy account back into their own accounts when
		/// they are removed.
		///
		/// Emits `UsersRemoved(pot, Vec<T::AccountId>)` with a list of those removed when
		/// successful.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_users(users.len() as u32))]
		pub fn remove_users(origin: OriginFor<T>, pot: T::PotId, users: Vec<T::AccountId>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
			for user in users.clone() {
				let user_details = User::<T>::get(pot, &user).ok_or(Error::<T>::UserNotRegistered)?;
				Self::remove_proxy(
					&pot_details.sponsor,
					&user,
					&user_details.proxy,
					user_details.reserve.balance(),
				)?;
				<User<T>>::remove(pot, &user);
			}
			Self::deposit_event(Event::UsersRemoved(pot, users));
			Ok(())
		}

		/// Remove inactive users from a pot.
		/// Only pot sponsor can do this.
		/// An inactive user is deemed to have no reserve balance in their proxy account.
		/// Users receive the free balance in their proxy account back into their own accounts when
		/// they are removed.
		/// `limit` is the maximum number of users to remove. If there are fewer inactive users than
		/// this, then all inactive users are removed.
		///
		/// Emits `UsersRemoved(pot, Vec<T::AccountId>)` with a list of those removed when
		/// successful.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::remove_inactive_users(u32::from(*limit)))]
		pub fn remove_inactive_users(origin: OriginFor<T>, pot: T::PotId, limit: NonZeroU32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
			let mut users_to_remove = Vec::<T::AccountId>::new();
			let limit = u32::from(limit) as usize;
			for (user, user_details) in User::<T>::iter_prefix(pot) {
				if Self::remove_proxy(
					&pot_details.sponsor,
					&user,
					&user_details.proxy,
					user_details.reserve.balance(),
				)
				.is_ok()
				{
					users_to_remove.push(user);
					if users_to_remove.len() == limit {
						break;
					}
				}
			}
			for user in &users_to_remove {
				<User<T>>::remove(pot, user);
			}
			Self::deposit_event(Event::UsersRemoved(pot, users_to_remove));
			Ok(())
		}

		/// Sponsor me for the given call from the specified pot.
		/// The caller must be registered for the pot.
		/// The call must be consistent with the pot's sponsorship type.
		///
		/// Returns Error if the pot doesn't exist or the user is not registered for the pot or if
		/// their call is not matching the sponsorship type in which case the error would be
		/// `frame_system::Error::CallFiltered`. Also returns error if the call itself should fail
		/// for any reason related to either the call or the available fund for the user.
		/// In this case the actual error will be depending on the call itself.  
		/// Regardless of the sponsorship type, users are not allowed to dispatch calls that would
		/// leak fund from their proxy account. If they try to do so they will get
		/// `Error::BalanceLeak`. For example they cannot transfer fund to another account even if
		/// the sponsorship type allows `Balances` calls.
		///
		/// Emits `Sponsored {top_up, refund}` when successful. The `top_up` is the amount initially
		/// transferred to the proxy account of the user by the sponsor. The `refund` is the amount
		/// refunded to the sponsor after the call has been successfully executed.
		/// Please note `refund` can be bigger than `top_up` if for any reason the user is able to
		/// partially or fully pay back their previous debt to the sponsor too.
		/// Also the top_up might be less than what the limit for the user allows if the user can
		/// support themselves partially or fully based on their free balance in their proxy account
		/// . Lastly, the top_up is limited by the remaining reserve quota for the pot too.
		#[pallet::call_index(5)]
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight + <T as Config>::WeightInfo::sponsor_for(), dispatch_info.class, Pays::No)
		})]
		pub fn sponsor_for(
			origin: OriginFor<T>,
			pot: T::PotId,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			let mut user_details = User::<T>::get(pot, &who).ok_or(Error::<T>::UserNotRegistered)?;
			let mut proxy_origin: T::RuntimeOrigin = frame_system::RawOrigin::Signed(user_details.proxy.clone()).into();
			let sponsorship = pot_details.sponsorship_type.clone();

			proxy_origin.add_filter(move |c: &<T as frame_system::Config>::RuntimeCall| {
				let c = <T as Config>::RuntimeCall::from_ref(c);
				let b = sponsorship.filter(c);
				b
			});

			let fund_for_reserve = user_details
				.reserve
				.available_margin()
				.min(pot_details.remained_reserve_quota);
			let top_up = user_details
				.reserve
				.limit()
				.saturating_sub(T::Currency::free_balance(&user_details.proxy))
				.min(fund_for_reserve);
			T::Currency::transfer(&pot_details.sponsor, &user_details.proxy, top_up, KeepAlive)?;
			pot_details.remained_reserve_quota = pot_details.remained_reserve_quota.saturating_sub(top_up);
			user_details.reserve.saturating_add(top_up);

			let proxy_balance = T::Currency::total_balance(&user_details.proxy);
			call.dispatch(proxy_origin).map_err(|e| e.error)?;
			let new_proxy_balance = T::Currency::total_balance(&user_details.proxy);
			ensure!(new_proxy_balance >= proxy_balance, Error::<T>::BalanceLeak);

			let refundable =
				T::Currency::free_balance(&user_details.proxy).saturating_sub(T::Currency::minimum_balance());
			let refund = refundable.min(user_details.reserve.balance());
			T::Currency::transfer(&user_details.proxy, &pot_details.sponsor, refund, KeepAlive)?;

			user_details.reserve.saturating_sub(refund);
			pot_details.remained_reserve_quota = pot_details.remained_reserve_quota.saturating_add(refund);

			Pot::<T>::insert(pot, pot_details);
			User::<T>::insert(pot, &who, user_details);

			Self::deposit_event(Event::Sponsored { top_up, refund });
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Calculate the address of a pure account.
	///
	/// A single user will always have the same proxy address for the same pot.
	///
	/// - `who`: The spawner account.
	/// - `pot_id`: The pot id this proxy is created for.
	pub fn pure_account(who: &T::AccountId, pot_id: &T::PotId) -> Option<T::AccountId> {
		let entropy = (b"modlsp/sponsorship", who, pot_id).using_encoded(blake2_256);
		Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref())).ok()
	}
	/// Transfer the left over balance from proxy to user and sponsor based on the given owing.
	/// Let the account die afterwards.
	///
	/// Returns `Ok` if the proxy is removed successfully.
	pub fn remove_proxy(
		sponsor: &T::AccountId,
		user: &T::AccountId,
		proxy: &T::AccountId,
		owing: BalanceOf<T>,
	) -> DispatchResult {
		ensure!(
			T::Currency::reserved_balance(proxy) == Zero::zero(),
			Error::<T>::CannotRemoveProxy
		);
		let proxy_free_balance = T::Currency::free_balance(proxy);
		let refund = proxy_free_balance.min(owing);
		T::Currency::transfer(proxy, sponsor, refund, AllowDeath)?;
		T::Currency::transfer(proxy, user, proxy_free_balance.saturating_sub(refund), AllowDeath)?;
		Ok(())
	}
}
