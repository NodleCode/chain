#![cfg_attr(not(feature = "std"), no_std)]

//! A runtime module to handle help managing validators through the `membership`,
//! support the deletion and addition of validators by a root authority n.

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::traits::{
    ChangeMembers, Currency, Get, InitializeMembers, LockIdentifier, LockableCurrency,
    WithdrawReasons,
};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use session::SessionManager;
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Convert},
    RuntimeDebug,
};
use sp_std::prelude::Vec;
use system::ensure_signed;

const POA_LOCK_ID: LockIdentifier = *b"poastake";

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
// type PositiveImbalanceOf<T> =
//     <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
// type NegativeImbalanceOf<T> =
//     <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
//type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

    /// Minimum amount of coins in stash needed for the validators to validate
    type MinimumStash: Get<BalanceOf<Self>>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Initial stake amount is _dust_
        InsufficientValue,
        /// Balance is not big enough to stake the amount wanted
        NotEnoughFunds,
        /// No stash found
        StashNotFound,
        /// Stash already exists, should use stake_extra
        StashAlreadyExists,
        /// Trying to unstake more coins than staked
        TooBigUnstake,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as PoaModule {
        Validators get(validators): Vec<T::AccountId>;
        Stashes get(stashes): map hasher(blake2_256) T::AccountId => Stash<BalanceOf<T>>;
        PendingUnstaking get(pending_unstaking): Vec<(T::AccountId, BalanceOf<T>)>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        UnstakeSuccess(AccountId, Balance),
        UnstakeFailure(AccountId, Balance),
        /// The matching stash was deleted
        StashKilled(AccountId),
    }
);

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Stake more coins in the stash
        pub fn stake(origin, amount: BalanceOf<T>) -> DispatchResult {
            let signer = ensure_signed(origin)?;

            if <Stashes<T>>::exists(signer.clone()) {
                Err(Error::<T>::StashAlreadyExists)?
            }

            // reject a bond which is considered to be _dust_.
            if amount <= T::Currency::minimum_balance() {
                Err(Error::<T>::InsufficientValue)?
            }

            let stash = Stash { total: amount };
            Self::lock_stash(signer, stash)
        }

        /// Add more stake in an existing stash
        pub fn stake_extra(origin, amount: BalanceOf<T>) -> DispatchResult {
            let signer = ensure_signed(origin)?;

            if !Stashes::<T>::exists(signer.clone()) {
                Err(Error::<T>::StashNotFound)?
            }

            let mut stash = <Stashes<T>>::get(signer.clone());
            stash.total = stash.total.checked_add(&amount).ok_or("overflow")?;
            Self::lock_stash(signer, stash)
        }

        /// Remove some of the stake from an existing stash
        pub fn unstake(origin, amount: BalanceOf<T>) -> DispatchResult {
            let signer = ensure_signed(origin)?;

            if !Stashes::<T>::exists(signer.clone()) {
                Err(Error::<T>::StashNotFound)?
            }

            if <Stashes<T>>::get(signer.clone()).total < amount {
                Err(Error::<T>::TooBigUnstake)?;
            }

            // A validator should not unstake in the middle of a session,
            // we record its will to unstake and will execute when a new
            // session begins.
            <PendingUnstaking<T>>::mutate(|m| m.push((signer, amount)));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn lock_stash(who: T::AccountId, stash: Stash<BalanceOf<T>>) -> DispatchResult {
        let free_balance = T::Currency::free_balance(&who);
        if free_balance < stash.total {
            Err(Error::<T>::NotEnoughFunds)?
        }

        Self::execute_lock_stash(who, stash);
        Ok(())
    }

    fn execute_lock_stash(who: T::AccountId, stash: Stash<BalanceOf<T>>) {
        T::Currency::set_lock(POA_LOCK_ID, &who, stash.total, WithdrawReasons::all());
        <Stashes<T>>::insert(who, stash);
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
        // First of all, we unstake the coins that are pending
        <PendingUnstaking<T>>::get()
            .into_iter()
            .for_each(|(acc, bal)| {
                // Attempt to reduce stash, if it doesn't work we log the failure
                let stash = <Stashes<T>>::get(acc.clone());
                match stash.total.checked_sub(&bal) {
                    Some(new_stash) => {
                        // If the new stash would be 0 we delete the stash,
                        // else we update it
                        if new_stash == 0.into() {
                            <Stashes<T>>::remove(acc.clone());
                            T::Currency::remove_lock(POA_LOCK_ID, &acc);

                            Self::deposit_event(RawEvent::StashKilled(acc.clone()));
                        } else {
                            Self::execute_lock_stash(acc.clone(), Stash { total: new_stash });
                        }

                        Self::deposit_event(RawEvent::UnstakeSuccess(acc, bal));
                    }
                    None => Self::deposit_event(RawEvent::UnstakeFailure(acc, bal)),
                };
            });

        <PendingUnstaking<T>>::kill();

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
