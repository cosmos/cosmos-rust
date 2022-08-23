/// ===================== MsgJoinPool
/// This is really MsgJoinPoolNoSwap
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPoolResponse {
}
/// ===================== MsgExitPool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitPoolResponse {
}
/// ===================== MsgSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag="3")]
    pub token_in: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag="3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub token_out: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapExternAmountIn
/// TODO: Rename to MsgJoinSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, optional, tag="3")]
    pub token_in: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// repeated cosmos.base.v1beta1.Coin tokensIn = 5 [
    ///    (gogoproto.moretags) = "yaml:\"tokens_in\"",
    ///    (gogoproto.nullable) = false
    /// ];
    #[prost(string, tag="4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountInResponse {
    #[prost(string, tag="1")]
    pub share_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapShareAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapShareAmountOutResponse {
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapShareAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapShareAmountInResponse {
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapExternAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, optional, tag="3")]
    pub token_out: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub share_in_max_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExitSwapExternAmountOutResponse {
    #[prost(string, tag="1")]
    pub share_in_amount: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct Msg2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl Msg2Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> Msg2Client<T>
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
        ) -> Msg2Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            Msg2Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn join_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinPool>,
        ) -> Result<tonic::Response<super::MsgJoinPoolResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/JoinPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exit_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitPool>,
        ) -> Result<tonic::Response<super::MsgExitPoolResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/ExitPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountIn>,
        ) -> Result<
            tonic::Response<super::MsgSwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/SwapExactAmountIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountOut>,
        ) -> Result<
            tonic::Response<super::MsgSwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/SwapExactAmountOut",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn join_swap_extern_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinSwapExternAmountIn>,
        ) -> Result<
            tonic::Response<super::MsgJoinSwapExternAmountInResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/JoinSwapExternAmountIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn join_swap_share_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinSwapShareAmountOut>,
        ) -> Result<
            tonic::Response<super::MsgJoinSwapShareAmountOutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/JoinSwapShareAmountOut",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exit_swap_extern_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitSwapExternAmountOut>,
        ) -> Result<
            tonic::Response<super::MsgExitSwapExternAmountOutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/ExitSwapExternAmountOut",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exit_swap_share_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitSwapShareAmountIn>,
        ) -> Result<
            tonic::Response<super::MsgExitSwapShareAmountInResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Msg2/ExitSwapShareAmountIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// =============================== Pool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag="1")]
    pub pool: ::core::option::Option<::prost_types::Any>,
}
/// =============================== Pools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== NumPools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNumPoolsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNumPoolsResponse {
    #[prost(uint64, tag="1")]
    pub num_pools: u64,
}
/// =============================== PoolParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolParamsRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<::prost_types::Any>,
}
/// =============================== PoolLiquidity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalPoolLiquidityRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalPoolLiquidityResponse {
    #[prost(message, repeated, tag="1")]
    pub liquidity: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// =============================== TotalShares
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSharesRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSharesResponse {
    #[prost(message, optional, tag="1")]
    pub total_shares: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QuerySpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// QuerySpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag="1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountInRequest {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(string, tag="3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountInResponse {
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountOutRequest {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, repeated, tag="3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag="4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountOutResponse {
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLiquidityRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLiquidityResponse {
    #[prost(message, repeated, tag="1")]
    pub liquidity: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
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
        pub async fn pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryPoolsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/Pools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn num_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNumPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryNumPoolsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/NumPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalLiquidityRequest>,
        ) -> Result<tonic::Response<super::QueryTotalLiquidityResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/TotalLiquidity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Per Pool gRPC Endpoints
        pub async fn pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/Pool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pool_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolParamsRequest>,
        ) -> Result<tonic::Response<super::QueryPoolParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/PoolParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_pool_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalPoolLiquidityRequest>,
        ) -> Result<
            tonic::Response<super::QueryTotalPoolLiquidityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalSharesRequest>,
        ) -> Result<tonic::Response<super::QueryTotalSharesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/TotalShares",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SpotPrice defines a gRPC query handler that returns the spot price given
        /// a base denomination and a quote denomination.
        pub async fn spot_price(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotPriceRequest>,
        ) -> Result<tonic::Response<super::QuerySpotPriceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/SpotPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Estimate the swap.
        pub async fn estimate_swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySwapExactAmountInRequest>,
        ) -> Result<
            tonic::Response<super::QuerySwapExactAmountInResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn estimate_swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySwapExactAmountOutRequest>,
        ) -> Result<
            tonic::Response<super::QuerySwapExactAmountOutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Parameters for changing the weights in a balancer pool smoothly from
/// a start weight and end weight over a period of time.
/// Currently, the only smooth change supported is linear changing between
/// the two weights, but more types may be added in the future.
/// When these parameters are set, the weight w(t) for pool time `t` is the
/// following:
///    t <= start_time: w(t) = initial_pool_weights
///    start_time < t <= start_time + duration:
///      w(t) = initial_pool_weights + (t - start_time) *
///        (target_pool_weights - initial_pool_weights) / (duration)
///    t > start_time + duration: w(t) = target_pool_weights
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmoothWeightChangeParams {
    /// The start time for beginning the weight change.
    /// If a parameter change / pool instantiation leaves this blank,
    /// it should be generated by the state_machine as the current time.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Duration for the weights to change over
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The initial pool weights. These are copied from the pool's settings
    /// at the time of weight change instantiation.
    /// The amount PoolAsset.token.amount field is ignored if present,
    /// future type refactorings should just have a type with the denom & weight
    /// here.
    #[prost(message, repeated, tag="3")]
    pub initial_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
    /// The target pool weights. The pool weights will change linearly with respect
    /// to time between start_time, and start_time + duration. The amount
    /// PoolAsset.token.amount field is ignored if present, future type
    /// refactorings should just have a type with the denom & weight here.
    ///
    /// Intermediate variable for the 'slope' of pool weights. This is equal to
    /// (target_pool_weights - initial_pool_weights) / (duration)
    /// TODO: Work out precision, and decide if this is good to add
    /// repeated PoolAsset poolWeightSlope = 5 [
    ///   (gogoproto.moretags) = "yaml:\"pool_weight_slope\"",
    ///   (gogoproto.nullable) = false
    /// ];
    #[prost(message, repeated, tag="4")]
    pub target_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
}
/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag="1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exit_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub smooth_weight_change_params: ::core::option::Option<SmoothWeightChangeParams>,
}
/// Pool asset is an internal struct that combines the amount of the
/// token in the pool, and its balancer weight.
/// This is an awkward packaging of data,
/// and should be revisited in a future state migration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAsset {
    /// Coins we are talking about,
    /// the denomination must be unique amongst all PoolAssets for this pool.
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Weight that is not normalized. This weight must be less than 2^50
    #[prost(string, tag="2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(message, optional, tag="3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    /// TODO: Further improve these docs
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP tokens sent out
    #[prost(message, optional, tag="5")]
    pub total_shares: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// These are assumed to be sorted by denomiation.
    /// They contain the pool asset and the information about the weight
    #[prost(message, repeated, tag="6")]
    pub pool_assets: ::prost::alloc::vec::Vec<PoolAsset>,
    /// sum of all non-normalized pool weights
    #[prost(string, tag="7")]
    pub total_weight: ::prost::alloc::string::String,
}
