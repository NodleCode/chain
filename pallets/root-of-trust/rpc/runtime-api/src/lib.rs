#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    pub trait RootOfTrustApi<CertificateId> where
        CertificateId: parity_scale_codec::Codec
    {
        fn is_root_certificate_valid(cert: &CertificateId) -> bool;
        fn is_child_certificate_valid(root: &CertificateId, child: &CertificateId) -> bool;
    }
}
