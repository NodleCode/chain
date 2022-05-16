use super::*;
use crate::mock::{Call as TestCall, *};
use frame_support::{assert_noop, assert_ok};

use sp_runtime::traits::{One, Zero};

#[test]
fn update_xcm_fees_should_work() {
    new_test_ext().execute_with(|| {
        // check error code
        assert_noop!(
            XcmHelpers::update_xcm_weight_fee(
                frame_system::RawOrigin::Root.into(), // origin
                XcmCall::Bond,
                XcmWeightFeeMisc {
                    weight: One::one(),
                    fee: Zero::zero()
                }
            ),
            Error::<Test>::ZeroXcmFees
        );

        assert_noop!(
            XcmHelpers::update_xcm_weight_fee(
                frame_system::RawOrigin::Root.into(), // origin
                XcmCall::Bond,
                XcmWeightFeeMisc {
                    weight: Zero::zero(),
                    fee: One::one()
                }
            ),
            Error::<Test>::ZeroXcmWeightMisc
        );

        assert_ok!(XcmHelpers::update_xcm_weight_fee(
            frame_system::RawOrigin::Root.into(), // origin
            XcmCall::Bond,
            XcmWeightFeeMisc::default()
        ));

        assert_eq!(
            XcmWeightFee::<Test>::get(XcmCall::Bond),
            XcmWeightFeeMisc::default()
        );
    });
}

#[test]
fn withdraw_should_work() {
    new_test_ext().execute_with(|| {
        let para_id = ParaId::from(1337u32);

        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::withdraw(
            frame_system::RawOrigin::Root.into(), // origin
            para_id,
            ALICE,
            Box::new(call)
        ));
    });
}

#[test]
fn contribute_should_work() {
    new_test_ext().execute_with(|| {
        let para_id = ParaId::from(1337u32);
        let amount = 1_000;
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::contribute(
            frame_system::RawOrigin::Root.into(), // origin
            para_id,
            amount,
            ALICE,
            Box::new(call)
        ));
    });
}

#[test]
fn bond_should_work() {
    new_test_ext().execute_with(|| {
        let amount = 1_000;
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::bond(
            frame_system::RawOrigin::Root.into(), // origin
            amount,
            RewardDestination::Staked,
            ALICE,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn bond_extra_should_work() {
    new_test_ext().execute_with(|| {
        let amount = 1_000;
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::bond_extra(
            frame_system::RawOrigin::Root.into(), // origin
            amount,
            ALICE,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn unbond_should_work() {
    new_test_ext().execute_with(|| {
        let amount = 1_000;
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::unbond(
            frame_system::RawOrigin::Root.into(), // origin
            amount,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn rebond_should_work() {
    new_test_ext().execute_with(|| {
        let amount = 1_000;
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::rebond(
            frame_system::RawOrigin::Root.into(), // origin
            amount,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn withdraw_unbonded_should_work() {
    new_test_ext().execute_with(|| {
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::withdraw_unbonded(
            frame_system::RawOrigin::Root.into(), // origin
            1,
            ALICE,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn nominate_should_work() {
    new_test_ext().execute_with(|| {
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::nominate(
            frame_system::RawOrigin::Root.into(), // origin
            vec![ALICE],
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn send_as_sovereign_should_work() {
    new_test_ext().execute_with(|| {
        use xcm::latest::OriginKind::SovereignAccount;

        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });

        let assets: MultiAsset = (Here, 1_000_000_000).into();
        assert_ok!(XcmHelpers::send_as_sovereign(
            frame_system::RawOrigin::Root.into(), // origin
            Box::new(Parent.into()),
            Box::new(VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                BuyExecution {
                    fees: assets,
                    weight_limit: Limited(2_000_000)
                },
                Instruction::Transact {
                    origin_type: SovereignAccount,
                    require_weight_at_most: 1_000_000,
                    call: call.encode().into(),
                }
            ])))
        ));
    });
}

#[test]
fn ump_transact_should_work() {
    new_test_ext().execute_with(|| {
        let xcm_weight_fee_misc = XcmHelpers::xcm_weight_fee(XcmCall::AddProxy);
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::ump_transact(
            frame_system::RawOrigin::Root.into(), // origin
            call.encode().into(),
            xcm_weight_fee_misc.weight,
            Box::new(XcmHelpers::refund_location()),
            xcm_weight_fee_misc.fee,
        ));
    });
}

#[test]
fn add_proxy_should_work() {
    new_test_ext().execute_with(|| {
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::add_proxy(
            frame_system::RawOrigin::Root.into(), // origin
            ALICE,
            None,
            1,
            Box::new(call)
        ));
    });
}

#[test]
fn remove_proxy_should_work() {
    new_test_ext().execute_with(|| {
        let remark = "test".as_bytes().to_vec();
        let call = TestCall::System(frame_system::Call::remark { remark });
        assert_ok!(XcmHelpers::remove_proxy(
            frame_system::RawOrigin::Root.into(), // origin
            ALICE,
            None,
            1,
            Box::new(call)
        ));
    });
}
