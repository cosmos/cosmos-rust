use crate::staking::Validator;
use crate::{proto, ErrorReport, Result};

/// QueryValidatorResponse is response type for the Query/Validator RPC method
#[derive(Clone, Debug, PartialEq)]
pub struct QueryValidatorResponse {
    /// validator defines the validator info.
    pub validator: Option<Validator>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::QueryValidatorResponse> for QueryValidatorResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorResponse,
    ) -> Result<Self> {
        Ok(QueryValidatorResponse {
            validator: proto.validator.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<QueryValidatorResponse> for proto::cosmos::staking::v1beta1::QueryValidatorResponse {
    fn from(query_validator_response: QueryValidatorResponse) -> Self {
        proto::cosmos::staking::v1beta1::QueryValidatorResponse {
            validator: query_validator_response.validator.map(Into::into),
        }
    }
}
