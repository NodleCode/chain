use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(WNodl::initiate_wrapping(Origin::signed(1), 42, [0; 32]));
        assert_eq!(WNodl::total_initiated(), Some(42));
        assert_eq!(WNodl::total_settled(), Some(0));
    });
}
