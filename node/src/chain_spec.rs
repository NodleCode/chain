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

use std::{borrow::Cow, str::FromStr, vec};

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use primitives::{AccountId, Balance, Signature};
use runtime_eden::{
	constants::{EXISTENTIAL_DEPOSIT, NODL},
	AuraId, RuntimeGenesisConfig, SessionKeys, WASM_BINARY,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp_core::{crypto::Ss58Codec, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
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
	root_key: Vec<AccountId>,
	collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Option<Vec<AccountId>>,
	id: ParaId,
) -> Value {
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

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect::<Vec<_>>(),
		},
		"collatorSelection": {
			"invulnerables": collators.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
			"desiredCandidates": 0
		},
		"session": {
			"keys": collators
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),             // account id
						acc,                     // validator id
						eden_session_keys(aura), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"technicalMembership": {
			"members": root_key,
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
	})
}

fn development_config_genesis(id: ParaId) -> Value {
	eden_testnet_genesis(
		vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
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
	properties.insert("tokenSymbol".into(), "NotNODL".into());
	properties.insert("tokenDecimals".into(), 11.into());
	properties.insert("ss58Format".into(), 37.into());

	let collators = vec![(
		sp_core::sr25519::Public::from_ss58check("4jaftaRRK57EAGw2ooaLU7wQDQ4FpVaj2yTqxxkPpX49uRrW")?.into(),
		sp_core::sr25519::Public::from_ss58check("4jaftaRRK57EAGw2ooaLU7wQDQ4FpVaj2yTqxxkPpX49uRrW")?.into(),
	)];
	let patch_chain_spec = ChainSpec::builder(
		WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions {
			relay_chain: "paseo".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
	.with_genesis_config_patch(eden_testnet_genesis(
		vec![AccountId::new(hex![
			"6c806fe2f2f719ecd28cac83589e873800f6290ebee8b035abc2fcc91f8a6678"
		])],
		collators,
		None,
		id,
	))
	.build();

	let patched_spec = Value::from_str(&patch_chain_spec.clone().as_json(true)?)?;

	let exported_state: Value = serde_json::from_slice(&include_bytes!("../res/eden-export.json")[..])?;
	let mut working_cs_state = exported_state;

	working_cs_state["genesis"]["raw"]["top"]["0x15464cac3378d46f113cd5b7a4d71c845579297f4dfb9609e7e4c2ebab9ce40a"] =
		patched_spec["genesis"]["raw"]["top"]["0x15464cac3378d46f113cd5b7a4d71c845579297f4dfb9609e7e4c2ebab9ce40a"]
			.clone();

	working_cs_state["id"] = "hades".into();
	working_cs_state["bootNodes"] = Value::Array(vec![]);
	working_cs_state["telemetryEndpoints"] = Value::Array(vec![]);
	working_cs_state["relayChain"] = "paseo".into();



	let csbytes = Cow::from_iter(working_cs_state.to_string().bytes());
	let cs = ChainSpec::from_json_bytes(csbytes).unwrap();
	Ok(cs)
}

pub fn development_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "DevNODL".into());
	properties.insert("tokenDecimals".into(), 11.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
	.with_name("Parachain Eden Development")
	.with_id("para_eden_dev")
	.with_chain_type(ChainType::Development)
	.with_properties(properties)
	.with_genesis_config_patch(development_config_genesis(id))
	.build()
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
