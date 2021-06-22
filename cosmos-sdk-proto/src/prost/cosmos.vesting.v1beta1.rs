/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(int64, tag="4")]
    pub end_time: i64,
    #[prost(bool, tag="5")]
    pub delayed: bool,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreateVestingAccount response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccountResponse {
}
# [doc = r" Generated client implementations."] pub mod msg_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Msg defines the bank Msg service."] pub struct MsgClient < T > { inner : tonic :: client :: Grpc < T > , } impl < T > MsgClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " CreateVestingAccount defines a method that enables creating a vesting"] # [doc = " account."] pub async fn create_vesting_account (& mut self , request : impl tonic :: IntoRequest < super :: MsgCreateVestingAccount > ,) -> Result < tonic :: Response < super :: MsgCreateVestingAccountResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/cosmos.vesting.v1beta1.Msg/CreateVestingAccount") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for MsgClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for MsgClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "MsgClient {{ ... }}") } } }