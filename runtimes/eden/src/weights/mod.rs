pub mod cumulus_pallet_parachain_system;
pub mod frame_system;
pub mod pallet_allocations;
pub mod pallet_balances;
pub mod pallet_collator_selection;
pub mod pallet_collective;
pub mod pallet_contracts;
pub mod pallet_grants;
pub mod pallet_identity;
pub mod pallet_membership_allocations_oracles;
pub mod pallet_membership_technical_membership;
pub mod pallet_message_queue;
pub mod pallet_multisig;
pub mod pallet_nodle_uniques;
pub mod pallet_preimage;
pub mod pallet_proxy;
pub mod pallet_reserve_company_reserve;
pub mod pallet_reserve_dao_reserve;
pub mod pallet_reserve_international_reserve;
pub mod pallet_reserve_usa_reserve;
pub mod pallet_scheduler;
pub mod pallet_sponsorship;
pub mod pallet_timestamp;
pub mod pallet_uniques;
pub mod pallet_utility;
pub mod pallet_xcm;
mod pallet_xcm_benchmarks_fungible;
mod pallet_xcm_benchmarks_generic;

use crate::Runtime;
use frame_support::weights::Weight;

use pallet_xcm_benchmarks_fungible::WeightInfo as XcmBalancesWeight;
use pallet_xcm_benchmarks_generic::WeightInfo as XcmGeneric;

use cumulus_primitives_core::{
	All, AllCounted, AllOf, AllOfCounted, Asset, AssetFilter, AssetId, Assets, Fungible, Junction, Junctions, Location,
	OriginKind, QueryResponseInfo,
};
use sp_std::vec::Vec;
use xcm::{
	v3::{Error, MaybeErrorCode},
	DoubleEncoded,
};

/// Types of asset supported by the Nodle runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetTypes {
	/// An asset backed by `pallet-balances`.
	Balances,
	/// Unknown asset.
	Unknown,
}

impl From<&Asset> for AssetTypes {
	fn from(asset: &Asset) -> Self {
		match asset {
			Asset {
				id: AssetId(Location {
					parents: 0,
					interior: Junctions::Here,
				}),
				fun: Fungible(_),
			} => AssetTypes::Balances,
			Asset {
				id: AssetId(Location {
					parents: 0,
					interior: Junctions::X1(ref arc),
				}),
				fun: Fungible(_),
			} if matches!(arc.as_ref(), [Junction::PalletInstance(2)]) => AssetTypes::Balances,
			_ => AssetTypes::Unknown,
		}
	}
}

trait WeighMultiAssets {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> Weight;
}

// Nodle only knows about one asset, the balances pallet.
const MAX_ASSETS: u64 = 1;

impl WeighMultiAssets for AssetFilter {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> Weight {
		match self {
			Self::Definite(assets) => assets
				.inner()
				.iter()
				.map(From::from)
				.map(|t| match t {
					AssetTypes::Balances => balances_weight,
					AssetTypes::Unknown => Weight::MAX,
				})
				.fold(Weight::zero(), |acc, x| acc.saturating_add(x)),
			// We don't support any NFTs on Kusama, so these two variants will always match
			// only 1 kind of fungible asset.
			Self::Wild(AllOf { .. } | AllOfCounted { .. }) => balances_weight,
			Self::Wild(AllCounted(count)) => balances_weight.saturating_mul(MAX_ASSETS.min(*count as u64)),
			Self::Wild(All) => balances_weight.saturating_mul(MAX_ASSETS),
		}
	}
}

impl WeighMultiAssets for Assets {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> Weight {
		self.inner()
			.iter()
			.map(<AssetTypes as From<&Asset>>::from)
			.map(|t| match t {
				AssetTypes::Balances => balances_weight,
				AssetTypes::Unknown => Weight::MAX,
			})
			.fold(Weight::zero(), |acc, x| acc.saturating_add(x))
	}
}

