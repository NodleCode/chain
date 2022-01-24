use super::*;
use crate::mock::*;

use frame_support::{assert_noop, assert_ok};

#[test]
fn known_customer_can_initiate_wrapping() {
    new_test_ext().execute_with(|| {
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(KNOWN_CUSTOMERS[0]),
            42,
            EthAddress::from(&[0u8; 20])
        ));
        assert_eq!(Wnodl::total_initiated(), Some(42));
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), Some((42, 0)));
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
            Error::<Test>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), None);
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(NON_ELIGIBLE_CUSTOMERS[0]), None);
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
            Error::<Test>::BalanceNotEnough
        );
        assert_eq!(Wnodl::total_initiated(), None);
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), None);
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

        assert_eq!(Wnodl::total_initiated(), Some(amount1 + amount2));
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), Some((amount1, 0)));
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[1]), Some((amount2, 0)));
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

        assert_eq!(Wnodl::total_initiated(), Some(amount1 + amount2));
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(
            Wnodl::balances(KNOWN_CUSTOMERS[0]),
            Some((amount1 + amount2, 0))
        );
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
        assert_eq!(Wnodl::total_initiated(), Some(amount));
        assert_eq!(Wnodl::total_settled(), Some(amount));
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), Some((amount, amount)));
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
            Error::<Test>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), Some(amount));
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), Some((amount, 0)));
    });
}

#[test]
fn trusted_oracle_cannot_settle_for_unknown_customer() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Wnodl::settle(
                Origin::signed(TRUSTED_ORACLES[0]),
                NON_ELIGIBLE_CUSTOMERS[0],
                0,
                EthTxHash::from(&[0u8; 32])
            ),
            Error::<Test>::NotEligible
        );
        assert_eq!(Wnodl::total_initiated(), None);
        assert_eq!(Wnodl::total_settled(), None);
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
        assert_eq!(Wnodl::total_initiated(), Some(amount));
        assert_eq!(Wnodl::total_settled(), Some(amount - 1));
        assert_eq!(
            Wnodl::balances(KNOWN_CUSTOMERS[0]),
            Some((amount, amount - 1))
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
            Error::<Test>::InvalidSettle
        );
        assert_eq!(Wnodl::total_initiated(), Some(amount));
        assert_eq!(Wnodl::total_settled(), None);
        assert_eq!(Wnodl::balances(KNOWN_CUSTOMERS[0]), Some((amount, 0)));
    });
}
