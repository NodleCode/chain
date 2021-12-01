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

use crate::chain_spec::{build_local_properties, get_account_id_from_seed, get_from_seed};
use primitives::{AccountId, AuraId, Balance, BlockNumber, ParaId};
use runtime_eden::{
    constants::*, wasm_binary_unwrap, BalancesConfig, GenesisConfig, ParachainInfoConfig,
    SessionConfig, SessionKeys, SudoConfig, SystemConfig, ValidatorsSetConfig, VestingConfig,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::sr25519;

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn eden_session_keys(keys: AuraId) -> SessionKeys {
    SessionKeys { aura: keys }
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

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

/// Helper function to create GenesisConfig for testing
pub fn eden_genesis(
    root_key: AccountId,
    invulnerables: Vec<(AccountId, AuraId)>,
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

    const ENDOWMENT: Balance = 10_000 * NODL;

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
                .chain([(root_key.clone(), ENDOWMENT)].to_vec())
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
        sudo: SudoConfig { key: root_key },
        vesting: VestingConfig {
            vesting: vested_grants,
        },

        // Consensus
        validators_set: ValidatorsSetConfig {
            members: invulnerables
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            phantom: Default::default(),
        },
        session: SessionConfig {
            keys: invulnerables
                .clone()
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),             // account id
                        acc,                     // validator id
                        eden_session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        parachain_info: ParachainInfoConfig { parachain_id: id },

        // Governance
        company_reserve: Default::default(),
        international_reserve: Default::default(),
        usa_reserve: Default::default(),

        // Allocations
        allocations_oracles: Default::default(),
    }
}

fn development_config_genesis(id: ParaId) -> GenesisConfig {
    eden_genesis(
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
        ],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
        None,
        id.into(),
    )
}

/// Development config (single validator Alice)
pub fn development_config(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "ParaChain Eden Development",
        "para_eden_dev",
        ChainType::Local,
        move || development_config_genesis(id),
        vec![],
        None,
        Some("nodl"),
        Some(build_local_properties()),
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: id.into(),
        },
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;
    // default to the Statemint/Statemine/Westmint id
    const DEFAULT_PARA_ID: u32 = 1000;

    #[test]
    fn test_create_development_chain_spec() {
        development_config(ParaId::from(DEFAULT_PARA_ID))
            .build_storage()
            .unwrap();
    }
}
