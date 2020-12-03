// This file is copied from http://github.com/tendermint/abci
// NOTE: When using custom types, mind the warnings.
// https://github.com/gogo/protobuf/blob/master/custom_types.md#warnings-and-issues

//----------------------------------------
// Request types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15")]
    pub value: ::std::option::Option<request::Value>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Echo(super::RequestEcho),
        #[prost(message, tag="2")]
        Flush(super::RequestFlush),
        #[prost(message, tag="3")]
        Info(super::RequestInfo),
        #[prost(message, tag="4")]
        SetOption(super::RequestSetOption),
        #[prost(message, tag="5")]
        InitChain(super::RequestInitChain),
        #[prost(message, tag="6")]
        Query(super::RequestQuery),
        #[prost(message, tag="7")]
        BeginBlock(super::RequestBeginBlock),
        #[prost(message, tag="8")]
        CheckTx(super::RequestCheckTx),
        #[prost(message, tag="9")]
        DeliverTx(super::RequestDeliverTx),
        #[prost(message, tag="10")]
        EndBlock(super::RequestEndBlock),
        #[prost(message, tag="11")]
        Commit(super::RequestCommit),
        #[prost(message, tag="12")]
        ListSnapshots(super::RequestListSnapshots),
        #[prost(message, tag="13")]
        OfferSnapshot(super::RequestOfferSnapshot),
        #[prost(message, tag="14")]
        LoadSnapshotChunk(super::RequestLoadSnapshotChunk),
        #[prost(message, tag="15")]
        ApplySnapshotChunk(super::RequestApplySnapshotChunk),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEcho {
    #[prost(string, tag="1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFlush {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    #[prost(string, tag="1")]
    pub version: std::string::String,
    #[prost(uint64, tag="2")]
    pub block_version: u64,
    #[prost(uint64, tag="3")]
    pub p2p_version: u64,
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSetOption {
    #[prost(string, tag="1")]
    pub key: std::string::String,
    #[prost(string, tag="2")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInitChain {
    #[prost(message, optional, tag="1")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub consensus_params: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="4")]
    pub validators: ::std::vec::Vec<ValidatorUpdate>,
    #[prost(bytes, tag="5")]
    pub app_state_bytes: std::vec::Vec<u8>,
    #[prost(int64, tag="6")]
    pub initial_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuery {
    #[prost(bytes, tag="1")]
    pub data: std::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub path: std::string::String,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(bool, tag="4")]
    pub prove: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBeginBlock {
    #[prost(bytes, tag="1")]
    pub hash: std::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub header: ::std::option::Option<super::types::Header>,
    #[prost(message, optional, tag="3")]
    pub last_commit_info: ::std::option::Option<LastCommitInfo>,
    #[prost(message, repeated, tag="4")]
    pub byzantine_validators: ::std::vec::Vec<Evidence>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCheckTx {
    #[prost(bytes, tag="1")]
    pub tx: std::vec::Vec<u8>,
    #[prost(enumeration="CheckTxType", tag="2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDeliverTx {
    #[prost(bytes, tag="1")]
    pub tx: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEndBlock {
    #[prost(int64, tag="1")]
    pub height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCommit {
}
/// lists available snapshots
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListSnapshots {
}
/// offers a snapshot to the application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOfferSnapshot {
    /// snapshot offered by peers
    #[prost(message, optional, tag="1")]
    pub snapshot: ::std::option::Option<Snapshot>,
    /// light client-verified app hash for snapshot height
    #[prost(bytes, tag="2")]
    pub app_hash: std::vec::Vec<u8>,
}
/// loads a snapshot chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLoadSnapshotChunk {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint32, tag="2")]
    pub format: u32,
    #[prost(uint32, tag="3")]
    pub chunk: u32,
}
/// Applies a snapshot chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestApplySnapshotChunk {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(bytes, tag="2")]
    pub chunk: std::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub sender: std::string::String,
}
//----------------------------------------
// Response types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16")]
    pub value: ::std::option::Option<response::Value>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Exception(super::ResponseException),
        #[prost(message, tag="2")]
        Echo(super::ResponseEcho),
        #[prost(message, tag="3")]
        Flush(super::ResponseFlush),
        #[prost(message, tag="4")]
        Info(super::ResponseInfo),
        #[prost(message, tag="5")]
        SetOption(super::ResponseSetOption),
        #[prost(message, tag="6")]
        InitChain(super::ResponseInitChain),
        #[prost(message, tag="7")]
        Query(super::ResponseQuery),
        #[prost(message, tag="8")]
        BeginBlock(super::ResponseBeginBlock),
        #[prost(message, tag="9")]
        CheckTx(super::ResponseCheckTx),
        #[prost(message, tag="10")]
        DeliverTx(super::ResponseDeliverTx),
        #[prost(message, tag="11")]
        EndBlock(super::ResponseEndBlock),
        #[prost(message, tag="12")]
        Commit(super::ResponseCommit),
        #[prost(message, tag="13")]
        ListSnapshots(super::ResponseListSnapshots),
        #[prost(message, tag="14")]
        OfferSnapshot(super::ResponseOfferSnapshot),
        #[prost(message, tag="15")]
        LoadSnapshotChunk(super::ResponseLoadSnapshotChunk),
        #[prost(message, tag="16")]
        ApplySnapshotChunk(super::ResponseApplySnapshotChunk),
    }
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseException {
    #[prost(string, tag="1")]
    pub error: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEcho {
    #[prost(string, tag="1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlush {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInfo {
    #[prost(string, tag="1")]
    pub data: std::string::String,
    #[prost(string, tag="2")]
    pub version: std::string::String,
    #[prost(uint64, tag="3")]
    pub app_version: u64,
    #[prost(int64, tag="4")]
    pub last_block_height: i64,
    #[prost(bytes, tag="5")]
    pub last_block_app_hash: std::vec::Vec<u8>,
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSetOption {
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// bytes data = 2;
    #[prost(string, tag="3")]
    pub log: std::string::String,
    #[prost(string, tag="4")]
    pub info: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInitChain {
    #[prost(message, optional, tag="1")]
    pub consensus_params: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="2")]
    pub validators: ::std::vec::Vec<ValidatorUpdate>,
    #[prost(bytes, tag="3")]
    pub app_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuery {
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub index: i64,
    #[prost(bytes, tag="6")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="7")]
    pub value: std::vec::Vec<u8>,
    #[prost(message, optional, tag="8")]
    pub proof_ops: ::std::option::Option<super::crypto::ProofOps>,
    #[prost(int64, tag="9")]
    pub height: i64,
    #[prost(string, tag="10")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBeginBlock {
    #[prost(message, repeated, tag="1")]
    pub events: ::std::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCheckTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::std::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDeliverTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::std::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEndBlock {
    #[prost(message, repeated, tag="1")]
    pub validator_updates: ::std::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="2")]
    pub consensus_param_updates: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="3")]
    pub events: ::std::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCommit {
    /// reserve 1
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub retain_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListSnapshots {
    #[prost(message, repeated, tag="1")]
    pub snapshots: ::std::vec::Vec<Snapshot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOfferSnapshot {
    #[prost(enumeration="response_offer_snapshot::Result", tag="1")]
    pub result: i32,
}
pub mod response_offer_snapshot {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result, abort all snapshot restoration
        Unknown = 0,
        /// Snapshot accepted, apply chunks
        Accept = 1,
        /// Abort all snapshot restoration
        Abort = 2,
        /// Reject this specific snapshot, try others
        Reject = 3,
        /// Reject all snapshots of this format, try others
        RejectFormat = 4,
        /// Reject all snapshots from the sender(s), try others
        RejectSender = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLoadSnapshotChunk {
    #[prost(bytes, tag="1")]
    pub chunk: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseApplySnapshotChunk {
    #[prost(enumeration="response_apply_snapshot_chunk::Result", tag="1")]
    pub result: i32,
    /// Chunks to refetch and reapply
    #[prost(uint32, repeated, tag="2")]
    pub refetch_chunks: ::std::vec::Vec<u32>,
    /// Chunk senders to reject and ban
    #[prost(string, repeated, tag="3")]
    pub reject_senders: ::std::vec::Vec<std::string::String>,
}
pub mod response_apply_snapshot_chunk {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result, abort all snapshot restoration
        Unknown = 0,
        /// Chunk successfully accepted
        Accept = 1,
        /// Abort all snapshot restoration
        Abort = 2,
        /// Retry chunk (combine with refetch and reject)
        Retry = 3,
        /// Retry snapshot (combine with refetch and reject)
        RetrySnapshot = 4,
        /// Reject this snapshot, try others
        RejectSnapshot = 5,
    }
}
//----------------------------------------
// Misc.

/// ConsensusParams contains all consensus-relevant parameters
/// that can be adjusted by the abci app
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParams {
    #[prost(message, optional, tag="1")]
    pub block: ::std::option::Option<BlockParams>,
    #[prost(message, optional, tag="2")]
    pub evidence: ::std::option::Option<super::types::EvidenceParams>,
    #[prost(message, optional, tag="3")]
    pub validator: ::std::option::Option<super::types::ValidatorParams>,
    #[prost(message, optional, tag="4")]
    pub version: ::std::option::Option<super::types::VersionParams>,
}
/// BlockParams contains limits on the block size.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParams {
    /// Note: must be greater than 0
    #[prost(int64, tag="1")]
    pub max_bytes: i64,
    /// Note: must be greater or equal to -1
    #[prost(int64, tag="2")]
    pub max_gas: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastCommitInfo {
    #[prost(int32, tag="1")]
    pub round: i32,
    #[prost(message, repeated, tag="2")]
    pub votes: ::std::vec::Vec<VoteInfo>,
}
/// Event allows application developers to attach additional information to
/// ResponseBeginBlock, ResponseEndBlock, ResponseCheckTx and ResponseDeliverTx.
/// Later, transactions may be queried using these events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::std::vec::Vec<EventAttribute>,
}
/// EventAttribute is a single key-value pair, associated with an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttribute {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub value: std::vec::Vec<u8>,
    /// nondeterministic
    #[prost(bool, tag="3")]
    pub index: bool,
}
/// TxResult contains results of executing the transaction.
///
/// One usage is indexing transaction results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(int64, tag="1")]
    pub height: i64,
    #[prost(uint32, tag="2")]
    pub index: u32,
    #[prost(bytes, tag="3")]
    pub tx: std::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub result: ::std::option::Option<ResponseDeliverTx>,
}
//----------------------------------------
// Blockchain Types

/// Validator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// The first 20 bytes of SHA256(public key)
    #[prost(bytes, tag="1")]
    pub address: std::vec::Vec<u8>,
    /// PubKey pub_key = 2 [(gogoproto.nullable)=false];
    ///
    /// The voting power
    #[prost(int64, tag="3")]
    pub power: i64,
}
/// ValidatorUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUpdate {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::std::option::Option<super::crypto::PublicKey>,
    #[prost(int64, tag="2")]
    pub power: i64,
}
/// VoteInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    #[prost(message, optional, tag="1")]
    pub validator: ::std::option::Option<Validator>,
    #[prost(bool, tag="2")]
    pub signed_last_block: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evidence {
    #[prost(enumeration="EvidenceType", tag="1")]
    pub r#type: i32,
    /// The offending validator
    #[prost(message, optional, tag="2")]
    pub validator: ::std::option::Option<Validator>,
    /// The height when the offense occurred
    #[prost(int64, tag="3")]
    pub height: i64,
    /// The corresponding time where the offense occurred
    #[prost(message, optional, tag="4")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    /// Total voting power of the validator set in case the ABCI application does
    /// not store historical validators.
    /// https://github.com/tendermint/tendermint/issues/4581
    #[prost(int64, tag="5")]
    pub total_voting_power: i64,
}
//----------------------------------------
// State Sync Types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The height at which the snapshot was taken
    #[prost(uint64, tag="1")]
    pub height: u64,
    /// The application-specific snapshot format
    #[prost(uint32, tag="2")]
    pub format: u32,
    /// Number of chunks in the snapshot
    #[prost(uint32, tag="3")]
    pub chunks: u32,
    /// Arbitrary snapshot hash, equal only if identical
    #[prost(bytes, tag="4")]
    pub hash: std::vec::Vec<u8>,
    /// Arbitrary application metadata
    #[prost(bytes, tag="5")]
    pub metadata: std::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckTxType {
    New = 0,
    Recheck = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvidenceType {
    Unknown = 0,
    DuplicateVote = 1,
    LightClientAttack = 2,
}
# [doc = r" Generated client implementations."] pub mod abci_application_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; pub struct AbciApplicationClient < T > { inner : tonic :: client :: Grpc < T > , } impl AbciApplicationClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > AbciApplicationClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } pub async fn echo (& mut self , request : impl tonic :: IntoRequest < super :: RequestEcho > ,) -> Result < tonic :: Response < super :: ResponseEcho > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/Echo") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn flush (& mut self , request : impl tonic :: IntoRequest < super :: RequestFlush > ,) -> Result < tonic :: Response < super :: ResponseFlush > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/Flush") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn info (& mut self , request : impl tonic :: IntoRequest < super :: RequestInfo > ,) -> Result < tonic :: Response < super :: ResponseInfo > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/Info") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn set_option (& mut self , request : impl tonic :: IntoRequest < super :: RequestSetOption > ,) -> Result < tonic :: Response < super :: ResponseSetOption > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/SetOption") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn deliver_tx (& mut self , request : impl tonic :: IntoRequest < super :: RequestDeliverTx > ,) -> Result < tonic :: Response < super :: ResponseDeliverTx > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/DeliverTx") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn check_tx (& mut self , request : impl tonic :: IntoRequest < super :: RequestCheckTx > ,) -> Result < tonic :: Response < super :: ResponseCheckTx > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/CheckTx") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn query (& mut self , request : impl tonic :: IntoRequest < super :: RequestQuery > ,) -> Result < tonic :: Response < super :: ResponseQuery > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/Query") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn commit (& mut self , request : impl tonic :: IntoRequest < super :: RequestCommit > ,) -> Result < tonic :: Response < super :: ResponseCommit > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/Commit") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn init_chain (& mut self , request : impl tonic :: IntoRequest < super :: RequestInitChain > ,) -> Result < tonic :: Response < super :: ResponseInitChain > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/InitChain") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn begin_block (& mut self , request : impl tonic :: IntoRequest < super :: RequestBeginBlock > ,) -> Result < tonic :: Response < super :: ResponseBeginBlock > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/BeginBlock") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn end_block (& mut self , request : impl tonic :: IntoRequest < super :: RequestEndBlock > ,) -> Result < tonic :: Response < super :: ResponseEndBlock > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/EndBlock") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn list_snapshots (& mut self , request : impl tonic :: IntoRequest < super :: RequestListSnapshots > ,) -> Result < tonic :: Response < super :: ResponseListSnapshots > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/ListSnapshots") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn offer_snapshot (& mut self , request : impl tonic :: IntoRequest < super :: RequestOfferSnapshot > ,) -> Result < tonic :: Response < super :: ResponseOfferSnapshot > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/OfferSnapshot") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn load_snapshot_chunk (& mut self , request : impl tonic :: IntoRequest < super :: RequestLoadSnapshotChunk > ,) -> Result < tonic :: Response < super :: ResponseLoadSnapshotChunk > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/LoadSnapshotChunk") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn apply_snapshot_chunk (& mut self , request : impl tonic :: IntoRequest < super :: RequestApplySnapshotChunk > ,) -> Result < tonic :: Response < super :: ResponseApplySnapshotChunk > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/tendermint.abci.ABCIApplication/ApplySnapshotChunk") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for AbciApplicationClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for AbciApplicationClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "AbciApplicationClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod abci_application_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with AbciApplicationServer."] # [async_trait] pub trait AbciApplication : Send + Sync + 'static { async fn echo (& self , request : tonic :: Request < super :: RequestEcho >) -> Result < tonic :: Response < super :: ResponseEcho > , tonic :: Status > ; async fn flush (& self , request : tonic :: Request < super :: RequestFlush >) -> Result < tonic :: Response < super :: ResponseFlush > , tonic :: Status > ; async fn info (& self , request : tonic :: Request < super :: RequestInfo >) -> Result < tonic :: Response < super :: ResponseInfo > , tonic :: Status > ; async fn set_option (& self , request : tonic :: Request < super :: RequestSetOption >) -> Result < tonic :: Response < super :: ResponseSetOption > , tonic :: Status > ; async fn deliver_tx (& self , request : tonic :: Request < super :: RequestDeliverTx >) -> Result < tonic :: Response < super :: ResponseDeliverTx > , tonic :: Status > ; async fn check_tx (& self , request : tonic :: Request < super :: RequestCheckTx >) -> Result < tonic :: Response < super :: ResponseCheckTx > , tonic :: Status > ; async fn query (& self , request : tonic :: Request < super :: RequestQuery >) -> Result < tonic :: Response < super :: ResponseQuery > , tonic :: Status > ; async fn commit (& self , request : tonic :: Request < super :: RequestCommit >) -> Result < tonic :: Response < super :: ResponseCommit > , tonic :: Status > ; async fn init_chain (& self , request : tonic :: Request < super :: RequestInitChain >) -> Result < tonic :: Response < super :: ResponseInitChain > , tonic :: Status > ; async fn begin_block (& self , request : tonic :: Request < super :: RequestBeginBlock >) -> Result < tonic :: Response < super :: ResponseBeginBlock > , tonic :: Status > ; async fn end_block (& self , request : tonic :: Request < super :: RequestEndBlock >) -> Result < tonic :: Response < super :: ResponseEndBlock > , tonic :: Status > ; async fn list_snapshots (& self , request : tonic :: Request < super :: RequestListSnapshots >) -> Result < tonic :: Response < super :: ResponseListSnapshots > , tonic :: Status > ; async fn offer_snapshot (& self , request : tonic :: Request < super :: RequestOfferSnapshot >) -> Result < tonic :: Response < super :: ResponseOfferSnapshot > , tonic :: Status > ; async fn load_snapshot_chunk (& self , request : tonic :: Request < super :: RequestLoadSnapshotChunk >) -> Result < tonic :: Response < super :: ResponseLoadSnapshotChunk > , tonic :: Status > ; async fn apply_snapshot_chunk (& self , request : tonic :: Request < super :: RequestApplySnapshotChunk >) -> Result < tonic :: Response < super :: ResponseApplySnapshotChunk > , tonic :: Status > ; } # [derive (Debug)] pub struct AbciApplicationServer < T : AbciApplication > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : AbciApplication > AbciApplicationServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for AbciApplicationServer < T > where T : AbciApplication , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/tendermint.abci.ABCIApplication/Echo" => { # [allow (non_camel_case_types)] struct EchoSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestEcho > for EchoSvc < T > { type Response = super :: ResponseEcho ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestEcho >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . echo (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = EchoSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/Flush" => { # [allow (non_camel_case_types)] struct FlushSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestFlush > for FlushSvc < T > { type Response = super :: ResponseFlush ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestFlush >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . flush (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = FlushSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/Info" => { # [allow (non_camel_case_types)] struct InfoSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestInfo > for InfoSvc < T > { type Response = super :: ResponseInfo ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestInfo >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . info (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = InfoSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/SetOption" => { # [allow (non_camel_case_types)] struct SetOptionSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestSetOption > for SetOptionSvc < T > { type Response = super :: ResponseSetOption ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestSetOption >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . set_option (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = SetOptionSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/DeliverTx" => { # [allow (non_camel_case_types)] struct DeliverTxSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestDeliverTx > for DeliverTxSvc < T > { type Response = super :: ResponseDeliverTx ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestDeliverTx >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . deliver_tx (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = DeliverTxSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/CheckTx" => { # [allow (non_camel_case_types)] struct CheckTxSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestCheckTx > for CheckTxSvc < T > { type Response = super :: ResponseCheckTx ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestCheckTx >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . check_tx (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = CheckTxSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/Query" => { # [allow (non_camel_case_types)] struct QuerySvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestQuery > for QuerySvc < T > { type Response = super :: ResponseQuery ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestQuery >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . query (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = QuerySvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/Commit" => { # [allow (non_camel_case_types)] struct CommitSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestCommit > for CommitSvc < T > { type Response = super :: ResponseCommit ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestCommit >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . commit (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = CommitSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/InitChain" => { # [allow (non_camel_case_types)] struct InitChainSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestInitChain > for InitChainSvc < T > { type Response = super :: ResponseInitChain ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestInitChain >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . init_chain (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = InitChainSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/BeginBlock" => { # [allow (non_camel_case_types)] struct BeginBlockSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestBeginBlock > for BeginBlockSvc < T > { type Response = super :: ResponseBeginBlock ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestBeginBlock >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . begin_block (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = BeginBlockSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/EndBlock" => { # [allow (non_camel_case_types)] struct EndBlockSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestEndBlock > for EndBlockSvc < T > { type Response = super :: ResponseEndBlock ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestEndBlock >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . end_block (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = EndBlockSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/ListSnapshots" => { # [allow (non_camel_case_types)] struct ListSnapshotsSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestListSnapshots > for ListSnapshotsSvc < T > { type Response = super :: ResponseListSnapshots ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestListSnapshots >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . list_snapshots (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ListSnapshotsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/OfferSnapshot" => { # [allow (non_camel_case_types)] struct OfferSnapshotSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestOfferSnapshot > for OfferSnapshotSvc < T > { type Response = super :: ResponseOfferSnapshot ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestOfferSnapshot >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . offer_snapshot (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = OfferSnapshotSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/LoadSnapshotChunk" => { # [allow (non_camel_case_types)] struct LoadSnapshotChunkSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestLoadSnapshotChunk > for LoadSnapshotChunkSvc < T > { type Response = super :: ResponseLoadSnapshotChunk ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestLoadSnapshotChunk >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . load_snapshot_chunk (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = LoadSnapshotChunkSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/tendermint.abci.ABCIApplication/ApplySnapshotChunk" => { # [allow (non_camel_case_types)] struct ApplySnapshotChunkSvc < T : AbciApplication > (pub Arc < T >) ; impl < T : AbciApplication > tonic :: server :: UnaryService < super :: RequestApplySnapshotChunk > for ApplySnapshotChunkSvc < T > { type Response = super :: ResponseApplySnapshotChunk ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: RequestApplySnapshotChunk >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . apply_snapshot_chunk (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ApplySnapshotChunkSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : AbciApplication > Clone for AbciApplicationServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : AbciApplication > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : AbciApplication > tonic :: transport :: NamedService for AbciApplicationServer < T > { const NAME : & 'static str = "tendermint.abci.ABCIApplication" ; } }