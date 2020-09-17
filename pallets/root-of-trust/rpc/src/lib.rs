use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use pallet_root_of_trust_runtime_api::RootOfTrustApi as RootOfTrustRuntimeApi;
use parity_scale_codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

mod rpc_errors;
use rpc_errors::misc_rpc_error;

#[rpc]
pub trait RootOfTrustApi<BlockHash, CertificateId>
where
    CertificateId: Codec,
{
    #[rpc(name = "rootOfTrust_isRootCertificateValid")]
    fn is_root_certificate_valid(&self, cert: CertificateId, at: Option<BlockHash>)
        -> Result<bool>;
    #[rpc(name = "rootOfTrust_isChildCertificateValid")]
    fn is_child_certificate_valid(
        &self,
        root: CertificateId,
        child: CertificateId,
        at: Option<BlockHash>,
    ) -> Result<bool>;
}

pub struct RootOfTrust<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> RootOfTrust<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, CertificateId> RootOfTrustApi<<Block as BlockT>::Hash, CertificateId>
    for RootOfTrust<C, Block>
where
    CertificateId: Codec,
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: RootOfTrustRuntimeApi<Block, CertificateId>,
{
    fn is_root_certificate_valid(
        &self,
        cert: CertificateId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        api.is_root_certificate_valid(&at, &cert)
            .map_err(misc_rpc_error)
    }

    fn is_child_certificate_valid(
        &self,
        root: CertificateId,
        child: CertificateId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        api.is_child_certificate_valid(&at, &root, &child)
            .map_err(misc_rpc_error)
    }
}
