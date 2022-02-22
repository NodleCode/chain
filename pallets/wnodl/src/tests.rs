use super::*;
use crate::mock::*;

use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::Currency};
use support::WithAccountId;

#[test]
fn known_customer_can_initiate_wrapping() {
    new_test_ext().execute_with(|| {
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            42,
            EthAddress::from(&[0u8; 20])
        ));
        assert_eq!(Wnodl::total_initiated(), 42);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (42, 0, 0));
    });
}

#[test]
fn known_customer_cannot_initiate_wrapping_zero_amount() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::initiate_wrapping(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                0,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::UselessZero
        );
    });
}

#[test]
fn non_eligible_customer_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::initiate_wrapping(
                Origin::signed(NON_ELIGIBLE_CUSTOMERS[0]),
                42,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&NON_ELIGIBLE_CUSTOMERS[0]), (0, 0, 0));
    });
}

#[test]
fn customer_on_low_balance_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::initiate_wrapping(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                CUSTOMER_BALANCE + 1,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::BalanceNotEnough
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (0, 0, 0));
    });
}

#[test]
fn amount_to_initiate_wrapping_should_be_greater_than_or_equal_min() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::initiate_wrapping(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                MIN_WRAP_AMOUNT - 1,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::FundNotWithinLimits
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (0, 0, 0));
    });
}

#[test]
fn amount_to_initiate_wrapping_should_be_less_than_or_equal_max() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::initiate_wrapping(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                MAX_WRAP_AMOUNT + 1,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::FundNotWithinLimits
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (0, 0, 0));
    });
}

#[test]
fn keep_track_of_initiated_wnodl() {
    new_test_ext().execute_with(|| {
        let amount1 = 42u64;
        let amount2 = 36u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount1,
            EthAddress::from(&[0u8; 20])
        ),);
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[1]),
            amount2,
            EthAddress::from(&[0u8; 20])
        ),);

        assert_eq!(Wnodl::total_initiated(), amount1 + amount2);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount1, 0, 0));
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[1]), (amount2, 0, 0));
    });
}

#[test]
fn keep_track_of_initiated_wnodl_per_customer() {
    new_test_ext().execute_with(|| {
        let amount1 = CUSTOMER_BALANCE / 2;
        let amount2 = CUSTOMER_BALANCE / 2;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount1,
            EthAddress::from(&[0u8; 20])
        ),);
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount2,
            EthAddress::from(&[0u8; 20])
        ),);

        assert_eq!(Wnodl::total_initiated(), amount1 + amount2);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount1 + amount2, 0, 0)
        );
    });
}

#[test]
fn reserve_fund_on_initiate_wrapping() {
    new_test_ext().execute_with(|| {
        let amount = CUSTOMER_BALANCE / 2;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ),);
        assert!(mock::Balances::reserved_balance(&KNOWN_CUSTOMERS[0]) == amount);
    });
}

#[test]
fn settling_slash_reserved_fund() {
    new_test_ext().execute_with(|| {
        let amount = CUSTOMER_BALANCE / 3;
        assert!(mock::Balances::total_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE);
        assert!(mock::Balances::free_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE);
        assert!(mock::Balances::reserved_balance(&KNOWN_CUSTOMERS[0]) == 0);
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ),);
        assert!(mock::Balances::total_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE);
        assert!(mock::Balances::free_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE - amount);
        assert!(mock::Balances::reserved_balance(&KNOWN_CUSTOMERS[0]) == amount);
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount,
            EthTxHash::from(&[0u8; 32])
        ));
        assert!(mock::Balances::total_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE - amount);
        assert!(mock::Balances::free_balance(&KNOWN_CUSTOMERS[0]) == CUSTOMER_BALANCE - amount);
        assert!(mock::Balances::reserved_balance(&KNOWN_CUSTOMERS[0]) == 0);
    });
}

#[test]
fn initiate_wrapping_generate_expected_event() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        let eth_address = EthAddress::from(&[
            0u8, 1, 2, 3, 4, 5, 7, 11, 13, 22, 33, 12, 26, 14, 45, 48, 17, 36, 19, 99,
        ]);

        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            eth_address
        ));
        assert_eq!(
            last_event(),
            mock::Event::Wnodl(
                crate::Event::WrappingInitiated(KNOWN_CUSTOMERS[0], amount, eth_address).into()
            )
        );
    });
}

#[test]
fn trusted_oracle_can_settle() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount,
            EthTxHash::from(&[0u8; 32])
        ));
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), amount);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, amount, 0));
    });
}

