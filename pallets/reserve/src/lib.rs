#![cfg_attr(not(feature = "std"), no_std)]

//! A module that is called by the `collective` and is in charge of holding
//! the company funds.

#[cfg(test)]
mod tests;

use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Imbalance, OnUnbalanced},
};
use sp_runtime::{
    traits::{AccountIdConversion, EnsureOrigin},
    ModuleId,
};
use system::ensure_signed;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

const MODULE_ID: ModuleId = ModuleId(*b"py/resrv");

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type ExternalOrigin: EnsureOrigin<Self::Origin>;
    type Currency: Currency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Reserve {}
    add_extra_genesis {
        build(|_config| {
            // Create account
            let _ = T::Currency::make_free_balance_be(
                &<Module<T>>::account_id(),
                T::Currency::minimum_balance(),
            );
        });
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Spend `amount` funds from the reserve account to `to`.
        pub fn spend(origin, to: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            T::ExternalOrigin::ensure_origin(origin)?;

            // TODO: we currently `AllowDeath` for our source account, shall we use `KeepAlive` instead?
            let _ = T::Currency::transfer(&Self::account_id(), &to, amount, ExistenceRequirement::AllowDeath);

            Self::deposit_event(RawEvent::SpentFunds(to, amount));

            Ok(())
        }

        /// Deposit `amount` tokens in the treasure account
        pub fn tip(origin, amount: BalanceOf<T>) -> DispatchResult {
            let tipper = ensure_signed(origin)?;

            let _ = T::Currency::transfer(&tipper, &Self::account_id(), amount, ExistenceRequirement::AllowDeath);

            Self::deposit_event(RawEvent::TipReceived(tipper, amount));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        /// Some amount was deposited (e.g. for transaction fees).
        Deposit(Balance),
        /// Some funds were spent from the reserve.
        SpentFunds(AccountId, Balance),
        /// Someone tipped the company reserve
        TipReceived(AccountId, Balance),
    }
);

impl<T: Trait> Module<T> {
    pub fn account_id() -> T::AccountId {
        MODULE_ID.into_account()
    }
}

impl<T: Trait> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
    fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
        let numeric_amount = amount.peek();

        // Must resolve into existing but better to be safe.
        let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

        Self::deposit_event(RawEvent::Deposit(numeric_amount));
    }
}
