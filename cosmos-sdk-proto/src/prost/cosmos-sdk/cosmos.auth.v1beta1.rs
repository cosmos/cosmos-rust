// @generated
/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
}
/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<BaseAccount>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the auth module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub max_memo_characters: u64,
    #[prost(uint64, tag = "2")]
    pub tx_sig_limit: u64,
    #[prost(uint64, tag = "3")]
    pub tx_size_cost_per_byte: u64,
    #[prost(uint64, tag = "4")]
    pub sig_verify_cost_ed25519: u64,
    #[prost(uint64, tag = "5")]
    pub sig_verify_cost_secp256k1: u64,
}
/// GenesisState defines the auth module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// accounts are the accounts present at genesis.
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// QueryAccountsRequest is the request type for the Query/Accounts RPC method.
///
/// Since: cosmos-sdk 0.43
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAccountsResponse is the response type for the Query/Accounts RPC method.
///
/// Since: cosmos-sdk 0.43
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountsResponse {
    /// accounts are the existing accounts
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<::prost_types::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryModuleAccountsRequest is the request type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountsRequest {}
/// QueryModuleAccountsResponse is the response type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// QueryModuleAccountByNameRequest is the request type for the Query/ModuleAccountByName RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountByNameRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryModuleAccountByNameResponse is the response type for the Query/ModuleAccountByName RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountByNameResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<::prost_types::Any>,
}
/// Bech32PrefixRequest is the request type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bech32PrefixRequest {}
/// Bech32PrefixResponse is the response type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bech32PrefixResponse {
    #[prost(string, tag = "1")]
    pub bech32_prefix: ::prost::alloc::string::String,
}
/// AddressBytesToStringRequest is the request type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytesToStringRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// AddressBytesToStringResponse is the response type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytesToStringResponse {
    #[prost(string, tag = "1")]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesRequest is the request type for AccountBytes rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressStringToBytesRequest {
    #[prost(string, tag = "1")]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesResponse is the response type for AddressBytes rpc method.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressStringToBytesResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAccountAddressByIDRequest is the request type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressByIdRequest {
    /// id is the account number of the address to be queried. This field
    /// should have been an uint64 (like all account numbers), and will be
    /// updated to uint64 in a future version of the auth query.
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// QueryAccountAddressByIDResponse is the response type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressByIdResponse {
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
}
include!("cosmos.auth.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
