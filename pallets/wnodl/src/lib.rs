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
    use ethereum_types::Address as EthAddress;
    use frame_support::{
        pallet_prelude::*,
        traits::{Contains, LockableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::Codec;
    use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, Zero};
    use sp_std::fmt::Debug;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The balance of the accounts and funds to wrap
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + MaxEncodedLen
            + TypeInfo;

        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

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
    #[pallet::getter(fn total_initiated)]
    /// The sum of wNodl funds that is initiated by this pallet so far.
    pub type TotalInitiated<T: Config> = StorageValue<_, T::Balance>;

    #[pallet::storage]
    #[pallet::getter(fn total_settled)]
    /// The sum of wNodl funds that is settled by this pallet so far.
    pub type TotalSettled<T: Config> = StorageValue<_, T::Balance>;

    #[pallet::storage]
    #[pallet::getter(fn balances)]
    /// The amount of initiated and settled wnodl for an account id.
    /// NOTE: keeping the trace of the jobs can be done fullly offchain through our oracle wNodl-bot
    /// and by monitoring the events. We will however keep them here for our customers convenienve
    /// This would make sense to create a custom rpc and an off-chain storage for this purpose later.
    pub type Balances<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, (T::Balance, T::Balance)>;

    #[cfg(feature = "runtime-benchmarks")]
    #[pallet::storage]
    #[pallet::getter(fn benchmark_known_customers)]
    /// An internally kept list for the benchmark tests.
    pub type BenchmarkKnownCustomers<T: Config> = StorageValue<_, Vec<T::AccountId>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Wrapping Nodl is initiated
        /// parameters. [account's address on Nodle chain, amount of Nodl fund, destination address on Ethereum main-net]
        WrappingInitiated(T::AccountId, T::Balance, EthAddress),

        /// Wrapping Nodl is settles
        /// parameters. [account's address on Nodle chain, amount of Nodl fund settled, Transaction hash on Ethereum main-net, reporting oracle's address]
        WrappingSettled(T::AccountId, T::Balance, EthAddress, T::AccountId),
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate wrapping an amount of Nodl into wnodl on Ethereum
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn initiate_wrapping(
            origin: OriginFor<T>,
            amount: T::Balance,
            eth_dest: EthAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            if let Some(benchmark_customers) = BenchmarkKnownCustomers::<T>::get() {
                ensure!(benchmark_customers.contains(&who), Error::<T>::NotEligible);
            } else {
                ensure!(T::KnownCustomers::contains(&who), Error::<T>::NotEligible);
            }
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(T::KnownCustomers::contains(&who), Error::<T>::NotEligible);

            let current_sum = TotalInitiated::<T>::get().unwrap_or_else(Zero::zero);
            let total = current_sum
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            let balances = Balances::<T>::get(who.clone()).unwrap_or((Zero::zero(), Zero::zero()));
            let total_for_origin = balances
                .0
                .checked_add(&amount)
                .ok_or::<Error<T>>(Error::BalanceOverflow)?;

            TotalInitiated::<T>::put(total);
            Balances::<T>::mutate(who.clone(), |x| {
                *x = Some((total_for_origin, balances.1));
            });

            Self::deposit_event(Event::WrappingInitiated(who, amount, eth_dest));
            Ok(())
        }
    }
}
