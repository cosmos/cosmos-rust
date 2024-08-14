// @generated
/// GenesisState defines 08-wasm's keeper genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// uploaded light client wasm contracts
    #[prost(message, repeated, tag = "1")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
/// Contract stores contract code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// contract byte code
    #[prost(bytes = "vec", tag = "1")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryChecksumsRequest is the request type for the Query/Checksums RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChecksumsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryChecksumsResponse is the response type for the Query/Checksums RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChecksumsResponse {
    /// checksums is a list of the hex encoded checksums of all wasm codes stored.
    #[prost(string, repeated, tag = "1")]
    pub checksums: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// checksum is a hex encoded string of the code stored.
    #[prost(string, tag = "1")]
    pub checksum: ::prost::alloc::string::String,
}
/// QueryCodeResponse is the response type for the Query/Code RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgStoreCode defines the request type for the StoreCode rpc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCode {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// wasm byte code of light client contract. It can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
}
/// MsgStoreCodeResponse defines the response type for the StoreCode rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCodeResponse {
    /// checksum is the sha256 hash of the stored code
    #[prost(bytes = "vec", tag = "1")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// MsgRemoveChecksum defines the request type for the MsgRemoveChecksum rpc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveChecksum {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// checksum is the sha256 hash to be removed from the store
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// MsgStoreChecksumResponse defines the response type for the StoreCode rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveChecksumResponse {}
/// MsgMigrateContract defines the request type for the MigrateContract rpc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContract {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// the client id of the contract
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// checksum is the sha256 hash of the new wasm byte code for the contract
    #[prost(bytes = "vec", tag = "3")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    /// the json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContractResponse defines the response type for the MigrateContract rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContractResponse {}
/// Wasm light client's Client state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// bytes encoding the client state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
/// Wasm light client's ConsensusState
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// bytes encoding the consensus state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Wasm light client message (either header(s) or misbehaviour)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Checksums defines a list of all checksums that are stored
///
/// Deprecated: This message is deprecated in favor of storing the checksums
/// using a Collections.KeySet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checksums {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub checksums: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
include!("ibc.lightclients.wasm.v1.serde.rs");
include!("ibc.lightclients.wasm.v1.tonic.rs");
// @@protoc_insertion_point(module)
