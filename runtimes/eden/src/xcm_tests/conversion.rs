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
fn test_convert_multi_location_to_currency_id() {
	ExtBuilder::default().build().execute_with(|| {
		let pallet_balance_index = <Balances as PalletInfoAccess>::index() as u8; //using same index as the built runtime
		let para_id_test = ParachainInfo::parachain_id();
		let nodle_multilocation_inside = MultiLocation {
			parents: 0,
			interior: Junctions::X2(Parachain(para_id_test.into()), PalletInstance(pallet_balance_index)),
		};

		assert_eq!(
			CurrencyIdConvert::convert(nodle_multilocation_inside),
			Some(CurrencyId::NodleNative)
		);

		let nodle_multilocation_outside = MultiLocation {
			parents: 1,
			interior: Junctions::X2(Parachain(para_id_test.into()), PalletInstance(pallet_balance_index)),
		};

		assert_eq!(
			CurrencyIdConvert::convert(nodle_multilocation_outside),
			Some(CurrencyId::NodleNative)
		);

		let random_multi = MultiLocation {
			parents: 0,
			interior: Junctions::X1(AccountId32 {
				network: NetworkId::Any,
				id: [0u8; 32],
			}),
		};

		assert_eq!(CurrencyIdConvert::convert(random_multi), None);

		let parent_multi = MultiLocation {
			parents: 1,
			interior: Here,
		};

		assert!(CurrencyIdConvert::convert(parent_multi).is_none());
	})
}

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
