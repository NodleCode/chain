#![cfg_attr(not(feature = "std"), no_std)]

//! Any entity (such as the committee) with access to the "root mandate" (this module)
//! can use the `apply` function to dispatch calls as root. Think of this module as an
//! other `sudo` module controlled by another module (ex: a multisig).

use frame_support::{decl_module, dispatch::DispatchResult, Parameter};
use sp_runtime::{
    traits::{Dispatchable, EnsureOrigin},
    DispatchError,
};
use sp_std::prelude::Box;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Proposal: Parameter + Dispatchable<Origin = Self::Origin>;

    /// Origin that can call this module and execute sudo actions. Typically
    /// the `collective` module.
    type ExternalOrigin: EnsureOrigin<Self::Origin>;
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        pub fn apply(origin, proposal: Box<T::Proposal>) -> DispatchResult {
            T::ExternalOrigin::ensure_origin(origin)?;

            // Shamelessly stollen from the `sudo` module
            match proposal.dispatch(system::RawOrigin::Root.into()) {
                Ok(_) => true,
                Err(e) => {
                    let e: DispatchError = e.into();
                    sp_runtime::print(e);
                    false
                }
            };

            Ok(())
        }
    }
}
