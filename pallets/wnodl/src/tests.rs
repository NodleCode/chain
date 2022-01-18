use crate::mock::*;
use ethereum_types::Address as EthAddress;
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(Wnodl::initiate_wrapping(
            Origin::signed(1),
            42,
            EthAddress::from(&[0u8; 20])
        ));
        assert_eq!(Wnodl::total_initiated(), Some(42));
        assert_eq!(Wnodl::total_settled(), None);
    });
}
