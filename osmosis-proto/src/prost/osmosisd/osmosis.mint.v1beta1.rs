/// Minter represents the minting state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// current epoch provisions
    #[prost(string, tag="1")]
    pub epoch_provisions: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedAddress {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionProportions {
    /// staking defines the proportion of the minted minted_denom that is to be
    /// allocated as staking rewards.
    #[prost(string, tag="1")]
    pub staking: ::prost::alloc::string::String,
    /// pool_incentives defines the proportion of the minted minted_denom that is
    /// to be allocated as pool incentives.
    #[prost(string, tag="2")]
    pub pool_incentives: ::prost::alloc::string::String,
    /// developer_rewards defines the proportion of the minted minted_denom that is
    /// to be allocated to developer rewards address.
    #[prost(string, tag="3")]
    pub developer_rewards: ::prost::alloc::string::String,
    /// community_pool defines the proportion of the minted minted_denom that is
    /// to be allocated to the community pool.
    #[prost(string, tag="4")]
    pub community_pool: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// epoch provisions from the first epoch
    #[prost(string, tag="2")]
    pub genesis_epoch_provisions: ::prost::alloc::string::String,
    /// mint epoch identifier
    #[prost(string, tag="3")]
    pub epoch_identifier: ::prost::alloc::string::String,
    /// number of epochs take to reduce rewards
    #[prost(int64, tag="4")]
    pub reduction_period_in_epochs: i64,
    /// reduction multiplier to execute on each period
    #[prost(string, tag="5")]
    pub reduction_factor: ::prost::alloc::string::String,
    /// distribution_proportions defines the proportion of the minted denom
    #[prost(message, optional, tag="6")]
    pub distribution_proportions: ::core::option::Option<DistributionProportions>,
    /// address to receive developer rewards
    #[prost(message, repeated, tag="7")]
    pub weighted_developer_rewards_receivers: ::prost::alloc::vec::Vec<WeightedAddress>,
    /// start epoch to distribute minting rewards
    #[prost(int64, tag="8")]
    pub minting_rewards_distribution_start_epoch: i64,
}
/// GenesisState defines the mint module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is a space for holding current rewards information.
    #[prost(message, optional, tag="1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
    /// current halven period start epoch
    #[prost(int64, tag="3")]
    pub halven_started_epoch: i64,
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
/// QueryEpochProvisionsRequest is the request type for the
/// Query/EpochProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsRequest {
}
/// QueryEpochProvisionsResponse is the response type for the
/// Query/EpochProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsResponse {
    /// epoch_provisions is the current minting per epoch provisions value.
    #[prost(bytes="vec", tag="1")]
    pub epoch_provisions: ::prost::alloc::vec::Vec<u8>,
}
