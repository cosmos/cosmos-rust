// @generated
/// FileDescriptorsRequest is the Query/FileDescriptors request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorsRequest {}
/// FileDescriptorsResponse is the Query/FileDescriptors response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorsResponse {
    /// files is the file descriptors.
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<::prost_types::FileDescriptorProto>,
}
include!("cosmos.reflection.v1.tonic.rs");
// @@protoc_insertion_point(module)
