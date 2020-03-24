use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use im_online::sr25519::AuthorityId as ImOnlineId;
use nodle_chain_runtime::constants::*;
use nodle_chain_runtime::opaque_primitives::{AccountId, Balance, Signature};
use nodle_chain_runtime::GenesisConfig;
use nodle_chain_runtime::{
    AllocationsConfig, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, GrandpaConfig,
    ImOnlineConfig, IndicesConfig, OraclesSetConfig, SessionConfig, SessionKeys, SystemConfig,
    TechnicalMembershipConfig, ValidatorsSetConfig, WASM_BINARY,
};
use sc_telemetry::TelemetryEndpoints;
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "nodl";

/// Build a `Properties` for a `ChainSpec` which will use the defined
/// `token_symbol`.
pub fn build_properties(token_symbol: &str) -> sc_service::Properties {
    let mut props = sc_service::Properties::new();
    props.insert("tokenDecimals".to_string(), json!(12));
    props.insert("tokenSymbol".to_string(), json!(token_symbol));

    props
}

pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option.
#[derive(Clone, Debug, PartialEq)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth and
    /// Ferdie as oracle.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths and
    /// Ferdie as oracle.
    LocalTestnet,

    Arcadia,
}

/// Get a chain config from a spec setting.
impl Alternative {
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Alternative::Development => development_config(),
            Alternative::LocalTestnet => local_testnet_config(),
            Alternative::Arcadia => arcadia_config(),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "local" => Some(Alternative::LocalTestnet),
            "arcadia" | _ => Some(Alternative::Arcadia),
        }
    }
}

pub fn load_spec(id: &str) -> Result<Option<ChainSpec>, String> {
    Ok(match Alternative::from(id) {
        Some(spec) => Some(spec.load()?),
        None => None,
    })
}

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    roots: Vec<AccountId>,
    oracles: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });

    const ENDOWMENT: Balance = 100 * NODL;

    GenesisConfig {
        // Core
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(oracles.iter().map(|x| (x.clone(), ENDOWMENT)))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), ENDOWMENT)))
                .collect(),
        }),
        indices: Some(IndicesConfig { indices: vec![] }),
        vesting: Some(Default::default()),

        // Consensus
        session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        babe: Some(BabeConfig {
            authorities: vec![],
        }),
        im_online: Some(ImOnlineConfig { keys: vec![] }),
        authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        membership_Instance3: Some(ValidatorsSetConfig {
            members: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            phantom: Default::default(),
        }),

        // Governance
        collective_Instance2: Some(Default::default()),
        membership_Instance1: Some(TechnicalMembershipConfig {
            members: roots,
            phantom: Default::default(),
        }),
        reserve: Some(Default::default()),

        // Nodle Core
        membership_Instance2: Some(OraclesSetConfig {
            members: oracles,
            phantom: Default::default(),
        }),
        allocations: Some(AllocationsConfig {
            coins_left: 10000000000000,
        }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![get_authority_keys_from_seed("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
        ],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn arcadia_genesis() -> GenesisConfig {
    let e = hex!["728462774923165b6d8a0f578432ec423745d0cd471af33eeda2d830b343467f"].into(); // 5EereDWgaMi7dPFFnBUq2nqJWMaRTWsNVUcac2x868PV3GCA
    let l = hex!["088b9603f874bf7a35155a4d6f55a580896a8635102b36ad0c0150a09f02242e"].into(); // 5CFuhu3AKYieoeRZtMBaYb2ad1LwDMuFzLFi6aQiXLFss4SR
    let g = hex!["e20b3e7084955505dc8cd0c51181850f41eaac7901c81bdee01a5361a8c22e32"].into(); // 5HB624ynh6mL5TD4z9BfgpDKLsJcdV7HeGuFk79KThCqsDch
    let m = hex!["5aa336fd5f8a6cd73b096bb43f6c60b7c6814307374aa53b6fbece28d7a0a535"].into(); // 5E7YekbgySR9cbCxFwocUxgzhJ2y6TgFVPh4FWgE5J29qjbN

    let initial_authorities = vec![(
        // 5CB5B5dW14sF3cNakCZtA5gGMdxKzaopgsBBrrU5qYT5xj3F
        hex!["04db3bbca0a736d460974b34f8f2281d8a627dcca42705da8a5fac12c0af3172"].into(), // AccountId
        hex!["04db3bbca0a736d460974b34f8f2281d8a627dcca42705da8a5fac12c0af3172"].into(), // AccountId / ValidatorId
        // 5GutzsbjWqB4PFvzS5gpLvXTerJKnQ5ubyHu3T7ujfbgcEAU
        hex!["d67576cff120e5d812e1046b8dced579be1cf2c4d36f440a5d6e4a9e6bc97d3e"].unchecked_into(), // GrandPa
        // 5G9FBgaEJzkpuaS86mm1wVENhzaUVuBpguimGkJorZwXmC87
        hex!["b4675fd3551f71fb3da347c820e54fd09fa04a7e554983d4d4623f5ce8c20d36"].unchecked_into(), // Babe
        hex!["b4675fd3551f71fb3da347c820e54fd09fa04a7e554983d4d4623f5ce8c20d36"].unchecked_into(), // ImOnline
        hex!["b4675fd3551f71fb3da347c820e54fd09fa04a7e554983d4d4623f5ce8c20d36"].unchecked_into(), // AuthorityDiscovery
    )];
    let roots = vec![e, l, g, m];
    let oracles = vec![];
    let other_endowed_accounts = None;

    testnet_genesis(initial_authorities, roots, oracles, other_endowed_accounts)
}

pub fn arcadia_config() -> ChainSpec {
    let boot_nodes = vec![
        "/ip4/35.200.78.9/tcp/30333/p2p/QmWZ3CfMuZ8U15SWoGSnRZ1cpzDsVHqmuvi4ThCB5zQVGg".to_string(),
        "/ip4/34.73.134.186/tcp/30333/p2p/QmZcst3MYHxJXoW5RgHnLrfz1ZZB9C7KgaJ6P69pABRJav"
            .to_string(),
    ];

    ChainSpec::from_genesis(
        "Arcadia Nodle Network",
        "arcadia",
        arcadia_genesis,
        boot_nodes,
        Some(TelemetryEndpoints::new(vec![(
            STAGING_TELEMETRY_URL.to_string(),
            0,
        )])),
        Some(DEFAULT_PROTOCOL_ID),
        Some(build_properties("aNODL")),
        Default::default(),
    )
}
