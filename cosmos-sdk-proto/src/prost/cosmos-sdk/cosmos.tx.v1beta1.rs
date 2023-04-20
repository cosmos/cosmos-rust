// @generated
/// Tx is the standard type used for broadcasting transactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    /// body is the processable content of the transaction
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<TxBody>,
    /// auth_info is the authorization related content of the transaction,
    /// specifically signers, signer modes and fee
    #[prost(message, optional, tag = "2")]
    pub auth_info: ::core::option::Option<AuthInfo>,
    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// TxRaw is a variant of Tx that pins the signer's exact binary representation
/// of body and auth_info. This is used for signing, broadcasting and
/// verification. The binary `serialize(tx: TxRaw)` is stored in Tendermint and
/// the hash `sha256(serialize(tx: TxRaw))` becomes the "txhash", commonly used
/// as the transaction ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRaw {
    /// body_bytes is a protobuf serialization of a TxBody that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "1")]
    pub body_bytes: ::prost::alloc::vec::Vec<u8>,
    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "2")]
    pub auth_info_bytes: ::prost::alloc::vec::Vec<u8>,
    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// SignDoc is the type used for generating sign bytes for SIGN_MODE_DIRECT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignDoc {
    /// body_bytes is protobuf serialization of a TxBody that matches the
    /// representation in TxRaw.
    #[prost(bytes = "vec", tag = "1")]
    pub body_bytes: ::prost::alloc::vec::Vec<u8>,
    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in TxRaw.
    #[prost(bytes = "vec", tag = "2")]
    pub auth_info_bytes: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the unique identifier of the chain this transaction targets.
    /// It prevents signed transactions from being used on another chain by an
    /// attacker
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    /// account_number is the account number of the account in state
    #[prost(uint64, tag = "4")]
    pub account_number: u64,
}
/// SignDocDirectAux is the type used for generating sign bytes for
/// SIGN_MODE_DIRECT_AUX.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignDocDirectAux {
    /// body_bytes is protobuf serialization of a TxBody that matches the
    /// representation in TxRaw.
    #[prost(bytes = "vec", tag = "1")]
    pub body_bytes: ::prost::alloc::vec::Vec<u8>,
    /// public_key is the public key of the signing account.
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<::prost_types::Any>,
    /// chain_id is the identifier of the chain this transaction targets.
    /// It prevents signed transactions from being used on another chain by an
    /// attacker.
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    /// account_number is the account number of the account in state.
    #[prost(uint64, tag = "4")]
    pub account_number: u64,
    /// sequence is the sequence number of the signing account.
    #[prost(uint64, tag = "5")]
    pub sequence: u64,
    /// Tip is the optional tip used for transactions fees paid in another denom.
    /// It should be left empty if the signer is not the tipper for this
    /// transaction.
    ///
    /// This field is ignored if the chain didn't enable tips, i.e. didn't add the
    /// `TipDecorator` in its posthandler.
    #[prost(message, optional, tag = "6")]
    pub tip: ::core::option::Option<Tip>,
}
/// TxBody is the body of a transaction that all signers sign over.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBody {
    /// messages is a list of messages to be executed. The required signers of
    /// those messages define the number and order of elements in AuthInfo's
    /// signer_infos and Tx's signatures. Each required signer address is added to
    /// the list only the first time it occurs.
    /// By convention, the first required signer (usually from the first message)
    /// is referred to as the primary signer and pays the fee for the whole
    /// transaction.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// memo is any arbitrary note/comment to be added to the transaction.
    /// WARNING: in clients, any publicly exposed text should not be called memo,
    /// but should be called `note` instead (see <https://github.com/cosmos/cosmos-sdk/issues/9122>).
    #[prost(string, tag = "2")]
    pub memo: ::prost::alloc::string::String,
    /// timeout is the block height after which this transaction will not
    /// be processed by the chain
    #[prost(uint64, tag = "3")]
    pub timeout_height: u64,
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, the transaction will be rejected
    #[prost(message, repeated, tag = "1023")]
    pub extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, they will be ignored
    #[prost(message, repeated, tag = "2047")]
    pub non_critical_extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// AuthInfo describes the fee and signer modes that are used to sign a
/// transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthInfo {
    /// signer_infos defines the signing modes for the required signers. The number
    /// and order of elements must match the required signers from TxBody's
    /// messages. The first element is the primary signer and the one which pays
    /// the fee.
    #[prost(message, repeated, tag = "1")]
    pub signer_infos: ::prost::alloc::vec::Vec<SignerInfo>,
    /// Fee is the fee and gas limit for the transaction. The first signer is the
    /// primary signer and the one which pays the fee. The fee can be calculated
    /// based on the cost of evaluating the body and doing signature verification
    /// of the signers. This can be estimated via simulation.
    #[prost(message, optional, tag = "2")]
    pub fee: ::core::option::Option<Fee>,
    /// Tip is the optional tip used for transactions fees paid in another denom.
    ///
    /// This field is ignored if the chain didn't enable tips, i.e. didn't add the
    /// `TipDecorator` in its posthandler.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(message, optional, tag = "3")]
    pub tip: ::core::option::Option<Tip>,
}
/// SignerInfo describes the public key and signing mode of a single top-level
/// signer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerInfo {
    /// public_key is the public key of the signer. It is optional for accounts
    /// that already exist in state. If unset, the verifier can use the required \
    /// signer address for this position and lookup the public key.
    #[prost(message, optional, tag = "1")]
    pub public_key: ::core::option::Option<::prost_types::Any>,
    /// mode_info describes the signing mode of the signer and is a nested
    /// structure to support nested multisig pubkey's
    #[prost(message, optional, tag = "2")]
    pub mode_info: ::core::option::Option<ModeInfo>,
    /// sequence is the sequence of the account, which describes the
    /// number of committed transactions signed by a given address. It is used to
    /// prevent replay attacks.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
/// ModeInfo describes the signing mode of a single or nested multisig signer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModeInfo {
    /// sum is the oneof that specifies whether this represents a single or nested
    /// multisig signer
    #[prost(oneof = "mode_info::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<mode_info::Sum>,
}
/// Nested message and enum types in `ModeInfo`.
pub mod mode_info {
    /// Single is the mode info for a single signer. It is structured as a message
    /// to allow for additional fields such as locale for SIGN_MODE_TEXTUAL in the
    /// future
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Single {
        /// mode is the signing mode of the single signer
        #[prost(enumeration = "super::super::signing::v1beta1::SignMode", tag = "1")]
        pub mode: i32,
    }
    /// Multi is the mode info for a multisig public key
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Multi {
        /// bitarray specifies which keys within the multisig are signing
        #[prost(message, optional, tag = "1")]
        pub bitarray:
            ::core::option::Option<super::super::super::crypto::multisig::v1beta1::CompactBitArray>,
        /// mode_infos is the corresponding modes of the signers of the multisig
        /// which could include nested multisig public keys
        #[prost(message, repeated, tag = "2")]
        pub mode_infos: ::prost::alloc::vec::Vec<super::ModeInfo>,
    }
    /// sum is the oneof that specifies whether this represents a single or nested
    /// multisig signer
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        /// single represents a single signer
        #[prost(message, tag = "1")]
        Single(Single),
        /// multi represents a nested multisig signer
        #[prost(message, tag = "2")]
        Multi(Multi),
    }
}
/// Fee includes the amount of coins paid in fees and the maximum
/// gas to be used by the transaction. The ratio yields an effective "gasprice",
/// which must be above some miminum to be accepted into the mempool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    /// amount is the amount of coins to be paid as a fee
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// gas_limit is the maximum gas that can be used in transaction processing
    /// before an out of gas error occurs
    #[prost(uint64, tag = "2")]
    pub gas_limit: u64,
    /// if unset, the first signer is responsible for paying the fees. If set, the specified account must pay the fees.
    /// the payer must be a tx signer (and thus have signed this field in AuthInfo).
    /// setting this field does *not* change the ordering of required signers for the transaction.
    #[prost(string, tag = "3")]
    pub payer: ::prost::alloc::string::String,
    /// if set, the fee payer (either the first signer or the value of the payer field) requests that a fee grant be used
    /// to pay fees instead of the fee payer's own balance. If an appropriate fee grant does not exist or the chain does
    /// not support fee grants, this will fail
    #[prost(string, tag = "4")]
    pub granter: ::prost::alloc::string::String,
}
/// Tip is the tip used for meta-transactions.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tip {
    /// amount is the amount of the tip
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// tipper is the address of the account paying for the tip
    #[prost(string, tag = "2")]
    pub tipper: ::prost::alloc::string::String,
}
/// AuxSignerData is the intermediary format that an auxiliary signer (e.g. a
/// tipper) builds and sends to the fee payer (who will build and broadcast the
/// actual tx). AuxSignerData is not a valid tx in itself, and will be rejected
/// by the node if sent directly as-is.
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuxSignerData {
    /// address is the bech32-encoded address of the auxiliary signer. If using
    /// AuxSignerData across different chains, the bech32 prefix of the target
    /// chain (where the final transaction is broadcasted) should be used.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// sign_doc is the SIGN_MODE_DIRECT_AUX sign doc that the auxiliary signer
    /// signs. Note: we use the same sign doc even if we're signing with
    /// LEGACY_AMINO_JSON.
    #[prost(message, optional, tag = "2")]
    pub sign_doc: ::core::option::Option<SignDocDirectAux>,
    /// mode is the signing mode of the single signer.
    #[prost(enumeration = "super::signing::v1beta1::SignMode", tag = "3")]
    pub mode: i32,
    /// sig is the signature of the sign doc.
    #[prost(bytes = "vec", tag = "4")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
/// GetTxsEventRequest is the request type for the Service.TxsByEvents
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsEventRequest {
    /// events is the list of transaction event type.
    #[prost(string, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines a pagination for the request.
    /// Deprecated post v0.46.x: use page and limit instead.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    #[prost(enumeration = "OrderBy", tag = "3")]
    pub order_by: i32,
    /// page is the page number to query, starts at 1. If not provided, will default to first page.
    #[prost(uint64, tag = "4")]
    pub page: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    #[prost(uint64, tag = "5")]
    pub limit: u64,
}
/// GetTxsEventResponse is the response type for the Service.TxsByEvents
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsEventResponse {
    /// txs is the list of queried transactions.
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<Tx>,
    /// tx_responses is the list of queried TxResponses.
    #[prost(message, repeated, tag = "2")]
    pub tx_responses: ::prost::alloc::vec::Vec<super::super::base::abci::v1beta1::TxResponse>,
    /// pagination defines a pagination for the response.
    /// Deprecated post v0.46.x: use total instead.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
    /// total is total number of results available
    #[prost(uint64, tag = "4")]
    pub total: u64,
}
/// BroadcastTxRequest is the request type for the Service.BroadcastTxRequest
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxRequest {
    /// tx_bytes is the raw transaction.
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "BroadcastMode", tag = "2")]
    pub mode: i32,
}
/// BroadcastTxResponse is the response type for the
/// Service.BroadcastTx method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxResponse {
    /// tx_response is the queried TxResponses.
    #[prost(message, optional, tag = "1")]
    pub tx_response: ::core::option::Option<super::super::base::abci::v1beta1::TxResponse>,
}
/// SimulateRequest is the request type for the Service.Simulate
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateRequest {
    /// tx is the transaction to simulate.
    /// Deprecated. Send raw tx bytes instead.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<Tx>,
    /// tx_bytes is the raw transaction.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(bytes = "vec", tag = "2")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// SimulateResponse is the response type for the
/// Service.SimulateRPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateResponse {
    /// gas_info is the information about gas used in the simulation.
    #[prost(message, optional, tag = "1")]
    pub gas_info: ::core::option::Option<super::super::base::abci::v1beta1::GasInfo>,
    /// result is the result of the simulation.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<super::super::base::abci::v1beta1::Result>,
}
/// GetTxRequest is the request type for the Service.GetTx
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxRequest {
    /// hash is the tx hash to query, encoded as a hex string.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// GetTxResponse is the response type for the Service.GetTx method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxResponse {
    /// tx is the queried transaction.
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<Tx>,
    /// tx_response is the queried TxResponses.
    #[prost(message, optional, tag = "2")]
    pub tx_response: ::core::option::Option<super::super::base::abci::v1beta1::TxResponse>,
}
/// GetBlockWithTxsRequest is the request type for the Service.GetBlockWithTxs
/// RPC method.
///
/// Since: cosmos-sdk 0.45.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockWithTxsRequest {
    /// height is the height of the block to query.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// pagination defines a pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// GetBlockWithTxsResponse is the response type for the Service.GetBlockWithTxs method.
