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
    decl_error, decl_event, decl_module, decl_storage,
    traits::{schedule::Named as ScheduleNamed, EnsureOrigin, Get, LockIdentifier},
    weights::{FunctionOf, GetDispatchInfo, Pays},
    Parameter,
};
use frame_system::{self as system, ensure_root};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{traits::Dispatchable, DispatchResult};
use sp_std::prelude::Box;

mod tests;

const AMENDMENTS_ID: LockIdentifier = *b"amendmen";

type AmendmentId = u64;

#[derive(Encode, Decode, Clone, PartialEq)]
pub struct PendingAmendment<Amendment> {
    amendment: Amendment,
    scheduler_id: Vec<u8>,
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

    /// How much blocks have to be produced before executing the amendment
    type Delay: Get<Self::BlockNumber>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We failed to schedule the amendment
        FailedToScheduleAmendment,
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as frame_system::Trait>::Hash,
        <T as frame_system::Trait>::BlockNumber,
    {
        /// A new amendment has been scheduled to be executed at the given block number
        AmendmentScheduled(Vec<u8>, BlockNumber),
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
        pub PendingAmendments get(fn pending_amendments): Vec<PendingAmendment<T::Amendment>>;
        /// Internal variable to keep track of amendment ids for scheduling purposes
        pub AmendmentsScheduled get(fn amendments_scheduled): u64;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Trigger a new challenge to remove an existing member
        #[weight = 50_000_000]
        fn propose(origin, amendment: T::Amendment) -> DispatchResult {
            T::SubmissionOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            let nb_scheduled = <AmendmentsScheduled>::get();
            let scheduler_id = (AMENDMENTS_ID, nb_scheduled).encode();
            let when = <system::Module<T>>::block_number() + T::Delay::get();

            if T::Scheduler::schedule_named(
                scheduler_id.clone(),
                when,
                None,
                62,
                amendment.clone(),
            ).is_err() {
                Err(Error::<T>::FailedToScheduleAmendment)?;
            }

            <AmendmentsScheduled>::put(nb_scheduled.clone());
            <PendingAmendments<T>>::mutate(|p| p.push(PendingAmendment{
                amendment: amendment,
                scheduler_id: scheduler_id.clone(),
            }));

            Self::deposit_event(RawEvent::AmendmentScheduled(scheduler_id, when));
            Ok(())
        }
    }
}
