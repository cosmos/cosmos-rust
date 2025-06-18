// @generated
/// Module is the config object of the epochs module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.epochs.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.epochs.module.v1.{}", Self::NAME)
    }
}
include!("cosmos.epochs.module.v1.serde.rs");
// @@protoc_insertion_point(module)
