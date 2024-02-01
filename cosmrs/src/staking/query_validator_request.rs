use crate::{proto, AccountId, ErrorReport, Result};

/// QueryValidatorRequest is response type for the Query/Validator RPC method
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryValidatorRequest {
    /// validator_addr defines the validator address to query for.
    pub validator_addr: AccountId,
}

impl TryFrom<proto::cosmos::staking::v1beta1::QueryValidatorRequest> for QueryValidatorRequest {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorRequest,
    ) -> Result<Self> {
        Ok(QueryValidatorRequest {
            validator_addr: proto.validator_addr.parse()?,
        })
    }
}

impl From<QueryValidatorRequest> for proto::cosmos::staking::v1beta1::QueryValidatorRequest {
    fn from(query_validator_request: QueryValidatorRequest) -> Self {
        proto::cosmos::staking::v1beta1::QueryValidatorRequest {
            validator_addr: query_validator_request.validator_addr.to_string(),
        }
    }
}
