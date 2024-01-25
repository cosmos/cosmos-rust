// @generated
/// Module is the config object of the distribution module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, tag = "1")]
    pub fee_collector_name: ::prost::alloc::string::String,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
