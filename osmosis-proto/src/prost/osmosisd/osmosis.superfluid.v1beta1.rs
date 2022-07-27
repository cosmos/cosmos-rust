/// SetSuperfluidAssetsProposal is a gov Content type to update the superfluid
/// assets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSuperfluidAssetsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub assets: ::prost::alloc::vec::Vec<super::SuperfluidAsset>,
}
/// RemoveSuperfluidAssetsProposal is a gov Content type to remove the superfluid
/// assets by denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSuperfluidAssetsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub superfluid_asset_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
