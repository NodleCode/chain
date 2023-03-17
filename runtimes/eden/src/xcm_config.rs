use super::{
	AccountId, Balance, Balances, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent,
	RuntimeOrigin, XcmpQueue,
};
use crate::implementations::DealWithFees;
use codec::{Decode, Encode};
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::BenchmarkError;
use frame_support::{
	parameter_types,
	traits::{Everything, Nothing, PalletInfoAccess},
	weights::IdentityFee,
	RuntimeDebug,
};
use frame_system::EnsureRoot;
use orml_traits::{location::RelativeReserveProvider, parameter_type_with_key};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use scale_info::TypeInfo;
use sp_runtime::traits::Convert;
use xcm::{latest::NetworkId, latest::Weight as XcmWeight, prelude::*};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
	CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents, WeightInfoBounds,
};
use xcm_executor::XcmExecutor;
/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch RuntimeOrigin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the default `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type Barrier = (
	TakeWeightCredit,
	// If the message is one that immediately attempts to pay for execution, then allow it.
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Expected responses are OK.
	AllowKnownQueryResponses<PolkadotXcm>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);
parameter_types! {
	pub RelayLocation: MultiLocation = MultiLocation::parent();
	pub NodlLocation: MultiLocation = MultiLocation {
		parents:0,
		interior: Junctions::X1(
			PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
		)
	};
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);
/// Means for transacting assets on this chain.
pub type AssetTransactors = CurrencyTransactor;

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<NodlLocation>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Balances`.
	(),
>;
parameter_types! {
		// One XCM operation is 200_000_000 weight - almost certainly a conservative estimate.
		pub UnitWeightCost: u64 = 200_000_000;
		pub const MaxInstructions: u32 = 100;
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = ();
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<crate::weights::NodleXcmWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
	type Trader = UsingComponents<IdentityFee<Balance>, NodlLocation, AccountId, Balances, DealWithFees>;
	type ResponseHandler = (); // Don't handle responses for now.
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = WeightInfoBounds<crate::weights::NodleXcmWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}
impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(AccountId32 {
			network: NetworkId::Any,
			id: account.into(),
		})
		.into()
	}
}
parameter_types! {
	pub const BaseXcmWeight: XcmWeight = 100_000_000; // TODO: update based on the results of https://github.com/NodleCode/chain-workspace/issues/259
	pub const MaxAssetsForTransfer: usize = 2;
}
parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::here();
}
parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> Option<u128> {
		None
	};
}
#[derive(Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, TypeInfo, RuntimeDebug)]
pub enum CurrencyId {
	// NODL native token
	NodleNative,
}
pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::NodleNative => Some(NodlLocation::get()),
		}
	}
}

impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = RelativeReserveProvider;
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub const TrustedReserve: Option<(MultiLocation, MultiAsset)> = Some((
		MultiLocation::here(),
		MultiAsset{ id: Concrete(Here.into()), fun: Fungible(100) }
	));

}
#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub const TrustedTeleporter: Option<(MultiLocation, MultiAsset)> = Some((
		MultiLocation::parent(),
		MultiAsset{ id: Concrete(MultiLocation::parent()), fun: Fungible(100) }
	));
}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::generic::Config for Runtime {
	type RuntimeCall = RuntimeCall;

	fn worst_case_response() -> (u64, Response) {
		(0u64, Response::Version(Default::default()))
	}

	fn transact_origin() -> Result<MultiLocation, BenchmarkError> {
		Ok(MultiLocation::parent())
	}

	fn subscribe_origin() -> Result<MultiLocation, BenchmarkError> {
		Ok(MultiLocation::parent())
	}

	fn claimable_asset() -> Result<(MultiLocation, MultiLocation, MultiAssets), BenchmarkError> {
		let origin = MultiLocation::parent();
		let assets: MultiAssets = (Concrete(MultiLocation::parent()), 1_000u128).into();
		let ticket = MultiLocation {
			parents: 0,
			interior: Here,
		};
		Ok((origin, ticket, assets))
	}
}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::fungible::Config for Runtime {
	type TransactAsset = Balances;
	type CheckedAccount = ();
	type TrustedTeleporter = TrustedTeleporter;
	type TrustedReserve = TrustedReserve;
	fn get_multi_asset() -> MultiAsset {
		MultiAsset {
			id: Concrete(NodlLocation::get()),
			fun: Fungible(u128::MAX),
		}
	}
}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::Config for Runtime {
	type XcmConfig = XcmConfig;
	type AccountIdConverter = LocationToAccountId;
	fn valid_destination() -> Result<MultiLocation, BenchmarkError> {
		Ok(MultiLocation::parent())
	}
	fn worst_case_holding() -> MultiAssets {
		// 1 fungibles can be traded in the worst case: TODO: https://github.com/NodleCode/chain/issues/717
		let location: MultiLocation = GeneralIndex(0).into();
		let assets = MultiAsset {
			id: Concrete(location),
			fun: Fungible(u128::MAX),
		};
		assets.into()
	}
}

#[cfg(test)]
mod tests {

	use super::*;
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
	#[test]
	fn convert_accountid_to_multi_location() {
		let alice: sp_runtime::AccountId32 = [
			0x7e, 0xc8, 0x3e, 0x09, 0x72, 0xf3, 0xf3, 0xbe, 0xb9, 0x1b, 0xf3, 0x91, 0xf4, 0x57, 0x1a, 0x1a, 0xd5, 0x07,
			0x06, 0x71, 0x24, 0x4c, 0x36, 0x57, 0xf1, 0x13, 0xaf, 0xea, 0xa6, 0x27, 0x15, 0x1b,
		]
		.into();

		let expected_multilocation = MultiLocation {
			parents: 0,
			interior: X1(AccountId32 {
				network: Any,
				id: [
					126, 200, 62, 9, 114, 243, 243, 190, 185, 27, 243, 145, 244, 87, 26, 26, 213, 7, 6, 113, 36, 76,
					54, 87, 241, 19, 175, 234, 166, 39, 21, 27,
				],
			}),
		};
		assert_eq!(AccountIdToMultiLocation::convert(alice), expected_multilocation);
	}
}
