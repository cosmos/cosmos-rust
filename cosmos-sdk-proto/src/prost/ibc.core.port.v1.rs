/// QueryAppVersionRequest is the request type for the Query/AppVersion RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppVersionRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// connection unique identifier
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "super::super::channel::v1::Order", tag = "3")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<super::super::channel::v1::Counterparty>,
    /// proposed version
    #[prost(string, tag = "5")]
    pub proposed_version: ::prost::alloc::string::String,
}
/// QueryAppVersionResponse is the response type for the Query/AppVersion RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppVersionResponse {
    /// port id associated with the request identifiers
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// supported app version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service"]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " AppVersion queries an IBC Port and determines the appropriate application version to be used"]
        pub async fn app_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppVersionRequest>,
        ) -> Result<tonic::Response<super::QueryAppVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ibc.core.port.v1.Query/AppVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}
