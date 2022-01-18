use super::*;
use crate::mock::*;
use ethereum_types::Address as EthAddress;
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
    });
}
