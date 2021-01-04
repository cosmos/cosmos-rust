/// GetValidatorSetByHeightRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::std::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightResponse {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::std::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::std::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// GetLatestValidatorSetRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::std::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetLatestValidatorSetResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetResponse {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::std::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::std::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// Validator is the type for the validator-set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::std::option::Option<::prost_types::Any>,
    #[prost(int64, tag = "3")]
    pub voting_power: i64,
    #[prost(int64, tag = "4")]
    pub proposer_priority: i64,
}
/// GetBlockByHeightRequest is the request type for the Query/GetBlockByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// GetBlockByHeightResponse is the response type for the Query/GetBlockByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::std::option::Option<::tendermint_proto::types::BlockId>,
    #[prost(message, optional, tag = "2")]
    pub block: ::std::option::Option<::tendermint_proto::types::Block>,
}
/// GetLatestBlockRequest is the request type for the Query/GetLatestBlock RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockRequest {}
/// GetLatestBlockResponse is the response type for the Query/GetLatestBlock RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::std::option::Option<::tendermint_proto::types::BlockId>,
    #[prost(message, optional, tag = "2")]
    pub block: ::std::option::Option<::tendermint_proto::types::Block>,
}
/// GetSyncingRequest is the request type for the Query/GetSyncing RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingRequest {}
/// GetSyncingResponse is the response type for the Query/GetSyncing RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingResponse {
    #[prost(bool, tag = "1")]
    pub syncing: bool,
}
/// GetNodeInfoRequest is the request type for the Query/GetNodeInfo RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoRequest {}
/// GetNodeInfoResponse is the request type for the Query/GetNodeInfo RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub default_node_info: ::std::option::Option<::tendermint_proto::p2p::DefaultNodeInfo>,
    #[prost(message, optional, tag = "2")]
    pub application_version: ::std::option::Option<VersionInfo>,
}
/// VersionInfo is the type for the GetNodeInfoResponse message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionInfo {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub app_name: std::string::String,
    #[prost(string, tag = "3")]
    pub version: std::string::String,
    #[prost(string, tag = "4")]
    pub git_commit: std::string::String,
    #[prost(string, tag = "5")]
    pub build_tags: std::string::String,
    #[prost(string, tag = "6")]
    pub go_version: std::string::String,
    #[prost(message, repeated, tag = "7")]
    pub build_deps: ::std::vec::Vec<Module>,
}
/// Module is the type for VersionInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// module path
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// module version
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    /// checksum
    #[prost(string, tag = "3")]
    pub sum: std::string::String,
}
