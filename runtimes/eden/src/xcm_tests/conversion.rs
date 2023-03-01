use super::{super::Balances, mock::ExtBuilder};
use crate::{
	xcm_config::{CurrencyId, CurrencyIdConvert},
	xcm_tests::mock::ParachainInfo,
};
use frame_support::traits::PalletInfoAccess;
use sp_runtime::traits::Convert;
use xcm::prelude::*;
use xcm::v2::MultiLocation;
#[test]
fn test_convert_currency_id_to_multi_location() {
	let pallet_balance_index = <Balances as PalletInfoAccess>::index() as u8; //using same index as the built runtime
	let expected_nodl_location = MultiLocation {
		parents: 0,
		interior: Junctions::X1(PalletInstance(pallet_balance_index)), // Index of the pallet balance in the runtime
	};
	assert_eq!(
		CurrencyIdConvert::convert(CurrencyId::NodleNative),
		Some(expected_nodl_location)
	);
}
