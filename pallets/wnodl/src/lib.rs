#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    pub use ethereum_types::Address as EthAddress;
    use frame_support::{
        pallet_prelude::*,
        traits::{Contains, Currency, OnUnbalanced, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    pub use sp_core::{Bytes, H256 as EthTxHash};
    use sp_runtime::traits::{Bounded, CheckedAdd, Zero};
    pub use support::WithAccountId;

    pub type CurrencyOf<T> = <T as Config>::Currency;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

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
    pub type TotalInitiated<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn total_settled)]
    /// The sum of wNodl funds that is settled by this pallet so far.
    pub type TotalSettled<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn total_rejected)]
    /// The sum of wrapping that couldn't be settled for any reasons and thus rejected.
    pub type TotalRejected<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn balances)]
    /// The amount of initiated `wNODL`, settled and rejected amount per known customer.
    pub type Balances<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>)>;

    #[cfg(feature = "runtime-benchmarks")]
    use frame_benchmarking::Vec;
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

        /// Wrapping customer's fund is settled \[account's address on Nodle chain, amount of Nodl fund settled, destination address on Ethereum main-net, reason\]
        WrappingRejected(T::AccountId, BalanceOf<T>, EthAddress, Vec<u8>),

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
                min_wrapping: Zero::zero(),
                max_wrapping: Bounded::max_value(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            CurrentMin::<T>::put(self.min_wrapping);
            CurrentMax::<T>::put(self.max_wrapping);
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
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn initiate_wrapping(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), Error::<T>::NotEligible);
            } else {
                ensure!(T::KnownCustomers::contains(&who), Error::<T>::NotEligible);
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::KnownCustomers::contains(&who), Error::<T>::NotEligible);

            let current_min = CurrentMin::<T>::get().unwrap_or_else(Zero::zero);
            let current_max = CurrentMax::<T>::get().unwrap_or_else(Bounded::max_value);
            ensure!(
                amount >= current_min && amount <= current_max,
                Error::<T>::FundNotWithinLimits
            );

            ensure!(
                T::Currency::can_reserve(&who, amount),
                Error::<T>::BalanceNotEnough
            );

            let current_total_initiated = TotalInitiated::<T>::get().unwrap_or_else(Zero::zero);
            let total_initiated = current_total_initiated
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = Balances::<T>::get(who.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let initiated = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            T::Currency::reserve(&who, amount)?;
            TotalInitiated::<T>::put(total_initiated);
            Balances::<T>::insert(who.clone(), (initiated, balances.1, balances.2));

            Self::deposit_event(Event::WrappingInitiated(who, amount, eth_dest));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl from the reserve account. No min or max check.
        /// The reserve shoud only have that much Nodl
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn initiate_wrapping_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let reserve_account_id = T::Reserve::account_id();
            ensure!(
                T::Currency::can_reserve(&reserve_account_id, amount),
                Error::<T>::BalanceNotEnough
            );

            let current_total_initiated = TotalInitiated::<T>::get().unwrap_or_else(Zero::zero);
            let total_initiated = current_total_initiated
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = Balances::<T>::get(reserve_account_id.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let initiated = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            T::Currency::reserve(&reserve_account_id, amount)?;
            TotalInitiated::<T>::put(total_initiated);
            Balances::<T>::insert(
                reserve_account_id.clone(),
                (initiated, balances.1, balances.2),
            );

            Self::deposit_event(Event::WrappingReserveInitiated(amount, eth_dest));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn settle(
            origin: OriginFor<T>,
            customer: T::AccountId,
            amount: BalanceOf<T>,
            eth_hash: EthTxHash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), Error::<T>::NotEligible);
            } else {
                ensure!(T::Oracles::contains(&who), Error::<T>::NotEligible);
                ensure!(
                    T::KnownCustomers::contains(&customer),
                    Error::<T>::NotEligible
                );
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::Oracles::contains(&who), Error::<T>::NotEligible);
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(
                T::KnownCustomers::contains(&customer),
                Error::<T>::NotEligible
            );

            let balances = Balances::<T>::get(customer.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let settled = balances
                .1
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = settled
                .checked_add(&balances.2)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, Error::<T>::InvalidSettle);

            let current_total_settled = TotalSettled::<T>::get().unwrap_or_else(Zero::zero);
            let total_settled = current_total_settled
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalSettled::<T>::put(total_settled);
            Balances::<T>::insert(customer.clone(), (balances.0, settled, balances.2));

            let (negative_imbalance, _) = T::Currency::slash_reserved(&customer, amount);
            T::Reserve::on_nonzero_unbalanced(negative_imbalance);

            Self::deposit_event(Event::WrappingSettled(customer, amount, eth_hash));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn reject(
            origin: OriginFor<T>,
            customer: T::AccountId,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
            reason: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(whitelisted_callers) = WhitelistedCallers::<T>::get() {
                ensure!(whitelisted_callers.contains(&who), Error::<T>::NotEligible);
            } else {
                ensure!(T::Oracles::contains(&who), Error::<T>::NotEligible);
                ensure!(
                    T::KnownCustomers::contains(&customer),
                    Error::<T>::NotEligible
                );
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::Oracles::contains(&who), Error::<T>::NotEligible);
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(
                T::KnownCustomers::contains(&customer),
                Error::<T>::NotEligible
            );

            let balances = Balances::<T>::get(customer.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let rejected = balances
                .2
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = rejected
                .checked_add(&balances.1)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, Error::<T>::InvalidReject);

            let current_total_rejected = TotalRejected::<T>::get().unwrap_or_else(Zero::zero);
            let total_rejected = current_total_rejected
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalRejected::<T>::put(total_rejected);
            Balances::<T>::insert(customer.clone(), (balances.0, balances.1, rejected));

            let _ = T::Currency::unreserve(&customer, amount);

            Self::deposit_event(Event::WrappingRejected(customer, amount, eth_dest, reason));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn settle_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_hash: EthTxHash,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let reserve_account_id = T::Reserve::account_id();

            let balances = Balances::<T>::get(reserve_account_id.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let settled = balances
                .1
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = settled
                .checked_add(&balances.2)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, Error::<T>::InvalidSettle);

            let current_total_settled = TotalSettled::<T>::get().unwrap_or_else(Zero::zero);
            let total_settled = current_total_settled
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalSettled::<T>::put(total_settled);
            Balances::<T>::insert(
                reserve_account_id.clone(),
                (balances.0, settled, balances.2),
            );

            // We burn the settled reserve fund and thus drop the following imbalance.
            let _ = T::Currency::slash_reserved(&reserve_account_id, amount);

            Self::deposit_event(Event::WrappingReserveSettled(amount, eth_hash));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn reject_reserve_fund(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_dest: EthAddress,
            reason: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let reserve_account_id = T::Reserve::account_id();

            let balances = Balances::<T>::get(reserve_account_id.clone()).unwrap_or((
                Zero::zero(),
                Zero::zero(),
                Zero::zero(),
            ));
            let rejected = balances
                .2
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            let settled_or_rejected = rejected
                .checked_add(&balances.1)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            // The amount of initiated wrapping should always be greater than or equal the sum of settled and rejected
            ensure!(balances.0 >= settled_or_rejected, Error::<T>::InvalidReject);

            let current_total_rejected = TotalRejected::<T>::get().unwrap_or_else(Zero::zero);
            let total_rejected = current_total_rejected
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalRejected::<T>::put(total_rejected);
            Balances::<T>::insert(
                reserve_account_id.clone(),
                (balances.0, balances.1, rejected),
            );

            let _ = T::Currency::unreserve(&reserve_account_id, amount);

            Self::deposit_event(Event::WrappingReserveRejected(amount, eth_dest, reason));
            Ok(())
        }

        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_wrapping_limits(
            origin: OriginFor<T>,
            min: BalanceOf<T>,
            max: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(min < max, Error::<T>::InvalidLimits);
            CurrentMin::<T>::put(min);
            CurrentMax::<T>::put(max);

            Self::deposit_event(Event::LimitSet(min, max));
            Ok(())
        }
    }
}
