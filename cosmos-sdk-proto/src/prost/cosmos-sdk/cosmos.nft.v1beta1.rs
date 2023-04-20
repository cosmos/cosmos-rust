// @generated
/// EventSend is emitted on Msg/Send
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSend {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// EventMint is emitted on Mint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMint {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventBurn is emitted on Burn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurn {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// Class defines the class of the nft type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// id defines the unique identifier of the NFT classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the NFT classification. Optional
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for nft classification. Optional
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    /// description is a brief description of nft classification. Optional
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// uri for the class metadata stored off chain. It can define schema for Class and NFT `Data` attributes. Optional
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri. Optional
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is the app specific metadata of the NFT class. Optional
    #[prost(message, optional, tag = "7")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
/// NFT defines the NFT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    /// class_id associated with the NFT, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// uri for the NFT metadata stored off chain
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri
    #[prost(string, tag = "4")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is an app specific data of the NFT. Optional
    #[prost(message, optional, tag = "10")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
/// GenesisState defines the nft module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// class defines the class of the nft type.
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
}
/// Entry Defines all nft owned by a person
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// owner is the owner address of the following nft
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// nfts is a group of nfts of the same owner
    #[prost(message, repeated, tag = "2")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// QueryOwnerRequest is the request type for the Query/Owner RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryOwnerResponse is the response type for the Query/Owner RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerResponse {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// QuerySupplyRequest is the request type for the Query/Supply RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/Supply RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// QueryNFTstRequest is the request type for the Query/NFTs RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNfTsRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryNFTsResponse is the response type for the Query/NFTs RPC methods
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNfTsResponse {
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryNFTRequest is the request type for the Query/NFT RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryNFTResponse is the response type for the Query/NFT RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftResponse {
    #[prost(message, optional, tag = "1")]
    pub nft: ::core::option::Option<Nft>,
}
/// QueryClassRequest is the request type for the Query/Class RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryClassResponse is the response type for the Query/Class RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassResponse {
    #[prost(message, optional, tag = "1")]
    pub class: ::core::option::Option<Class>,
}
/// QueryClassesRequest is the request type for the Query/Classes RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryClassesResponse is the response type for the Query/Classes RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesResponse {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSend represents a message to send a nft from one account to another account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// class_id defines the unique identifier of the nft classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id defines the unique identification of nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sender is the address of the owner of nft
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver address of nft
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgSendResponse defines the Msg/Send response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
include!("cosmos.nft.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
