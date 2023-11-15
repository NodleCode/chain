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

//! Missing RPC utils for Substrate.
//! Ripped off https://github.com/paritytech/substrate/compare/master...cheme:example_rpc
//! and brought up to date with current version on substrate.

use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use sc_rpc_api::DenyUnsafe;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

use sp_core::{
	storage::{ChildInfo, ChildType, PrefixedStorageKey},
	Hasher,
};
use sp_state_machine::backend::AsTrieBackend;
use sp_trie::{
	trie_types::{TrieDB, TrieDBBuilder},
	KeySpacedDB, Trie,
};
use trie_db::{
	node::{NodePlan, ValuePlan},
	TrieDBNodeIterator,
};

fn count_migrate<'a, H: Hasher>(
	storage: &'a dyn trie_db::HashDBRef<H, Vec<u8>>,
	root: &'a H::Out,
	max_key_size: &mut usize,
) -> std::result::Result<(u64, TrieDB<'a, 'a, H>), String> {
	let mut nb = 0u64;
	let trie = TrieDBBuilder::new(storage, root).build();
	let iter_node = TrieDBNodeIterator::new(&trie).map_err(|e| format!("TrieDB node iterator error: {}", e))?;
	for node in iter_node {
		let node = node.map_err(|e| format!("TrieDB node iterator error: {}", e))?;
		match node.2.node_plan() {
			NodePlan::Leaf { value, partial, .. }
			| NodePlan::NibbledBranch {
				value: Some(value),
				partial,
				..
			} => {
				if let ValuePlan::Inline(range) = value {
					if (range.end - range.start) as u32 >= sp_core::storage::TRIE_VALUE_NODE_THRESHOLD {
						nb += 1;
					}
				}
				// false for branch that got an extra nibble but we will always get a bigger leaf
				// comming after
				*max_key_size = core::cmp::max((node.0.len() + partial.len()) / 2, *max_key_size);
			}

			_ => (),
		}
	}
	Ok((nb, trie))
}

/// Check trie migration status.
pub fn migration_status<H, B>(backend: &B) -> std::result::Result<(u64, u64, u64), String>
where
	H: Hasher,
	H::Out: codec::Codec,
	B: AsTrieBackend<H>,
{
	let mut max_key_size = 0;
	let trie_backend = backend.as_trie_backend();
	let essence = trie_backend.essence();
	let (nb_to_migrate, trie) = count_migrate(essence, essence.root(), &mut max_key_size)?;

	let mut nb_to_migrate_child = 0;
	let mut child_roots: Vec<(ChildInfo, Vec<u8>)> = Vec::new();
	// get all child trie roots
	for key_value in trie.iter().map_err(|e| format!("TrieDB node iterator error: {}", e))? {
		let (key, value) = key_value.map_err(|e| format!("TrieDB node iterator error: {}", e))?;
		if key[..].starts_with(sp_core::storage::well_known_keys::DEFAULT_CHILD_STORAGE_KEY_PREFIX) {
			let prefixed_key = PrefixedStorageKey::new(key);
			let (_type, unprefixed) = ChildType::from_prefixed_key(&prefixed_key).unwrap();
			child_roots.push((ChildInfo::new_default(unprefixed), value));
		}
	}
	for (child_info, root) in child_roots {
		let mut child_root = H::Out::default();
		let storage = KeySpacedDB::new(essence, child_info.keyspace());

		child_root.as_mut()[..].copy_from_slice(&root[..]);
		nb_to_migrate_child += count_migrate(&storage, &child_root, &mut max_key_size)?.0;
	}

	Ok((nb_to_migrate, nb_to_migrate_child, max_key_size as u64))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MigrationStatusResult {
	top_remaining_to_migrate: u64,
	child_remaining_to_migrate: u64,
	max_key_size: u64,
}

/// Migration RPC methods.
#[rpc(server)]
pub trait StateMigrationApi<BlockHash> {
	/// Check current migration state.
	///
	/// This call is performed locally without submitting any transactions. Thus executing this
	/// won't change any state. Nonetheless it is a VERY costy call that should be
	/// only exposed to trusted peers.
	#[method(name = "utils_trieMigrationStatus")]
	fn call(&self, at: Option<BlockHash>) -> RpcResult<MigrationStatusResult>;
}

/// An implementation of state migration specific RPC methods.
pub struct StateMigration<C, B, BA> {
	client: Arc<C>,
	backend: Arc<BA>,
	deny_unsafe: DenyUnsafe,
	_marker: std::marker::PhantomData<(B, BA)>,
}

impl<C, B, BA> StateMigration<C, B, BA> {
	/// Create new state migration rpc for the given reference to the client.
	pub fn new(client: Arc<C>, backend: Arc<BA>, deny_unsafe: DenyUnsafe) -> Self {
		StateMigration {
			client,
			backend,
			deny_unsafe,
			_marker: Default::default(),
		}
	}
}

impl<C, B, BA> StateMigrationApiServer<<B as BlockT>::Hash> for StateMigration<C, B, BA>
where
	B: BlockT,
	C: Send + Sync + 'static + sc_client_api::HeaderBackend<B>,
	BA: 'static + sc_client_api::backend::Backend<B>,
{
	fn call(&self, at: Option<<B as BlockT>::Hash>) -> RpcResult<MigrationStatusResult> {
		self.deny_unsafe.check_if_safe()?;

		let hash = at.unwrap_or_else(|| self.client.info().best_hash);
		let state = self.backend.state_at(hash).map_err(error_into_rpc_err)?;
		let (top, child, max_key_size) = migration_status(&state).map_err(error_into_rpc_err)?;

		Ok(MigrationStatusResult {
			top_remaining_to_migrate: top,
			child_remaining_to_migrate: child,
			max_key_size,
		})
	}
}

fn error_into_rpc_err(err: impl std::fmt::Display) -> JsonRpseeError {
	JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
		ErrorCode::InternalError.code(),
		"Error while checking migration state",
		Some(err.to_string()),
	)))
}
