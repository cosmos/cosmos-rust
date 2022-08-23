/// EpochInfo is a struct that describes the data going into
/// a timer defined by the x/epochs module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInfo {
    /// identifier is a unique reference to this particular timer.
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    /// start_time is the time at which the timer first ever ticks.
    /// If start_time is in the future, the epoch will not begin until the start
    /// time.
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// duration is the time in between epoch ticks.
    /// In order for intended behavior to be met, duration should
    /// be greater than the chains expected block time.
    /// Duration must be non-zero.
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// current_epoch is the current epoch number, or in other words,
    /// how many times has the timer 'ticked'.
    /// The first tick (current_epoch=1) is defined as
    /// the first block whose blocktime is greater than the EpochInfo start_time.
    #[prost(int64, tag="4")]
    pub current_epoch: i64,
    /// current_epoch_start_time describes the start time of the current timer
    /// interval. The interval is (current_epoch_start_time,
    /// current_epoch_start_time + duration] When the timer ticks, this is set to
    /// current_epoch_start_time = last_epoch_start_time + duration only one timer
    /// tick for a given identifier can occur per block.
    ///
    /// NOTE! The current_epoch_start_time may diverge significantly from the
    /// wall-clock time the epoch began at. Wall-clock time of epoch start may be
    /// >> current_epoch_start_time. Suppose current_epoch_start_time = 10,
    /// duration = 5. Suppose the chain goes offline at t=14, and comes back online
    /// at t=30, and produces blocks at every successive time. (t=31, 32, etc.)
    /// * The t=30 block will start the epoch for (10, 15]
    /// * The t=31 block will start the epoch for (15, 20]
    /// * The t=32 block will start the epoch for (20, 25]
    /// * The t=33 block will start the epoch for (25, 30]
    /// * The t=34 block will start the epoch for (30, 35]
    /// * The **t=36** block will start the epoch for (35, 40]
    #[prost(message, optional, tag="5")]
    pub current_epoch_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// epoch_counting_started is a boolean, that indicates whether this
    /// epoch timer has began yet.
    #[prost(bool, tag="6")]
    pub epoch_counting_started: bool,
    /// current_epoch_start_height is the block height at which the current epoch
    /// started. (The block height at which the timer last ticked)
    #[prost(int64, tag="8")]
    pub current_epoch_start_height: i64,
}
/// GenesisState defines the epochs module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag="1")]
    pub current_epoch: i64,
}