#[test]
fn trusted_oracle_can_reject() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        let eth_address = EthAddress::from(&[
            0u8, 1, 2, 3, 4, 5, 7, 11, 13, 22, 33, 12, 26, 14, 45, 48, 17, 36, 19, 99,
        ]);
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            eth_address
        ));
        let rejection_code = 127u32;
        assert_ok!(Wnodl::reject(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount,
            eth_address,
            rejection_code
        ));
        assert_eq!(
            last_event(),
            mock::Event::Wnodl(
                crate::Event::WrappingRejected(
                    KNOWN_CUSTOMERS[0],
                    amount,
                    eth_address,
                    rejection_code
                )
                .into()
            )
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), amount);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, amount));
    });
}

#[test]
fn trusted_oracle_cannot_settle_or_reject_zero_amount() {
    new_test_ext().execute_with(|| {
        let amount = 0u64;
        assert_noop!(
            Wnodl::settle(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount,
                EthTxHash::from(&[0u8; 32])
            ),
            <Error<Test>>::UselessZero
        );
        assert_noop!(
            Wnodl::reject(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount,
                EthAddress::from(&[0u8; 20]),
                0
            ),
            <Error<Test>>::UselessZero
        );
    });
}

#[test]
fn unknown_oracle_cannot_settle() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_noop!(
            Wnodl::settle(
                Origin::signed(NON_ELIGIBLE_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount,
                EthTxHash::from(&[0u8; 32])
            ),
            <Error<Test>>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, 0));
    });
}

#[test]
fn unknown_oracle_cannot_reject() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_noop!(
            Wnodl::reject(
                Origin::signed(NON_ELIGIBLE_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount,
                EthAddress::from(&[0u8; 20]),
                0
            ),
            <Error<Test>>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, 0));
    });
}

#[test]
fn trusted_oracle_cannot_settle_for_unknown_customer() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::settle(
                Origin::signed(TRUSTED_ORACLES[0]),
                NON_ELIGIBLE_CUSTOMERS[0],
                1,
                EthTxHash::from(&[0u8; 32])
            ),
            <Error<Test>>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
    });
}

#[test]
fn trusted_oracle_cannot_reject_for_unknown_customer() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::reject(
                Origin::signed(TRUSTED_ORACLES[0]),
                NON_ELIGIBLE_CUSTOMERS[0],
                1,
                EthAddress::from(&[0u8; 20]),
                0
            ),
            <Error<Test>>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), 0);
        assert_eq!(Wnodl::total_settled(), 0);
    });
}

#[test]
fn settling_les_than_initiated_is_ok() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount - 1,
            EthTxHash::from(&[0u8; 32])
        ));
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), amount - 1);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount, amount - 1, 0)
        );
    });
}

#[test]
fn partly_settled_partly_rejected_works() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount - 1,
            EthTxHash::from(&[0u8; 32])
        ));
        assert_ok!(Wnodl::reject(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            1,
            EthAddress::from(&[0u8; 20]),
            0
        ));
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), amount - 1);
        assert_eq!(Wnodl::total_rejected(), 1);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount, amount - 1, 1)
        );
    });
}

#[test]
fn partly_reject_fails_when_above_unsettled_part() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount - 1,
            EthTxHash::from(&[0u8; 32])
        ));
        assert_noop!(
            Wnodl::reject(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                2,
                EthAddress::from(&[0u8; 20]),
                0
            ),
            <Error<Test>>::InvalidReject
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), amount - 1);
        assert_eq!(Wnodl::total_rejected(), 0);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount, amount - 1, 0)
        );
    });
}

#[test]
fn partly_settle_fails_when_above_unsettled_part() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::reject(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            2,
            EthAddress::from(&[0u8; 20]),
            0
        ));
        assert_noop!(
            Wnodl::settle(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount - 1,
                EthTxHash::from(&[0u8; 32]),
            ),
            <Error<Test>>::InvalidSettle
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), 2);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, 2));
    });
}

#[test]
fn several_initiation_can_be_responded_by_mix_of_settle_and_reject() {
    new_test_ext().execute_with(|| {
        let amount1 = 16u64;
        let amount2 = 21u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount1,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount2,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::reject(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount1,
            EthAddress::from(&[0u8; 20]),
            0
        ));
        assert_ok!(Wnodl::settle(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount2,
            EthTxHash::from(&[0u8; 32])
        ));
        assert_eq!(Wnodl::total_initiated(), amount1 + amount2);
        assert_eq!(Wnodl::total_settled(), amount2);
        assert_eq!(Wnodl::total_rejected(), amount1);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount1 + amount2, amount2, amount1)
        );
    });
}

