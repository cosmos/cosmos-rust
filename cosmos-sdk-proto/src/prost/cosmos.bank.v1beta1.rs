/// Params defines the parameters for the bank module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag="1")]
    pub send_enabled: ::std::vec::Vec<SendEnabled>,
    #[prost(bool, tag="2")]
    pub default_send_enabled: bool,
}
/// SendEnabled maps coin denom to a send_enabled status (whether a denom is
/// sendable).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEnabled {
    #[prost(string, tag="1")]
    pub denom: std::string::String,
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
/// Input models transaction input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(string, tag="1")]
    pub address: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Output models transaction outputs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(string, tag="1")]
    pub address: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Supply represents a struct that passively keeps track of the total supply
/// amounts in the network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Supply {
    #[prost(message, repeated, tag="1")]
    pub total: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DenomUnit represents a struct that describes a given
/// denomination unit of the basic token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomUnit {
    /// denom represents the string name of the given denom unit (e.g uatom).
    #[prost(string, tag="1")]
    pub denom: std::string::String,
    /// exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 1^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    #[prost(uint32, tag="2")]
    pub exponent: u32,
    /// aliases is a list of string aliases for the given denom
    #[prost(string, repeated, tag="3")]
    pub aliases: ::std::vec::Vec<std::string::String>,
}
/// Metadata represents a struct that describes
/// a basic token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag="1")]
    pub description: std::string::String,
    /// denom_units represents the list of DenomUnit's for a given coin
    #[prost(message, repeated, tag="2")]
    pub denom_units: ::std::vec::Vec<DenomUnit>,
    /// base represents the base denom (should be the DenomUnit with exponent = 0).
    #[prost(string, tag="3")]
    pub base: std::string::String,
    /// display indicates the suggested denom that should be
    /// displayed in clients.
    #[prost(string, tag="4")]
    pub display: std::string::String,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the address to query balances for.
    #[prost(string, tag="1")]
    pub address: std::string::String,
    /// denom is the coin denom to query balances for.
    #[prost(string, tag="2")]
    pub denom: std::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the coin.
    #[prost(message, optional, tag="1")]
    pub balance: ::std::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryBalanceRequest is the request type for the Query/AllBalances RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesRequest {
    /// address is the address to query balances for.
    #[prost(string, tag="1")]
    pub address: std::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesResponse {
    /// balances is the balances of all the coins.
    #[prost(message, repeated, tag="1")]
    pub balances: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryTotalSupplyRequest is the request type for the Query/TotalSupply RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSupplyRequest {
}
/// QueryTotalSupplyResponse is the response type for the Query/TotalSupply RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSupplyResponse {
    /// supply is the supply of the coins
    #[prost(message, repeated, tag="1")]
    pub supply: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// QuerySupplyOfRequest is the request type for the Query/SupplyOf RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyOfRequest {
    /// denom is the coin denom to query balances for.
    #[prost(string, tag="1")]
    pub denom: std::string::String,
}
/// QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyOfResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag="1")]
    pub amount: ::std::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryParamsRequest defines the request type for querying x/bank parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse defines the response type for querying x/bank parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::std::option::Option<Params>,
}
# [doc = r" Generated client implementations."] pub mod query_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Query defines the gRPC querier service."] pub struct QueryClient < T > { inner : tonic :: client :: Grpc < T > , } impl QueryClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > QueryClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " Balance queries the balance of a single coin for a single account."] pub async fn balance (& mut self , request : impl tonic :: IntoRequest < super :: QueryBalanceRequest > ,) -> Result < tonic :: Response < super :: QueryBalanceResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.bank.v1beta1.Query/Balance") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " AllBalances queries the balance of all coins for a single account."] pub async fn all_balances (& mut self , request : impl tonic :: IntoRequest < super :: QueryAllBalancesRequest > ,) -> Result < tonic :: Response < super :: QueryAllBalancesResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.bank.v1beta1.Query/AllBalances") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " TotalSupply queries the total supply of all coins."] pub async fn total_supply (& mut self , request : impl tonic :: IntoRequest < super :: QueryTotalSupplyRequest > ,) -> Result < tonic :: Response < super :: QueryTotalSupplyResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.bank.v1beta1.Query/TotalSupply") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " SupplyOf queries the supply of a single coin."] pub async fn supply_of (& mut self , request : impl tonic :: IntoRequest < super :: QuerySupplyOfRequest > ,) -> Result < tonic :: Response < super :: QuerySupplyOfResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.bank.v1beta1.Query/SupplyOf") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Params queries the parameters of x/bank module."] pub async fn params (& mut self , request : impl tonic :: IntoRequest < super :: QueryParamsRequest > ,) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.bank.v1beta1.Query/Params") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for QueryClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for QueryClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "QueryClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod query_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."] # [async_trait] pub trait Query : Send + Sync + 'static { # [doc = " Balance queries the balance of a single coin for a single account."] async fn balance (& self , request : tonic :: Request < super :: QueryBalanceRequest >) -> Result < tonic :: Response < super :: QueryBalanceResponse > , tonic :: Status > ; # [doc = " AllBalances queries the balance of all coins for a single account."] async fn all_balances (& self , request : tonic :: Request < super :: QueryAllBalancesRequest >) -> Result < tonic :: Response < super :: QueryAllBalancesResponse > , tonic :: Status > ; # [doc = " TotalSupply queries the total supply of all coins."] async fn total_supply (& self , request : tonic :: Request < super :: QueryTotalSupplyRequest >) -> Result < tonic :: Response < super :: QueryTotalSupplyResponse > , tonic :: Status > ; # [doc = " SupplyOf queries the supply of a single coin."] async fn supply_of (& self , request : tonic :: Request < super :: QuerySupplyOfRequest >) -> Result < tonic :: Response < super :: QuerySupplyOfResponse > , tonic :: Status > ; # [doc = " Params queries the parameters of x/bank module."] async fn params (& self , request : tonic :: Request < super :: QueryParamsRequest >) -> Result < tonic :: Response < super :: QueryParamsResponse > , tonic :: Status > ; } # [doc = " Query defines the gRPC querier service."] # [derive (Debug)] pub struct QueryServer < T : Query > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : Query > QueryServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for QueryServer < T > where T : Query , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/cosmos.bank.v1beta1.Query/Balance" => { # [allow (non_camel_case_types)] struct BalanceSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryBalanceRequest > for BalanceSvc < T > { type Response = super :: QueryBalanceResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryBalanceRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . balance (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = BalanceSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.bank.v1beta1.Query/AllBalances" => { # [allow (non_camel_case_types)] struct AllBalancesSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryAllBalancesRequest > for AllBalancesSvc < T > { type Response = super :: QueryAllBalancesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryAllBalancesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . all_balances (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = AllBalancesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.bank.v1beta1.Query/TotalSupply" => { # [allow (non_camel_case_types)] struct TotalSupplySvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryTotalSupplyRequest > for TotalSupplySvc < T > { type Response = super :: QueryTotalSupplyResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryTotalSupplyRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . total_supply (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = TotalSupplySvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.bank.v1beta1.Query/SupplyOf" => { # [allow (non_camel_case_types)] struct SupplyOfSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QuerySupplyOfRequest > for SupplyOfSvc < T > { type Response = super :: QuerySupplyOfResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QuerySupplyOfRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . supply_of (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = SupplyOfSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/cosmos.bank.v1beta1.Query/Params" => { # [allow (non_camel_case_types)] struct ParamsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryParamsRequest > for ParamsSvc < T > { type Response = super :: QueryParamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryParamsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . params (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ParamsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : Query > Clone for QueryServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : Query > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : Query > tonic :: transport :: NamedService for QueryServer < T > { const NAME : & 'static str = "cosmos.bank.v1beta1.Query" ; } }