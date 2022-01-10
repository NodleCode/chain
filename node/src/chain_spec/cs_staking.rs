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

fn tnet_templ_staking_config_genesis() -> GenesisConfig {
    use hex_literal::hex;
    use sp_core::crypto::UncheckedInto;

    let root_accounts = vec![
        // 4i66anCnZUrwEKr7rfbQmqmAwSZVRQejrmanMRmZLFt9meSW
        hex!["2a793621f3626a583f9d6e342acf7ea81c720cfd4b795f5cf0a649160ec2b549"].into(),
    ];

    let endowed_accounts = vec![
        // 4i66anCnZUrwEKr7rfbQmqmAwSZVRQejrmanMRmZLFt9meSW
        hex!["2a793621f3626a583f9d6e342acf7ea81c720cfd4b795f5cf0a649160ec2b549"].into(),
        //4nCGGvHoeEibsWwxixJqMAnau4GdvCQuJT5zJkKZf7F3oDUY
        hex!["e01e3e7b8b348410ff4e7bcc4cb4c8aff9c9e43610d5be7f4eff6cf123bc2619"].into(),
        //4hBKBK2TaVz34Fz2GVhU83k44k3N8UPw6FCLFryBggzxN2dU
        hex!["02379ad5430066b25e24c0babca76979ec958e69882212ed557ca76b56824e72"].into(),
        //4mGG3uKWxEWwU4TWjZX4Tcg583c9mfP688jy9fJvDGR9v5Gf
        hex!["b6ee3c9baced31630d49c6fe426b730b0b75a0d66bb916a02addb2f1a5bc7211"].into(),
    ];

	// Use script "scripts/vnv-testnet/generate-authority-keys.sh" to
	// generate the below keys.
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            //4nCGGvHoeEibsWwxixJqMAnau4GdvCQuJT5zJkKZf7F3oDUY
            hex!["e01e3e7b8b348410ff4e7bcc4cb4c8aff9c9e43610d5be7f4eff6cf123bc2619"].into(),
            //4myaHkPRg9kBViRkBEYEkkuVxgHFiYFKkt81VVfYLU4sboKx
            hex!["d670b2024436425f465ea9a7feb559f60a6138fd68f3c51bd4e53e598bd2650f"].into(),
            //4jSg7Qiov5yGJc2kvsfxC2jHBS4uGU2nRLUaM6BFcBMcPK3Y
            hex!["666737f82bf1c1c37b211955677e715f15d79323e6f6b67ecff91a1db7e9240c"]
                .unchecked_into(),
            //4moZgjHiPcdYuHh1dCFfh3Sp374ecgDfsG7fmbbT5B3UbuaA
            hex!["cece3574fb7c00ad701bdc599e645035ae3ed332933e4bbf1e8ca7c570881c12"]
                .unchecked_into(),
            //4jaCkfYraBaH9CpgManeA4NNm5RhuJFez7NK4tjY2RjZn4CT
            hex!["6c25167809d65094c600c11877dacf1cc3bfde5de19802c4f41ef124e6349c49"]
                .unchecked_into(),
            //4j33Ty1jZuviYkZamwHkYRn1Dfn8LYr3mjnTpXWrZRD4dbJF
            hex!["546136de9fa37703517617b61ad16ae6a4e953af3d58e745e20910ae6be0465b"]
                .unchecked_into(),
        ),
        (
            //4hBKBK2TaVz34Fz2GVhU83k44k3N8UPw6FCLFryBggzxN2dU
            hex!["02379ad5430066b25e24c0babca76979ec958e69882212ed557ca76b56824e72"].into(),
            //4iywYZSNbHpsS63g1GdEeky3LoQAMNsHpZEySqGht3radu59
            hex!["52038c0377624c0db2cd206b26b508b82e6d64efee4af6e9c5853bd0e8426447"].into(),
            //4iNioAAUszLufzQqH6UYvaEkx4xdQZDPm8RR384U5r3mytAU
            hex!["37270b0dcab087ca0e4671cd980012ceb34f36debaaa5ae9a29ef38140ec579d"]
                .unchecked_into(),
            //4kSXnDKXrJNXi4t4833ATmeMFHXq3NEweUV3mpu6fvty5eHb
            hex!["92876853409ea402a852304706b2e97f3bc34bea385629db49216c6f755e9c7b"]
                .unchecked_into(),
            //4k5VJCrv8ZkEsGBe1FZYsy31ea3YPbbdmVVERShDbXGJbMMn
            hex!["827ae55703e09a5e95230fb14729edbe282dcb8f8e2e0d8fdaae464b8c0e1338"]
                .unchecked_into(),
            //4jDyy7o7gUoqvoBWRt2QCSUeN3GZCUhazW77mAc3wdHMJEA4
            hex!["5cb923f928d5be56b36f4e56dc99c606003d43d0e3fd6912adcac8470b88ff5b"]
                .unchecked_into(),
        ),
        (
            //4mGG3uKWxEWwU4TWjZX4Tcg583c9mfP688jy9fJvDGR9v5Gf
            hex!["b6ee3c9baced31630d49c6fe426b730b0b75a0d66bb916a02addb2f1a5bc7211"].into(),
            //4jN4WRagSDQ17KothXQHQTamvHtEGY8LZBqpUn9uve2HCgjq
            hex!["62e262cfc6ef617c16bc4e9ddf8a50b3271cc6e40584ce2124cc1827f8004f7e"].into(),
            //4iVBDKWZtAbudNzB4qa5usnBnMLbcRakyMZc6aWbyeZCdE4E
            hex!["3c13711405f7c68d0c9240ccf09784493071d9f27cbe25c59da0fe4124198a53"]
                .unchecked_into(),
            //4iVgviTzXvxnLEB3Er5cbfArMdY3XnHFJayPijkvjzVeNm3g
            hex!["3c77778e0b636d40d6676a34f99f719da033aa87607dbd739dade550a286ef45"]
                .unchecked_into(),
            //4j17pFBMDj4hqqbTsCVGK6uHhMkj5xR8Bjn2c46JxeDgn4Lr
            hex!["52e95d9e40ba04454e28febfb176721c137a38bd8e52efbb33e4d8dd8613501a"]
                .unchecked_into(),
            //4j3SPAhbjHZnWPjidAhgCHv8cwnXMU3ucw7ZnTkQMpeqQgfV
            hex!["54ae5c59035068d79ab0630a952fe7dfc8b60d5e37715730b942f98ec5e0332e"]
                .unchecked_into(),
        ),
    ];

    testnet_genesis(
        initial_authorities,
        root_accounts,
        vec![],
        Some(endowed_accounts),
    )
}

pub fn tnet_templ_staking_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Staking Testnet V1",
        "staking_testnet_v1",
        ChainType::Development,
        tnet_templ_staking_config_genesis,
        vec![],
        None,
        Some("nodl"),
        Some(build_local_properties()),
        Default::default(),
    )
}

pub fn tnet_staking_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../../res/tnet-staking.json")[..]).unwrap()
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

    #[test]
    fn test_tnet_templ_staking_config() {
        tnet_templ_staking_config().build_storage().unwrap();
    }

    #[test]
    fn test_tnet_staking_config() {
        tnet_staking_config().build_storage().unwrap();
    }
}
