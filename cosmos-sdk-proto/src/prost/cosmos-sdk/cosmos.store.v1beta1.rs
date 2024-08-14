// @generated
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag = "1")]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag = "2")]
    pub delete: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// BlockMetadata contains all the abci event data of a block
/// the file streamer dump them into files together with the state changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(message, optional, tag = "6")]
    pub response_commit: ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseCommit>,
    #[prost(message, optional, tag = "7")]
    pub request_finalize_block:
        ::core::option::Option<::tendermint_proto::v0_34::abci::RequestFinalizeBlock>,
    /// TODO: should we renumber this?
    #[prost(message, optional, tag = "8")]
    pub response_finalize_block:
        ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseFinalizeBlock>,
}
/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub commit_id: ::core::option::Option<CommitId>,
}
/// CommitID defines the commitment information when a specific store is
/// committed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitId {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
include!("cosmos.store.v1beta1.serde.rs");
// @@protoc_insertion_point(module)
