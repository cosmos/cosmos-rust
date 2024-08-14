// @generated
/// Module is the config object of the params module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.params.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.params.module.v1.{}", Self::NAME)
    }
}
include!("cosmos.params.module.v1.serde.rs");
// @@protoc_insertion_point(module)
