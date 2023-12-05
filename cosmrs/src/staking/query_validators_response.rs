use crate::base::query::PageResponse;
use crate::staking::Validator;
use crate::{proto, ErrorReport, Result};

/// QueryValidatorsResponse is response type for the Query/Validators RPC method
#[derive(Clone, Debug, PartialEq)]
pub struct QueryValidatorsResponse {
    /// validators contains all the queried validators.
    pub validators: Vec<Validator>,

    /// pagination defines the pagination in the response.
    pub pagination: Option<PageResponse>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::QueryValidatorsResponse> for QueryValidatorsResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsResponse,
    ) -> Result<Self> {
        Ok(QueryValidatorsResponse {
            validators: proto
                .validators
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            pagination: proto.pagination.map(Into::into),
        })
    }
}

impl From<QueryValidatorsResponse> for proto::cosmos::staking::v1beta1::QueryValidatorsResponse {
    fn from(query_validators_response: QueryValidatorsResponse) -> Self {
        proto::cosmos::staking::v1beta1::QueryValidatorsResponse {
            validators: query_validators_response
                .validators
                .into_iter()
                .map(Into::into)
                .collect(),
            pagination: query_validators_response.pagination.map(Into::into),
        }
    }
}
