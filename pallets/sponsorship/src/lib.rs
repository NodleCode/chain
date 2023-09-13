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

use frame_support::pallet_prelude::{ensure, Decode, Encode, MaxEncodedLen, PhantomData, RuntimeDebug, TypeInfo};
use frame_support::{
	dispatch::{DispatchInfo, DispatchResult, Dispatchable, GetDispatchInfo, Pays, PostDispatchInfo},
	traits::{
		Currency,
		ExistenceRequirement::{AllowDeath, KeepAlive},
		InstanceFilter, IsSubType, IsType, OriginTrait, ReservableCurrency,
	},
};
use pallet_transaction_payment::OnChargeTransaction;
use sp_io::hashing::blake2_256;
use sp_runtime::{
	traits::{DispatchInfoOf, PostDispatchInfoOf, SignedExtension, TrailingZeroInput, Zero},
	transaction_validity::{InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction},
};
use sp_runtime::{FixedPointOperand, Saturating};
use sp_std::fmt::{Debug, Formatter, Result as FmtResult};
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
type OnChargeTransactionBalanceOf<T> =
	<<T as pallet_transaction_payment::Config>::OnChargeTransaction as OnChargeTransaction<T>>::Balance;
type LiquidityInfoOf<T> =
	<<T as pallet_transaction_payment::Config>::OnChargeTransaction as OnChargeTransaction<T>>::LiquidityInfo;
type PotDetailsOf<T> = PotDetails<<T as frame_system::Config>::AccountId, <T as Config>::SponsorshipType, BalanceOf<T>>;
type UserDetailsOf<T> = UserDetails<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

