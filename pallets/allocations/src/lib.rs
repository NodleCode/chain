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

//mod benchmarking;
mod tests;

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{ChangeMembers, Currency, Get, Imbalance, InitializeMembers, OnUnbalanced},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::DispatchResult;
use sp_runtime::Perbill;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

/// The module's configuration trait.
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type ProtocolFee: Get<Perbill>;
    type ProtocolFeeReceiver: OnUnbalanced<PositiveImbalanceOf<Self>>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Function is restricted to oracles only
        OracleAccessDenied,
        /// We are trying to allocate 0 coins
        ZeroAllocation,
        /// We are trying to allocate more coins than we can
        TooManyCoinsToAllocate,
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
