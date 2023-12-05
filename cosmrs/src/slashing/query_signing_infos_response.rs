use crate::base::query::PageResponse;
use crate::slashing::ValidatorSigningInfo;
use crate::{proto, ErrorReport, Result};

/// QuerySigningInfosResponse is the response type for the Query/SigningInfos RPC
/// method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySigningInfosResponse {
    /// info is the signing info of all validators
    pub info: Vec<ValidatorSigningInfo>,

    /// pagination information
    pub pagination: Option<PageResponse>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::QuerySigningInfosResponse>
    for QuerySigningInfosResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QuerySigningInfosResponse,
    ) -> Result<Self> {
        Ok(QuerySigningInfosResponse {
            info: proto
                .info
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            pagination: proto.pagination.map(Into::into),
        })
    }
}

impl From<QuerySigningInfosResponse>
    for proto::cosmos::slashing::v1beta1::QuerySigningInfosResponse
{
    fn from(signing_infos_response: QuerySigningInfosResponse) -> Self {
        proto::cosmos::slashing::v1beta1::QuerySigningInfosResponse {
            info: signing_infos_response
                .info
                .into_iter()
                .map(Into::into)
                .collect(),
            pagination: signing_infos_response.pagination.map(Into::into),
        }
    }
}