/// A pot details a sponsorship and its limits. The remained fee/reserve quota of a pot is not
/// withdrawn from the sponsor. So a valid pot does not guarantee that the sponsor has enough funds
/// to cover the fees/reserves of the sponsored transactions.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct PotDetails<AccountId, SponsorshipType, Balance: frame_support::traits::tokens::Balance> {
	/// The sponsor of the pot
	///
	/// The fees will be deducted from this account. The reserve funds will be taken from this.
	sponsor: AccountId,
	/// The category of the calls this pot will sponsor for its registered users.
	sponsorship_type: SponsorshipType,
	/// The limit and balance for covering fees of sponsored transactions.
	///
	/// When `fee_quota` reaches its limit, the pot is considered inactive.
	/// Any amount paid as fee is considered a permanent loss.
	fee_quota: LimitedBalance<Balance>,
	/// The limit and balance for covering reserves needed for some of sponsored transactions.
	///
	/// When `reserve_quota` reaches its limit, the pot could still be active but not suitable for
	/// some transactions.
	/// Any amount used as reserve may be returned to the sponsor when unreserved.
	reserve_quota: LimitedBalance<Balance>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct UserDetails<AccountId, Balance: frame_support::traits::tokens::Balance> {
	/// The pure proxy account that is created for the user of a pot.
	///
	/// Same users would have different proxy accounts for different pots.
	proxy: AccountId,
	/// The limit and balance for covering fees of sponsored transactions for this user.
	///
	/// When `fee_quota` reaches its limit, the user is no longer sponsored by the pot.
	fee_quota: LimitedBalance<Balance>,
	/// The limit and balance  for covering existential deposit needed to maintain proxy accounts plus the
	/// fund to be used as a reserve for some of the sponsored transactions of this user.
	///
	/// When `reserve_quota` reaches its limit, the use may still be sponsored for those transactions
	/// which do not require any reserve. Any amount used as reserve will be returned to the sponsor
	/// when unreserved. This is to prevent malicious users from draining sponsor funds.
	reserve_quota: LimitedBalance<Balance>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::{boxed::Box, vec::Vec};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_transaction_payment::Config {
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
		PotCreated { pot: T::PotId },
		/// Event emitted when a pot is removed.
		PotRemoved { pot: T::PotId },
		/// Event emitted when a pot is updated.
		PotUpdated { pot: T::PotId },
		/// Event emitted when a pot is updated.
		PotSponsorshipTypeUpdated { pot: T::PotId },
		/// Event emitted when user/users are registered indicating the list of them
		UsersRegistered { pot: T::PotId, users: Vec<T::AccountId> },
		/// Event emitted when user/users are removed indicating the list of them
		UsersRemoved { pot: T::PotId, users: Vec<T::AccountId> },
		/// Event emitted when fee_quota or reserve_quota or both are updated for the given list
		UsersLimitsUpdated { pot: T::PotId, users: Vec<T::AccountId> },
		/// Event emitted when a sponsor_me call has been successful indicating the reserved amount
		Sponsored { paid: BalanceOf<T>, repaid: BalanceOf<T> },
		/// Event emitted when the transaction fee is paid showing the payer and the amount
		TransactionFeePaid { sponsor: T::AccountId, fee: BalanceOf<T> },
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
		/// Cannot update the fee limit most likely due to being below the current commitment.
		CannotUpdateFeeLimit,
		/// Cannot update the reserve limit most likely due to being below the current commitment.
		CannotUpdateReserveLimit,
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
					fee_quota: LimitedBalance::with_limit(fee_quota),
					reserve_quota: LimitedBalance::with_limit(reserve_quota),
				},
			);

			Self::deposit_event(Event::PotCreated { pot });
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
			Self::deposit_event(Event::PotRemoved { pot });
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
				frame_system::Pallet::<T>::inc_providers(&proxy);
				if !Self::registered_for_any_pots(&user) {
					frame_system::Pallet::<T>::inc_providers(&user);
				}
				<User<T>>::insert(
					pot,
					user,
					UserDetailsOf::<T> {
						proxy,
						fee_quota: LimitedBalance::with_limit(common_fee_quota),
						reserve_quota: LimitedBalance::with_limit(common_reserve_quota),
					},
				);
			}
			Self::deposit_event(Event::UsersRegistered { pot, users });
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
			let mut pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
			for user in users.clone() {
				let user_details = User::<T>::get(pot, &user).ok_or(Error::<T>::UserNotRegistered)?;
				let repaid = Self::settle_user_accounts(
					&pot_details.sponsor,
					&user,
					&user_details.proxy,
					user_details.reserve_quota.balance(),
				)?;
				pot_details.reserve_quota.saturating_sub(repaid);
				Self::remove_user(&pot, &user);
			}
			<Pot<T>>::insert(pot, pot_details);
			Self::deposit_event(Event::UsersRemoved { pot, users });
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
		/// Emits `Sponsored {paid, repaid}` when successful. The `paid` is the amount initially
		/// transferred to the proxy account of the user by the sponsor. The `repaid` is the amount
		/// repaid to the sponsor after the call has been successfully executed.
		/// Please note `repaid` can be bigger than `paid` if for any reason the user is able to
		/// partially or fully pay back their previous debt to the sponsor too.
		/// Also the `paid` might be less than what the limit for the user allows if the user can
		/// support themselves partially or fully based on their free balance in their proxy account
		/// . Finally, the `paid` is limited by the remaining reserve quota for the pot too.
		///
		/// Note: The addition of `T::DbWeight::get().reads_writes(2, 2)` to the weight is to account
		/// for the reads and writes of the `pot_details` and `user_details` storage items which
		/// are needed during pre and post dispatching this call.
		#[pallet::call_index(4)]
		#[pallet::weight({
		let dispatch_info = call.get_dispatch_info();
		(dispatch_info.weight + < T as Config >::WeightInfo::pre_sponsor() + < T as Config >::WeightInfo::post_sponsor() + T::DbWeight::get().reads_writes(2, 2), dispatch_info.class, Pays::No)
		})]
		pub fn sponsor_for(
			origin: OriginFor<T>,
			pot: T::PotId,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let preps = Self::pre_sponsor_for(who.clone(), pot)?;

			call.dispatch(preps.proxy_origin).map_err(|e| e.error)?;

			Self::post_sponsor_for(
				who,
				pot,
				preps.pot_details,
				preps.user_details,
				preps.paid,
				preps.proxy_balance,
			)?;

			Ok(().into())
		}

		/// Update the pot details. Only the sponsor can do this. If the sponsor is lowering their
		/// support, it can work only if the corresponding fee or reserve balance has enough
		/// available margin. In other words, the sponsor cannot lower the limit for the fee below
		/// what users have already taken from the pot. Similarly, the sponsor cannot lower the
		/// reserve below what the users have already borrowed.
		#[pallet::call_index(5)]
		#[pallet::weight(< T as Config >::WeightInfo::update_pot_limits())]
		pub fn update_pot_limits(
			origin: OriginFor<T>,
			pot: T::PotId,
			new_fee_quota: BalanceOf<T>,
			new_reserve_quota: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);

			pot_details
				.fee_quota
				.update_limit(new_fee_quota)
				.map_err(|_| Error::<T>::CannotUpdateFeeLimit)?;
			pot_details
				.reserve_quota
				.update_limit(new_reserve_quota)
				.map_err(|_| Error::<T>::CannotUpdateReserveLimit)?;

			<Pot<T>>::insert(pot, pot_details);
			Self::deposit_event(Event::PotUpdated { pot });
			Ok(())
		}

		/// Update limits for a number of users in a single call. Only the sponsor can do this. If
		/// the sponsor is lowering their support, it can work only if the corresponding fee or
		/// reserve balance of all those users have enough available margin.
		#[pallet::call_index(6)]
		#[pallet::weight(< T as Config >::WeightInfo::update_users_limits(users.len() as u32))]
		pub fn update_users_limits(
			origin: OriginFor<T>,
			pot: T::PotId,
			new_fee_quota: BalanceOf<T>,
			new_reserve_quota: BalanceOf<T>,
			users: Vec<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
			ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);

			for user in &users {
				let mut user_details = User::<T>::get(pot, user).ok_or(Error::<T>::UserNotRegistered)?;
				user_details
					.fee_quota
					.update_limit(new_fee_quota)
					.map_err(|_| Error::<T>::CannotUpdateFeeLimit)?;
				user_details
					.reserve_quota
					.update_limit(new_reserve_quota)
					.map_err(|_| Error::<T>::CannotUpdateReserveLimit)?;
				<User<T>>::insert(pot, user, user_details);
			}

			<Pot<T>>::insert(pot, pot_details);
			Self::deposit_event(Event::UsersLimitsUpdated { pot, users });
			Ok(())
		}

		/// Update the pot's sponsorship type. Only the sponsor can do this.
		/// Emits `PotSponsorshipTypeUpdated` event when successful.
		#[pallet::call_index(7)]
		#[pallet::weight(< T as Config >::WeightInfo::update_sponsorship_type())]
		pub fn update_sponsorship_type(
			origin: OriginFor<T>,
			pot: T::PotId,
			sponsorship_type: T::SponsorshipType,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Pot::<T>::try_mutate(pot, |maybe_pot_details| -> DispatchResult {
				let pot_details = maybe_pot_details.as_mut().ok_or(Error::<T>::PotNotExist)?;
				ensure!(pot_details.sponsor == who, Error::<T>::NoPermission);
				pot_details.sponsorship_type = sponsorship_type;
				Ok(())
			})?;

			Self::deposit_event(Event::PotSponsorshipTypeUpdated { pot });
			Ok(())
		}
	}
}

