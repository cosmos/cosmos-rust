/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Timeout timestamp in absolute nanoseconds since unix epoch.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,
}
/// MsgTransferResponse defines the Msg/Transfer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the ibc/transfer Msg service.
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
        /// Transfer defines a rpc handler method for MsgTransfer.
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransfer>,
        ) -> std::result::Result<tonic::Response<super::MsgTransferResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ibc.applications.transfer.v1.Msg/Transfer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.applications.transfer.v1.Msg",
                "Transfer",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// DenomTrace contains the base denomination for ICS20 fungible tokens and the
/// source tracing information path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the
    /// source of the fungible token.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// base denomination of the relayed fungible token.
    #[prost(string, tag = "2")]
    pub base_denom: ::prost::alloc::string::String,
}
/// Params defines the set of IBC transfer parameters.
/// NOTE: To prevent a single token from being transferred, set the
/// TransfersEnabled parameter to true and then set the bank module's SendEnabled
/// parameter for the denomination to false.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// send_enabled enables or disables all cross-chain token transfers from this
    /// chain.
    #[prost(bool, tag = "1")]
    pub send_enabled: bool,
    /// receive_enabled enables or disables all cross-chain token transfers to this
    /// chain.
    #[prost(bool, tag = "2")]
    pub receive_enabled: bool,
}
/// QueryDenomTraceRequest is the request type for the Query/DenomTrace RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTraceRequest {
    /// hash (in hex format) of the denomination trace information.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryDenomTraceResponse is the response type for the Query/DenomTrace RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTraceResponse {
    /// denom_trace returns the requested denomination trace information.
    #[prost(message, optional, tag = "1")]
    pub denom_trace: ::core::option::Option<DenomTrace>,
}
/// QueryConnectionsRequest is the request type for the Query/DenomTraces RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTracesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConnectionsResponse is the response type for the Query/DenomTraces RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTracesResponse {
    /// denom_traces returns all denominations trace information.
    #[prost(message, repeated, tag = "1")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomHashRequest is the request type for the Query/DenomHash RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomHashRequest {
    /// The denomination trace (\[port_id]/[channel_id])+/[denom\]
    #[prost(string, tag = "1")]
    pub trace: ::prost::alloc::string::String,
}
/// QueryDenomHashResponse is the response type for the Query/DenomHash RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomHashResponse {
    /// hash (in hex format) of the denomination trace information.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query provides defines the gRPC querier service.
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
        /// DenomTrace queries a denomination trace information.
        pub async fn denom_trace(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomTraceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomTraceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.applications.transfer.v1.Query/DenomTrace",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.applications.transfer.v1.Query",
                "DenomTrace",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DenomTraces queries all denomination traces.
        pub async fn denom_traces(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomTracesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomTracesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.applications.transfer.v1.Query/DenomTraces",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.applications.transfer.v1.Query",
                "DenomTraces",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Params queries all parameters of the ibc-transfer module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ibc.applications.transfer.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.applications.transfer.v1.Query",
                "Params",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DenomHash queries a denomination hash information.
        pub async fn denom_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomHashRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomHashResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.applications.transfer.v1.Query/DenomHash",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ibc.applications.transfer.v1.Query",
                "DenomHash",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// GenesisState defines the ibc-transfer genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
