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
    });
}

#[test]
fn initiate_wrapping_generate_expected_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

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
