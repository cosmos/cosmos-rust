/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSigningInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// height at which validator was first a candidate OR was unjailed
    #[prost(int64, tag = "2")]
    pub start_height: i64,
    /// index offset into signed block bit array
    #[prost(int64, tag = "3")]
    pub index_offset: i64,
    /// timestamp validator cannot be unjailed until
    #[prost(message, optional, tag = "4")]
    pub jailed_until: ::core::option::Option<::prost_types::Timestamp>,
    /// whether or not a validator has been tombstoned (killed out of validator
    /// set)
    #[prost(bool, tag = "5")]
    pub tombstoned: bool,
    /// missed blocks counter (to avoid scanning the array every time)
    #[prost(int64, tag = "6")]
    pub missed_blocks_counter: i64,
}
/// Params represents the parameters used for by the slashing module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(int64, tag = "1")]
    pub signed_blocks_window: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub min_signed_per_window: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub downtime_jail_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(bytes = "vec", tag = "4")]
    pub slash_fraction_double_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub slash_fraction_downtime: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySigningInfoRequest is the request type for the Query/SigningInfo RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfoRequest {
    /// cons_address is the address to query signing info of
    #[prost(string, tag = "1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QuerySigningInfoResponse is the response type for the Query/SigningInfo RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfoResponse {
    /// val_signing_info is the signing info of requested val cons address
    #[prost(message, optional, tag = "1")]
    pub val_signing_info: ::core::option::Option<ValidatorSigningInfo>,
}
/// QuerySigningInfosRequest is the request type for the Query/SigningInfos RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfosRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySigningInfosResponse is the response type for the Query/SigningInfos RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfosResponse {
    /// info is the signing info of all validators
    #[prost(message, repeated, tag = "1")]
    pub info: ::prost::alloc::vec::Vec<ValidatorSigningInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query provides defines the gRPC querier service"]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
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
        #[doc = " Params queries the parameters of slashing module"]
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.slashing.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SigningInfo queries the signing info of given cons address"]
        pub async fn signing_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningInfoRequest>,
        ) -> Result<tonic::Response<super::QuerySigningInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.slashing.v1beta1.Query/SigningInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SigningInfos queries signing info of all validators"]
        pub async fn signing_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningInfosRequest>,
        ) -> Result<tonic::Response<super::QuerySigningInfosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.slashing.v1beta1.Query/SigningInfos");
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
/// MsgUnjail defines the Msg/Unjail request type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjail {
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// MsgUnjailResponse defines the Msg/Unjail response type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjailResponse {}
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Msg defines the slashing Msg service."]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MsgClient<T>
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
        #[doc = " Unjail defines a method for unjailing a jailed validator, thus returning"]
        #[doc = " them into the bonded validator set, so they can begin receiving provisions"]
        #[doc = " and rewards again."]
        pub async fn unjail(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnjail>,
        ) -> Result<tonic::Response<super::MsgUnjailResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.slashing.v1beta1.Msg/Unjail");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MsgClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MsgClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MsgClient {{ ... }}")
        }
    }
}
