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

//! An amendment module instance manages amendments to the chain. There could be a security
//! delay configured along with a veto capability.

use frame_support::{
    decl_event, decl_module, decl_storage,
    traits::{schedule::Named as ScheduleNamed, EnsureOrigin, LockIdentifier},
    weights::{FunctionOf, GetDispatchInfo, Pays},
    Parameter,
};
use frame_system::{self as system, ensure_root};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{traits::Dispatchable, DispatchResult};
use sp_std::prelude::Box;

mod tests;

const AMENDMENTS_ID: LockIdentifier = *b"amendmen";

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct PendingAmendment<Amendment, BlockNumber> {
    amendment: Amendment,
    execute_on: BlockNumber,
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Amendment: Parameter + Dispatchable<Origin = Self::Origin> + GetDispatchInfo;
    type Scheduler: ScheduleNamed<Self::BlockNumber, Self::Amendment>;

    /// Origin that can submit amendments
    type SubmissionOrigin: EnsureOrigin<Self::Origin>;

    /// Origin that can veto amendments
    type VetoOrigin: EnsureOrigin<Self::Origin>;

    /// Origin that can speed up amendments
    type AccelerationOrigin: EnsureOrigin<Self::Origin>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as frame_system::Trait>::Hash,
        <T as frame_system::Trait>::BlockNumber,
    {
        /// A new amendment has been scheduled to be executed at the given block number
        AmendmentScheduled(Hash, BlockNumber),
        /// An amendment has been applied with success to the ledger
        AmendmentSuccess(Hash),
        /// An amendment has not been applied with success to the ledger
        AmendmentFailure(Hash),
        /// An amendment has been vetoed and will never be triggered
        AmendmentVetoed(Hash),
        /// An amendment has been fast tracked
        AmendmentAccelerated(Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Amendments {
        /// This keeps track of the upcoming amendments
        pub PendingAmendments get(fn pending_amendments):
            map hasher(blake2_128_concat) T::Hash => PendingAmendment<T::Amendment, T::BlockNumber>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Trigger a new challenge to remove an existing member
        #[weight = 100_000_000]
        fn propose(origin, amendment: Box<T::Amendment>) -> DispatchResult {
            T::SubmissionOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            Ok(())
        }
    }
}
