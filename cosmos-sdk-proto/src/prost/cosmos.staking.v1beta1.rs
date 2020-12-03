/// HistoricalInfo contains header and validator information for a given block.
/// It is stored as part of staking module's state, which persists the `n` most
/// recent HistoricalInfo
/// (`n` is set by the staking module's `historical_entries` parameter).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalInfo {
    #[prost(message, optional, tag="1")]
    pub header: ::std::option::Option<tendermint_proto::types::Header>,
    #[prost(message, repeated, tag="2")]
    pub valset: ::std::vec::Vec<Validator>,
}
/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommissionRates {
    #[prost(string, tag="1")]
    pub rate: std::string::String,
    #[prost(string, tag="2")]
    pub max_rate: std::string::String,
    #[prost(string, tag="3")]
    pub max_change_rate: std::string::String,
}
/// Commission defines commission parameters for a given validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commission {
    #[prost(message, optional, tag="1")]
    pub commission_rates: ::std::option::Option<CommissionRates>,
    #[prost(message, optional, tag="2")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Description defines a validator description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    #[prost(string, tag="1")]
    pub moniker: std::string::String,
    #[prost(string, tag="2")]
    pub identity: std::string::String,
    #[prost(string, tag="3")]
    pub website: std::string::String,
    #[prost(string, tag="4")]
    pub security_contact: std::string::String,
    #[prost(string, tag="5")]
    pub details: std::string::String,
}
/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag="1")]
    pub operator_address: std::string::String,
    #[prost(message, optional, tag="2")]
    pub consensus_pubkey: ::std::option::Option<::prost_types::Any>,
    #[prost(bool, tag="3")]
    pub jailed: bool,
    #[prost(enumeration="BondStatus", tag="4")]
    pub status: i32,
    #[prost(string, tag="5")]
    pub tokens: std::string::String,
    #[prost(string, tag="6")]
    pub delegator_shares: std::string::String,
    #[prost(message, optional, tag="7")]
    pub description: ::std::option::Option<Description>,
    #[prost(int64, tag="8")]
    pub unbonding_height: i64,
    #[prost(message, optional, tag="9")]
    pub unbonding_time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub commission: ::std::option::Option<Commission>,
    #[prost(string, tag="11")]
    pub min_self_delegation: std::string::String,
}
/// ValAddresses defines a repeated set of validator addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValAddresses {
    #[prost(string, repeated, tag="1")]
    pub addresses: ::std::vec::Vec<std::string::String>,
}
/// DVPair is struct that just has a delegator-validator pair with no other data.
/// It is intended to be used as a marshalable pointer. For example, a DVPair can
/// be used to construct the key to getting an UnbondingDelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPair {
    #[prost(string, tag="1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag="2")]
    pub validator_address: std::string::String,
}
/// DVPairs defines an array of DVPair objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPairs {
    #[prost(message, repeated, tag="1")]
    pub pairs: ::std::vec::Vec<DvPair>,
}
/// DVVTriplet is struct that just has a delegator-validator-validator triplet
/// with no other data. It is intended to be used as a marshalable pointer. For
/// example, a DVVTriplet can be used to construct the key to getting a
/// Redelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplet {
    #[prost(string, tag="1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag="2")]
    pub validator_src_address: std::string::String,
    #[prost(string, tag="3")]
    pub validator_dst_address: std::string::String,
}
/// DVVTriplets defines an array of DVVTriplet objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplets {
    #[prost(message, repeated, tag="1")]
    pub triplets: ::std::vec::Vec<DvvTriplet>,
}
/// Delegation represents the bond with tokens held by an account. It is
/// owned by one delegator, and is associated with the voting power of one
/// validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegation {
    #[prost(string, tag="1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag="2")]
    pub validator_address: std::string::String,
    #[prost(string, tag="3")]
    pub shares: std::string::String,
}
/// UnbondingDelegation stores all of a single delegator's unbonding bonds
/// for a single validator in an time-ordered list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegation {
    #[prost(string, tag="1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag="2")]
    pub validator_address: std::string::String,
    /// unbonding delegation entries
    #[prost(message, repeated, tag="3")]
    pub entries: ::std::vec::Vec<UnbondingDelegationEntry>,
}
/// UnbondingDelegationEntry defines an unbonding object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegationEntry {
    #[prost(int64, tag="1")]
    pub creation_height: i64,
    #[prost(message, optional, tag="2")]
    pub completion_time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub initial_balance: std::string::String,
    #[prost(string, tag="4")]
    pub balance: std::string::String,
}
/// RedelegationEntry defines a redelegation object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntry {
    #[prost(int64, tag="1")]
    pub creation_height: i64,
    #[prost(message, optional, tag="2")]
    pub completion_time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub initial_balance: std::string::String,
    #[prost(string, tag="4")]
    pub shares_dst: std::string::String,
}
/// Redelegation contains the list of a particular delegator's redelegating bonds
/// from a particular source validator to a particular destination validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Redelegation {
    #[prost(string, tag="1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag="2")]
    pub validator_src_address: std::string::String,
    #[prost(string, tag="3")]
    pub validator_dst_address: std::string::String,
    /// redelegation entries
    #[prost(message, repeated, tag="4")]
    pub entries: ::std::vec::Vec<RedelegationEntry>,
}
/// Params defines the parameters for the staking module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag="1")]
    pub unbonding_time: ::std::option::Option<::prost_types::Duration>,
    #[prost(uint32, tag="2")]
    pub max_validators: u32,
    #[prost(uint32, tag="3")]
    pub max_entries: u32,
    #[prost(uint32, tag="4")]
    pub historical_entries: u32,
    #[prost(string, tag="5")]
    pub bond_denom: std::string::String,
}
/// DelegationResponse is equivalent to Delegation except that it contains a
/// balance in addition to shares which is more suitable for client responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationResponse {
    #[prost(message, optional, tag="1")]
    pub delegation: ::std::option::Option<Delegation>,
    #[prost(message, optional, tag="2")]
    pub balance: ::std::option::Option<super::super::base::v1beta1::Coin>,
}
/// RedelegationEntryResponse is equivalent to a RedelegationEntry except that it
/// contains a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntryResponse {
    #[prost(message, optional, tag="1")]
    pub redelegation_entry: ::std::option::Option<RedelegationEntry>,
    #[prost(string, tag="4")]
    pub balance: std::string::String,
}
/// RedelegationResponse is equivalent to a Redelegation except that its entries
/// contain a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationResponse {
    #[prost(message, optional, tag="1")]
    pub redelegation: ::std::option::Option<Redelegation>,
    #[prost(message, repeated, tag="2")]
    pub entries: ::std::vec::Vec<RedelegationEntryResponse>,
}
/// Pool is used for tracking bonded and not-bonded token supply of the bond
/// denomination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub not_bonded_tokens: std::string::String,
    #[prost(string, tag="2")]
    pub bonded_tokens: std::string::String,
}
/// BondStatus is the status of a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}
/// QueryValidatorsRequest is request type for Query/Validators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag="1")]
    pub status: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorsResponse is response type for the Query/Validators RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag="1")]
    pub validators: ::std::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorRequest is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: std::string::String,
}
/// QueryValidatorResponse is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag="1")]
    pub validator: ::std::option::Option<Validator>,
}
/// QueryValidatorDelegationsRequest is request type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorDelegationsResponse is response type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsResponse {
    #[prost(message, repeated, tag="1")]
    pub delegation_responses: ::std::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorUnbondingDelegationsRequest is required type for the
