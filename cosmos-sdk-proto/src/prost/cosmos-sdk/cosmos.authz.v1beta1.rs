// @generated
/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
/// Grant gives permissions to execute
/// the provide method with expiration time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    #[prost(message, optional, tag = "1")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    /// time when the grant will expire and will be pruned. If null, then the grant
    /// doesn't have a time expiration (other conditions  in `authorization`
    /// may apply to invalidate the grant)
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// GrantAuthorization extends a grant with both the addresses of the grantee and granter.
/// It is used in genesis.proto and query.proto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag = "4")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// GrantQueueItem contains the list of TypeURL of a sdk.Msg.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantQueueItem {
    /// msg_type_urls contains the list of TypeURL of a sdk.Msg.
    #[prost(string, repeated, tag = "1")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventGrant is emitted on Msg/Grant
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrant {
    /// Msg type URL for which an autorization is granted
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag = "3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag = "4")]
    pub grantee: ::prost::alloc::string::String,
}
/// EventRevoke is emitted on Msg/Revoke
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRevoke {
    /// Msg type URL for which an autorization is revoked
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag = "3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag = "4")]
    pub grantee: ::prost::alloc::string::String,
}
/// GenesisState defines the authz module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub authorization: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
/// QueryGrantsRequest is the request type for the Query/Grants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// Optional, msg_type_url, when set, will query only grants matching given msg type.
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGrantsResponse is the response type for the Query/Authorizations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee by granter.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGranterGrantsRequest is the request type for the Query/GranterGrants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranterGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGranterGrantsResponse is the response type for the Query/GranterGrants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranterGrantsResponse {
    /// grants is a list of grants granted by the granter.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGranteeGrantsRequest is the request type for the Query/IssuedGrants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGranteeGrantsResponse is the response type for the Query/GranteeGrants RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsResponse {
    /// grants is a list of grants granted to the grantee.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrant {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub grant: ::core::option::Option<Grant>,
}
/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// Authorization Msg requests to execute. Each msg must implement Authorization interface
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag = "2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantResponse {}
/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevoke {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeResponse {}
include!("cosmos.authz.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
