/// Module is the config object of the bank module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// blocked_module_accounts configures exceptional module accounts which should be blocked from receiving funds.
    /// If left empty it defaults to the list of account names supplied in the auth module configuration as
    /// module_account_permissions
    #[prost(string, repeated, tag = "1")]
    pub blocked_module_accounts_override: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
