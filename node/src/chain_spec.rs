/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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

// clippy complains about ChainSpecGroup which we cannot modify
#![allow(clippy::derive_partial_eq_without_eq)]

use std::borrow::Cow;
use std::{fs::File, str::FromStr};
use std::io::Read;

use bzip2_rs::DecoderReader;
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use primitives::{AccountId, Balance, Signature};
use runtime_eden::{
	constants::{EXISTENTIAL_DEPOSIT, NODL},
	AuraId, BalancesConfig, CollatorSelectionConfig, ParachainInfoConfig, PolkadotXcmConfig, RuntimeGenesisConfig,
	SessionConfig, SessionKeys, SystemConfig, TechnicalMembershipConfig, WASM_BINARY,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp_core::crypto::Ss58Codec;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	bounded_vec,
	traits::{IdentifyAccount, Verify},
};
const SAFE_XCM_VERSION: u32 = xcm::latest::VERSION;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
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

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn eden_session_keys(keys: AuraId) -> SessionKeys {
	SessionKeys { aura: keys }
}

/// Helper function to create RuntimeGenesisConfig for testing
fn eden_testnet_genesis(
	root_key: AccountId,
	collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Option<Vec<AccountId>>,
	id: ParaId,
) -> RuntimeGenesisConfig {
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

	RuntimeGenesisConfig {
		// Core
		system: SystemConfig {
			code: WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		vesting: Default::default(),

		// Consensus
		collator_selection: CollatorSelectionConfig {
			invulnerables: collators.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: SessionConfig {
			keys: collators
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
		parachain_info: ParachainInfoConfig {
			parachain_id: id,
			_config: Default::default(),
		},
		transaction_payment: Default::default(),

		// Governance
		company_reserve: Default::default(),
		international_reserve: Default::default(),
		usa_reserve: Default::default(),
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: bounded_vec![root_key],
			phantom: Default::default(),
		},

		// Allocations
		allocations_oracles: Default::default(),

		// DAO
		dao_reserve: Default::default(),

		polkadot_xcm: PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			_config: Default::default(),
		},
	}
}

fn development_config_genesis(id: ParaId) -> RuntimeGenesisConfig {
	eden_testnet_genesis(
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
		None,
		id,
	)
}

pub fn paradis_config(id: ParaId) -> Result<ChainSpec, Box<dyn std::error::Error>> {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "notNodl".into());
	properties.insert("tokenDecimals".into(), 11.into());
	properties.insert("ss58Format".into(), 37.into());

	let patch_chain_spec = ChainSpec::from_genesis(
		// Name
		"Parachain Hades testnet",
		// ID
		"hades",
		ChainType::Live,
		move || eden_testnet_genesis(			
			sp_core::sr25519::Public::from_ss58check("4jmcpevRTaN4sd4KGwAQ48w1UkC1Ky38HKCBdNpEC8jbv2N9").expect("Internal Error").into(),
			vec![
				(
					sp_core::sr25519::Public::from_ss58check("4jaftaRRK57EAGw2ooaLU7wQDQ4FpVaj2yTqxxkPpX49uRrW").expect("Internal Error").into(),
					sp_core::sr25519::Public::from_ss58check("4jaftaRRK57EAGw2ooaLU7wQDQ4FpVaj2yTqxxkPpX49uRrW").expect("Internal Error").into(),
				),
				(
					sp_core::sr25519::Public::from_ss58check("4j5pvMg65HBK4KvUcgSGMPEqrRjXkWJmjuqhb7AktmrB9auf").expect("Internal Error").into(),
					sp_core::sr25519::Public::from_ss58check("4j5pvMg65HBK4KvUcgSGMPEqrRjXkWJmjuqhb7AktmrB9auf").expect("Internal Error").into(),
				),
				(
					sp_core::sr25519::Public::from_ss58check("4mXXuWGQNgUW8QzWH8bg2Ao654d99CSYJXfELCCyd7qx2ACs").expect("Internal Error").into(),
					sp_core::sr25519::Public::from_ss58check("4mXXuWGQNgUW8QzWH8bg2Ao654d99CSYJXfELCCyd7qx2ACs").expect("Internal Error").into(),
				)
			],
			None,
			id,
		),
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			// TODO change to paseo
			relay_chain: "paseo".into(), // You MUST set this to the correct network! 
			para_id: id.into(),
		},
	);

	let mut target_spec = Value::from_str(&patch_chain_spec.clone().as_json(true)?)?;
	// black list all data with values in current spec
	let mut blacklist = Value::from_str(&patch_chain_spec.clone().as_json(true)?)?;

	// black list block number of relay chain we are a parachain under.
	blacklist["genesis"]["raw"]["top"]["0x45323df7cc47150b3930e2666b0aa313a2bca190d36bd834cc73a38fc213ecbd"] = true.into();

	let compressed_file = File::open("/home/simson/nodle/chain/node/res/eden-export.json.bz2")?;

	let mut reader = DecoderReader::new(compressed_file);

	let mut exported_bytes = Vec::<u8>::with_capacity(500_000_000);
	let _ = reader.read_to_end(&mut exported_bytes)?;

	let exported_state: Value = serde_json::from_slice(&exported_bytes[..])?;

	if let Some(top) = exported_state["genesis"]["raw"]["top"].as_object() {
		for key in top.keys() {
			if blacklist["genesis"]["raw"]["top"][key].is_null() {
				target_spec["genesis"]["raw"]["top"][key] = top[key].clone();
			}
		}
	}

	// working_cs_state["id"] = "hades".into();
	// working_cs_state["bootNodes"] = Value::Array(vec![]);
	// working_cs_state["telemetryEndpoints"] = Value::Array(vec![]);
	// working_cs_state["relayChain"] = "paseo".into();

	let cs_string = target_spec.to_string();
	
	// Maybe this debug is nneded soon again
	// println!("{cs_string}");

	let cs_bytes = Cow::from_iter(cs_string.bytes());
	let cs = ChainSpec::from_json_bytes(cs_bytes)?;
	Ok(cs)
}

