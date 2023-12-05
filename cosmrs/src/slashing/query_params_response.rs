use crate::slashing::Params;
use crate::{proto, ErrorReport, Result};

/// QueryParamsResponse is the response type for the Query/Params RPC method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryParamsResponse {
    /// Parameters of the slashing module
    pub params: Option<Params>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::QueryParamsResponse> for QueryParamsResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QueryParamsResponse,
    ) -> Result<Self> {
        Ok(QueryParamsResponse {
            params: proto.params.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<QueryParamsResponse> for proto::cosmos::slashing::v1beta1::QueryParamsResponse {
    fn from(params_response: QueryParamsResponse) -> Self {
        proto::cosmos::slashing::v1beta1::QueryParamsResponse {
            params: params_response.params.map(Into::into),
        }
    }
}
