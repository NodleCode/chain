//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use ethereum_types::Address as EthAddress;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    initiate_wrapping {
        let caller: T::AccountId = whitelisted_caller();
        WhitelistedCallers::<T>::put(vec![caller.clone()]);

        let eth_dest = EthAddress::from(&[0;20]);

    }: _(RawOrigin::Signed(caller), 100u32.into(), eth_dest)
    verify {
        assert_eq!(TotalInitiated::<T>::get(), Some(100u32.into()));
        assert_eq!(TotalSettled::<T>::get(), None);
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