#[test]
fn rejecting_les_than_initiated_is_ok() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_ok!(Wnodl::reject(
            Origin::signed(TRUSTED_ORACLES[0]),
            KNOWN_CUSTOMERS[0],
            amount - 1,
            EthAddress::from(&[0u8; 20]),
            0
        ));
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), amount - 1);
        assert_eq!(
            Wnodl::balances(&KNOWN_CUSTOMERS[0]),
            (amount, 0, amount - 1)
        );
    });
}

#[test]
fn settling_more_than_initiated_should_fail() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_noop!(
            Wnodl::settle(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount + 1,
                EthTxHash::from(&[0u8; 32])
            ),
            <Error<Test>>::InvalidSettle
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, 0));
    });
}

#[test]
fn rejecting_more_than_initiated_should_fail() {
    new_test_ext().execute_with(|| {
        let amount = 42u64;
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            amount,
            EthAddress::from(&[0u8; 20])
        ));
        assert_noop!(
            Wnodl::reject(
                Origin::signed(TRUSTED_ORACLES[0]),
                KNOWN_CUSTOMERS[0],
                amount + 1,
                EthAddress::from(&[0u8; 20]),
                0
            ),
            <Error<Test>>::InvalidReject
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), 0);
        assert_eq!(Wnodl::balances(&KNOWN_CUSTOMERS[0]), (amount, 0, 0));
    });
}

#[test]
fn root_can_change_fund_limits() {
    new_test_ext().execute_with(|| {
        assert!(Wnodl::current_min().unwrap() == MIN_WRAP_AMOUNT);
        assert!(Wnodl::current_max().unwrap() == MAX_WRAP_AMOUNT);
        assert_ok!(Wnodl::set_wrapping_limits(
            Origin::root(),
            MIN_WRAP_AMOUNT - 1,
            MAX_WRAP_AMOUNT + 1
        ));
        assert!(Wnodl::current_min().unwrap() == MIN_WRAP_AMOUNT - 1);
        assert!(Wnodl::current_max().unwrap() == MAX_WRAP_AMOUNT + 1);
    });
}

#[test]
fn non_root_cannot_change_fund_limits() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::set_wrapping_limits(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                MIN_WRAP_AMOUNT - 1,
                MAX_WRAP_AMOUNT + 1
            ),
            BadOrigin
        );
    });
}

#[test]
fn root_cannot_set_min_greater_than_max() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::set_wrapping_limits(Origin::root(), MAX_WRAP_AMOUNT, MIN_WRAP_AMOUNT),
            <Error<Test>>::InvalidLimits
        );
    });
}

#[test]
fn root_cannot_set_min_to_zero() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::set_wrapping_limits(Origin::root(), 0, MAX_WRAP_AMOUNT),
            <Error<Test>>::InvalidLimits
        );
    });
}

#[test]
fn root_can_initiate_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        let eth_address = EthAddress::from(&[
            255u8, 1, 2, 3, 4, 5, 7, 11, 13, 22, 33, 12, 26, 14, 45, 48, 17, 36, 19, 99,
        ]);
        assert_ok!(Wnodl::initiate_wrapping_reserve_fund(
            Origin::root(),
            amount,
            eth_address
        ));
        assert_eq!(
            last_event(),
            mock::Event::Wnodl(crate::Event::WrappingReserveInitiated(amount, eth_address).into())
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);

        let reserve_account_id = mock::Reserve::account_id();
        assert_eq!(Wnodl::balances(reserve_account_id), (amount, 0, 0));
        assert!(mock::Balances::free_balance(&reserve_account_id) == RESERVE_BALANCE - amount);
        assert!(mock::Balances::reserved_balance(&reserve_account_id) == amount);
    });
}

#[test]
fn root_can_settle_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        let eth_address = EthAddress::from(&[
            255u8, 1, 2, 3, 4, 5, 7, 11, 13, 22, 33, 12, 26, 14, 45, 48, 17, 36, 19, 99,
        ]);
        assert_ok!(Wnodl::initiate_wrapping_reserve_fund(
            Origin::root(),
            amount,
            eth_address
        ));
        let eth_hash = EthTxHash::from(&[0u8; 32]);
        assert_ok!(Wnodl::settle_reserve_fund(
            Origin::root(),
            amount,
            eth_hash.clone()
        ));
        assert_eq!(
            last_event(),
            mock::Event::Wnodl(crate::Event::WrappingReserveSettled(amount, eth_hash).into())
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), amount);
        assert_eq!(Wnodl::total_rejected(), 0);

        let reserve_account_id = mock::Reserve::account_id();
        assert_eq!(Wnodl::balances(reserve_account_id), (amount, amount, 0));
        assert!(mock::Balances::free_balance(&reserve_account_id) == RESERVE_BALANCE - amount);
        assert!(mock::Balances::reserved_balance(&reserve_account_id) == 0);
    });
}

