use crate::base::query::PageRequest;
use crate::proto;

/// QuerySigningInfosRequest is the request type for the Query/SigningInfos RPC
/// method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySigningInfosRequest {
    /// Pagination control of the request
    pub pagination: Option<PageRequest>,
}

impl From<proto::cosmos::slashing::v1beta1::QuerySigningInfosRequest> for QuerySigningInfosRequest {
    fn from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QuerySigningInfosRequest) -> Self {
        QuerySigningInfosRequest {
            pagination: proto.pagination.map(Into::into),
        }
    }
}

impl From<QuerySigningInfosRequest> for proto::cosmos::slashing::v1beta1::QuerySigningInfosRequest {
    fn from(signing_infos_request: QuerySigningInfosRequest) -> Self {
        proto::cosmos::slashing::v1beta1::QuerySigningInfosRequest {
            pagination: signing_infos_request.pagination.map(Into::into),
        }
    }
}
