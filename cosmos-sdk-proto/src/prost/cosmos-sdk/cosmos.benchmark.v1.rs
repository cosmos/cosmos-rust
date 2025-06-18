// @generated
/// Op is a message describing a benchmark operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Op {
    #[prost(uint64, tag = "1")]
    pub seed: u64,
    #[prost(string, tag = "2")]
    pub actor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub key_length: u64,
    #[prost(uint64, tag = "4")]
    pub value_length: u64,
    #[prost(uint32, tag = "5")]
    pub iterations: u32,
    #[prost(bool, tag = "6")]
    pub delete: bool,
    #[prost(bool, tag = "7")]
    pub exists: bool,
}
impl ::prost::Name for Op {
    const NAME: &'static str = "Op";
    const PACKAGE: &'static str = "cosmos.benchmark.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.benchmark.v1.{}", Self::NAME)
    }
}
/// MsgLoadTestOps defines a message containing a sequence of load test operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLoadTest {
    #[prost(bytes = "vec", tag = "1")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub ops: ::prost::alloc::vec::Vec<Op>,
}
impl ::prost::Name for MsgLoadTest {
    const NAME: &'static str = "MsgLoadTest";
    const PACKAGE: &'static str = "cosmos.benchmark.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.benchmark.v1.{}", Self::NAME)
    }
}
/// MsgLoadTestResponse defines a message containing the results of a load test operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLoadTestResponse {
    #[prost(uint64, tag = "1")]
    pub total_time: u64,
    #[prost(uint64, tag = "2")]
    pub total_errors: u64,
}
impl ::prost::Name for MsgLoadTestResponse {
    const NAME: &'static str = "MsgLoadTestResponse";
    const PACKAGE: &'static str = "cosmos.benchmark.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.benchmark.v1.{}", Self::NAME)
    }
}
include!("cosmos.benchmark.v1.serde.rs");
include!("cosmos.benchmark.v1.tonic.rs");
// @@protoc_insertion_point(module)
