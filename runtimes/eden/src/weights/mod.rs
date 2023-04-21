pub mod frame_system;
pub mod pallet_balances;
pub mod pallet_collator_selection;
pub mod pallet_contracts;
pub mod pallet_membership;
pub mod pallet_multisig;
pub mod pallet_preimage;
pub mod pallet_scheduler;
pub mod pallet_timestamp;
pub mod pallet_uniques;
pub mod pallet_utility;

mod pallet_xcm_benchmarks_fungible;
mod pallet_xcm_benchmarks_generic;

use frame_support::weights::Weight;

use sp_std::vec::Vec;
use xcm::latest::{Error, MaybeErrorCode, QueryResponseInfo};
use xcm::{
	v2::{prelude::*, Weight as XCMWeight},
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

impl From<&MultiAsset> for AssetTypes {
	fn from(asset: &MultiAsset) -> Self {
		match asset {
			MultiAsset {
				id: Concrete(MultiLocation {
					parents: 0,
					interior: Here,
				}),
				fun: Fungible(_),
			} => AssetTypes::Balances,
			MultiAsset {
				id: Concrete(MultiLocation {
					parents: 0,
					interior: X1(PalletInstance(2)),
				}),
				fun: Fungible(_),
			} => AssetTypes::Balances,
			_ => AssetTypes::Unknown,
		}
	}
}

trait WeighMultiAssets {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight;
}

// Kusama only knows about one asset, the balances pallet.
const MAX_ASSETS: u32 = 1;

impl WeighMultiAssets for MultiAssetFilter {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight {
		let weight = match self {
			Self::Definite(assets) => assets
				.inner()
				.iter()
				.map(From::from)
				.map(|t| match t {
					AssetTypes::Balances => balances_weight,
					AssetTypes::Unknown => Weight::MAX,
				})
				.fold(Weight::zero(), |acc, x| acc.saturating_add(x)),
			Self::Wild(_) => balances_weight.saturating_mul(MAX_ASSETS as u64),
		};

		weight.ref_time()
	}
}

impl WeighMultiAssets for MultiAssets {
	fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight {
		let weight = self
			.inner()
			.iter()
			.map(AssetTypes::from)
			.map(|t| match t {
				AssetTypes::Balances => balances_weight,
				AssetTypes::Unknown => Weight::MAX,
			})
			.fold(Weight::zero(), |acc, x| acc.saturating_add(x));

		weight.ref_time()
	}
}

pub struct NodleXcmWeight<RuntimeCall>(core::marker::PhantomData<RuntimeCall>);
impl<RuntimeCall> cumulus_primitives_core::XcmWeightInfo<RuntimeCall> for NodleXcmWeight<RuntimeCall> {
	fn withdraw_asset(_0: &xcm::latest::MultiAssets) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn reserve_asset_deposited(_0: &xcm::latest::MultiAssets) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn receive_teleported_asset(_0: &xcm::latest::MultiAssets) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn query_response(
		_query_id: &xcm::latest::QueryId,
		_response: &xcm::latest::Response,
		_max_weight: &Weight,
		_querier: &Option<xcm::latest::MultiLocation>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn transfer_asset(_assets: &xcm::latest::MultiAssets, _beneficiary: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn transfer_reserve_asset(
		_assets: &xcm::latest::MultiAssets,
		_dest: &xcm::latest::MultiLocation,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn transact(
		_origin_kind: &OriginKind,
		_require_weight_at_most: &Weight,
		_call: &DoubleEncoded<RuntimeCall>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn hrmp_new_channel_open_request(_sender: &u32, _max_message_size: &u32, _max_capacity: &u32) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn hrmp_channel_accepted(_recipient: &u32) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn hrmp_channel_closing(_initiator: &u32, _sender: &u32, _recipient: &u32) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn clear_origin() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn descend_origin(_0: &xcm::latest::InteriorMultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn report_error(_0: &QueryResponseInfo) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn deposit_asset(_assets: &xcm::latest::MultiAssetFilter, _beneficiary: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn deposit_reserve_asset(
		_assets: &xcm::latest::MultiAssetFilter,
		_dest: &xcm::latest::MultiLocation,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn exchange_asset(
		_give: &xcm::latest::MultiAssetFilter,
		_want: &xcm::latest::MultiAssets,
		_maximal: &bool,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn initiate_reserve_withdraw(
		_assets: &xcm::latest::MultiAssetFilter,
		_reserve: &xcm::latest::MultiLocation,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn initiate_teleport(
		_assets: &xcm::latest::MultiAssetFilter,
		_dest: &xcm::latest::MultiLocation,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn report_holding(_response_info: &QueryResponseInfo, _assets: &xcm::latest::MultiAssetFilter) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn buy_execution(_fees: &xcm::latest::MultiAsset, _weight_limit: &xcm::latest::WeightLimit) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn refund_surplus() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn set_error_handler(_0: &xcm::latest::Xcm<RuntimeCall>) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn set_appendix(_0: &xcm::latest::Xcm<RuntimeCall>) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn clear_error() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn claim_asset(_assets: &xcm::latest::MultiAssets, _ticket: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn trap(_0: &u64) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn subscribe_version(_query_id: &xcm::latest::QueryId, _max_response_weight: &Weight) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn unsubscribe_version() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn burn_asset(_0: &xcm::latest::MultiAssets) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn expect_asset(_0: &xcm::latest::MultiAssets) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn expect_origin(_0: &Option<xcm::latest::MultiLocation>) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn expect_error(_0: &Option<(u32, Error)>) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn expect_transact_status(_0: &MaybeErrorCode) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn query_pallet(_module_name: &Vec<u8>, _response_info: &QueryResponseInfo) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn expect_pallet(
		_index: &u32,
		_name: &Vec<u8>,
		_module_name: &Vec<u8>,
		_crate_major: &u32,
		_min_crate_minor: &u32,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn report_transact_status(_0: &QueryResponseInfo) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn clear_transact_status() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn universal_origin(_0: &xcm::latest::Junction) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn export_message(
		_network: &xcm::latest::NetworkId,
		_destination: &xcm::latest::InteriorMultiLocation,
		_xcm: &xcm::latest::Xcm<()>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn lock_asset(_asset: &xcm::latest::MultiAsset, _unlocker: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn unlock_asset(_asset: &xcm::latest::MultiAsset, _target: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn note_unlockable(_asset: &xcm::latest::MultiAsset, _owner: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn request_unlock(_asset: &xcm::latest::MultiAsset, _locker: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn set_fees_mode(_jit_withdraw: &bool) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn set_topic(_0: &[u8; 32]) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn clear_topic() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn alias_origin(_0: &xcm::latest::MultiLocation) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn unpaid_execution(
		_weight_limit: &xcm::latest::WeightLimit,
		_check_origin: &Option<xcm::latest::MultiLocation>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_multi_asset_conversion_to_asset_types() {
		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 0,
				interior: X1(PalletInstance(2)),
			}),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Balances);

		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 0,
				interior: Here,
			}),
			fun: Fungible(43),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Balances);

		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 0,
				interior: X1(PalletInstance(3)),
			}),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 1,
				interior: Here,
			}),
			fun: Fungible(43),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = MultiAsset {
			id: Abstract(vec![]),
			fun: Fungible(100),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = MultiAsset {
			id: Abstract(vec![]),
			fun: NonFungible(AssetInstance::Index(0)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 0,
				interior: Here,
			}),
			fun: NonFungible(AssetInstance::Index(2)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);

		let asset = MultiAsset {
			id: Concrete(MultiLocation {
				parents: 0,
				interior: X1(PalletInstance(2)),
			}),
			fun: NonFungible(AssetInstance::Index(0)),
		};
		let asset_type = AssetTypes::from(&asset);
		assert_eq!(asset_type, AssetTypes::Unknown);
	}
}