/// The pre-sponsor call preps are the details returned from `pre_sponsor_for` that are needed
/// in `post_sponsor_for`.
struct SponsorCallPreps<T: Config> {
	/// The pot details.
	pot_details: PotDetailsOf<T>,
	/// The user details.
	user_details: UserDetailsOf<T>,
	/// The proxy origin after the right filter is installed for it.
	proxy_origin: <T as frame_system::Config>::RuntimeOrigin,
	/// The amount paid by the sponsor to be used as reserve.
	paid: BalanceOf<T>,
	/// The total balance of the user's proxy account after the sponsor has paid.
	proxy_balance: BalanceOf<T>,
}

impl<T: Config> Pallet<T> {
	/// Calculate the address of a pure account.
	///
	/// A single user will always have the same proxy address for the same pot.
	///
	/// - `who`: The spawner account.
	/// - `pot_id`: The pot id this proxy is created for.
	fn pure_account(who: &T::AccountId, pot_id: &T::PotId) -> Option<T::AccountId> {
		let entropy = (b"modlsp/sponsorship", who, pot_id).using_encoded(blake2_256);
		Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref())).ok()
	}
	/// Transfer the left over balance from proxy to user and sponsor based on the given owing.
	/// Let the account die afterwards.
	///
	/// Returns `Ok(repay)` if the proxy is removed successfully. `repay` is the amount repaid to
	/// the sponsor.
	fn settle_user_accounts(
		sponsor: &T::AccountId,
		user: &T::AccountId,
		proxy: &T::AccountId,
		owing: BalanceOf<T>,
	) -> Result<BalanceOf<T>, sp_runtime::DispatchError> {
		ensure!(
			T::Currency::reserved_balance(proxy) == Zero::zero(),
			Error::<T>::CannotRemoveProxy
		);
		let proxy_free_balance = T::Currency::free_balance(proxy);
		let repay = proxy_free_balance.min(owing);
		T::Currency::transfer(proxy, sponsor, repay, AllowDeath)?;
		T::Currency::transfer(proxy, user, proxy_free_balance.saturating_sub(repay), AllowDeath)?;
		frame_system::Pallet::<T>::dec_providers(proxy)?;
		Ok(repay)
	}
	/// Remove the user from the pot.
	fn remove_user(pot: &T::PotId, who: &T::AccountId) {
		<User<T>>::remove(pot, who);
		if !Self::registered_for_any_pots(who) {
			let _ = frame_system::Pallet::<T>::dec_providers(who);
		}
	}
	/// Check if the account is registered for any pots.
	fn registered_for_any_pots(who: &T::AccountId) -> bool {
		Pot::<T>::iter_keys().any(|pot| User::<T>::contains_key(pot, who.clone()))
	}

	fn pre_sponsor_for(who: T::AccountId, pot: T::PotId) -> Result<SponsorCallPreps<T>, sp_runtime::DispatchError> {
		let mut pot_details = Pot::<T>::get(pot).ok_or(Error::<T>::PotNotExist)?;
		let mut user_details = User::<T>::get(pot, &who).ok_or(Error::<T>::UserNotRegistered)?;
		let mut proxy_origin: T::RuntimeOrigin = frame_system::RawOrigin::Signed(user_details.proxy.clone()).into();
		let sponsorship = pot_details.sponsorship_type.clone();

		proxy_origin.add_filter(move |c: &<T as frame_system::Config>::RuntimeCall| {
			let c = <T as Config>::RuntimeCall::from_ref(c);
			sponsorship.filter(c)
		});

		let fund_for_reserve = user_details
			.reserve_quota
			.available_margin()
			.min(pot_details.reserve_quota.available_margin());
		let paid = user_details
			.reserve_quota
			.limit()
			.saturating_sub(T::Currency::free_balance(&user_details.proxy))
			.min(fund_for_reserve);
		T::Currency::transfer(&pot_details.sponsor, &user_details.proxy, paid, KeepAlive)?;
		pot_details.reserve_quota.saturating_add(paid);
		user_details.reserve_quota.saturating_add(paid);

		let proxy_balance = T::Currency::total_balance(&user_details.proxy);
		Ok(SponsorCallPreps {
			pot_details,
			user_details,
			proxy_origin,
			paid,
			proxy_balance,
		})
	}

	fn post_sponsor_for(
		who: T::AccountId,
		pot: T::PotId,
		mut pot_details: PotDetailsOf<T>,
		mut user_details: UserDetailsOf<T>,
		paid: BalanceOf<T>,
		proxy_balance: BalanceOf<T>,
	) -> Result<(), sp_runtime::DispatchError> {
		let new_proxy_balance = T::Currency::total_balance(&user_details.proxy);
		ensure!(new_proxy_balance >= proxy_balance, Error::<T>::BalanceLeak);

		let repayable = T::Currency::free_balance(&user_details.proxy).saturating_sub(T::Currency::minimum_balance());
		let repaid = repayable.min(user_details.reserve_quota.balance());
		T::Currency::transfer(&user_details.proxy, &pot_details.sponsor, repaid, KeepAlive)?;

		user_details.reserve_quota.saturating_sub(repaid);
		pot_details.reserve_quota.saturating_sub(repaid);

		Pot::<T>::insert(pot, pot_details);
		User::<T>::insert(pot, &who, user_details);

		Self::deposit_event(Event::Sponsored { paid, repaid });
		Ok(())
	}
}

