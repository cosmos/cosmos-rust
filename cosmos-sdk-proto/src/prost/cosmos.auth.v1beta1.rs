/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseAccount {
    #[prost(string, tag="1")]
    pub address: std::string::String,
    #[prost(message, optional, tag="2")]
    pub pub_key: ::std::option::Option<::prost_types::Any>,
    #[prost(uint64, tag="3")]
    pub account_number: u64,
    #[prost(uint64, tag="4")]
    pub sequence: u64,
}
/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccount {
    #[prost(message, optional, tag="1")]
    pub base_account: ::std::option::Option<BaseAccount>,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(string, repeated, tag="3")]
    pub permissions: ::std::vec::Vec<std::string::String>,
}
/// Params defines the parameters for the auth module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag="1")]
    pub max_memo_characters: u64,
    #[prost(uint64, tag="2")]
    pub tx_sig_limit: u64,
    #[prost(uint64, tag="3")]
    pub tx_size_cost_per_byte: u64,
    #[prost(uint64, tag="4")]
    pub sig_verify_cost_ed25519: u64,
    #[prost(uint64, tag="5")]
    pub sig_verify_cost_secp256k1: u64,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    #[prost(string, tag="1")]
    pub address: std::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    #[prost(message, optional, tag="1")]
    pub account: ::std::option::Option<::prost_types::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::std::option::Option<Params>,
}
# [doc = r" Generated client implementations."] pub mod query_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Query defines the gRPC querier service."] pub struct QueryClient < T > { inner : tonic :: client :: Grpc < T > , } impl QueryClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > QueryClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " Account returns account details based on address."] pub async fn account (& mut self , request : impl tonic :: IntoRequest < super :: QueryAccountRequest > ,) -> Result < tonic :: Response < super :: QueryAccountResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.auth.v1beta1.Query/Account") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Params queries all parameters."] pub async fn params (& mut self , request : impl tonic :: IntoRequest < super :: QueryParamsRequest > ,) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.auth.v1beta1.Query/Params") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for QueryClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for QueryClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "QueryClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod query_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."] # [async_trait] pub trait Query : Send + Sync + 'static { # [doc = " Account returns account details based on address."] async fn account (& self , request : tonic :: Request < super :: QueryAccountRequest >) -> Result < tonic :: Response < super :: QueryAccountResponse > , tonic :: Status > ; # [doc = " Params queries all parameters."] async fn params (& self , request : tonic :: Request < super :: QueryParamsRequest >) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > ; } # [doc = " Query defines the gRPC querier service."] # [derive (Debug)] pub struct QueryServer < T : Query > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : Query > QueryServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for QueryServer < T > where T : Query , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/cosmos.auth.v1beta1.Query/Account" => { # [allow (non_camel_case_types)] struct AccountSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryAccountRequest > for AccountSvc < T > { type Response = super :: QueryAccountResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryAccountRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . account (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = AccountSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.auth.v1beta1.Query/Params" => { # [allow (non_camel_case_types)] struct ParamsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryParamsRequest > for ParamsSvc < T > { type Response = super :: QueryParamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryParamsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . params (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ParamsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : Query > Clone for QueryServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : Query > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : Query > tonic :: transport :: NamedService for QueryServer < T > { const NAME : & 'static str = "cosmos.auth.v1beta1.Query" ; } }