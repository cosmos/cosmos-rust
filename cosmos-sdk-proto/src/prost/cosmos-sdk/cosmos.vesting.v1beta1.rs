// @generated
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(int64, tag = "4")]
    pub end_time: i64,
    #[prost(bool, tag = "5")]
    pub delayed: bool,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreateVestingAccount response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccountResponse {}
/// BaseVestingAccount implements the VestingAccount interface. It contains all
/// the necessary fields needed for any vesting account implementation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::auth::v1beta1::BaseAccount>,
    #[prost(message, repeated, tag = "2")]
    pub original_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub delegated_free: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub delegated_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    pub end_time: i64,
}
/// ContinuousVestingAccount implements the VestingAccount interface. It
/// continuously vests by unlocking coins linearly with respect to time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    #[prost(int64, tag = "2")]
    pub start_time: i64,
}
/// DelayedVestingAccount implements the VestingAccount interface. It vests all
/// coins after a specific time, but non prior. In other words, it keeps them
/// locked until a specified time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayedVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// Period defines a length of time and amount of coins that will vest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    #[prost(int64, tag = "1")]
    pub length: i64,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// PeriodicVestingAccount implements the VestingAccount interface. It
/// periodically vests by unlocking coins during each specified period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodicVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// PermanentLockedAccount implements the VestingAccount interface. It does
/// not ever release coins, locking them indefinitely. Coins in this account can
/// still be used for delegating and for governance votes even while locked.
///
/// Since: cosmos-sdk 0.43
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermanentLockedAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
include!("cosmos.vesting.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
