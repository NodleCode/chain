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

use cumulus_primitives_core::ParaId;

use runtime_eden::{development_config_genesis, WASM_BINARY};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

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

pub fn paradis_config() -> Result<ChainSpec, Box<dyn std::error::Error>> {
	Ok(ChainSpec::from_json_file("/usr/local/share/nodle/paradis.json".into())?)
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
}
