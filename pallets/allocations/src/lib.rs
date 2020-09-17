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

#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
mod tests;

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{ChangeMembers, Currency, Get, InitializeMembers},
};
use frame_system::ensure_signed;
use nodle_support::WithAccountId;
use sp_runtime::{
    traits::{CheckedAdd, Saturating},
    DispatchResult, Perbill,
};
use sp_std::prelude::Vec;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// The module's configuration trait.
pub trait Trait: frame_system::Trait + pallet_emergency_shutdown::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type ProtocolFee: Get<Perbill>;
    type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;
    type MaximumCoinsEverAllocated: Get<BalanceOf<Self>>;

    /// Runtime existential deposit
    type ExistentialDeposit: Get<BalanceOf<Self>>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Function is restricted to oracles only
        OracleAccessDenied,
        /// We are trying to allocate more coins than we can
        TooManyCoinsToAllocate,
        /// Emergency shutdown is active, operations suspended
        UnderShutdown,
        /// Amount is too low and will conflict with the ExistentialDeposit parameter
        DoesNotSatisfyExistentialDeposit,
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        /// An allocation was triggered
        NewAllocation(AccountId, Balance, Balance, Vec<u8>),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Allocations {
        Oracles get(fn oracles): Vec<T::AccountId>;
        CoinsConsumed get(fn coins_consumed): BalanceOf<T>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Can only be called by an oracle, trigger a coin creation and an event
        #[weight = 50_000_000]
        pub fn allocate(origin, to: T::AccountId, amount: BalanceOf<T>, proof: Vec<u8>) -> DispatchResult {
            Self::ensure_oracle(origin)?;
            ensure!(!pallet_emergency_shutdown::Module::<T>::shutdown(), Error::<T>::UnderShutdown);

            let coins_already_allocated = Self::coins_consumed();
            let coins_that_will_be_consumed = coins_already_allocated.checked_add(&amount).ok_or("Overflow computing coins consumed")?;

            ensure!(coins_that_will_be_consumed <= T::MaximumCoinsEverAllocated::get(), Error::<T>::TooManyCoinsToAllocate);

            // When using a Perbill type as T::ProtocolFee::get() returns the default way to go is to used the standard mathematic
            // operands. The risk of {over, under}flow is void as this operation will effectively take a part of `amount` and thus
            // always produce a lower number. (We use Perbill to represent percentages)
            let amount_for_protocol = T::ProtocolFee::get() * amount;
            let amount_for_grantee = amount.saturating_sub(amount_for_protocol);

            Self::ensure_satisfy_existential_deposit(&T::ProtocolFeeReceiver::account_id(), amount_for_protocol)?;
            Self::ensure_satisfy_existential_deposit(&to, amount_for_grantee)?;

            <CoinsConsumed<T>>::put(coins_that_will_be_consumed);

            T::Currency::resolve_creating(&T::ProtocolFeeReceiver::account_id(), T::Currency::issue(amount_for_protocol));
            T::Currency::resolve_creating(&to, T::Currency::issue(amount_for_grantee));

            Self::deposit_event(RawEvent::NewAllocation(to, amount_for_grantee, amount_for_protocol, proof));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn is_oracle(who: T::AccountId) -> bool {
        Self::oracles().contains(&who)
    }

    fn ensure_oracle(origin: T::Origin) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        ensure!(Self::is_oracle(sender), Error::<T>::OracleAccessDenied);

        Ok(())
    }

    fn ensure_satisfy_existential_deposit(
        who: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let already_over_existential_deposit =
            T::Currency::free_balance(who) >= T::ExistentialDeposit::get();
        let amount_over_existential_deposit = amount >= T::ExistentialDeposit::get();

        match already_over_existential_deposit || amount_over_existential_deposit {
            true => Ok(()),
            false => Err(Error::<T>::DoesNotSatisfyExistentialDeposit.into()),
        }
    }
}

impl<T: Trait> ChangeMembers<T::AccountId> for Module<T> {
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        <Oracles<T>>::put(new);
    }
}

impl<T: Trait> InitializeMembers<T::AccountId> for Module<T> {
    fn initialize_members(init: &[T::AccountId]) {
        <Oracles<T>>::put(init);
    }
}
