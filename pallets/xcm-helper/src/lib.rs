// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Common XCM Helper pallet
//!
//! ## Overview
//! This pallet should be in charge of everything XCM related including callbacks and sending XCM calls.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::{DispatchResult, GetDispatchInfo},
    pallet_prelude::*,
    traits::fungibles::{Inspect, Mutate, Transfer},
    transactional, PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::{AccountIdConversion, BlockNumberProvider, Convert, StaticLookup};
use sp_std::{boxed::Box, prelude::*, vec, vec::Vec};
use xcm::{latest::prelude::*, DoubleEncoded, VersionedMultiLocation, VersionedXcm};
use xcm_executor::traits::InvertLocation;

pub use pallet::*;
use pallet_traits::{switch_relay, ump::*};
use primitives::{AccountId, Balance, BlockNumber, CurrencyId, ParaId};

mod benchmarking;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CallIdOf<T> = <T as pallet_xcm::Config>::Call;
pub type AssetIdOf<T> =
    <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
pub type BalanceOf<T> =
    <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
    use sp_runtime::traits::{Convert, Zero};

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_xcm::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Assets for deposit/withdraw assets to/from crowdloan account
        type Assets: Transfer<AccountIdOf<Self>, AssetId = CurrencyId, Balance = Balance>
            + Inspect<AccountIdOf<Self>, AssetId = CurrencyId, Balance = Balance>
            + Mutate<AccountIdOf<Self>, AssetId = CurrencyId, Balance = Balance>;

        /// XCM message sender
        type XcmSender: SendXcm;

        /// Relay network
        #[pallet::constant]
        type RelayNetwork: Get<NetworkId>;

        /// Pallet account for collecting xcm fees
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Account on relaychain for receiving refunded fees
        #[pallet::constant]
        type RefundLocation: Get<Self::AccountId>;

        /// Convert `T::AccountId` to `MultiLocation`.
        type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;

        /// Notify call timeout
        #[pallet::constant]
        type NotifyTimeout: Get<BlockNumberFor<Self>>;

        /// The block number provider
        type BlockNumberProvider: BlockNumberProvider<BlockNumber = BlockNumberFor<Self>>;

        /// The origin which can update reserve_factor, xcm_fees etc
        type UpdateOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;

        /// The origin which can call XCM helper functions
        type XcmOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;

        /// Weight information
        type WeightInfo: WeightInfo;

        /// Relay currency
        #[pallet::constant]
        type RelayCurrency: Get<AssetIdOf<Self>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Xcm fee and weight updated
        XcmWeightFeeUpdated(XcmWeightFeeMisc<Weight, BalanceOf<T>>),
        /// Withdrawing
        Withdrawing,
        /// Contributing
        Contributing,
        /// Bonding
        Bonding,
        /// BondingExtra
        BondingExtra,
        /// Unbonding
        Unbonding,
        /// Rebonding
        Rebonding,
        /// WithdrawingUnbonded
        WithdrawingUnbonded,
        /// Nominating
        Nominating,
        /// XCM message sent. \[to, message\]
        Sent {
            to: Box<MultiLocation>,
            message: Xcm<()>,
        },
        /// ProxyAdded
        ProxyAdded,
        /// ProxyRemoved
        ProxyRemoved,
    }

    #[pallet::storage]
    #[pallet::getter(fn xcm_weight_fee)]
    pub type XcmWeightFee<T: Config> =
        StorageMap<_, Twox64Concat, XcmCall, XcmWeightFeeMisc<Weight, BalanceOf<T>>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::error]
    pub enum Error<T> {
        /// `MultiLocation` value ascend more parents than known ancestors of local location.
        MultiLocationNotInvertible,
        /// XcmWeightMisc cannot have zero value
        ZeroXcmWeightMisc,
        /// Xcm fees cannot be zero
        ZeroXcmFees,
        /// Insufficient xcm fees
        InsufficientXcmFees,
        /// The message and destination combination was not recognized as being
        /// reachable.
        Unreachable,
        /// The message and destination was recognized as being reachable but
        /// the operation could not be completed.
        SendFailure,
        /// The version of the `Versioned` value used is not able to be
        /// interpreted.
        BadVersion,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update xcm fees amount to be used in xcm.Withdraw message
        #[pallet::weight(<T as Config>::WeightInfo::update_xcm_weight_fee())]
        #[transactional]
        pub fn update_xcm_weight_fee(
            origin: OriginFor<T>,
            xcm_call: XcmCall,
            xcm_weight_fee_misc: XcmWeightFeeMisc<Weight, BalanceOf<T>>,
        ) -> DispatchResult {
            T::UpdateOrigin::ensure_origin(origin)?;

            ensure!(!xcm_weight_fee_misc.fee.is_zero(), Error::<T>::ZeroXcmFees);
            ensure!(
                !xcm_weight_fee_misc.weight.is_zero(),
                Error::<T>::ZeroXcmWeightMisc
            );

            XcmWeightFee::<T>::mutate(xcm_call, |v| *v = xcm_weight_fee_misc);
            Self::deposit_event(Event::<T>::XcmWeightFeeUpdated(xcm_weight_fee_misc));
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn withdraw(
            origin: OriginFor<T>,
            para_id: ParaId,
            para_account_id: AccountIdOf<T>,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_withdraw(para_id, para_account_id, *notify)?;

            Self::deposit_event(Event::<T>::Withdrawing);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn contribute(
            origin: OriginFor<T>,
            para_id: ParaId,
            amount: BalanceOf<T>,
            who: AccountIdOf<T>,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_contribute(para_id, amount, &who, *notify)?;

            Self::deposit_event(Event::<T>::Contributing);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn bond(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            payee: RewardDestination<AccountIdOf<T>>,
            stash: AccountIdOf<T>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_bond(value, payee, stash, index, *notify)?;

            Self::deposit_event(Event::<T>::Bonding);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn bond_extra(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            stash: AccountIdOf<T>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_bond_extra(value, stash, index, *notify)?;

            Self::deposit_event(Event::<T>::BondingExtra);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn unbond(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_unbond(value, index, *notify)?;

            Self::deposit_event(Event::<T>::Unbonding);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn rebond(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_rebond(value, index, *notify)?;

            Self::deposit_event(Event::<T>::Rebonding);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn withdraw_unbonded(
            origin: OriginFor<T>,
            num_slashing_spans: u32,
            para_account_id: AccountIdOf<T>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_withdraw_unbonded(num_slashing_spans, para_account_id, index, *notify)?;

            Self::deposit_event(Event::<T>::WithdrawingUnbonded);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn nominate(
            origin: OriginFor<T>,
            targets: Vec<AccountIdOf<T>>,
            index: u16,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_nominate(targets, index, *notify)?;

            Self::deposit_event(Event::<T>::Nominating);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        pub fn send_as_sovereign(
            origin: OriginFor<T>,
            dest: Box<VersionedMultiLocation>,
            message: Box<VersionedXcm<()>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            let dest = MultiLocation::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
            let message: Xcm<()> = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;

            pallet_xcm::Pallet::<T>::send_xcm(Here, dest.clone(), message.clone()).map_err(
                |e| match e {
                    SendError::CannotReachDestination(..) => Error::<T>::Unreachable,
                    _ => Error::<T>::SendFailure,
                },
            )?;
            Self::deposit_event(Event::Sent {
                to: Box::new(dest),
                message,
            });
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn ump_transact(
            origin: OriginFor<T>,
            call: DoubleEncoded<()>,
            weight: Weight,
            beneficiary: Box<MultiLocation>,
            fees: BalanceOf<T>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_ump_transact(call, weight, *beneficiary, fees)?;

            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn add_proxy(
            origin: OriginFor<T>,
            delegate: AccountId,
            proxy_type: Option<ProxyType>,
            delay: BlockNumber,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_add_proxy(delegate, proxy_type, delay, *notify)?;

            Self::deposit_event(Event::<T>::ProxyAdded);
            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[transactional]
        pub fn remove_proxy(
            origin: OriginFor<T>,
            delegate: AccountId,
            proxy_type: Option<ProxyType>,
            delay: BlockNumber,
            notify: Box<CallIdOf<T>>,
        ) -> DispatchResult {
            T::XcmOrigin::ensure_origin(origin)?;

            Self::do_remove_proxy(delegate, proxy_type, delay, *notify)?;

            Self::deposit_event(Event::<T>::ProxyRemoved);
            Ok(())
        }
    }
}

pub trait XcmHelper<T: pallet_xcm::Config, Balance, TAccountId> {
    fn add_xcm_fees(payer: &TAccountId, amount: Balance) -> DispatchResult;

    fn do_ump_transact(
        call: DoubleEncoded<()>,
        weight: Weight,
        beneficiary: MultiLocation,
        fees: Balance,
    ) -> Result<Xcm<()>, DispatchError>;

    fn do_withdraw(
        para_id: ParaId,
        para_account_id: TAccountId,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_contribute(
        para_id: ParaId,
        amount: Balance,
        who: &TAccountId,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_bond(
        value: Balance,
        payee: RewardDestination<TAccountId>,
        stash: TAccountId,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_bond_extra(
        value: Balance,
        stash: TAccountId,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_unbond(
        value: Balance,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_rebond(
        value: Balance,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_withdraw_unbonded(
        num_slashing_spans: u32,
        para_account_id: TAccountId,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_nominate(
        targets: Vec<TAccountId>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_add_proxy(
        delegate: AccountId,
        proxy_type: Option<ProxyType>,
        delay: BlockNumber,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;

    fn do_remove_proxy(
        delegate: AccountId,
        proxy_type: Option<ProxyType>,
        delay: BlockNumber,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError>;
}

impl<T: Config> Pallet<T> {
    pub fn account_id() -> AccountIdOf<T> {
        T::PalletId::get().into_account()
    }

    pub fn refund_location() -> MultiLocation {
        T::AccountIdToMultiLocation::convert(T::RefundLocation::get())
    }

    pub fn report_outcome_notify(
        message: &mut Xcm<()>,
        responder: impl Into<MultiLocation>,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
        timeout: BlockNumberFor<T>,
    ) -> Result<QueryId, DispatchError> {
        let responder = responder.into();
        let dest = <T as pallet_xcm::Config>::LocationInverter::invert_location(&responder)
            .map_err(|()| Error::<T>::MultiLocationNotInvertible)?;
        let notify: <T as pallet_xcm::Config>::Call = notify.into();
        let max_response_weight = notify.get_dispatch_info().weight;
        let query_id = pallet_xcm::Pallet::<T>::new_notify_query(responder, notify, timeout);
        let report_error = Xcm(vec![ReportError {
            dest,
            query_id,
            max_response_weight,
        }]);
        // Prepend SetAppendix(Xcm(vec![ReportError])) wont be able to pass barrier check
        // so we need to insert it after Withdraw, BuyExecution
        message.0.insert(2, SetAppendix(report_error));
        Ok(query_id)
    }

    pub fn get_xcm_weight_fee_to_sibling(
        location: MultiLocation,
    ) -> XcmWeightFeeMisc<Weight, BalanceOf<T>> {
        let call = XcmCall::TransferToSiblingchain(Box::new(location));
        Self::xcm_weight_fee(call)
    }
}

impl<T: Config> XcmHelper<T, BalanceOf<T>, AccountIdOf<T>> for Pallet<T> {
    fn add_xcm_fees(payer: &AccountIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
        T::Assets::transfer(
            T::RelayCurrency::get(),
            payer,
            &Self::account_id(),
            amount,
            false,
        )?;
        Ok(())
    }

    fn do_ump_transact(
        call: DoubleEncoded<()>,
        weight: Weight,
        beneficiary: MultiLocation,
        fees: BalanceOf<T>,
    ) -> Result<Xcm<()>, DispatchError> {
        let asset: MultiAsset = (MultiLocation::here(), fees).into();
        T::Assets::burn_from(T::RelayCurrency::get(), &Self::account_id(), fees)
            .map_err(|_| Error::<T>::InsufficientXcmFees)?;

        Ok(Xcm(vec![
            WithdrawAsset(MultiAssets::from(asset.clone())),
            BuyExecution {
                fees: asset.clone(),
                weight_limit: Unlimited,
            },
            Transact {
                origin_type: OriginKind::SovereignAccount,
                require_weight_at_most: weight,
                call,
            },
            RefundSurplus,
            DepositAsset {
                assets: asset.into(),
                max_assets: 1,
                beneficiary,
            },
        ]))
    }

    fn do_add_proxy(
        delegate: AccountId,
        proxy_type: Option<ProxyType>,
        delay: BlockNumber,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::AddProxy);
        Ok(switch_relay!({
            let call =
                RelaychainCall::<T>::Proxy(Box::new(ProxyCall::AddProxy(ProxyAddProxyCall {
                    delegate,
                    proxy_type,
                    delay,
                })));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_e) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_remove_proxy(
        delegate: AccountId,
        proxy_type: Option<ProxyType>,
        delay: BlockNumber,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::RemoveProxy);
        Ok(switch_relay!({
            let call = RelaychainCall::<T>::Proxy(Box::new(ProxyCall::RemoveProxy(
                ProxyRemoveProxyCall {
                    delegate,
                    proxy_type,
                    delay,
                },
            )));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_e) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_withdraw(
        para_id: ParaId,
        para_account_id: AccountIdOf<T>,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Withdraw);
        Ok(switch_relay!({
            let call =
                RelaychainCall::<T>::Crowdloans(CrowdloansCall::Withdraw(CrowdloansWithdrawCall {
                    who: para_account_id,
                    index: para_id,
                }));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_e) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_contribute(
        para_id: ParaId,
        amount: BalanceOf<T>,
        _who: &AccountIdOf<T>,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Contribute);
        Ok(switch_relay!({
            let call = RelaychainCall::<T>::Crowdloans(CrowdloansCall::Contribute(
                CrowdloansContributeCall {
                    index: para_id,
                    value: amount,
                    signature: None,
                },
            ));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_e) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_bond(
        value: BalanceOf<T>,
        payee: RewardDestination<AccountIdOf<T>>,
        stash: AccountIdOf<T>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let controller = stash.clone();
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Bond);
        Ok(switch_relay!({
            let call =
                RelaychainCall::Utility(Box::new(UtilityCall::BatchAll(UtilityBatchAllCall {
                    calls: vec![
                        RelaychainCall::Balances(BalancesCall::TransferKeepAlive(
                            BalancesTransferKeepAliveCall {
                                dest: T::Lookup::unlookup(stash),
                                value,
                            },
                        )),
                        RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                            UtilityAsDerivativeCall {
                                index,
                                call: RelaychainCall::Staking::<T>(StakingCall::Bond(
                                    StakingBondCall {
                                        controller: T::Lookup::unlookup(controller),
                                        value,
                                        payee,
                                    },
                                )),
                            },
                        ))),
                    ],
                })));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_bond_extra(
        value: BalanceOf<T>,
        stash: AccountIdOf<T>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::BondExtra);
        Ok(switch_relay!({
            let call =
                RelaychainCall::Utility(Box::new(UtilityCall::BatchAll(UtilityBatchAllCall {
                    calls: vec![
                        RelaychainCall::Balances(BalancesCall::TransferKeepAlive(
                            BalancesTransferKeepAliveCall {
                                dest: T::Lookup::unlookup(stash),
                                value,
                            },
                        )),
                        RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                            UtilityAsDerivativeCall {
                                index,
                                call: RelaychainCall::Staking::<T>(StakingCall::BondExtra(
                                    StakingBondExtraCall { value },
                                )),
                            },
                        ))),
                    ],
                })));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_unbond(
        value: BalanceOf<T>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Unbond);
        Ok(switch_relay!({
            let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                UtilityAsDerivativeCall {
                    index,
                    call: RelaychainCall::Staking::<T>(StakingCall::Unbond(StakingUnbondCall {
                        value,
                    })),
                },
            )));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_rebond(
        value: BalanceOf<T>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Rebond);
        Ok(switch_relay!({
            let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                UtilityAsDerivativeCall {
                    index,
                    call: RelaychainCall::Staking::<T>(StakingCall::Rebond(StakingRebondCall {
                        value,
                    })),
                },
            )));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_withdraw_unbonded(
        num_slashing_spans: u32,
        para_account_id: AccountIdOf<T>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::WithdrawUnbonded);
        Ok(switch_relay!({
            let call =
                RelaychainCall::Utility(Box::new(UtilityCall::BatchAll(UtilityBatchAllCall {
                    calls: vec![
                        RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                            UtilityAsDerivativeCall {
                                index,
                                call: RelaychainCall::Staking::<T>(StakingCall::WithdrawUnbonded(
                                    StakingWithdrawUnbondedCall { num_slashing_spans },
                                )),
                            },
                        ))),
                        RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                            UtilityAsDerivativeCall {
                                index,
                                call: RelaychainCall::Balances::<T>(BalancesCall::TransferAll(
                                    BalancesTransferAllCall {
                                        dest: T::Lookup::unlookup(para_account_id),
                                        keep_alive: true,
                                    },
                                )),
                            },
                        ))),
                    ],
                })));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }

    fn do_nominate(
        targets: Vec<AccountIdOf<T>>,
        index: u16,
        notify: impl Into<<T as pallet_xcm::Config>::Call>,
    ) -> Result<QueryId, DispatchError> {
        let targets_source = targets.into_iter().map(T::Lookup::unlookup).collect();
        let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Nominate);
        Ok(switch_relay!({
            let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
                UtilityAsDerivativeCall {
                    index,
                    call: RelaychainCall::Staking::<T>(StakingCall::Nominate(
                        StakingNominateCall {
                            targets: targets_source,
                        },
                    )),
                },
            )));

            let mut msg = Self::do_ump_transact(
                call.encode().into(),
                xcm_weight_fee_misc.weight,
                Self::refund_location(),
                xcm_weight_fee_misc.fee,
            )?;

            let query_id = Self::report_outcome_notify(
                &mut msg,
                MultiLocation::parent(),
                notify,
                T::NotifyTimeout::get(),
            )?;

            if let Err(_err) = T::XcmSender::send_xcm(MultiLocation::parent(), msg) {
                return Err(Error::<T>::SendFailure.into());
            }

            query_id
        }))
    }
}
