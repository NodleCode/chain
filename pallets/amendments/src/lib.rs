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
    traits::{
        schedule::DispatchTime::At, schedule::Named as ScheduleNamed, EnsureOrigin, Get,
        LockIdentifier,
    },
    weights::GetDispatchInfo,
    Parameter,
};
use frame_system::{self as system, ensure_root};
use parity_scale_codec::Encode;
use sp_runtime::{traits::Dispatchable, DispatchResult};
use sp_std::prelude::Box;

mod benchmarking;
mod tests;

const AMENDMENTS_ID: LockIdentifier = *b"amendmen";

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Amendment: Parameter
        + Dispatchable<Origin = Self::Origin>
        + From<frame_system::Call<Self>>
        + GetDispatchInfo;
    type Scheduler: ScheduleNamed<Self::BlockNumber, Self::Amendment, Self::PalletsOrigin>;
    type PalletsOrigin: From<system::RawOrigin<Self::AccountId>>;

    /// Origin that can submit amendments
    type SubmissionOrigin: EnsureOrigin<Self::Origin>;

    /// Origin that can veto amendments
    type VetoOrigin: EnsureOrigin<Self::Origin>;

    /// How much blocks have to be produced before executing the amendment
    type Delay: Get<Self::BlockNumber>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We failed to schedule the amendment
        FailedToScheduleAmendment,
        /// We failed to cancel the amendment
        FailedToCancelAmendment,
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as frame_system::Trait>::BlockNumber,
    {
        /// A new amendment has been scheduled to be executed at the given block number
        AmendmentScheduled(u64, BlockNumber),
        /// An amendment has been vetoed and will never be triggered
        AmendmentVetoed(u64),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Amendments {
        /// Internal variable to keep track of amendment ids for scheduling purposes
        pub AmendmentsScheduled get(fn amendments_scheduled): u64;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Schedule `amendment` to be executed after the configured time, unless vetoed by `VetoOrigin`
        #[weight = 100_000_000]
        fn propose(origin, amendment: Box<T::Amendment>) -> DispatchResult {
            T::SubmissionOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            let nb_scheduled = <AmendmentsScheduled>::get();
            let scheduler_id = (AMENDMENTS_ID, nb_scheduled).encode();
            let when = <system::Module<T>>::block_number() + T::Delay::get();

            if T::Scheduler::schedule_named(
                scheduler_id,
                At(when),
                None,
                // This number defines a priority of execution of the scheduled calls. We basically took the number
                // from parity's democracy pallet and substracted 1 to make sure we have priority over it if a chain
                // uses both modules.
                62,
                system::RawOrigin::Root.into(),
                *amendment,
            ).is_err() {
                return Err(Error::<T>::FailedToScheduleAmendment.into());
            }

            <AmendmentsScheduled>::put(nb_scheduled + 1);

            Self::deposit_event(RawEvent::AmendmentScheduled(nb_scheduled, when));
            Ok(())
        }

        /// Veto and cancel a scheduled amendment
        #[weight = 20_000_000]
        fn veto(origin, amendment_id: u64) -> DispatchResult {
            T::VetoOrigin::try_origin(origin)
                .map(|_| ())
                .or_else(ensure_root)?;

            let scheduler_id = (AMENDMENTS_ID, amendment_id).encode();
            if T::Scheduler::cancel_named(scheduler_id).is_err() {
                return Err(Error::<T>::FailedToCancelAmendment.into());
            }

            Self::deposit_event(RawEvent::AmendmentVetoed(amendment_id));
            Ok(())
        }
    }
}