/// Require the sponsor to pay for their transactors.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeSponsor<T: Config>(PhantomData<BalanceOf<T>>);

impl<T: Config> Debug for ChargeSponsor<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "ChargeTransactionPayment<{:?}>", self.0)
	}
	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut Formatter) -> FmtResult {
		Ok(())
	}
}

pub struct PreDispatchSponsorCallData<T: Config> {
	pot: T::PotId,
	pot_details: PotDetailsOf<T>,
	user: T::AccountId,
	user_details: UserDetailsOf<T>,
	fee_imbalance: LiquidityInfoOf<T>,
}
pub type Pre<T> = Option<PreDispatchSponsorCallData<T>>;

impl<T: Config> ChargeSponsor<T>
where
	BalanceOf<T>: IsType<OnChargeTransactionBalanceOf<T>>,
	OnChargeTransactionBalanceOf<T>: FixedPointOperand,
	<T as frame_system::Config>::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	<T as Config>::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	fn validate_sponsor_call(
		user: &T::AccountId,
		call: &<T as Config>::RuntimeCall,
		info: &DispatchInfoOf<<T as Config>::RuntimeCall>,
		len: usize,
	) -> Result<Pre<T>, TransactionValidityError> {
		match call.is_sub_type() {
			Some(Call::sponsor_for { pot, .. }) => {
				let pot_details = Pot::<T>::get(pot).ok_or(InvalidTransaction::Call)?;
				let user_details = User::<T>::get(pot, user).ok_or(InvalidTransaction::BadSigner)?;

				let mut info = *info;
				info.pays_fee = Pays::Yes;
				let fee = pallet_transaction_payment::Pallet::<T>::compute_fee(len as u32, &info, Zero::zero());
				let available_fee_margin = pot_details
					.fee_quota
					.available_margin()
					.min(user_details.fee_quota.available_margin());
				if available_fee_margin.into_ref() < &fee {
					Err(TransactionValidityError::Invalid(InvalidTransaction::Payment))?
				}

				let fee_imbalance = <T as pallet_transaction_payment::Config>::OnChargeTransaction::withdraw_fee(
					&pot_details.sponsor,
					call.into_ref(),
					&info,
					fee,
					Zero::zero(),
				)
				.map_err(|_| InvalidTransaction::Payment)?;

				Ok(Some(PreDispatchSponsorCallData {
					pot: *pot,
					pot_details,
					user: user.clone(),
					user_details,
					fee_imbalance,
				}))
			}
			_ => Ok(None),
		}
	}
}

