use crate::base::query::PageRequest;
use crate::{proto, ErrorReport, Result};

/// QueryValidatorsRequest is request type for Query/Validators RPC method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryValidatorsRequest {
    /// status enables to query for validators matching a given status.
    pub status: String,

    /// pagination defines an optional pagination for the request.
    pub pagination: Option<PageRequest>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::QueryValidatorsRequest> for QueryValidatorsRequest {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsRequest,
    ) -> Result<Self> {
        Ok(QueryValidatorsRequest {
            status: proto.status,
            pagination: proto.pagination.map(Into::into),
        })
    }
}

impl From<QueryValidatorsRequest> for proto::cosmos::staking::v1beta1::QueryValidatorsRequest {
    fn from(query_validators_request: QueryValidatorsRequest) -> Self {
        proto::cosmos::staking::v1beta1::QueryValidatorsRequest {
            status: query_validators_request.status,
            pagination: query_validators_request.pagination.map(Into::into),
        }
    }
}
