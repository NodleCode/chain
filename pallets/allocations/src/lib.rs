#![cfg_attr(not(feature = "std"), no_std)]

//! A runtime module to handle Nodle Cash allocations to network
//! contributors, has a list of oracles that can submit Merkle
//! Root Hashes to be paid for.

#[cfg(test)]
mod tests;

use frame_support::traits::{ChangeMembers, Currency, Imbalance, InitializeMembers, OnUnbalanced};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use sp_runtime::traits::CheckedSub;
use sp_std::prelude::Vec;
use system::ensure_signed;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    // Currency minting
    type Currency: Currency<Self::AccountId>;
    type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
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

decl_storage! {
    trait Store for Module<T: Trait> as AllocationsModule {
        Oracles get(oracles): Vec<T::AccountId>;
        CoinsLeft get(coins_left) config(): BalanceOf<T>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        // As an oracle, submit a merkle root for reward
        pub fn submit_reward(origin, merkle_root_hash: T::Hash, who: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            Self::ensure_oracle(origin)?;

            ensure!(amount > 0.into(), Error::<T>::ZeroAllocation);
            ensure!(<CoinsLeft<T>>::get() >= amount, Error::<T>::TooManyCoinsToAllocate);

            // Record the coins as spent
            <CoinsLeft<T>>::put(
                <CoinsLeft<T>>::get().checked_sub(&amount).ok_or("Underflow computing coins left")?
            );

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();
            let r = T::Currency::deposit_creating(&who, amount);
            total_imbalance.subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            Self::deposit_event(RawEvent::RewardAllocated(who, amount, merkle_root_hash));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        Hash = <T as system::Trait>::Hash,
    {
        /// Some rewards were allocated to a network contributor.
        RewardAllocated(AccountId, Balance, Hash),
    }
);

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