impl<T: Config> Default for ChargeSponsor<T> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<T: Config> SignedExtension for ChargeSponsor<T>
where
	BalanceOf<T>: Send + Sync + IsType<OnChargeTransactionBalanceOf<T>>,
	OnChargeTransactionBalanceOf<T>: FixedPointOperand,
	<T as frame_system::Config>::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	<T as Config>::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	const IDENTIFIER: &'static str = "ChargeSponsor";
	type AccountId = T::AccountId;
	type Call = <T as Config>::RuntimeCall;
	type AdditionalSigned = ();
	type Pre = Pre<T>;
	fn additional_signed(&self) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		Self::validate_sponsor_call(who, call, info, len).map(|_| Ok(ValidTransaction::default()))?
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Self::validate_sponsor_call(who, call, info, len)
	}

	fn post_dispatch(
		maybe_pre: Option<Self::Pre>,
		info: &DispatchInfoOf<Self::Call>,
		post_info: &PostDispatchInfoOf<Self::Call>,
		len: usize,
		_result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		if let Some(Some(PreDispatchSponsorCallData {
			pot,
			mut pot_details,
			user,
			mut user_details,
			fee_imbalance,
		})) = maybe_pre
		{
			let mut info = *info;
			info.pays_fee = Pays::Yes;
			let actual_fee =
				pallet_transaction_payment::Pallet::<T>::compute_actual_fee(len as u32, &info, post_info, Zero::zero());
			<T as pallet_transaction_payment::Config>::OnChargeTransaction::correct_and_deposit_fee(
				&pot_details.sponsor,
				&info,
				post_info,
				actual_fee,
				Zero::zero(),
				fee_imbalance,
			)?;
			let actual_fee = *<BalanceOf<T> as IsType<OnChargeTransactionBalanceOf<T>>>::from_ref(&actual_fee);

			pot_details
				.fee_quota
				.add(actual_fee)
				.map_err(|_| InvalidTransaction::Payment)?;
			user_details
				.fee_quota
				.add(actual_fee)
				.map_err(|_| InvalidTransaction::Payment)?;

			Pot::<T>::try_mutate(pot, |maybe_pot_details| -> DispatchResult {
				let pot_details_to_overwrite = maybe_pot_details.as_mut().ok_or(Error::<T>::PotNotExist)?;
				pot_details_to_overwrite.fee_quota = pot_details.fee_quota.clone();
				Ok(())
			})
			.map_err(|_| InvalidTransaction::Call)?;
			User::<T>::try_mutate(pot, &user, |maybe_user_details| -> DispatchResult {
				let user_details_to_overwrite = maybe_user_details.as_mut().ok_or(Error::<T>::UserNotRegistered)?;
				user_details_to_overwrite.fee_quota = user_details.fee_quota.clone();
				Ok(())
			})
			.map_err(|_| InvalidTransaction::Call)?;

			Pallet::<T>::deposit_event(Event::<T>::TransactionFeePaid {
				sponsor: pot_details.sponsor,
				fee: actual_fee,
			});
		}
		Ok(())
	}
}
