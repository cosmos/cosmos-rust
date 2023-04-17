// @generated
/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equivocation {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "3")]
    pub power: i64,
    #[prost(string, tag = "4")]
    pub consensus_address: ::prost::alloc::string::String,
}
/// GenesisState defines the evidence module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// evidence defines all the evidence at genesis.
    #[prost(message, repeated, tag = "1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/Evidence RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceRequest {
    /// evidence_hash defines the hash of the requested evidence.
    #[prost(bytes = "vec", tag = "1")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryEvidenceResponse is the response type for the Query/Evidence RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceResponse {
    /// evidence returns the requested evidence.
    #[prost(message, optional, tag = "1")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/AllEvidence RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceResponse {
    /// evidence returns all evidences.
    #[prost(message, repeated, tag = "1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSubmitEvidence represents a message that supports submitting arbitrary
/// Evidence of misbehavior such as equivocation or counterfactual signing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidence {
    #[prost(string, tag = "1")]
    pub submitter: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
/// MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidenceResponse {
    /// hash defines the hash of the evidence.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
include!("cosmos.evidence.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
