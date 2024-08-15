// @generated
/// ListenEndBlockRequest is the request type for the ListenEndBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenFinalizeBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub req: ::core::option::Option<::tendermint_proto::abci::RequestFinalizeBlock>,
    #[prost(message, optional, tag = "2")]
    pub res: ::core::option::Option<::tendermint_proto::abci::ResponseFinalizeBlock>,
}
impl ::prost::Name for ListenFinalizeBlockRequest {
    const NAME: &'static str = "ListenFinalizeBlockRequest";
    const PACKAGE: &'static str = "cosmos.store.streaming.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.store.streaming.abci.{}", Self::NAME)
    }
}
/// ListenEndBlockResponse is the response type for the ListenEndBlock RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenFinalizeBlockResponse {}
impl ::prost::Name for ListenFinalizeBlockResponse {
    const NAME: &'static str = "ListenFinalizeBlockResponse";
    const PACKAGE: &'static str = "cosmos.store.streaming.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.store.streaming.abci.{}", Self::NAME)
    }
}
/// ListenCommitRequest is the request type for the ListenCommit RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenCommitRequest {
    /// explicitly pass in block height as ResponseCommit does not contain this info
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, optional, tag = "2")]
    pub res: ::core::option::Option<::tendermint_proto::abci::ResponseCommit>,
    #[prost(message, repeated, tag = "3")]
    pub change_set: ::prost::alloc::vec::Vec<super::super::v1beta1::StoreKvPair>,
}
impl ::prost::Name for ListenCommitRequest {
    const NAME: &'static str = "ListenCommitRequest";
    const PACKAGE: &'static str = "cosmos.store.streaming.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.store.streaming.abci.{}", Self::NAME)
    }
}
/// ListenCommitResponse is the response type for the ListenCommit RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenCommitResponse {}
impl ::prost::Name for ListenCommitResponse {
    const NAME: &'static str = "ListenCommitResponse";
    const PACKAGE: &'static str = "cosmos.store.streaming.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.store.streaming.abci.{}", Self::NAME)
    }
}
include!("cosmos.store.streaming.abci.serde.rs");
include!("cosmos.store.streaming.abci.tonic.rs");
// @@protoc_insertion_point(module)
