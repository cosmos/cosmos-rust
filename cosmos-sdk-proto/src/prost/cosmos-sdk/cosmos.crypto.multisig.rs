/// LegacyAminoPubKey specifies a public key type
/// which nests multiple public keys and a threshold,
/// it uses legacy amino address rules.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAminoPubKey {
    #[prost(uint32, tag="1")]
    pub threshold: u32,
    #[prost(message, repeated, tag="2")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
