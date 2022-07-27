/// FeeToken is a struct that specifies a coin denom, and pool ID pair.
/// This marks the token as eligible for use as a tx fee asset in Osmosis.
/// Its price in osmo is derived through looking at the provided pool ID.
/// The pool ID must have osmo as one of its assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeToken {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
}
/// UpdateFeeTokenProposal is a gov Content type for adding a new whitelisted fee
/// token. It must specify a denom along with gamm pool ID to use as a spot price
/// calculator. It can be used to add a new denom to the whitelist It can also be
/// used to update the Pool to associate with the denom. If Pool ID is set to 0,
/// it will remove the denom from the whitelisted set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeeTokenProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub feetoken: ::core::option::Option<FeeToken>,
}
/// GenesisState defines the txfees module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag="1")]
    pub basedenom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub feetokens: ::prost::alloc::vec::Vec<FeeToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensResponse {
    #[prost(message, repeated, tag="1")]
    pub fee_tokens: ::prost::alloc::vec::Vec<FeeToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomResponse {
    #[prost(string, tag="1")]
    pub base_denom: ::prost::alloc::string::String,
}
