// @generated
/// Module is the config object for the auth module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// bech32_prefix is the bech32 account prefix for the app.
    #[prost(string, tag = "1")]
    pub bech32_prefix: ::prost::alloc::string::String,
    /// module_account_permissions are module account permissions.
    #[prost(message, repeated, tag = "2")]
    pub module_account_permissions: ::prost::alloc::vec::Vec<ModuleAccountPermission>,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.auth.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.auth.module.v1.{}", Self::NAME)
    }
}
/// ModuleAccountPermission represents permissions for a module account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccountPermission {
    /// account is the name of the module.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// permissions are the permissions this module has. Currently recognized
    /// values are minter, burner and staking.
    #[prost(string, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for ModuleAccountPermission {
    const NAME: &'static str = "ModuleAccountPermission";
    const PACKAGE: &'static str = "cosmos.auth.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.auth.module.v1.{}", Self::NAME)
    }
}
include!("cosmos.auth.module.v1.serde.rs");
// @@protoc_insertion_point(module)
