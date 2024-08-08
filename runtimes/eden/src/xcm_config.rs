use super::{
	AccountId, AllPalletsWithSystem, Balance, Balances, MessageQueue, ParachainInfo, ParachainSystem, PolkadotXcm,
	Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmpQueue,
};
use crate::{implementations::ToAuthor, pallets_system::TransactionByteFee};
use cumulus_primitives_core::{
	AggregateMessageOrigin, AssetId,
	Junction::{PalletInstance, Parachain},
	Junctions::{self, Here, X1},
	Location, NetworkId, ParaId, Weight as XcmWeight,
};
use frame_support::{
	match_types, parameter_types,
	traits::{ConstU32, Everything, Nothing, PalletInfoAccess, TransformOrigin},
	weights::{IdentityFee, Weight},
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use parachains_common::message_queue::ParaIdToSibling;
use polkadot_parachain_primitives::primitives::Sibling;
use sp_std::sync::Arc;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
	EnsureXcmOrigin, FrameTransactionalProcessor, FungibleAdapter, IsConcrete, NativeAsset, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents, WeightInfoBounds,
	WithComputedOrigin,
};
use xcm_executor::XcmExecutor;
#[cfg(feature = "runtime-benchmarks")]
use {
	crate::constants::NODL,
	cumulus_primitives_core::{Asset, Assets, Fungible, Junction, Response},
	frame_benchmarking::BenchmarkError,
	sp_std::vec,
};

/// Type for specifying how a `Location` can be converted into an `AccountId`. This is used
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

match_types! {
	pub type ParentOrSiblings: impl Contains<Location> = {
		Location { parents: 1, interior: Here } |
		Location { parents: 1, interior: X1(_) }
	};
}

pub type Barrier = (
	TakeWeightCredit,
	// Expected responses are OK.
	AllowKnownQueryResponses<PolkadotXcm>,
	// Allow XCMs with some computed origins to pass through.
	WithComputedOrigin<
		(
			// If the message is one that immediately attemps to pay for execution, then allow it.
			AllowTopLevelPaidExecutionFrom<Everything>,
			// Subscriptions for version tracking are OK.
			AllowSubscriptionsFrom<ParentOrSiblings>,
		),
		UniversalLocation,
		ConstU32<8>,
	>,
);

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

parameter_types! {
	pub RelayLocation: Location = Location::parent();
	pub NodlLocation: Location = Location {
		parents:0,
		interior: Junctions::X1(Arc::new([PalletInstance(<Balances as PalletInfoAccess>::index() as u8)]))
	};
	pub const RelayNetwork: Option<NetworkId> = None;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: Junctions = Parachain(ParachainInfo::parachain_id().into()).into();
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

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = FungibleAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<NodlLocation>,
	// Do a simple punn to convert an AccountId32 Location into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

parameter_types! {
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = CurrencyTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<crate::weights::NodleXcmWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
	type Trader = UsingComponents<IdentityFee<Balance>, NodlLocation, AccountId, Balances, ToAuthor>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = ConstU32<8>;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = PolkadotXcm;
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
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = crate::weights::pallet_xcm::WeightInfo<Runtime>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

parameter_types! {
	/// The asset ID for the asset that we use to pay for message delivery fees.
	pub FeeAssetId: AssetId = AssetId(NodlLocation::get());
	/// The base fee for the message delivery fees.
	pub const BaseDeliveryFee: u128 = crate::constants::POLKADOT_CENT.saturating_mul(3);
}

pub type PriceForSiblingParachainDelivery =
	polkadot_runtime_common::xcm_sender::ExponentialPrice<FeeAssetId, BaseDeliveryFee, TransactionByteFee, XcmpQueue>;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type MaxActiveOutboundChannels = ConstU32<128>;
	type MaxPageSize = ConstU32<{ 1 << 16 }>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
	type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
	type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
	type MaxInboundSuspended = sp_core::ConstU32<1_000>;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
parameter_types! {
	pub const BaseXcmWeight: XcmWeight = XcmWeight::from_parts(100_000_000, 0);
	// TODO: update based on the results of CHA-407
	pub const MaxAssetsForTransfer: usize = 2;
}
parameter_types! {
	pub SelfLocation: Location = Location::here();
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub const TrustedTeleporter: Option<(Location, Asset)> = Some((
		Location::parent(),
		Asset{ id: AssetId(Location::parent()), fun: Fungible(100) }
	));
	pub const TrustedReserve: Option<(Location, Asset)> = None;

}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::generic::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type TransactAsset = Balances;

	//TODO put a realistic value here:
	fn fee_asset() -> Result<Asset, BenchmarkError> {
		let assets: Asset = (AssetId(NodlLocation::get()), 10_000_000 * NODL).into();
		Ok(assets)
	}

	fn worst_case_response() -> (u64, Response) {
		(0u64, Response::Version(Default::default()))
	}

	fn worst_case_asset_exchange() -> Result<(Assets, Assets), BenchmarkError> {
		// Eden doesn't support asset exchanges
		Err(BenchmarkError::Skip)
	}

	fn universal_alias() -> Result<(Location, Junction), BenchmarkError> {
		// The XCM executor of Eden doesn't have a configured `UniversalAliases`
		Err(BenchmarkError::Skip)
	}

	fn export_message_origin_and_destination() -> Result<(Location, NetworkId, Junctions), BenchmarkError> {
		// The XCM executor of Eden doesn't support exporting messages
		Err(BenchmarkError::Skip)
	}

	fn transact_origin_and_runtime_call() -> Result<(Location, RuntimeCall), BenchmarkError> {
		Ok((
			Location::parent(),
			frame_system::Call::remark_with_event { remark: vec![] }.into(),
		))
	}

	fn subscribe_origin() -> Result<Location, BenchmarkError> {
		Ok(Location::parent())
	}

	fn claimable_asset() -> Result<(Location, Location, Assets), BenchmarkError> {
		let origin = Location::parent();
		let assets: Assets = (AssetId(NodlLocation::get()), 10_000_000 * NODL).into();
		let ticket = Location {
			parents: 0,
			interior: Here,
		};
		Ok((origin, ticket, assets))
	}

	fn unlockable_asset() -> Result<(Location, Location, Asset), BenchmarkError> {
		// Eden doesn't support locking/unlocking assets
		Err(BenchmarkError::Skip)
	}

	fn alias_origin() -> Result<(Location, Location), BenchmarkError> {
		Err(BenchmarkError::Skip)
	}
}

#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::fungible::Config for Runtime {
	type TransactAsset = Balances;
	type CheckedAccount = ();
	type TrustedTeleporter = TrustedTeleporter;
	type TrustedReserve = TrustedReserve;
	fn get_asset() -> Asset {
		Asset {
			id: AssetId(NodlLocation::get()),
			fun: Fungible(crate::constants::NODL),
		}
	}
}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_xcm_benchmarks::Config for Runtime {
	type XcmConfig = XcmConfig;
	type AccountIdConverter = LocationToAccountId;
	type DeliveryHelper = ();

	fn valid_destination() -> Result<Location, BenchmarkError> {
		Ok(RelayLocation::get())
	}
	fn worst_case_holding(_depositable_count: u32) -> Assets {
		// 1 fungibles can be traded in the worst case: TODO: CHA-407 https://github.com/NodleCode/chain/issues/717
		let assets = Asset {
			id: AssetId(NodlLocation::get()),
			fun: Fungible(10_000_000 * NODL),
		};
		assets.into()
	}
}