/// Query/ValidatorUnbondingDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorUnbondingDelegationsResponse is response type for the
/// Query/ValidatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag="1")]
    pub unbonding_responses: ::std::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRequest is request type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="2")]
    pub validator_addr: std::string::String,
}
/// QueryDelegationResponse is response type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationResponse {
    /// delegation_responses defines the delegation info of a delegation.
    #[prost(message, optional, tag="1")]
    pub delegation_response: ::std::option::Option<DelegationResponse>,
}
/// QueryUnbondingDelegationRequest is request type for the
/// Query/UnbondingDelegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="2")]
    pub validator_addr: std::string::String,
}
/// QueryDelegationResponse is response type for the Query/UnbondingDelegation
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationResponse {
    /// unbond defines the unbonding information of a delegation.
    #[prost(message, optional, tag="1")]
    pub unbond: ::std::option::Option<UnbondingDelegation>,
}
/// QueryDelegatorDelegationsRequest is request type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorDelegationsResponse is response type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsResponse {
    /// delegation_responses defines all the delegations' info of a delegator.
    #[prost(message, repeated, tag="1")]
    pub delegation_responses: ::std::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorUnbondingDelegationsRequest is request type for the
/// Query/DelegatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryUnbondingDelegatorDelegationsResponse is response type for the
/// Query/UnbondingDelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag="1")]
    pub unbonding_responses: ::std::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryRedelegationsRequest is request type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// src_validator_addr defines the validator address to redelegate from.
    #[prost(string, tag="2")]
    pub src_validator_addr: std::string::String,
    /// dst_validator_addr defines the validator address to redelegate to.
    #[prost(string, tag="3")]
    pub dst_validator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="4")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryRedelegationsResponse is response type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsResponse {
    #[prost(message, repeated, tag="1")]
    pub redelegation_responses: ::std::vec::Vec<RedelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorsRequest is request type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorValidatorsResponse is response type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the the validators' info of a delegator.
    #[prost(message, repeated, tag="1")]
    pub validators: ::std::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorRequest is request type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag="1")]
    pub delegator_addr: std::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag="2")]
    pub validator_addr: std::string::String,
}
/// QueryDelegatorValidatorResponse response type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag="1")]
    pub validator: ::std::option::Option<Validator>,
}
/// QueryHistoricalInfoRequest is request type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoRequest {
    /// height defines at which height to query the historical info.
    #[prost(int64, tag="1")]
    pub height: i64,
}
/// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoResponse {
    /// hist defines the historical info at the given height.
    #[prost(message, optional, tag="1")]
    pub hist: ::std::option::Option<HistoricalInfo>,
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    /// pool defines the pool info.
    #[prost(message, optional, tag="1")]
    pub pool: ::std::option::Option<Pool>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag="1")]
    pub params: ::std::option::Option<Params>,
}
# [doc = r" Generated client implementations."] pub mod query_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Query defines the gRPC querier service."] pub struct QueryClient < T > { inner : tonic :: client :: Grpc < T > , } impl QueryClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > QueryClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " Validators queries all validators that match the given status."] pub async fn validators (& mut self , request : impl tonic :: IntoRequest < super :: QueryValidatorsRequest > ,) -> Result < tonic :: Response < super :: QueryValidatorsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Validators") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Validator queries validator info for given validator address."] pub async fn validator (& mut self , request : impl tonic :: IntoRequest < super :: QueryValidatorRequest > ,) -> Result < tonic :: Response < super :: QueryValidatorResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Validator") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ValidatorDelegations queries delegate info for given validator."] pub async fn validator_delegations (& mut self , request : impl tonic :: IntoRequest < super :: QueryValidatorDelegationsRequest > ,) -> Result < tonic :: Response < super :: QueryValidatorDelegationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/ValidatorDelegations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ValidatorUnbondingDelegations queries unbonding delegations of a validator."] pub async fn validator_unbonding_delegations (& mut self , request : impl tonic :: IntoRequest < super :: QueryValidatorUnbondingDelegationsRequest > ,) -> Result < tonic :: Response < super :: QueryValidatorUnbondingDelegationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/ValidatorUnbondingDelegations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Delegation queries delegate info for given validator delegator pair."] pub async fn delegation (& mut self , request : impl tonic :: IntoRequest < super :: QueryDelegationRequest > ,) -> Result < tonic :: Response < super :: QueryDelegationResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Delegation") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " UnbondingDelegation queries unbonding info for given validator delegator"] # [doc = " pair."] pub async fn unbonding_delegation (& mut self , request : impl tonic :: IntoRequest < super :: QueryUnbondingDelegationRequest > ,) -> Result < tonic :: Response < super :: QueryUnbondingDelegationResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/UnbondingDelegation") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " DelegatorDelegations queries all delegations of a given delegator address."] pub async fn delegator_delegations (& mut self , request : impl tonic :: IntoRequest < super :: QueryDelegatorDelegationsRequest > ,) -> Result < tonic :: Response < super :: QueryDelegatorDelegationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/DelegatorDelegations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " DelegatorUnbondingDelegations queries all unbonding delegations of a given"] # [doc = " delegator address."] pub async fn delegator_unbonding_delegations (& mut self , request : impl tonic :: IntoRequest < super :: QueryDelegatorUnbondingDelegationsRequest > ,) -> Result < tonic :: Response < super :: QueryDelegatorUnbondingDelegationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Redelegations queries redelegations of given address."] pub async fn redelegations (& mut self , request : impl tonic :: IntoRequest < super :: QueryRedelegationsRequest > ,) -> Result < tonic :: Response < super :: QueryRedelegationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Redelegations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " DelegatorValidators queries all validators info for given delegator"] # [doc = " address."] pub async fn delegator_validators (& mut self , request : impl tonic :: IntoRequest < super :: QueryDelegatorValidatorsRequest > ,) -> Result < tonic :: Response < super :: QueryDelegatorValidatorsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/DelegatorValidators") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " DelegatorValidator queries validator info for given delegator validator"] # [doc = " pair."] pub async fn delegator_validator (& mut self , request : impl tonic :: IntoRequest < super :: QueryDelegatorValidatorRequest > ,) -> Result < tonic :: Response < super :: QueryDelegatorValidatorResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/DelegatorValidator") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " HistoricalInfo queries the historical info for given height."] pub async fn historical_info (& mut self , request : impl tonic :: IntoRequest < super :: QueryHistoricalInfoRequest > ,) -> Result < tonic :: Response < super :: QueryHistoricalInfoResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/HistoricalInfo") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Pool queries the pool info."] pub async fn pool (& mut self , request : impl tonic :: IntoRequest < super :: QueryPoolRequest > ,) -> Result < tonic :: Response < super :: QueryPoolResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Pool") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Parameters queries the staking parameters."] pub async fn params (& mut self , request : impl tonic :: IntoRequest < super :: QueryParamsRequest > ,) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.staking.v1beta1.Query/Params") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for QueryClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for QueryClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "QueryClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod query_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."] # [async_trait] pub trait Query : Send + Sync + 'static { # [doc = " Validators queries all validators that match the given status."] async fn validators (& self , request : tonic :: Request < super :: QueryValidatorsRequest >) -> Result < tonic :: Response < super :: QueryValidatorsResponse > , tonic :: Status > ; # [doc = " Validator queries validator info for given validator address."] async fn validator (& self , request : tonic :: Request < super :: QueryValidatorRequest >) -> Result < tonic :: Response < super :: QueryValidatorResponse > , tonic :: Status > ; # [doc = " ValidatorDelegations queries delegate info for given validator."] async fn validator_delegations (& self , request : tonic :: Request < super :: QueryValidatorDelegationsRequest >) -> Result < tonic :: Response < super :: QueryValidatorDelegationsResponse > , tonic :: Status > ; # [doc = " ValidatorUnbondingDelegations queries unbonding delegations of a validator."] async fn validator_unbonding_delegations (& self , request : tonic :: Request < super :: QueryValidatorUnbondingDelegationsRequest >) -> Result < tonic :: Response < super :: QueryValidatorUnbondingDelegationsResponse > , tonic :: Status > ; # [doc = " Delegation queries delegate info for given validator delegator pair."] async fn delegation (& self , request : tonic :: Request < super :: QueryDelegationRequest >) -> Result < tonic :: Response < super :: QueryDelegationResponse > , tonic :: Status > ; # [doc = " UnbondingDelegation queries unbonding info for given validator delegator"] # [doc = " pair."] async fn unbonding_delegation (& self , request : tonic :: Request < super :: QueryUnbondingDelegationRequest >) -> Result < tonic :: Response < super :: QueryUnbondingDelegationResponse > , tonic :: Status > ; # [doc = " DelegatorDelegations queries all delegations of a given delegator address."] async fn delegator_delegations (& self , request : tonic :: Request < super :: QueryDelegatorDelegationsRequest >) -> Result < tonic :: Response < super :: QueryDelegatorDelegationsResponse > , tonic :: Status > ; # [doc = " DelegatorUnbondingDelegations queries all unbonding delegations of a given"] # [doc = " delegator address."] async fn delegator_unbonding_delegations (& self , request : tonic :: Request < super :: QueryDelegatorUnbondingDelegationsRequest >) -> Result < tonic :: Response < super :: QueryDelegatorUnbondingDelegationsResponse > , tonic :: Status > ; # [doc = " Redelegations queries redelegations of given address."] async fn redelegations (& self , request : tonic :: Request < super :: QueryRedelegationsRequest >) -> Result < tonic :: Response < super :: QueryRedelegationsResponse > , tonic :: Status > ; # [doc = " DelegatorValidators queries all validators info for given delegator"] # [doc = " address."] async fn delegator_validators (& self , request : tonic :: Request < super :: QueryDelegatorValidatorsRequest >) -> Result < tonic :: Response < super :: QueryDelegatorValidatorsResponse > , tonic :: Status > ; # [doc = " DelegatorValidator queries validator info for given delegator validator"] # [doc = " pair."] async fn delegator_validator (& self , request : tonic :: Request < super :: QueryDelegatorValidatorRequest >) -> Result < tonic :: Response < super :: QueryDelegatorValidatorResponse > , tonic :: Status > ; # [doc = " HistoricalInfo queries the historical info for given height."] async fn historical_info (& self , request : tonic :: Request < super :: QueryHistoricalInfoRequest >) -> Result < tonic :: Response < super :: QueryHistoricalInfoResponse > , tonic :: Status > ; # [doc = " Pool queries the pool info."] async fn pool (& self , request : tonic :: Request < super :: QueryPoolRequest >) -> Result < tonic :: Response < super :: QueryPoolResponse > , tonic :: Status > ; # [doc = " Parameters queries the staking parameters."] async fn params (& self , request : tonic :: Request < super :: QueryParamsRequest >) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > ; } # [doc = " Query defines the gRPC querier service."] # [derive (Debug)] pub struct QueryServer < T : Query > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : Query > QueryServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for QueryServer < T > where T : Query , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/cosmos.staking.v1beta1.Query/Validators" => { # [allow (non_camel_case_types)] struct ValidatorsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryValidatorsRequest > for ValidatorsSvc < T > { type Response = super :: QueryValidatorsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryValidatorsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . validators (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ValidatorsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/Validator" => { # [allow (non_camel_case_types)] struct ValidatorSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryValidatorRequest > for ValidatorSvc < T > { type Response = super :: QueryValidatorResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryValidatorRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . validator (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ValidatorSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/ValidatorDelegations" => { # [allow (non_camel_case_types)] struct ValidatorDelegationsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryValidatorDelegationsRequest > for ValidatorDelegationsSvc < T > { type Response = super :: QueryValidatorDelegationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryValidatorDelegationsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . validator_delegations (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ValidatorDelegationsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/ValidatorUnbondingDelegations" => { # [allow (non_camel_case_types)] struct ValidatorUnbondingDelegationsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryValidatorUnbondingDelegationsRequest > for ValidatorUnbondingDelegationsSvc < T > { type Response = super :: QueryValidatorUnbondingDelegationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryValidatorUnbondingDelegationsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . validator_unbonding_delegations (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ValidatorUnbondingDelegationsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/Delegation" => { # [allow (non_camel_case_types)] struct DelegationSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryDelegationRequest > for DelegationSvc < T > { type Response = super :: QueryDelegationResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryDelegationRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delegation (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DelegationSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/UnbondingDelegation" => { # [allow (non_camel_case_types)] struct UnbondingDelegationSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryUnbondingDelegationRequest > for UnbondingDelegationSvc < T > { type Response = super :: QueryUnbondingDelegationResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryUnbondingDelegationRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . unbonding_delegation (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = UnbondingDelegationSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/DelegatorDelegations" => { # [allow (non_camel_case_types)] struct DelegatorDelegationsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryDelegatorDelegationsRequest > for DelegatorDelegationsSvc < T > { type Response = super :: QueryDelegatorDelegationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryDelegatorDelegationsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delegator_delegations (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DelegatorDelegationsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations" => { # [allow (non_camel_case_types)] struct DelegatorUnbondingDelegationsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryDelegatorUnbondingDelegationsRequest > for DelegatorUnbondingDelegationsSvc < T > { type Response = super :: QueryDelegatorUnbondingDelegationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryDelegatorUnbondingDelegationsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delegator_unbonding_delegations (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DelegatorUnbondingDelegationsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/Redelegations" => { # [allow (non_camel_case_types)] struct RedelegationsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryRedelegationsRequest > for RedelegationsSvc < T > { type Response = super :: QueryRedelegationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryRedelegationsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . redelegations (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = RedelegationsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/DelegatorValidators" => { # [allow (non_camel_case_types)] struct DelegatorValidatorsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryDelegatorValidatorsRequest > for DelegatorValidatorsSvc < T > { type Response = super :: QueryDelegatorValidatorsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryDelegatorValidatorsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delegator_validators (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DelegatorValidatorsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/DelegatorValidator" => { # [allow (non_camel_case_types)] struct DelegatorValidatorSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryDelegatorValidatorRequest > for DelegatorValidatorSvc < T > { type Response = super :: QueryDelegatorValidatorResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryDelegatorValidatorRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delegator_validator (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DelegatorValidatorSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/HistoricalInfo" => { # [allow (non_camel_case_types)] struct HistoricalInfoSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryHistoricalInfoRequest > for HistoricalInfoSvc < T > { type Response = super :: QueryHistoricalInfoResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryHistoricalInfoRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . historical_info (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = HistoricalInfoSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/Pool" => { # [allow (non_camel_case_types)] struct PoolSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryPoolRequest > for PoolSvc < T > { type Response = super :: QueryPoolResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryPoolRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . pool (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = PoolSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.staking.v1beta1.Query/Params" => { # [allow (non_camel_case_types)] struct ParamsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryParamsRequest > for ParamsSvc < T > { type Response = super :: QueryParamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryParamsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . params (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ParamsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : Query > Clone for QueryServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : Query > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : Query > tonic :: transport :: NamedService for QueryServer < T > { const NAME : & 'static str = "cosmos.staking.v1beta1.Query" ; } }