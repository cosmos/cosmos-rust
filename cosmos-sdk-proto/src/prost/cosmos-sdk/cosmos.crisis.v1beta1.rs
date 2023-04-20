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
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub invariant_module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub invariant_route: ::prost::alloc::string::String,
}
/// MsgVerifyInvariantResponse defines the Msg/VerifyInvariant response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVerifyInvariantResponse {}
include!("cosmos.crisis.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
