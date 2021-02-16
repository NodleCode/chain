/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use cumulus_primitives_core::ParaId;
use nodle_chain_primitives::{AccountId, Balance, BlockNumber, Signature};
use nodle_chain_runtime::{
    constants::*, BalancesConfig, FinancialMembershipConfig, GenesisConfig, GrantsConfig,
    IndicesConfig, ParachainInfoConfig, RootMembershipConfig, SystemConfig,
    TechnicalMembershipConfig, WASM_BINARY,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

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

/// Development config (single validator Alice)
pub fn development_config(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
                vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
                None,
                None,
                id,
            )
        },
        vec![],
        None,
        None,
        None,
        Extensions {
            relay_chain: "nodle-dev".into(),
            para_id: id.into(),
        },
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
                None,
                None,
                id,
            )
        },
        vec![],
        None,
        None,
        None,
        Extensions {
            relay_chain: "nodle-local".into(),
            para_id: id.into(),
        },
    )
}

/// Dummy testnet config no balances, alice is a validator
pub fn dummy_testnet_config(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Dummy Network",
        "dummy_network",
        ChainType::Live,
        move || testnet_genesis(vec![], vec![], Some(vec![]), Some(vec![]), id),
        vec![],
        None,
        None,
        None,
        Extensions {
            relay_chain: "nodle-dummy".into(),
            para_id: id.into(),
        },
    )
}

/// Arcadia config, from json chainspec
pub fn arcadia_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../res/arcadia.json")[..]).unwrap()
}

// Main config, from json chainspec
pub fn main_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../res/main.json")[..]).unwrap()
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    roots: Vec<AccountId>,
    oracles: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    grants: Option<Vec<(AccountId, Vec<(BlockNumber, BlockNumber, u32, Balance)>)>>,
    id: ParaId,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        ]
    });

    let vested_grants = grants.unwrap_or_else(|| {
        vec![(
            // Ferdie has a network launch grant:
            // 1. after 1000 blocks a cliff of 1000 NODL unlocks
            // 2. for the next 100000 blocks a grant of 100 NODL unlocks every 1000 blocks
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            vec![
                (1_000, 1, 1, 1000 * NODL),      // Cliff
                (2_000, 1_000, 100, 100 * NODL), // Vesting
            ],
        )]
    });

    const ENDOWMENT: Balance = 1_000 * NODL;

    GenesisConfig {
        // Core
        frame_system: Some(SystemConfig {
            code: WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(oracles.iter().map(|x| (x.clone(), ENDOWMENT)))
                .chain(roots.iter().map(|x| (x.clone(), ENDOWMENT)))
                .fold(vec![], |mut acc, (account, endowment)| {
                    if acc
                        .iter()
                        .find(|(who, _endowment)| who == &account)
                        .is_some()
                    {
                        // Increase endowment
                        acc = acc
                            .iter()
                            .cloned()
                            .map(|(cur_account, cur_endowment)| (cur_account, cur_endowment))
                            .collect::<Vec<(AccountId, Balance)>>();
                    } else {
                        acc.push((account, endowment));
                    }

                    acc
                }),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_grants: Some(GrantsConfig {
            vesting: vested_grants,
        }),

        // Governance
        // Technical Committee
        pallet_collective_Instance2: Some(Default::default()),
        pallet_membership_Instance1: Some(TechnicalMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        }),
        // Financial Committee
        pallet_collective_Instance3: Some(Default::default()),
        pallet_membership_Instance3: Some(FinancialMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        }),
        pallet_reserve_Instance1: Some(Default::default()),
        pallet_reserve_Instance2: Some(Default::default()),
        pallet_reserve_Instance3: Some(Default::default()),
        // Root Committee
        pallet_collective_Instance4: Some(Default::default()),
        pallet_membership_Instance4: Some(RootMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        }),

        // Allocations
        pallet_membership_Instance5: Some(Default::default()),

        // Cumulus
        parachain_info: Some(ParachainInfoConfig { parachain_id: id }),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;

    #[test]
    fn test_create_development_chain_spec() {
        development_config(42.into()).build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config(42.into()).build_storage().unwrap();
    }

    #[test]
    fn test_create_dummy_testnet_chain_spec() {
        dummy_testnet_config(42.into()).build_storage().unwrap();
    }

    #[test]
    fn test_create_arcadia_chain_spec() {
        arcadia_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_main_chain_spec() {
        main_config().build_storage().unwrap();
    }
}
