// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod reflection_service_client {
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
    pub struct ReflectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl ReflectionServiceClient<tonic::transport::Channel> {
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
    impl<T> ReflectionServiceClient<T>
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
        ) -> ReflectionServiceClient<InterceptedService<T, F>>
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
            ReflectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_authn_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthnDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetAuthnDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetAuthnDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetAuthnDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_chain_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChainDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetChainDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetChainDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetChainDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_codec_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCodecDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetCodecDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetCodecDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetCodecDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_configuration_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigurationDescriptorRequest>,
        ) -> core::result::Result<
            tonic::Response<super::GetConfigurationDescriptorResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetConfigurationDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetConfigurationDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_query_services_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueryServicesDescriptorRequest>,
        ) -> core::result::Result<
            tonic::Response<super::GetQueryServicesDescriptorResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetQueryServicesDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetQueryServicesDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_tx_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(alloc::format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetTxDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetTxDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
pub mod reflection_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReflectionServiceServer.
    #[async_trait]
    pub trait ReflectionService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_authn_descriptor(
            &self,
            request: tonic::Request<super::GetAuthnDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetAuthnDescriptorResponse>, tonic::Status>;
        async fn get_chain_descriptor(
            &self,
            request: tonic::Request<super::GetChainDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetChainDescriptorResponse>, tonic::Status>;
        async fn get_codec_descriptor(
            &self,
            request: tonic::Request<super::GetCodecDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetCodecDescriptorResponse>, tonic::Status>;
        async fn get_configuration_descriptor(
            &self,
            request: tonic::Request<super::GetConfigurationDescriptorRequest>,
        ) -> core::result::Result<
            tonic::Response<super::GetConfigurationDescriptorResponse>,
            tonic::Status,
        >;
        async fn get_query_services_descriptor(
            &self,
            request: tonic::Request<super::GetQueryServicesDescriptorRequest>,
        ) -> core::result::Result<
            tonic::Response<super::GetQueryServicesDescriptorResponse>,
            tonic::Status,
        >;
        async fn get_tx_descriptor(
            &self,
            request: tonic::Request<super::GetTxDescriptorRequest>,
        ) -> core::result::Result<tonic::Response<super::GetTxDescriptorResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ReflectionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ReflectionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReflectionServiceServer<T>
    where
        T: ReflectionService,
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetAuthnDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetAuthnDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetAuthnDescriptorRequest>
                        for GetAuthnDescriptorSvc<T>
                    {
                        type Response = super::GetAuthnDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAuthnDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_authn_descriptor(&inner, request)
                                    .await
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
                        let method = GetAuthnDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetChainDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetChainDescriptorRequest>
                        for GetChainDescriptorSvc<T>
                    {
                        type Response = super::GetChainDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetChainDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_chain_descriptor(&inner, request)
                                    .await
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
                        let method = GetChainDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetCodecDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetCodecDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetCodecDescriptorRequest>
                        for GetCodecDescriptorSvc<T>
                    {
                        type Response = super::GetCodecDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCodecDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_codec_descriptor(&inner, request)
                                    .await
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
                        let method = GetCodecDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetConfigurationDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigurationDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetConfigurationDescriptorRequest>
                        for GetConfigurationDescriptorSvc<T>
                    {
                        type Response = super::GetConfigurationDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConfigurationDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_configuration_descriptor(
                                    &inner, request,
                                )
                                .await
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
                        let method = GetConfigurationDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetQueryServicesDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetQueryServicesDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetQueryServicesDescriptorRequest>
                        for GetQueryServicesDescriptorSvc<T>
                    {
                        type Response = super::GetQueryServicesDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQueryServicesDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_query_services_descriptor(
                                    &inner, request,
                                )
                                .await
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
                        let method = GetQueryServicesDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetTxDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetTxDescriptorRequest>
                        for GetTxDescriptorSvc<T>
                    {
                        type Response = super::GetTxDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReflectionService>::get_tx_descriptor(&inner, request).await
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
                        let method = GetTxDescriptorSvc(inner);
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
    impl<T> Clone for ReflectionServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "cosmos.base.reflection.v2alpha1.ReflectionService";
    impl<T> tonic::server::NamedService for ReflectionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
