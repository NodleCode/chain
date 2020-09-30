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

//! This module implements a Root Of Trust linked to a `membership` or `tcr` pallet which
//! can be used to let entities represented by their `AccountId` manage certificates
//! and off-chain certificates in Public Key Infrastructure fashion (SSL / TLS like).

mod benchmarking;
mod tests;

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
    traits::{ChangeMembers, Currency, ExistenceRequirement, Get, OnUnbalanced, WithdrawReasons},
    Parameter,
};
use frame_system::{self as system, ensure_signed};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::traits::{CheckedAdd, MaybeDisplay, MaybeSerializeDeserialize, Member};
use sp_std::{fmt::Debug, prelude::Vec};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct RootCertificate<AccountId, CertificateId, BlockNumber> {
    owner: AccountId,
    key: CertificateId,
    created: BlockNumber,
    renewed: BlockNumber,
    revoked: bool,
    validity: BlockNumber,
    child_revocations: Vec<CertificateId>,
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The currency used to represent the voting power
    type Currency: Currency<Self::AccountId>;

    /// How a certificate public key is represented, typically `AccountId`
    type CertificateId: Member
        + Parameter
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + Ord
        + Default;
    /// How much a new root certificate costs
    type SlotBookingCost: Get<BalanceOf<Self>>;
    /// How much renewing a root certificate costs
    type SlotRenewingCost: Get<BalanceOf<Self>>;
    /// How long a certificate is considered valid
    type SlotValidity: Get<Self::BlockNumber>;
    /// The module receiving funds paid by depositors, typically a company
    /// reserve
    type FundsCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        CertificateId = <T as Trait>::CertificateId,
    {
        /// A new slot has been booked
        SlotTaken(AccountId, CertificateId),
        /// An exisitng slot has been renewed (its validity period was extended)
        SlotRenewed(CertificateId),
        /// A slot has been revoked by its owner
        SlotRevoked(CertificateId),
        /// A child certificate was revoked
        ChildSlotRevoked(CertificateId, CertificateId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// `origin` a member, this function may need a member account id
        NotAMember,
        /// Slot was already taken, you will need to use another certificate id
        SlotTaken,
        /// Not enough funds to pay the fee
        NotEnoughFunds,
        /// Slot is no longer valid
        NoLongerValid,
        /// `origin` is not the slot owner
        NotTheOwner,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as RootOfTrustModule {
        Members get(fn members): Vec<T::AccountId>;
        Slots get(fn slots): map hasher(blake2_128_concat)
            T::CertificateId => RootCertificate<T::AccountId, T::CertificateId, T::BlockNumber>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Book a certificate slot
        #[weight = 160_000_000]
        fn book_slot(origin, certificate_id: T::CertificateId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Self::is_member(&sender), Error::<T>::NotAMember);
            ensure!(!<Slots<T>>::contains_key(&certificate_id), Error::<T>::SlotTaken);

            match T::Currency::withdraw(&sender, T::SlotBookingCost::get(), WithdrawReasons::all(), ExistenceRequirement::AllowDeath) {
                Ok(imbalance) => T::FundsCollector::on_unbalanced(imbalance),
                Err(_) => return Err(Error::<T>::NotEnoughFunds.into()),
            };

            let now = <system::Module<T>>::block_number();
            <Slots<T>>::insert(&certificate_id, RootCertificate {
                owner: sender.clone(),
                key: certificate_id.clone(),
                created: now,
                renewed: now,
                revoked: false,
                validity: T::SlotValidity::get(),
                child_revocations: Vec::new(),
            });

            Self::deposit_event(RawEvent::SlotTaken(sender, certificate_id));
            Ok(())
        }

        /// Renew a non expired slot and make it valid for a longer time
        #[weight = 150_000_000]
        fn renew_slot(origin, certificate_id: T::CertificateId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut slot = <Slots<T>>::get(&certificate_id);
            ensure!(Self::is_slot_valid(&slot), Error::<T>::NoLongerValid);
            ensure!(slot.owner == sender, Error::<T>::NotTheOwner);

            match T::Currency::withdraw(&sender, T::SlotRenewingCost::get(), WithdrawReasons::all(), ExistenceRequirement::AllowDeath) {
                Ok(imbalance) => T::FundsCollector::on_unbalanced(imbalance),
                Err(_) => return Err(Error::<T>::NotEnoughFunds.into()),
            };

            slot.renewed = <system::Module<T>>::block_number();
            <Slots<T>>::insert(&certificate_id, slot);

            Self::deposit_event(RawEvent::SlotRenewed(certificate_id));
            Ok(())
        }

        /// Revoke a slot before it is expired thus invalidating all child certificates
        #[weight = 75_000_000]
        fn revoke_slot(origin, certificate_id: T::CertificateId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut slot = <Slots<T>>::get(&certificate_id);
            ensure!(Self::is_slot_valid(&slot), Error::<T>::NoLongerValid);
            ensure!(slot.owner == sender, Error::<T>::NotTheOwner);

            slot.revoked = true;
            <Slots<T>>::insert(&certificate_id, slot);

            Self::deposit_event(RawEvent::SlotRevoked(certificate_id));
            Ok(())
        }

        /// Mark a slot's child as revoked thus invalidating it
        #[weight = 75_000_000]
        fn revoke_child(origin, root: T::CertificateId, child: T::CertificateId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut slot = <Slots<T>>::get(&root);
            ensure!(Self::is_slot_valid(&slot), Error::<T>::NoLongerValid);
            ensure!(slot.owner == sender, Error::<T>::NotTheOwner);
            ensure!(!slot.child_revocations.contains(&child), Error::<T>::NoLongerValid);

            slot.child_revocations.push(child.clone());
            <Slots<T>>::insert(&root, slot);

            Self::deposit_event(RawEvent::ChildSlotRevoked(root, child));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn is_member(who: &T::AccountId) -> bool {
        Self::members().contains(who)
    }

    fn is_slot_valid(
        slot: &RootCertificate<T::AccountId, T::CertificateId, T::BlockNumber>,
    ) -> bool {
        let owner_is_member = Self::is_member(&slot.owner);
        let revoked = slot.revoked;
        let expired = slot
            .renewed
            .checked_add(&slot.validity)
            .expect("we only sum block numbers that are not supposed to overflow; qed")
            <= <system::Module<T>>::block_number();

        owner_is_member && !revoked && !expired
    }

    /// This function is used as a helper in tests or when implementing the runtime APIs linked
    /// to this pallet.
    pub fn is_root_certificate_valid(cert: &T::CertificateId) -> bool {
        let exists = <Slots<T>>::contains_key(cert);
        let slot = <Slots<T>>::get(cert);

        exists && Self::is_slot_valid(&slot)
    }

    /// This function is used as a helper in tests or when implementing the runtime APIs linked
    /// to this pallet.
    pub fn is_child_certificate_valid(root: &T::CertificateId, child: &T::CertificateId) -> bool {
        let equals = root == child;
        let root_valid = Self::is_root_certificate_valid(root);
        let revoked = <Slots<T>>::get(root).child_revocations.contains(child);

        // At some point we could decide to have the clients submit complete certificates
        // to the nodes for verification purposes. However, this should probably be kept
        // off-chain anyways.

        !equals && root_valid && !revoked
    }

    /// A simple, benchmark only, function to replace or set the module's members
    #[cfg(feature = "runtime-benchmarks")]
    pub fn benchmark_set_members(members: &[T::AccountId]) {
        <Members<T>>::put(members);
    }
}

impl<T: Trait> ChangeMembers<T::AccountId> for Module<T> {
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        <Members<T>>::put(new);
    }
}
