#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(message, repeated, tag="1")]
    pub children: ::prost::alloc::vec::Vec<Child>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Child {
    #[prost(bytes="vec", tag="1")]
    pub index: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub accumulation: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Leaf {
    #[prost(message, optional, tag="1")]
    pub leaf: ::core::option::Option<Child>,
}
