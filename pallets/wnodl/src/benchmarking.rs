//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{account, benchmarks, vec, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

benchmarks! {
    initiate_wrapping {
        let caller: T::AccountId = whitelisted_caller();
        WhitelistedCallers::<T>::put(vec![caller.clone()]);
        CurrencyOf::<T>::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        let eth_dest = EthAddress::from(&[0;20]);

    }: _(RawOrigin::Signed(caller), 100u32.into(), eth_dest)
    verify {
        assert_eq!(TotalInitiated::<T>::get(), Some(100u32.into()));
        assert_eq!(TotalSettled::<T>::get(), None);
    }

    settle {
        let caller: T::AccountId = whitelisted_caller();
        let customer: T::AccountId = account("customer", 0, 0);
        WhitelistedCallers::<T>::put(vec![caller.clone(), customer.clone()]);
        CurrencyOf::<T>::make_free_balance_be(&customer, BalanceOf::<T>::max_value());

        let eth_dest = EthAddress::from(&[0;20]);
        let _ = Template::<T>::initiate_wrapping(RawOrigin::Signed(customer.clone()).into(), 100u32.into(), eth_dest);

        let eth_hash = EthTxHash::from(&[0;32]);
    }: _(RawOrigin::Signed(caller), customer, 100u32.into(), eth_hash)
    verify {
        assert_eq!(TotalInitiated::<T>::get(), Some(100u32.into()));
        assert_eq!(TotalSettled::<T>::get(), Some(100u32.into()));
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
