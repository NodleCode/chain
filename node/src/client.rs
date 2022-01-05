//! MainRT Client abstractions

#![allow(clippy::upper_case_acronyms)]

pub use crate::service::{FullBackend, FullClient, MainExecutorDispatch, StakingExecutorDispatch};
use primitives::{AccountId, Balance, Block, BlockNumber, Hash, Header, Index};
use sc_client_api::{Backend as BackendT, BlockchainEvents, KeyIterator};
use sp_api::{CallApiAt, NumberFor, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_consensus::BlockStatus;
use sp_runtime::{
    generic::{BlockId, SignedBlock},
    traits::{BlakeTwo256, Block as BlockT},
    Justifications,
};
use sp_storage::{ChildInfo, StorageData, StorageKey};
use std::sync::Arc;

/// A set of APIs that MainRT-like runtimes must implement.
pub trait RuntimeApiCollection:
    sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
    + sp_api::ApiExt<Block>
    + sp_consensus_babe::BabeApi<Block>
    + pallet_grandpa::fg_primitives::GrandpaApi<Block>
    + sp_block_builder::BlockBuilder<Block>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>
    + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
    + sp_api::Metadata<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_authority_discovery::AuthorityDiscoveryApi<Block>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
    Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::ApiExt<Block>
        + sp_consensus_babe::BabeApi<Block>
        + pallet_grandpa::fg_primitives::GrandpaApi<Block>
        + sp_block_builder::BlockBuilder<Block>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
        + sp_api::Metadata<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + sp_authority_discovery::AuthorityDiscoveryApi<Block>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// Trait that abstracts over all available client implementations.
///
/// For a concrete type there exists [`Client`].
pub trait AbstractClient<Block, Backend>:
    BlockchainEvents<Block>
    + Sized
    + Send
    + Sync
    + ProvideRuntimeApi<Block>
    + HeaderBackend<Block>
    + CallApiAt<Block, StateBackend = Backend::State>
where
    Block: BlockT,
    Backend: BackendT<Block>,
    Backend::State: sp_api::StateBackend<BlakeTwo256>,
    Self::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}

impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
where
    Block: BlockT,
    Backend: BackendT<Block>,
    Backend::State: sp_api::StateBackend<BlakeTwo256>,
    Client: BlockchainEvents<Block>
        + ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + Sized
        + Send
        + Sync
        + CallApiAt<Block, StateBackend = Backend::State>,
    Client::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}

/// Execute something with the client instance.
///
/// As there exist multiple chains inside MainRT, like MainRT itself and testing runtime,
/// there can exist different kinds of client types. As these client types differ in the generics
/// that are being used, we can not easily return them from a function. For returning them from a
/// function there exists [`Client`]. However, the problem on how to use this client instance still
/// exists. This trait "solves" it in a dirty way. It requires a type to implement this trait and
/// than the [`execute_with_client`](ExecuteWithClient::execute_with_client) function can be called
/// with any possible client instance.
///
/// In a perfect world, we could make a closure work in this way.
pub trait ExecuteWithClient {
    /// The return type when calling this instance.
    type Output;

    /// Execute whatever should be executed with the given client instance.
    fn execute_with_client<Client, Api, Backend>(self, client: Arc<Client>) -> Self::Output
    where
        <Api as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
        Backend: sc_client_api::Backend<Block> + 'static,
        Backend::State: sp_api::StateBackend<BlakeTwo256>,
        Api: RuntimeApiCollection<StateBackend = Backend::State>,
        Client: AbstractClient<Block, Backend, Api = Api> + 'static;
}

/// A handle to a MainRT client instance.
///
/// The MainRT service supports multiple different runtimes (MainRT itself or testing runtime). As each runtime has a
/// specialized client, we need to hide them behind a trait. This is this trait.
///
/// When wanting to work with the inner client, you need to use `execute_with`.
///
/// See [`ExecuteWithClient`](trait.ExecuteWithClient.html) for more information.
pub trait ClientHandle {
    /// Execute the given something with the client.
    fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output;
}

/// A client instance of MainRT.
///
/// See [`ExecuteWithClient`] for more information.
#[derive(Clone)]
pub enum Client {
    MainRT(Arc<FullClient<runtime_main::RuntimeApi, MainExecutorDispatch>>),
    StakingRT(Arc<FullClient<runtime_staking::RuntimeApi, StakingExecutorDispatch>>),
}

impl ClientHandle for Client {
    fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output {
        match self {
            Self::MainRT(client) => T::execute_with_client::<_, _, FullBackend>(t, client.clone()),
            Self::StakingRT(client) => {
                T::execute_with_client::<_, _, FullBackend>(t, client.clone())
            }
        }
    }
}

impl sc_client_api::UsageProvider<Block> for Client {
    fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {
        match self {
            Self::MainRT(client) => client.usage_info(),
            Self::StakingRT(client) => client.usage_info(),
        }
    }
}

