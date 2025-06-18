// @generated
/// QueryGetCountRequest defines the request type for querying x/mock count.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetCountRequest {}
impl ::prost::Name for QueryGetCountRequest {
    const NAME: &'static str = "QueryGetCountRequest";
    const PACKAGE: &'static str = "cosmos.counter.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.counter.v1.{}", Self::NAME)
    }
}
/// QueryGetCountResponse defines the response type for querying x/mock count.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetCountResponse {
    #[prost(int64, tag = "1")]
    pub total_count: i64,
}
impl ::prost::Name for QueryGetCountResponse {
    const NAME: &'static str = "QueryGetCountResponse";
    const PACKAGE: &'static str = "cosmos.counter.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.counter.v1.{}", Self::NAME)
    }
}
/// MsgIncreaseCounter defines a count Msg service counter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreaseCounter {
    /// signer is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// count is the number of times to increment the counter.
    #[prost(int64, tag = "2")]
    pub count: i64,
}
impl ::prost::Name for MsgIncreaseCounter {
    const NAME: &'static str = "MsgIncreaseCounter";
    const PACKAGE: &'static str = "cosmos.counter.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.counter.v1.{}", Self::NAME)
    }
}
/// MsgIncreaseCountResponse is the Msg/Counter response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreaseCountResponse {
    /// new_count is the number of times the counter was incremented.
    #[prost(int64, tag = "1")]
    pub new_count: i64,
}
impl ::prost::Name for MsgIncreaseCountResponse {
    const NAME: &'static str = "MsgIncreaseCountResponse";
    const PACKAGE: &'static str = "cosmos.counter.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.counter.v1.{}", Self::NAME)
    }
}
include!("cosmos.counter.v1.serde.rs");
include!("cosmos.counter.v1.tonic.rs");
// @@protoc_insertion_point(module)