///
/// Since: cosmos-sdk 0.45.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockWithTxsResponse {
    /// txs are the transactions in the block.
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<Tx>,
    #[prost(message, optional, tag = "2")]
    pub block_id: ::core::option::Option<::tendermint_proto::types::BlockId>,
    #[prost(message, optional, tag = "3")]
    pub block: ::core::option::Option<::tendermint_proto::types::Block>,
    /// pagination defines a pagination for the response.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// OrderBy defines the sorting order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderBy {
    /// ORDER_BY_UNSPECIFIED specifies an unknown sorting order. OrderBy defaults to ASC in this case.
    Unspecified = 0,
    /// ORDER_BY_ASC defines ascending order
    Asc = 1,
    /// ORDER_BY_DESC defines descending order
    Desc = 2,
}
impl OrderBy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderBy::Unspecified => "ORDER_BY_UNSPECIFIED",
            OrderBy::Asc => "ORDER_BY_ASC",
            OrderBy::Desc => "ORDER_BY_DESC",
        }
    }
}
/// BroadcastMode specifies the broadcast mode for the TxService.Broadcast RPC method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BroadcastMode {
    /// zero-value for mode ordering
    Unspecified = 0,
    /// BROADCAST_MODE_BLOCK defines a tx broadcasting mode where the client waits for
    /// the tx to be committed in a block.
    Block = 1,
    /// BROADCAST_MODE_SYNC defines a tx broadcasting mode where the client waits for
    /// a CheckTx execution response only.
    Sync = 2,
    /// BROADCAST_MODE_ASYNC defines a tx broadcasting mode where the client returns
    /// immediately.
    Async = 3,
}
impl BroadcastMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BroadcastMode::Unspecified => "BROADCAST_MODE_UNSPECIFIED",
            BroadcastMode::Block => "BROADCAST_MODE_BLOCK",
            BroadcastMode::Sync => "BROADCAST_MODE_SYNC",
            BroadcastMode::Async => "BROADCAST_MODE_ASYNC",
        }
    }
}
include!("cosmos.tx.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