pub struct NodleXcmWeight<RuntimeCall>(core::marker::PhantomData<RuntimeCall>);
impl<RuntimeCall> cumulus_primitives_core::XcmWeightInfo<RuntimeCall> for NodleXcmWeight<RuntimeCall> {
	fn withdraw_asset(assets: &Assets) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::withdraw_asset())
	}

	fn reserve_asset_deposited(assets: &Assets) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::reserve_asset_deposited())
	}

	fn receive_teleported_asset(assets: &Assets) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::receive_teleported_asset())
	}

	fn transfer_asset(assets: &Assets, _beneficiary: &Location) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::transfer_asset())
	}

	fn transfer_reserve_asset(assets: &Assets, _dest: &Location, _xcm: &xcm::latest::Xcm<()>) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::transfer_reserve_asset())
	}

	fn deposit_asset(assets: &AssetFilter, _beneficiary: &Location) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::deposit_asset())
	}

	fn deposit_reserve_asset(assets: &AssetFilter, _dest: &Location, _xcm: &xcm::latest::Xcm<()>) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::deposit_reserve_asset())
	}

	fn initiate_teleport(assets: &AssetFilter, _dest: &Location, _xcm: &xcm::latest::Xcm<()>) -> Weight {
		assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::initiate_teleport())
	}

	fn query_response(
		_query_id: &xcm::latest::QueryId,
		_response: &xcm::latest::Response,
		_max_weight: &Weight,
		_querier: &Option<Location>,
	) -> Weight {
		XcmGeneric::<Runtime>::query_response()
	}

	fn transact(
		_origin_kind: &OriginKind,
		_require_weight_at_most: &Weight,
		_call: &DoubleEncoded<RuntimeCall>,
	) -> Weight {
		XcmGeneric::<Runtime>::transact()
	}

	fn clear_origin() -> Weight {
		XcmGeneric::<Runtime>::clear_origin()
	}

	fn descend_origin(_who: &Junctions) -> Weight {
		XcmGeneric::<Runtime>::descend_origin()
	}

	fn report_error(_response_info: &QueryResponseInfo) -> Weight {
		XcmGeneric::<Runtime>::report_error()
	}

	fn initiate_reserve_withdraw(_assets: &AssetFilter, _reserve: &Location, _xcm: &xcm::latest::Xcm<()>) -> Weight {
		XcmBalancesWeight::<Runtime>::initiate_reserve_withdraw()
	}

	fn report_holding(_response_info: &QueryResponseInfo, _assets: &AssetFilter) -> Weight {
		XcmGeneric::<Runtime>::report_holding()
	}

	fn buy_execution(_fees: &Asset, _weight_limit: &xcm::latest::WeightLimit) -> Weight {
		XcmGeneric::<Runtime>::buy_execution()
	}

	fn refund_surplus() -> Weight {
		XcmGeneric::<Runtime>::refund_surplus()
	}

	fn set_error_handler(_xcm: &xcm::latest::Xcm<RuntimeCall>) -> Weight {
		XcmGeneric::<Runtime>::set_error_handler()
	}

	fn set_appendix(_xcm: &xcm::latest::Xcm<RuntimeCall>) -> Weight {
		XcmGeneric::<Runtime>::set_appendix()
	}

	fn clear_error() -> Weight {
		XcmGeneric::<Runtime>::clear_error()
	}

	fn claim_asset(_assets: &Assets, _ticket: &Location) -> Weight {
		XcmGeneric::<Runtime>::claim_asset()
	}

	fn trap(_code: &u64) -> Weight {
		XcmGeneric::<Runtime>::trap()
	}

	fn subscribe_version(_query_id: &xcm::latest::QueryId, _max_response_weight: &Weight) -> Weight {
		XcmGeneric::<Runtime>::subscribe_version()
	}

	fn unsubscribe_version() -> Weight {
		XcmGeneric::<Runtime>::unsubscribe_version()
	}

	fn burn_asset(_assets: &Assets) -> Weight {
		XcmGeneric::<Runtime>::burn_asset()
	}

	fn expect_asset(_assets: &Assets) -> Weight {
		XcmGeneric::<Runtime>::expect_asset()
	}

	fn expect_origin(_origin: &Option<Location>) -> Weight {
		XcmGeneric::<Runtime>::expect_origin()
	}

	fn expect_error(_error: &Option<(u32, Error)>) -> Weight {
		XcmGeneric::<Runtime>::expect_error()
	}

	fn expect_transact_status(_transact_status: &MaybeErrorCode) -> Weight {
		XcmGeneric::<Runtime>::expect_transact_status()
	}

	fn query_pallet(_module_name: &Vec<u8>, _response_info: &QueryResponseInfo) -> Weight {
		XcmGeneric::<Runtime>::query_pallet()
	}

	fn expect_pallet(
		_index: &u32,
		_name: &Vec<u8>,
		_module_name: &Vec<u8>,
		_crate_major: &u32,
		_min_crate_minor: &u32,
	) -> Weight {
		XcmGeneric::<Runtime>::expect_pallet()
	}

	fn report_transact_status(_response_info: &QueryResponseInfo) -> Weight {
		XcmGeneric::<Runtime>::report_transact_status()
	}

	fn clear_transact_status() -> Weight {
		XcmGeneric::<Runtime>::clear_transact_status()
	}

	fn set_fees_mode(_jit_withdraw: &bool) -> Weight {
		XcmGeneric::<Runtime>::set_fees_mode()
	}

	fn set_topic(_topic: &[u8; 32]) -> Weight {
		XcmGeneric::<Runtime>::set_topic()
	}

	fn clear_topic() -> Weight {
		XcmGeneric::<Runtime>::clear_topic()
	}

	fn unpaid_execution(_weight_limit: &xcm::latest::WeightLimit, _check_origin: &Option<Location>) -> Weight {
		XcmGeneric::<Runtime>::unpaid_execution()
	}

	fn hrmp_new_channel_open_request(_sender: &u32, _max_message_size: &u32, _max_capacity: &u32) -> Weight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}

	fn hrmp_channel_accepted(_recipient: &u32) -> Weight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}

	fn hrmp_channel_closing(_initiator: &u32, _sender: &u32, _recipient: &u32) -> Weight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}

	fn exchange_asset(_give: &AssetFilter, _want: &Assets, _maximal: &bool) -> Weight {
		// Nodle XCM Executor does not support exchange asset
		Weight::MAX
	}

	fn universal_origin(_junction: &xcm::latest::Junction) -> Weight {
		// Nodle Xcm Executor does not have a configured `UniversalAliases` needed for this
		Weight::MAX
	}

	fn export_message(
		_network: &xcm::latest::NetworkId,
		_destination: &Junctions,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		// To be fixed upstream
		Weight::MAX
	}

	fn lock_asset(_asset: &Asset, _unlocker: &Location) -> Weight {
		// Nodle Xcm Executor does not support locking/unlocking assets
		Weight::MAX
	}

	fn unlock_asset(_asset: &Asset, _target: &Location) -> Weight {
		// Nodle Xcm Executor does not support locking/unlocking assets
		Weight::MAX
	}

	fn note_unlockable(_asset: &Asset, _owner: &Location) -> Weight {
		// Nodle Xcm Executor does not support locking/unlocking assets
		Weight::MAX
	}

	fn request_unlock(_asset: &Asset, _locker: &Location) -> Weight {
		// Nodle Xcm Executor does not support locking/unlocking assets
		Weight::MAX
	}

	fn alias_origin(_origin: &Location) -> Weight {
		// XCM Executor does not currently support alias origin operations
		Weight::MAX
	}
}

