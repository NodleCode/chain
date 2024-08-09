/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2024  Nodle International
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

//! Genesis configs presets for the Rococo runtime

use crate::{
	constants::{EXISTENTIAL_DEPOSIT, NODL},
	AuraId, SessionKeys,
};
use cumulus_primitives_core::ParaId;
use primitives::{AccountId, Balance, Signature};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
#[cfg(not(feature = "std"))]
use sp_std::alloc::format;
use sp_std::vec;
use sp_std::vec::Vec;

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn testnet_accounts() -> Vec<AccountId> {
	Vec::from([
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
	])
}

pub fn eden_session_keys(keys: AuraId) -> SessionKeys {
	SessionKeys { aura: keys }
}

fn eden_testnet_genesis(
	root_key: Vec<AccountId>,
	collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Option<Vec<AccountId>>,
	id: ParaId,
) -> serde_json::Value {
	const ENDOWMENT: Balance = 10_000 * NODL;
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

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
			"safeXcmVersion": Some(xcm::latest::VERSION),
		},
	})
}

fn development_config_genesis(id: ParaId) -> serde_json::Value {
	eden_testnet_genesis(
		vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_from_seed::<AuraId>("Bob"),
			),
		],
		None,
		id,
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<sp_std::vec::Vec<u8>> {
	let patch = match id.try_into() {
		Ok("para_eden_dev") => development_config_genesis(2026.into()),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}
