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
use crate::chain_spec::{
    build_local_properties, get_account_id_from_seed, get_authority_keys_from_seed, Extensions,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use primitives::{AccountId, Balance};
use runtime_staking::{
    constants::*, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig,
    GenesisConfig, GrandpaConfig, ImOnlineConfig, RootMembershipConfig, SessionConfig, SessionKeys,
    StakingConfig, SystemConfig,
};
use sc_service::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

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
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        ]
    });

    const ENDOWMENT: Balance = 10_000 * NODL;
    const STASH: Balance = ENDOWMENT / 1_000;

    GenesisConfig {
        // Core
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
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
        },
        // Consensus
        session: SessionConfig {
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
        },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(runtime_main::constants::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        staking: StakingConfig {
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), None, STASH))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            ..Default::default()
        },

        // Governance
        // Root Committee
        root_committee: Default::default(),
        root_membership: RootMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        },
    }
}

fn local_staking_genesis() -> GenesisConfig {
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

/// Local testnet config to test the staking pallet
pub fn local_staking_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Staking Local Testnet",
        "staking_local_testnet",
        ChainType::Local,
        local_staking_genesis,
        vec![],
        None,
        Some("nodl"),
        Some(build_local_properties()),
        Default::default(),
    )
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![get_authority_keys_from_seed("Alice")],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
        ],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Staking Development Testnet",
        "staking_dev_testnet",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        Some("nodl"),
        Some(build_local_properties()),
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;

    #[test]
    fn test_create_staking_local_chain_spec() {
        local_staking_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_staking_dev_chain_spec() {
        development_config().build_storage().unwrap();
    }
}