impl sc_client_api::BlockBackend<Block> for Client {
    fn block_body(
        &self,
        id: &BlockId<Block>,
    ) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
        match self {
            Self::MainRT(client) => client.block_body(id),
            Self::StakingRT(client) => client.block_body(id),
        }
    }

    fn block(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
        match self {
            Self::MainRT(client) => client.block(id),
            Self::StakingRT(client) => client.block(id),
        }
    }

    fn block_status(&self, id: &BlockId<Block>) -> sp_blockchain::Result<BlockStatus> {
        match self {
            Self::MainRT(client) => client.block_status(id),
            Self::StakingRT(client) => client.block_status(id),
        }
    }

    fn justifications(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<Justifications>> {
        match self {
            Self::MainRT(client) => client.justifications(id),
            Self::StakingRT(client) => client.justifications(id),
        }
    }

    fn block_hash(
        &self,
        number: NumberFor<Block>,
    ) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
        match self {
            Self::MainRT(client) => client.block_hash(number),
            Self::StakingRT(client) => client.block_hash(number),
        }
    }

    fn indexed_transaction(
        &self,
        id: &<Block as BlockT>::Hash,
    ) -> sp_blockchain::Result<Option<Vec<u8>>> {
        match self {
            Self::MainRT(client) => client.indexed_transaction(id),
            Self::StakingRT(client) => client.indexed_transaction(id),
        }
    }

    fn block_indexed_body(
        &self,
        id: &BlockId<Block>,
    ) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {
        match self {
            Self::MainRT(client) => client.block_indexed_body(id),
            Self::StakingRT(client) => client.block_indexed_body(id),
        }
    }
}

impl sc_client_api::StorageProvider<Block, FullBackend> for Client {
    fn storage(
        &self,
        id: &BlockId<Block>,
        key: &StorageKey,
    ) -> sp_blockchain::Result<Option<StorageData>> {
        match self {
            Self::MainRT(client) => client.storage(id, key),
            Self::StakingRT(client) => client.storage(id, key),
        }
    }

    fn storage_keys(
        &self,
        id: &BlockId<Block>,
        key_prefix: &StorageKey,
    ) -> sp_blockchain::Result<Vec<StorageKey>> {
        match self {
            Self::MainRT(client) => client.storage_keys(id, key_prefix),
            Self::StakingRT(client) => client.storage_keys(id, key_prefix),
        }
    }

    fn storage_hash(
        &self,
        id: &BlockId<Block>,
        key: &StorageKey,
    ) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
        match self {
            Self::MainRT(client) => client.storage_hash(id, key),
            Self::StakingRT(client) => client.storage_hash(id, key),
        }
    }

    fn storage_pairs(
        &self,
        id: &BlockId<Block>,
        key_prefix: &StorageKey,
    ) -> sp_blockchain::Result<Vec<(StorageKey, StorageData)>> {
        match self {
            Self::MainRT(client) => client.storage_pairs(id, key_prefix),
            Self::StakingRT(client) => client.storage_pairs(id, key_prefix),
        }
    }

    fn storage_keys_iter<'a>(
        &self,
        id: &BlockId<Block>,
        prefix: Option<&'a StorageKey>,
        start_key: Option<&StorageKey>,
    ) -> sp_blockchain::Result<
        KeyIterator<'a, <FullBackend as sc_client_api::Backend<Block>>::State, Block>,
    > {
        match self {
            Self::MainRT(client) => client.storage_keys_iter(id, prefix, start_key),
            Self::StakingRT(client) => client.storage_keys_iter(id, prefix, start_key),
        }
    }

    fn child_storage_keys_iter<'a>(
        &self,
        id: &BlockId<Block>,
        child_info: ChildInfo,
        prefix: Option<&'a StorageKey>,
        start_key: Option<&StorageKey>,
    ) -> sp_blockchain::Result<
        KeyIterator<'a, <FullBackend as sc_client_api::Backend<Block>>::State, Block>,
    > {
        match self {
            Self::MainRT(client) => {
                client.child_storage_keys_iter(id, child_info, prefix, start_key)
            }
            Self::StakingRT(client) => {
                client.child_storage_keys_iter(id, child_info, prefix, start_key)
            }
        }
    }

    fn child_storage(
        &self,
        id: &BlockId<Block>,
        child_info: &ChildInfo,
        key: &StorageKey,
    ) -> sp_blockchain::Result<Option<StorageData>> {
        match self {
            Self::MainRT(client) => client.child_storage(id, child_info, key),
            Self::StakingRT(client) => client.child_storage(id, child_info, key),
        }
    }

    fn child_storage_keys(
        &self,
        id: &BlockId<Block>,
        child_info: &ChildInfo,
        key_prefix: &StorageKey,
    ) -> sp_blockchain::Result<Vec<StorageKey>> {
        match self {
            Self::MainRT(client) => client.child_storage_keys(id, child_info, key_prefix),
            Self::StakingRT(client) => client.child_storage_keys(id, child_info, key_prefix),
        }
    }

    fn child_storage_hash(
        &self,
        id: &BlockId<Block>,
        child_info: &ChildInfo,
        key: &StorageKey,
    ) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
        match self {
            Self::MainRT(client) => client.child_storage_hash(id, child_info, key),
            Self::StakingRT(client) => client.child_storage_hash(id, child_info, key),
        }
    }
}

impl sp_blockchain::HeaderBackend<Block> for Client {
    fn header(&self, id: BlockId<Block>) -> sp_blockchain::Result<Option<Header>> {
        match self {
            Self::MainRT(client) => client.header(&id),
            Self::StakingRT(client) => client.header(&id),
        }
    }

    fn info(&self) -> sp_blockchain::Info<Block> {
        match self {
            Self::MainRT(client) => client.info(),
            Self::StakingRT(client) => client.info(),
        }
    }

    fn status(&self, id: BlockId<Block>) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
        match self {
            Self::MainRT(client) => client.status(id),
            Self::StakingRT(client) => client.status(id),
        }
    }

    fn number(&self, hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
        match self {
            Self::MainRT(client) => client.number(hash),
            Self::StakingRT(client) => client.number(hash),
        }
    }

    fn hash(&self, number: BlockNumber) -> sp_blockchain::Result<Option<Hash>> {
        match self {
            Self::MainRT(client) => client.hash(number),
            Self::StakingRT(client) => client.hash(number),
        }
    }
}