#[test]
fn root_can_reject_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        let eth_address = EthAddress::from(&[
            255u8, 1, 2, 3, 4, 5, 7, 11, 13, 22, 33, 12, 26, 14, 45, 48, 17, 36, 19, 99,
        ]);
        assert_ok!(Wnodl::initiate_wrapping_reserve_fund(
            Origin::root(),
            amount,
            eth_address.clone()
        ));
        let reason = "EThereum transaction failed".as_bytes().to_vec();
        assert_ok!(Wnodl::reject_reserve_fund(
            Origin::root(),
            amount,
            eth_address.clone(),
            reason.clone()
        ));
        assert_eq!(
            last_event(),
            mock::Event::Wnodl(
                crate::Event::WrappingReserveRejected(amount, eth_address, reason).into()
            )
        );
        assert_eq!(Wnodl::total_initiated(), amount);
        assert_eq!(Wnodl::total_settled(), 0);
        assert_eq!(Wnodl::total_rejected(), amount);

        let reserve_account_id = mock::Reserve::account_id();
        assert_eq!(Wnodl::balances(reserve_account_id), (amount, 0, amount));
        assert!(mock::Balances::free_balance(&reserve_account_id) == RESERVE_BALANCE);
        assert!(mock::Balances::reserved_balance(&reserve_account_id) == 0);
    });
}

#[test]
fn root_cannot_initiate_or_settle_or_reject_wrapping_from_reserve_fund_with_zero_amount() {
    new_test_ext().execute_with(|| {
        let amount = 0u64;
        assert_noop!(
            Wnodl::initiate_wrapping_reserve_fund(
                Origin::root(),
                amount,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::UselessZero
        );
        assert_noop!(
            Wnodl::settle_reserve_fund(Origin::root(), amount, EthTxHash::from(&[0u8; 32]),),
            <Error<Test>>::UselessZero
        );
        assert_noop!(
            Wnodl::reject_reserve_fund(
                Origin::root(),
                amount,
                EthAddress::from(&[0u8; 20]),
                "EThereum transaction failed".as_bytes().to_vec()
            ),
            <Error<Test>>::UselessZero
        );
    });
}

#[test]
fn non_root_cannot_initiate_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        assert_noop!(
            Wnodl::initiate_wrapping_reserve_fund(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                amount,
                EthAddress::from(&[0u8; 20])
            ),
            BadOrigin
        );
    });
}

#[test]
fn non_root_cannot_settle_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        assert_noop!(
            Wnodl::settle_reserve_fund(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                amount,
                EthTxHash::from(&[0u8; 32]),
            ),
            BadOrigin
        );
    });
}

#[test]
fn non_root_cannot_reject_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount = 43u64;
        assert_noop!(
            Wnodl::reject_reserve_fund(
                Origin::signed(KNOWN_CUSTOMERS[0]),
                amount,
                EthAddress::from(&[0u8; 20]),
                "EThereum transaction failed".as_bytes().to_vec()
            ),
            BadOrigin
        );
    });
}

#[test]
fn root_is_not_limited_to_min_max_when_initiating_wrapping_reserve_fund() {
    new_test_ext().execute_with(|| {
        let amount1 = MAX_WRAP_AMOUNT + 2;
        assert_ok!(Wnodl::initiate_wrapping_reserve_fund(
            Origin::root(),
            amount1,
            EthAddress::from(&[0u8; 20])
        ));
        let amount2 = MIN_WRAP_AMOUNT - 1;
        assert_ok!(Wnodl::initiate_wrapping_reserve_fund(
            Origin::root(),
            amount2,
            EthAddress::from(&[0u8; 20])
        ));
        assert_eq!(Wnodl::total_initiated(), amount1 + amount2);
        assert_eq!(Wnodl::total_settled(), 0);

        let reserve_account_id = mock::Reserve::account_id();
        assert_eq!(
            Wnodl::balances(reserve_account_id),
            (amount1 + amount2, 0, 0)
        );
        assert!(
            mock::Balances::free_balance(&reserve_account_id)
                == RESERVE_BALANCE - amount1 - amount2
        );
        assert!(mock::Balances::reserved_balance(&reserve_account_id) == amount1 + amount2);
    });
}

#[test]
fn root_cannot_initiate_wrapping_reserve_fund_above_balance() {
    new_test_ext().execute_with(|| {
        let amount = RESERVE_BALANCE + 1;
        assert_noop!(
            Wnodl::initiate_wrapping_reserve_fund(
                Origin::root(),
                amount,
                EthAddress::from(&[0u8; 20])
            ),
            <Error<Test>>::BalanceNotEnough
        );
    });
}
