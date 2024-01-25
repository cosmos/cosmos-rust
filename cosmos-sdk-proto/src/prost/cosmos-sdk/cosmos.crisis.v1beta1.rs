// @generated
/// GenesisState defines the crisis module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// constant_fee is the fee used to verify the invariant in the crisis
    /// module.
    #[prost(message, optional, tag = "3")]
    pub constant_fee: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgVerifyInvariant represents a message to verify a particular invariance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVerifyInvariant {
    /// sender is the account address of private key to send coins to fee collector account.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// name of the invariant module.
    #[prost(string, tag = "2")]
    pub invariant_module_name: ::prost::alloc::string::String,
    /// invariant_route is the msg's invariant route.
    #[prost(string, tag = "3")]
    pub invariant_route: ::prost::alloc::string::String,
}
/// MsgVerifyInvariantResponse defines the Msg/VerifyInvariant response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVerifyInvariantResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// constant_fee defines the x/crisis parameter.
    #[prost(message, optional, tag = "2")]
    pub constant_fee: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("cosmos.crisis.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
