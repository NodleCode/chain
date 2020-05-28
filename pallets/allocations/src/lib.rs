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
    traits::{ChangeMembers, Currency, Get, Imbalance, InitializeMembers, OnUnbalanced},
};
use frame_system::{self as system, ensure_signed};
use nodle_support::WithAccountId;
use sp_runtime::{
    traits::{CheckedAdd, Saturating},
    DispatchResult, Perbill,
};
use sp_std::prelude::Vec;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

/// The module's configuration trait.
pub trait Trait: frame_system::Trait + pallet_emergency_shutdown::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type ProtocolFee: Get<Perbill>;
    type ProtocolFeeReceiver: WithAccountId<Self::AccountId>;
    type SourceOfTheCoins: OnUnbalanced<PositiveImbalanceOf<Self>>;
    type MaximumCoinsEverAllocated: Get<BalanceOf<Self>>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Function is restricted to oracles only
        OracleAccessDenied,
        /// We are trying to allocate more coins than we can
        TooManyCoinsToAllocate,
        /// Emergency shutdown is active, operations suspended
        UnderShutdown,
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

            <CoinsConsumed<T>>::put(coins_that_will_be_consumed);

            let amount_for_protocol = T::ProtocolFee::get() * amount;
            let amount_for_grantee = amount.saturating_sub(amount_for_protocol);

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();
            let r_grantee = T::Currency::deposit_creating(&to, amount_for_grantee);
            let r_protocol = T::Currency::deposit_creating(&T::ProtocolFeeReceiver::account_id(), amount_for_protocol);
            total_imbalance.subsume(r_grantee);
            total_imbalance.subsume(r_protocol);
            T::SourceOfTheCoins::on_unbalanced(total_imbalance);

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
