/// ConnectionEnd defines a stateful object on a chain connected to another
/// separate one.
/// NOTE: there must only be 2 defined ConnectionEnds to establish
/// a connection between two chains.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionEnd {
    /// client associated with this connection.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration = "State", tag = "3")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period that must pass before a consensus state can be used for
    /// packet-verification NOTE: delay period logic is only implemented by some
    /// clients.
    #[prost(uint64, tag = "5")]
    pub delay_period: u64,
}
/// IdentifiedConnection defines a connection with additional connection
/// identifier field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedConnection {
    /// connection identifier.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// client associated with this connection.
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection
    #[prost(message, repeated, tag = "3")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration = "State", tag = "4")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag = "5")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period associated with this connection.
    #[prost(uint64, tag = "6")]
    pub delay_period: u64,
}
/// Counterparty defines the counterparty chain associated with a connection end.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// identifies the client on the counterparty chain associated with a given
    /// connection.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// identifies the connection end on the counterparty chain associated with a
    /// given connection.
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// commitment merkle prefix of the counterparty chain.
    #[prost(message, optional, tag = "3")]
    pub prefix: ::core::option::Option<super::super::commitment::v1::MerklePrefix>,
}
/// ClientPaths define all the connection paths for a client state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPaths {
    /// list of connection paths
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConnectionPaths define all the connection paths for a given client state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionPaths {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// list of connection paths
    #[prost(string, repeated, tag = "2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Version defines the versioning scheme used to negotiate the IBC verison in
/// the connection handshake.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// unique version identifier
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// list of features compatible with the specified identifier
    #[prost(string, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of Connection parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// maximum expected time per block (in nanoseconds), used to enforce block delay. This parameter should reflect the
    /// largest amount of time that the chain might reasonably take to produce the next block under normal operating
    /// conditions. A safe choice is 3-5x the expected time per block.
    #[prost(uint64, tag = "1")]
    pub max_expected_time_per_block: u64,
}
/// State defines if a connection is in one of the following states:
/// INIT, TRYOPEN, OPEN or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A connection end has just started the opening handshake.
    Init = 1,
    /// A connection end has acknowledged the handshake step on the counterparty
    /// chain.
    Tryopen = 2,
    /// A connection end has completed the handshake.
    Open = 3,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::UninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
            State::Init => "STATE_INIT",
            State::Tryopen => "STATE_TRYOPEN",
            State::Open => "STATE_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
            "STATE_INIT" => Some(Self::Init),
            "STATE_TRYOPEN" => Some(Self::Tryopen),
            "STATE_OPEN" => Some(Self::Open),
            _ => None,
        }
    }
}
/// MsgConnectionOpenInit defines the msg sent by an account on Chain A to
/// initialize a connection with Chain B.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenInit {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<Version>,
    #[prost(uint64, tag = "4")]
    pub delay_period: u64,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenInitResponse defines the Msg/ConnectionOpenInit response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenInitResponse {}
/// MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a
/// connection on Chain B.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenTry {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// in the case of crossing hello's, when both chains call OpenInit, we need
    /// the connection identifier of the previous connection in state INIT
    #[prost(string, tag = "2")]
    pub previous_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(uint64, tag = "5")]
    pub delay_period: u64,
    #[prost(message, repeated, tag = "6")]
    pub counterparty_versions: ::prost::alloc::vec::Vec<Version>,
    #[prost(message, optional, tag = "7")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain A: `UNITIALIZED ->
    /// INIT`
    #[prost(bytes = "vec", tag = "8")]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    /// proof of client state included in message
    #[prost(bytes = "vec", tag = "9")]
    pub proof_client: ::prost::alloc::vec::Vec<u8>,
    /// proof of client consensus state
    #[prost(bytes = "vec", tag = "10")]
    pub proof_consensus: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "11")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "12")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenTryResponse defines the Msg/ConnectionOpenTry response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenTryResponse {}
/// MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to
/// acknowledge the change of connection state to TRYOPEN on Chain B.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenAck {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub counterparty_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<Version>,
    #[prost(message, optional, tag = "4")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain B: `UNITIALIZED ->
    /// TRYOPEN`
    #[prost(bytes = "vec", tag = "6")]
    pub proof_try: ::prost::alloc::vec::Vec<u8>,
    /// proof of client state included in message
    #[prost(bytes = "vec", tag = "7")]
    pub proof_client: ::prost::alloc::vec::Vec<u8>,
    /// proof of client consensus state
    #[prost(bytes = "vec", tag = "8")]
    pub proof_consensus: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "9")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "10")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenAckResponse defines the Msg/ConnectionOpenAck response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenAckResponse {}
/// MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of connection state to OPEN on Chain A.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenConfirm {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    /// proof for the change of the connection state on Chain A: `INIT -> OPEN`
    #[prost(bytes = "vec", tag = "2")]
    pub proof_ack: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenConfirmResponse defines the Msg/ConnectionOpenConfirm
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenConfirmResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the ibc/connection Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// ConnectionOpenInit defines a rpc handler method for MsgConnectionOpenInit.
        pub async fn connection_open_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConnectionOpenInit>,
        ) -> std::result::Result<tonic::Response<super::MsgConnectionOpenInitResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Msg/ConnectionOpenInit",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Msg",
                "ConnectionOpenInit",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionOpenTry defines a rpc handler method for MsgConnectionOpenTry.
        pub async fn connection_open_try(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConnectionOpenTry>,
        ) -> std::result::Result<tonic::Response<super::MsgConnectionOpenTryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Msg/ConnectionOpenTry",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Msg",
                "ConnectionOpenTry",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionOpenAck defines a rpc handler method for MsgConnectionOpenAck.
        pub async fn connection_open_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConnectionOpenAck>,
        ) -> std::result::Result<tonic::Response<super::MsgConnectionOpenAckResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Msg/ConnectionOpenAck",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Msg",
                "ConnectionOpenAck",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionOpenConfirm defines a rpc handler method for
        /// MsgConnectionOpenConfirm.
        pub async fn connection_open_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConnectionOpenConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgConnectionOpenConfirmResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Msg/ConnectionOpenConfirm",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Msg",
                "ConnectionOpenConfirm",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryConnectionRequest is the request type for the Query/Connection RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionRequest {
    /// connection unique identifier
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionResponse is the response type for the Query/Connection RPC
/// method. Besides the connection end, it includes a proof and the height from
/// which the proof was retrieved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionResponse {
    /// connection associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub connection: ::core::option::Option<ConnectionEnd>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionsRequest is the request type for the Query/Connections RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConnectionsResponse is the response type for the Query/Connections RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsResponse {
    /// list of stored connections of the chain.
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryClientConnectionsRequest is the request type for the
/// Query/ClientConnections RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsRequest {
    /// client identifier associated with a connection
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientConnectionsResponse is the response type for the
/// Query/ClientConnections RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsResponse {
    /// slice of all the connection paths associated with a client.
    #[prost(string, repeated, tag = "1")]
    pub connection_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was generated
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionClientStateRequest is the request type for the
/// Query/ConnectionClientState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionClientStateRequest {
    /// connection identifier
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionClientStateResponse is the response type for the
/// Query/ConnectionClientState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionClientStateResponse {
    /// client state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub identified_client_state:
        ::core::option::Option<super::super::client::v1::IdentifiedClientState>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionConsensusStateRequest is the request type for the
/// Query/ConnectionConsensusState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionConsensusStateRequest {
    /// connection identifier
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub revision_number: u64,
    #[prost(uint64, tag = "3")]
    pub revision_height: u64,
}
/// QueryConnectionConsensusStateResponse is the response type for the
/// Query/ConnectionConsensusState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionConsensusStateResponse {
    /// consensus state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub consensus_state: ::core::option::Option<::prost_types::Any>,
    /// client ID associated with the consensus state
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query provides defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Connection queries an IBC connection end.
        pub async fn connection(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryConnectionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ibc.core.connection.v1.Query/Connection");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Query",
                "Connection",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Connections queries all the IBC connections of a chain.
        pub async fn connections(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryConnectionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ibc.core.connection.v1.Query/Connections");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Query",
                "Connections",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ClientConnections queries the connection paths associated with a client
        /// state.
        pub async fn client_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientConnectionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ClientConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Query",
                "ClientConnections",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionClientState queries the client state associated with the
        /// connection.
        pub async fn connection_client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConnectionClientStateResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ConnectionClientState",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Query",
                "ConnectionClientState",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionConsensusState queries the consensus state associated with the
        /// connection.
        pub async fn connection_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConnectionConsensusStateResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ConnectionConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.core.connection.v1.Query",
                "ConnectionConsensusState",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// GenesisState defines the ibc connection submodule's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    #[prost(message, repeated, tag = "2")]
    pub client_connection_paths: ::prost::alloc::vec::Vec<ConnectionPaths>,
    /// the sequence for the next generated connection identifier
    #[prost(uint64, tag = "3")]
    pub next_connection_sequence: u64,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
