use sp_core::{crypto::Ss58Codec, ed25519, Pair, Public, sr25519};
use nodle_chain_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	IndicesConfig, SystemConfig, WASM_BINARY, Signature, TechnicalMembershipConfig,
	OraclesSetConfig, SessionConfig, ValidatorsSetConfig, opaque::SessionKeys
};
use sp_consensus_aura::sr25519::{AuthorityId as AuraId};
use grandpa_primitives::{AuthorityId as GrandpaId};
use sc_service;
use sp_runtime::traits::{Verify, IdentifyAccount};

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,

	Nergal,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate a crypto pair from an address
pub fn get_from_addr<TPublic: Public>(addr: &str) -> TPublic {
	TPublic::from_string(addr).expect("static values are valid; qed")
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an account ID from an address
pub fn get_account_id_from_addr<TPublic: Public>(addr: &str) -> AccountId where
	AccountPublic: From<TPublic>
{
	AccountPublic::from(get_from_addr::<TPublic>(addr)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn get_authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

// Return a `SessionKeys` structure with the specified IDs
pub fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						session_keys(
							get_authority_keys_from_seed("Alice").0,
							get_authority_keys_from_seed("Alice").1
						)
					)
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie")
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::Nergal => ChainSpec::from_genesis(
				"Nergal",
				"nergal",
				|| testnet_genesis(vec![
					(
						get_account_id_from_addr::<sr25519::Public>("5F6sSf67EXUcqxsZcjx9A5hgTmXFpHA4F2XJa65Q8ZmWVPXd"),
						session_keys(
							get_from_addr::<AuraId>("5CFUAAW7umifCuzdvH5KKjWHM5dKP2ADnL54b9fbjECpgXEM"),
							get_from_addr::<GrandpaId>("5CLAMY86UHqR4DYFL1ss3mPQJPJVBHxRyNFKHwwitNjK2KRM")
						)
					)
				],
				vec![
					get_account_id_from_addr::<sr25519::Public>("5F6sSf67EXUcqxsZcjx9A5hgTmXFpHA4F2XJa65Q8ZmWVPXd"),
					get_account_id_from_addr::<sr25519::Public>("5HB624ynh6mL5TD4z9BfgpDKLsJcdV7HeGuFk79KThCqsDch"),
					get_account_id_from_addr::<sr25519::Public>("5CFuhu3AKYieoeRZtMBaYb2ad1LwDMuFzLFi6aQiXLFss4SR"),
				],
				vec![
					get_account_id_from_addr::<sr25519::Public>("5F6sSf67EXUcqxsZcjx9A5hgTmXFpHA4F2XJa65Q8ZmWVPXd"), // Root
					get_account_id_from_addr::<ed25519::Public>("5CLAMY86UHqR4DYFL1ss3mPQJPJVBHxRyNFKHwwitNjK2KRM"), // Validator stash
					get_account_id_from_addr::<sr25519::Public>("5CFUAAW7umifCuzdvH5KKjWHM5dKP2ADnL54b9fbjECpgXEM"), // Validtor hot wallet
					get_account_id_from_addr::<sr25519::Public>("5EaAaHrJaegsiSNgKJch1QjwCE8DFXRKxxg2tdVQhBHTWsC5"), // Oracle
					get_account_id_from_addr::<sr25519::Public>("5HB624ynh6mL5TD4z9BfgpDKLsJcdV7HeGuFk79KThCqsDch"),
					get_account_id_from_addr::<sr25519::Public>("5CFuhu3AKYieoeRZtMBaYb2ad1LwDMuFzLFi6aQiXLFss4SR"),
				],
				vec![
					get_account_id_from_addr::<sr25519::Public>("5EaAaHrJaegsiSNgKJch1QjwCE8DFXRKxxg2tdVQhBHTWsC5")
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "nergal" => Some(Alternative::Nergal),
			_ => None,
		}
	}
}

fn testnet_genesis(initial_authorities: Vec<(AccountId, SessionKeys)>,
	roots: Vec<AccountId>,
	endowed_accounts: Vec<AccountId>,
	oracles: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),

		// Governance
		collective_Instance2: Some(Default::default()),
		membership_Instance1: Some(TechnicalMembershipConfig {
			members: roots,
			phantom: Default::default(),
		}),

		// Allocations
		membership_Instance2: Some(OraclesSetConfig {
			members: oracles,
			phantom: Default::default(),
		}),

		// Validators permissioning
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		membership_Instance3: Some(ValidatorsSetConfig {
			members: initial_authorities.iter().map(|(id, _session)| id.clone()).collect::<Vec<_>>(),
			phantom: Default::default(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities,
		})
	}
}
