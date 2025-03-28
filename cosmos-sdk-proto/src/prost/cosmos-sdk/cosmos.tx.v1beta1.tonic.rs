// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl ServiceClient<tonic::transport::Channel> {
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
    impl<T> ServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::Body>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn simulate(
            &mut self,
            request: impl tonic::IntoRequest<super::SimulateRequest>,
        ) -> core::result::Result<tonic::Response<super::SimulateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/Simulate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "Simulate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/GetTx");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "GetTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn broadcast_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastTxRequest>,
        ) -> core::result::Result<tonic::Response<super::BroadcastTxResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/BroadcastTx");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "BroadcastTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_txs_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxsEventRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxsEventResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/GetTxsEvent");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "GetTxsEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_with_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockWithTxsRequest>,
        ) -> core::result::Result<tonic::Response<super::GetBlockWithTxsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/GetBlockWithTxs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.tx.v1beta1.Service",
                "GetBlockWithTxs",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_decode(
            &mut self,
            request: impl tonic::IntoRequest<super::TxDecodeRequest>,
        ) -> core::result::Result<tonic::Response<super::TxDecodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/TxDecode");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "TxDecode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_encode(
            &mut self,
            request: impl tonic::IntoRequest<super::TxEncodeRequest>,
        ) -> core::result::Result<tonic::Response<super::TxEncodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/TxEncode");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.tx.v1beta1.Service", "TxEncode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_encode_amino(
            &mut self,
            request: impl tonic::IntoRequest<super::TxEncodeAminoRequest>,
        ) -> core::result::Result<tonic::Response<super::TxEncodeAminoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/TxEncodeAmino");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.tx.v1beta1.Service",
                "TxEncodeAmino",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_decode_amino(
            &mut self,
            request: impl tonic::IntoRequest<super::TxDecodeAminoRequest>,
        ) -> core::result::Result<tonic::Response<super::TxDecodeAminoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.tx.v1beta1.Service/TxDecodeAmino");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.tx.v1beta1.Service",
                "TxDecodeAmino",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
pub mod service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServiceServer.
    #[async_trait]
    pub trait Service: std::marker::Send + std::marker::Sync + 'static {
        async fn simulate(
            &self,
            request: tonic::Request<super::SimulateRequest>,
        ) -> core::result::Result<tonic::Response<super::SimulateResponse>, tonic::Status>;
        async fn get_tx(
            &self,
            request: tonic::Request<super::GetTxRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxResponse>, tonic::Status>;
        async fn broadcast_tx(
            &self,
            request: tonic::Request<super::BroadcastTxRequest>,
        ) -> core::result::Result<tonic::Response<super::BroadcastTxResponse>, tonic::Status>;
        async fn get_txs_event(
            &self,
            request: tonic::Request<super::GetTxsEventRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxsEventResponse>, tonic::Status>;
        async fn get_block_with_txs(
            &self,
            request: tonic::Request<super::GetBlockWithTxsRequest>,
        ) -> core::result::Result<tonic::Response<super::GetBlockWithTxsResponse>, tonic::Status>;
        async fn tx_decode(
            &self,
            request: tonic::Request<super::TxDecodeRequest>,
        ) -> core::result::Result<tonic::Response<super::TxDecodeResponse>, tonic::Status>;
        async fn tx_encode(
            &self,
            request: tonic::Request<super::TxEncodeRequest>,
        ) -> core::result::Result<tonic::Response<super::TxEncodeResponse>, tonic::Status>;
        async fn tx_encode_amino(
            &self,
            request: tonic::Request<super::TxEncodeAminoRequest>,
        ) -> core::result::Result<tonic::Response<super::TxEncodeAminoResponse>, tonic::Status>;
        async fn tx_decode_amino(
            &self,
            request: tonic::Request<super::TxDecodeAminoRequest>,
        ) -> core::result::Result<tonic::Response<super::TxDecodeAminoResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServiceServer<T>
    where
        T: Service,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<core::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cosmos.tx.v1beta1.Service/Simulate" => {
                    #[allow(non_camel_case_types)]
                    struct SimulateSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::SimulateRequest> for SimulateSvc<T> {
                        type Response = super::SimulateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimulateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Service>::simulate(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SimulateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/GetTx" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::GetTxRequest> for GetTxSvc<T> {
                        type Response = super::GetTxResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Service>::get_tx(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/BroadcastTx" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastTxSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::BroadcastTxRequest> for BroadcastTxSvc<T> {
                        type Response = super::BroadcastTxResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BroadcastTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Service>::broadcast_tx(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BroadcastTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/GetTxsEvent" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxsEventSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::GetTxsEventRequest> for GetTxsEventSvc<T> {
                        type Response = super::GetTxsEventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxsEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Service>::get_txs_event(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetTxsEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/GetBlockWithTxs" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockWithTxsSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::GetBlockWithTxsRequest>
                        for GetBlockWithTxsSvc<T>
                    {
                        type Response = super::GetBlockWithTxsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockWithTxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Service>::get_block_with_txs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetBlockWithTxsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/TxDecode" => {
                    #[allow(non_camel_case_types)]
                    struct TxDecodeSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::TxDecodeRequest> for TxDecodeSvc<T> {
                        type Response = super::TxDecodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxDecodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Service>::tx_decode(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TxDecodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/TxEncode" => {
                    #[allow(non_camel_case_types)]
                    struct TxEncodeSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::TxEncodeRequest> for TxEncodeSvc<T> {
                        type Response = super::TxEncodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxEncodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Service>::tx_encode(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TxEncodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/TxEncodeAmino" => {
                    #[allow(non_camel_case_types)]
                    struct TxEncodeAminoSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::TxEncodeAminoRequest> for TxEncodeAminoSvc<T> {
                        type Response = super::TxEncodeAminoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxEncodeAminoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Service>::tx_encode_amino(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TxEncodeAminoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.tx.v1beta1.Service/TxDecodeAmino" => {
                    #[allow(non_camel_case_types)]
                    struct TxDecodeAminoSvc<T: Service>(pub Arc<T>);
                    impl<T: Service> tonic::server::UnaryService<super::TxDecodeAminoRequest> for TxDecodeAminoSvc<T> {
                        type Response = super::TxDecodeAminoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxDecodeAminoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Service>::tx_decode_amino(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TxDecodeAminoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    let mut response = http::Response::new(tonic::body::Body::default());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for ServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cosmos.tx.v1beta1.Service";
    impl<T> tonic::server::NamedService for ServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
