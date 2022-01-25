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
        traits::{Contains, Currency, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    pub use sp_core::H256 as EthTxHash;
    use sp_runtime::traits::{Bounded, CheckedAdd, Zero};

    pub type CurrencyOf<T> = <T as Config>::Currency;
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: ReservableCurrency<Self::AccountId>;

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Trusted bots/oracles which can settle funds after wrapping is initiated
        type Oracles: Contains<Self::AccountId>;

        /// The customers who've gone under the KYC process and are eligible to wrap their Nodls.  
        type KnownCustomers: Contains<Self::AccountId>;
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
    #[pallet::getter(fn balances)]
    /// The amount of initiated and settled `wNODL` for an account id.
    /// NOTE: keeping the trace of the jobs can be done fullly offchain through our oracle wNodl-bot
    /// and by monitoring the events. We will however keep them here for our customers convenienve
    /// This would make sense to create a custom rpc and an off-chain storage for this purpose later.
    pub type Balances<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, (BalanceOf<T>, BalanceOf<T>)>;

    #[cfg(feature = "runtime-benchmarks")]
    #[pallet::storage]
    #[pallet::getter(fn benchmark_known_customers)]
    /// An internally kept list for the benchmark tests.
    pub type WhitelistedCallers<T: Config> = StorageValue<_, Vec<T::AccountId>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Wrapping Nodl is initiated
        /// parameters. [account's address on Nodle chain, amount of Nodl fund, destination address on Ethereum main-net]
        WrappingInitiated(T::AccountId, BalanceOf<T>, EthAddress),

        /// Wrapping `NODL` is settled
        /// parameters. [account's address on Nodle chain, amount of Nodl fund settled, Transaction hash on Ethereum main-net, reporting oracle's address]
        WrappingSettled(T::AccountId, BalanceOf<T>, EthTxHash),

        /// Wrapping limits is set
        /// parameters. [minimum amount, maximum amount]
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
        /// Settlment is only possible for an amount equal or smaller than an initiated wrapping for a known customer
        InvalidSettle,
        /// Min must be less than max whern setting limits
        InvalidLimits,
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

            let current_sum = TotalInitiated::<T>::get().unwrap_or_else(Zero::zero);
            let total = current_sum
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = Balances::<T>::get(who.clone()).unwrap_or((Zero::zero(), Zero::zero()));
            let total_for_origin = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            T::Currency::reserve(&who, amount)?;
            TotalInitiated::<T>::put(total);
            Balances::<T>::insert(who.clone(), (total_for_origin, balances.1));

            Self::deposit_event(Event::WrappingInitiated(who, amount, eth_dest));
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

            let balances =
                Balances::<T>::get(customer.clone()).unwrap_or((Zero::zero(), Zero::zero()));
            let total_for_customer = balances
                .1
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;
            ensure!(balances.0 >= total_for_customer, Error::<T>::InvalidSettle);

            let current_sum = TotalSettled::<T>::get().unwrap_or_else(Zero::zero);
            let total = current_sum
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalSettled::<T>::put(total);
            Balances::<T>::insert(customer.clone(), (balances.0, total_for_customer));

            // Dropping the imbalnce below so it goes to the reverve treasury
            let _ = T::Currency::slash_reserved(&who, amount);

            Self::deposit_event(Event::WrappingSettled(customer, amount, eth_hash));
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
