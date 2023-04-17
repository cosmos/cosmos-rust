// @generated
/// Minter represents the minting state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// current annual inflation rate
    #[prost(string, tag = "1")]
    pub inflation: ::prost::alloc::string::String,
    /// current annual expected provisions
    #[prost(string, tag = "2")]
    pub annual_provisions: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag = "1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// maximum annual change in inflation rate
    #[prost(string, tag = "2")]
    pub inflation_rate_change: ::prost::alloc::string::String,
    /// maximum inflation rate
    #[prost(string, tag = "3")]
    pub inflation_max: ::prost::alloc::string::String,
    /// minimum inflation rate
    #[prost(string, tag = "4")]
    pub inflation_min: ::prost::alloc::string::String,
    /// goal of percent bonded atoms
    #[prost(string, tag = "5")]
    pub goal_bonded: ::prost::alloc::string::String,
    /// expected blocks per year
    #[prost(uint64, tag = "6")]
    pub blocks_per_year: u64,
}
/// GenesisState defines the mint module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is a space for holding current inflation information.
    #[prost(message, optional, tag = "1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
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
/// QueryInflationRequest is the request type for the Query/Inflation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationRequest {}
/// QueryInflationResponse is the response type for the Query/Inflation RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationResponse {
    /// inflation is the current minting inflation value.
    #[prost(bytes = "vec", tag = "1")]
    pub inflation: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAnnualProvisionsRequest is the request type for the
/// Query/AnnualProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsRequest {}
/// QueryAnnualProvisionsResponse is the response type for the
/// Query/AnnualProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsResponse {
    /// annual_provisions is the current minting annual provisions value.
    #[prost(bytes = "vec", tag = "1")]
    pub annual_provisions: ::prost::alloc::vec::Vec<u8>,
}
include!("cosmos.mint.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
