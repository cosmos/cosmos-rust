use crate::proto;

/// QueryParamsRequest is the request type for the Query/Params RPC method
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueryParamsRequest {}

impl From<proto::cosmos::slashing::v1beta1::QueryParamsRequest> for QueryParamsRequest {
    fn from(_proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QueryParamsRequest) -> Self {
        QueryParamsRequest {}
    }
}
impl From<QueryParamsRequest> for proto::cosmos::slashing::v1beta1::QueryParamsRequest {
    fn from(_request: QueryParamsRequest) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::QueryParamsRequest {}
    }
}
