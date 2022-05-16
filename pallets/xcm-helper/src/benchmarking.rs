//! Crowdloans pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::{self, RawOrigin as SystemOrigin};

const XCM_WEIGHT_FEE: XcmWeightFeeMisc<Weight, Balance> = XcmWeightFeeMisc {
    weight: 3_000_000_000,
    fee: 50000000000u128,
};

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    where_clause {
        where
            <T as frame_system::Config>::Origin: From<pallet_xcm::Origin>
    }

    update_xcm_weight_fee {
     }: _(SystemOrigin::Root, XcmCall::AddMemo, XCM_WEIGHT_FEE)
    verify {
        assert_last_event::<T>(Event::XcmWeightFeeUpdated(XCM_WEIGHT_FEE).into())
    }

}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
