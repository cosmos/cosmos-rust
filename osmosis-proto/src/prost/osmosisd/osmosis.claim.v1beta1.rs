/// Params defines the claim module's parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag="1")]
    pub airdrop_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub duration_until_decay: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub duration_of_decay: ::core::option::Option<::prost_types::Duration>,
    /// denom of claimable asset
    #[prost(string, tag="4")]
    pub claim_denom: ::prost::alloc::string::String,
}
/// A Claim Records is the metadata of claim data per address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimRecord {
    /// address of claim user
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// total initial claimable amount for the user
    #[prost(message, repeated, tag="2")]
    pub initial_claimable_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// true if action is completed
    /// index of bool in array refers to action enum #
    #[prost(bool, repeated, packed="false", tag="3")]
    pub action_completed: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    AddLiquidity = 0,
    Swap = 1,
    Vote = 2,
    DelegateStake = 3,
}
/// GenesisState defines the claim module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// balance of the claim module's account
    #[prost(message, optional, tag="1")]
    pub module_account_balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
    /// list of claim records, one for every airdrop recipient
    #[prost(message, repeated, tag="3")]
    pub claim_records: ::prost::alloc::vec::Vec<ClaimRecord>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalanceRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalanceResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag="1")]
    pub module_account_balance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordResponse {
    #[prost(message, optional, tag="1")]
    pub claim_record: ::core::option::Option<ClaimRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableForActionRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration="Action", tag="2")]
    pub action: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableForActionResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalClaimableRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalClaimableResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
