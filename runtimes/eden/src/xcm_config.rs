use super::{AccountId, Balance, Call, Event, Origin, ParachainSystem, PolkadotXcm, Runtime, ParachainInfo, XcmpQueue, Balances};
use frame_support::{
	parameter_types,
	traits::{Everything, Nothing},
	weights::{IdentityFee},
};
use frame_system::EnsureRoot;
use polkadot_parachain::primitives::Sibling;
use pallet_xcm::XcmPassthrough;
use xcm::{latest::Weight as XcmWeight,latest::NetworkId, prelude::*};
use xcm_builder::{
	AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin,
	FixedWeightBounds, LocationInverter, NativeAsset, RelayChainAsNative, SiblingParachainAsNative,
	SignedAccountId32AsNative, SignedToAccountId32, TakeWeightCredit, UsingComponents, SovereignSignedViaLocation, AccountId32Aliases, SiblingParachainConvertsVia, ParentIsPreset,
};
use xcm_executor::{Config, XcmExecutor};

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
	// If the message is one that immediately attemps to pay for execution, then allow it.
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Expected responses are OK.
	AllowKnownQueryResponses<PolkadotXcm>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);

pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);
parameter_types! {
	pub DotLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}
/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayChainOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

parameter_types! {
	pub const MaxInstructions: u32 = 100;
	pub UnitWeightCost: XcmWeight = 1_000_000; // arbitrary value ToDo: replace with more specific value
}

pub struct XcmConfig;
impl Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = (); // ToDo: TBD orm token 
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = NativeAsset; // <- should be enough to allow teleportation of DOT
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type Trader = UsingComponents<IdentityFee<Balance>, DotLocation, AccountId, Balances, ()>;
	type ResponseHandler = (); // Don't handle responses for now.
	type AssetTrap= PolkadotXcm; //TBD
	type AssetClaims= PolkadotXcm; //TBD
	type SubscriptionService= PolkadotXcm; //TBD
}

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type Origin = Origin;
	type Call = Call;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}


impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>; // TBD if we need the council/root to send overweight messages
	type ControllerOrigin =  EnsureRoot<AccountId>; // TBD define who is allowed to suspend/resume the queue
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin; 
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}