#[cfg(test)]
mod test {
	use cumulus_primitives_core::{AssetInstance, Fungibility::NonFungible};
	use xcm::latest::prelude::*;
	use Junction::PalletInstance;

	use super::*;

	#[test]
	fn test_multi_asset_conversion_to_asset_types() {
		let asset = Asset {
			id: AssetId(Location {
				parents: 0,
				interior: PalletInstance(2).into(),
			}),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Balances);

		let asset = Asset {
			id: AssetId(Location {
				parents: 0,
				interior: Here,
			}),
			fun: Fungible(43),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Balances);

		let asset = Asset {
			id: AssetId(Location {
				parents: 0,
				interior: PalletInstance(3).into(),
			}),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = Asset {
			id: AssetId(Location {
				parents: 1,
				interior: Here,
			}),
			fun: Fungible(43),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = Asset {
			id: [0_u8; 32].into(),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = Asset {
			id: [0_u8; 32].into(),
			fun: NonFungible(AssetInstance::Index(0)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = Asset {
			id: AssetId(Location {
				parents: 0,
				interior: Here,
			}),
			fun: NonFungible(AssetInstance::Index(2)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = Asset {
			id: AssetId(Location {
				parents: 0,
				interior: PalletInstance(2).into(),
			}),
			fun: NonFungible(AssetInstance::Index(0)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);
	}
}
