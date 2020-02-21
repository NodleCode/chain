#![cfg_attr(not(feature = "std"), no_std)]

//! A runtime module to handle help managing validators through the `membership`,
//! support the deletion and addition of validators by a root authority n.

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::traits::{ChangeMembers, Currency, Get, InitializeMembers, LockableCurrency};
use frame_support::{decl_module, decl_storage};
use session::SessionManager;
use sp_runtime::{traits::Convert, RuntimeDebug};
use sp_std::prelude::Vec;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
// type PositiveImbalanceOf<T> =
//     <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
// type NegativeImbalanceOf<T> =
//     <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
//type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

/// The module's configuration trait.
pub trait Trait: system::Trait + session::Trait {
    type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
    /// Minimum amount of coins in stash needed for the validators to validate
    type MinimumStash: Get<BalanceOf<Self>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as AllocationsModule {
        Validators get(validators): Vec<T::AccountId>;
        Stashes get(stashes): map hasher(blake2_256) T::AccountId => Stash<BalanceOf<T>>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Nothing, just an empty shell for declaration purposes
    }
}

impl<T: Trait> ChangeMembers<T::AccountId> for Module<T> {
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        <Validators<T>>::put(new);
    }
}

impl<T: Trait> InitializeMembers<T::AccountId> for Module<T> {
    fn initialize_members(init: &[T::AccountId]) {
        <Validators<T>>::put(init);
        // Shouldn't need a flag update here as this should happen at genesis
    }
}

/// Represents the stash of a validator
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Stash<Balance> {
    /// Total amount of coins in the stash
    pub total: Balance,
}
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Trait> Convert<T::AccountId, Option<Stash<BalanceOf<T>>>> for StashOf<T> {
    fn convert(validator: T::AccountId) -> Option<Stash<BalanceOf<T>>> {
        Some(<Stashes<T>>::get(&validator))
    }
}

type SessionIndex = u32; // A shim while waiting for this type to be exposed by `session`
impl<T: Trait> SessionManager<T::AccountId> for Module<T> {
    fn new_session(_: SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(
            <Validators<T>>::get()
                .into_iter()
                .filter(|v| match StashOf::<T>::convert(v.clone()) {
                    None => false,
                    Some(stash) => stash.total >= T::MinimumStash::get(),
                })
                .collect(),
        )
    }

    fn end_session(_: SessionIndex) {}
}

impl<T: Trait> session::historical::SessionManager<T::AccountId, Stash<BalanceOf<T>>>
    for Module<T>
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, Stash<BalanceOf<T>>)>> {
        <Self as session::SessionManager<_>>::new_session(new_index).map(|validators| {
            validators
                .into_iter()
                .map(|v| (v.clone(), StashOf::<T>::convert(v.clone()).unwrap()))
                .collect()
        })
    }

    fn end_session(end_index: SessionIndex) {
        <Self as session::SessionManager<_>>::end_session(end_index)
    }
}
