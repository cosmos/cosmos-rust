/// Module is the config object of the staking module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// hooks_order specifies the order of staking hooks and should be a list
    /// of module names which provide a staking hooks instance. If no order is
    /// provided, then hooks will be applied in alphabetical order of module names.
    #[prost(string, repeated, tag = "1")]
    pub hooks_order: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
