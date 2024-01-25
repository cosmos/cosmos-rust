// @generated
/// Module is the config object of the slashing module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
