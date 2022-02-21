#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::WeightInfo;
    pub use ethereum_types::Address as EthAddress;
    use frame_support::{
        pallet_prelude::*,
        traits::{Contains, Currency, OnUnbalanced, ReservableCurrency},
        weights::Pays,
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    pub use sp_core::{Bytes, H256 as EthTxHash};
    use sp_runtime::traits::{Bounded, CheckedAdd, One, Zero};
    use sp_std::vec::Vec;
    pub use support::WithAccountId;

    pub type CurrencyOf<T> = <T as Config>::Currency;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;
    pub type RejectionCode = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: ReservableCurrency<Self::AccountId>;

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Trusted bots/oracles which can settle funds after wrapping is initiated
        type Oracles: Contains<Self::AccountId>;

        /// The customers who've gone under the KYC process and are eligible to wrap their NODLs  
        type KnownCustomers: Contains<Self::AccountId>;

        /// The chain's reserve that is assigned to this pallet
        type Reserve: OnUnbalanced<NegativeImbalanceOf<Self>> + WithAccountId<Self::AccountId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn current_min)]
    /// The min fund set by the root for initiate wrapping
    pub type CurrentMin<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn current_max)]
    /// The max fund set by the root for initiate wrapping
    pub type CurrentMax<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn total_initiated)]
    /// The sum of wNodl funds that is initiated by this pallet so far.
    pub type TotalInitiated<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_settled)]
    /// The sum of wNodl funds that is settled by this pallet so far.
    pub type TotalSettled<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_rejected)]
    /// The sum of wrapping that couldn't be settled for any reasons and thus rejected.
    pub type TotalRejected<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn balances)]
    /// The amount of initiated `wNODL`, settled and rejected amount per known customer.
    pub type Balances<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
        ValueQuery,
    >;

    #[cfg(feature = "runtime-benchmarks")]
    #[pallet::storage]
    #[pallet::getter(fn benchmark_known_customers)]
    /// An internally kept list for the benchmark tests.
    pub type WhitelistedCallers<T: Config> = StorageValue<_, Vec<T::AccountId>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Wrapping customer's fund is initiated \[account's address on Nodle chain, amount of Nodl fund, destination address on Ethereum main-net\]
        WrappingInitiated(T::AccountId, BalanceOf<T>, EthAddress),

        /// Wrapping Reserve fund is initiated \[account's address on Nodle chain, amount of Nodl fund, destination address on Ethereum main-net\]
        WrappingReserveInitiated(BalanceOf<T>, EthAddress),

        /// Wrapping customer's fund is settled \[account's address on Nodle chain, amount of Nodl fund settled, Transaction hash on Ethereum main-net\]
        WrappingSettled(T::AccountId, BalanceOf<T>, EthTxHash),

        /// Wrapping customer's fund is settled \[account's address on Nodle chain, amount of Nodl fund settled, destination address on Ethereum main-net, rejection code\]
        WrappingRejected(T::AccountId, BalanceOf<T>, EthAddress, RejectionCode),

        /// Wrapping Reserve fund is settled \[account's address on Nodle chain, amount of Nodl fund settled, Transaction hash on Ethereum main-net\]
        WrappingReserveSettled(BalanceOf<T>, EthTxHash),

        /// Wrapping customer's fund is settled \[account's address on Nodle chain, amount of Nodl fund settled, destination address on Ethereum main-net, reason\]
        WrappingReserveRejected(BalanceOf<T>, EthAddress, Vec<u8>),

        /// Wrapping limits is set \[minimum amount, maximum amount\]
        LimitSet(BalanceOf<T>, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The balance of the account is not sufficient for the requested transaction
        BalanceNotEnough,
        /// The amount of fund to wrap should be between the pre-specified limits for the pallet.
        FundNotWithinLimits,
        /// We do not expect this error ever happen. But if it happened we would not allow it to mess with the correctness of the storage.
        BalanceOverflow,
        /// The customer is not known, whitelisted for this operation.
        NotEligible,
        /// Settlement is only possible for an amount equal or smaller than an initiated wrapping for a known customer
        InvalidSettle,
        /// Reject is only possible for an amount equal or smaller than an initiated wrapping for a known customer
        InvalidReject,
        /// Min must be less than max when setting limits
        InvalidLimits,
        /// Initiating, settling or rejecting with zero amount is useless and prevented
        UselessZero,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub min_wrapping: BalanceOf<T>,
        pub max_wrapping: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                min_wrapping: One::one(),
                max_wrapping: Bounded::max_value(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <CurrentMin<T>>::put(self.min_wrapping);
            <CurrentMax<T>>::put(self.max_wrapping);
        }
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }

        /// Direct implementation of `GenesisBuild::assimilate_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
            <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(T::WeightInfo::initiate_wrapping())]
        pub fn initiate_wrapping(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), <Error<T>>::NotEligible);
            } else {
                ensure!(T::KnownCustomers::contains(&who), <Error<T>>::NotEligible);
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::KnownCustomers::contains(&who), <Error<T>>::NotEligible);

            let current_min = <CurrentMin<T>>::get().unwrap_or_else(Zero::zero);
            let current_max = <CurrentMax<T>>::get().unwrap_or_else(Bounded::max_value);
            ensure!(
                amount >= current_min && amount <= current_max,
                <Error<T>>::FundNotWithinLimits
            );

            ensure!(
                T::Currency::can_reserve(&who, amount),
                <Error<T>>::BalanceNotEnough
            );

            let current_total_initiated = <TotalInitiated<T>>::get();
            let total_initiated = current_total_initiated
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = <Balances<T>>::get(who.clone());
            let initiated = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            T::Currency::reserve(&who, amount)?;
            <TotalInitiated<T>>::put(total_initiated);
            <Balances<T>>::insert(who.clone(), (initiated, balances.1, balances.2));

            Self::deposit_event(Event::WrappingInitiated(who, amount, eth_dest));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl from the reserve account. No min or max check.
        /// The reserve should only have that much Nodl.
        #[pallet::weight(T::WeightInfo::initiate_wrapping_reserve_fund())]
        pub fn initiate_wrapping_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            let reserve_account_id = T::Reserve::account_id();
            ensure!(
                T::Currency::can_reserve(&reserve_account_id, amount),
                <Error<T>>::BalanceNotEnough
            );

            let current_total_initiated = <TotalInitiated<T>>::get();
            let total_initiated = current_total_initiated
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = <Balances<T>>::get(reserve_account_id.clone());
            let initiated = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            T::Currency::reserve(&reserve_account_id, amount)?;
            <TotalInitiated<T>>::put(total_initiated);
            <Balances<T>>::insert(
                reserve_account_id.clone(),
                (initiated, balances.1, balances.2),
            );

            Self::deposit_event(Event::WrappingReserveInitiated(amount, eth_dest));
            Ok(Pays::No.into())
        }

        /// Settle a previously initiated wrapping with an ethereum hash as the proof of the
        /// corresponding transaction on Ethereum. Please remember it's possible that one or more
        /// than one wrapping requests to be settled through one or more than one Ethereum
        /// transactions. So a settle is believed valid if its amount is not exceeding the part of
        /// the total initiated amount for a customer that's not settled yet.
        /// Settling will transfer the reserve fund to the reserve as the bot will be spending from
        /// the reserve on Ethereum to settle them.
        #[pallet::weight(T::WeightInfo::settle())]
        pub fn settle(
            origin: OriginFor<T>,
            customer: T::AccountId,
            amount: BalanceOf<T>,
            eth_hash: EthTxHash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), <Error<T>>::NotEligible);
            } else {
                ensure!(T::Oracles::contains(&who), <Error<T>>::NotEligible);
                ensure!(
                    T::KnownCustomers::contains(&customer),
                    <Error<T>>::NotEligible
                );
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::Oracles::contains(&who), <Error<T>>::NotEligible);
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(
                T::KnownCustomers::contains(&customer),
                <Error<T>>::NotEligible
            );

            let balances = <Balances<T>>::get(customer.clone());
            let settled = balances
                .1
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = settled
                .checked_add(&balances.2)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, <Error<T>>::InvalidSettle);

            let current_total_settled = <TotalSettled<T>>::get();
            let total_settled = current_total_settled
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            <TotalSettled<T>>::put(total_settled);
            <Balances<T>>::insert(customer.clone(), (balances.0, settled, balances.2));

            let (negative_imbalance, _) = T::Currency::slash_reserved(&customer, amount);
            T::Reserve::on_nonzero_unbalanced(negative_imbalance);

            Self::deposit_event(Event::WrappingSettled(customer, amount, eth_hash));
            Ok(())
        }

        /// The reject is for those requests that the oracle couldn't settle for various reasons
        /// and the issue seemed to be persistent on the side of Ethereum. The rejection has the
        /// benefit of un-reserving the customer's fund and allowing them to try again later
        /// themselves if they wish. ReasonCode is s number that the oracle itself can interpret.
        /// A current example of a failure for which the oracle would reject the request is the case
        /// Where the customer has tried to transfer wNODL to the eth address:
        /// "0x0000000000000000000000000000000000000000". wnodl-pallet would not give an
        /// immediate error on such a request as it's designed to be as agnostics as possible about
        /// the ethereum side.
        #[pallet::weight(T::WeightInfo::reject())]
        pub fn reject(
            origin: OriginFor<T>,
            customer: T::AccountId,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
            reason: RejectionCode,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), <Error<T>>::NotEligible);
            } else {
                ensure!(T::Oracles::contains(&who), <Error<T>>::NotEligible);
                ensure!(
                    T::KnownCustomers::contains(&customer),
                    <Error<T>>::NotEligible
                );
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::Oracles::contains(&who), <Error<T>>::NotEligible);
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(
                T::KnownCustomers::contains(&customer),
                <Error<T>>::NotEligible
            );

            let balances = <Balances<T>>::get(customer.clone());
            let rejected = balances
                .2
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = rejected
                .checked_add(&balances.1)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, <Error<T>>::InvalidReject);

            let current_total_rejected = <TotalRejected<T>>::get();
            let total_rejected = current_total_rejected
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            <TotalRejected<T>>::put(total_rejected);
            <Balances<T>>::insert(customer.clone(), (balances.0, balances.1, rejected));

            let _ = T::Currency::unreserve(&customer, amount);

            Self::deposit_event(Event::WrappingRejected(customer, amount, eth_dest, reason));
            Ok(())
        }

        /// The root committee settles an initiated wrapping of a fund from the reserve through
        /// this function. They will attach the ethereum hash of the corresponding transaction on
        /// Ethereum to this request. Settling fund from the reserve will burn it, as this amount
        /// should now exist on Ethereum.
        #[pallet::weight(T::WeightInfo::settle_reserve_fund())]
        pub fn settle_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_hash: EthTxHash,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            let reserve_account_id = T::Reserve::account_id();

            let balances = <Balances<T>>::get(reserve_account_id.clone());
            let settled = balances
                .1
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = settled
                .checked_add(&balances.2)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, <Error<T>>::InvalidSettle);

            let current_total_settled = <TotalSettled<T>>::get();
            let total_settled = current_total_settled
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            <TotalSettled<T>>::put(total_settled);
            <Balances<T>>::insert(
                reserve_account_id.clone(),
                (balances.0, settled, balances.2),
            );

            // We burn the settled reserve fund and thus drop the following imbalance.
            let _ = T::Currency::slash_reserved(&reserve_account_id, amount);

            Self::deposit_event(Event::WrappingReserveSettled(amount, eth_hash));
            Ok(Pays::No.into())
        }

        /// If the root committee need to un-reserve a fund that was taken from the reserve to be
        /// turned to wNODL, they can reject that and explain the reason to be recorded on chain.
        #[pallet::weight(T::WeightInfo::reject_reserve_fund(reason.len() as u32))]
        pub fn reject_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
            reason: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(!amount.is_zero(), <Error<T>>::UselessZero);

            let reserve_account_id = T::Reserve::account_id();

            let balances = <Balances<T>>::get(reserve_account_id.clone());
            let rejected = balances
                .2
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = rejected
                .checked_add(&balances.1)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, <Error<T>>::InvalidReject);

            let current_total_rejected = <TotalRejected<T>>::get();
            let total_rejected = current_total_rejected
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            <TotalRejected<T>>::put(total_rejected);
            <Balances<T>>::insert(
                reserve_account_id.clone(),
                (balances.0, balances.1, rejected),
            );

            let _ = T::Currency::unreserve(&reserve_account_id, amount);

            Self::deposit_event(Event::WrappingReserveRejected(amount, eth_dest, reason));
            Ok(Pays::No.into())
        }

        /// The root committee can set the limits (min and max) within which our known customers can
        /// try wrapping their funds in one request.
        #[pallet::weight(T::WeightInfo::set_wrapping_limits())]
        pub fn set_wrapping_limits(
            origin: OriginFor<T>,
            min: BalanceOf<T>,
            max: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(!min.is_zero() && min < max, <Error<T>>::InvalidLimits);
            <CurrentMin<T>>::put(min);
            <CurrentMax<T>>::put(max);

            Self::deposit_event(Event::LimitSet(min, max));
            Ok(Pays::No.into())
        }
    }
}
