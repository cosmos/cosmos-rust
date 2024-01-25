// @generated
/// Module is the config object for the runtime module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// app_name is the name of the app.
    #[prost(string, tag = "1")]
    pub app_name: ::prost::alloc::string::String,
    /// begin_blockers specifies the module names of begin blockers
    /// to call in the order in which they should be called. If this is left empty
    /// no begin blocker will be registered.
    #[prost(string, repeated, tag = "2")]
    pub begin_blockers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// end_blockers specifies the module names of the end blockers
    /// to call in the order in which they should be called. If this is left empty
    /// no end blocker will be registered.
    #[prost(string, repeated, tag = "3")]
    pub end_blockers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// init_genesis specifies the module names of init genesis functions
    /// to call in the order in which they should be called. If this is left empty
    /// no init genesis function will be registered.
    #[prost(string, repeated, tag = "4")]
    pub init_genesis: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// export_genesis specifies the order in which to export module genesis data.
    /// If this is left empty, the init_genesis order will be used for export genesis
    /// if it is specified.
    #[prost(string, repeated, tag = "5")]
    pub export_genesis: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// override_store_keys is an optional list of overrides for the module store keys
    /// to be used in keeper construction.
    #[prost(message, repeated, tag = "6")]
    pub override_store_keys: ::prost::alloc::vec::Vec<StoreKeyConfig>,
}
/// StoreKeyConfig may be supplied to override the default module store key, which
/// is the module name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKeyConfig {
    /// name of the module to override the store key of
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    /// the kv store key to use instead of the module name.
    #[prost(string, tag = "2")]
    pub kv_store_key: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