pub fn development_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "NODL".into());
	properties.insert("tokenDecimals".into(), 11.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Parachain Eden Development",
		// ID
		"para_eden_dev",
		ChainType::Development,
		move || development_config_genesis(id),
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn local_config_genesis(id: ParaId) -> RuntimeGenesisConfig {
	eden_testnet_genesis(
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
			(
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_collator_keys_from_seed("Charlie"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_collator_keys_from_seed("Dave"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_collator_keys_from_seed("Eve"),
			),
		],
		None,
		id,
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "NODL".into());
	properties.insert("tokenDecimals".into(), 11.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Eden Local Testnet",
		// ID
		"para_eden_local",
		ChainType::Local,
		move || local_config_genesis(id),
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("eden-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "westend".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn production_config() -> ChainSpec {
	ChainSpec::from_json_bytes(&include_bytes!("../res/eden.json")[..]).unwrap()
}

pub fn testing_config() -> ChainSpec {
	ChainSpec::from_json_bytes(&include_bytes!("../res/eden-testing.json")[..]).unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use hex_literal::hex;
	use sc_chain_spec::ChainSpec;
	use sp_core::blake2_256;
	use sp_runtime::BuildStorage;

	#[test]
	fn create_development_chain_spec() {
		assert!(development_config(ParaId::from(1000u32)).build_storage().is_ok());
	}

	#[test]
	fn create_local_chain_spec() {
		assert!(local_testnet_config(ParaId::from(1000u32)).build_storage().is_ok());
	}

	#[test]
	fn create_production_spec() {
		assert!(production_config().build_storage().is_ok());
	}

	#[test]
	fn production_has_substitutes_set() {
		// see https://github.com/NodleCode/chain/releases/tag/2.2.2-hotfix
		assert!(production_config().code_substitutes().contains_key("3351852"));
		assert_eq!(
			blake2_256(
				production_config()
					.code_substitutes()
					.get("3351852")
					.expect("we already tested for existence"),
			),
			hex!("207767fb73e1fcf8ae32455843419e51c94987228a4b77857aff7653d103cac3"),
		);
	}

	#[test]
	fn create_testing_spec() {
		assert!(testing_config().build_storage().is_ok());
	}
}
