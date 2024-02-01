use crate::staking::HistoricalInfo;
use crate::{proto, ErrorReport, Result};

/// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryHistoricalInfoResponse {
    /// hist defines the historical info at the given height.
    pub hist: Option<HistoricalInfo>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::QueryHistoricalInfoResponse>
    for QueryHistoricalInfoResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryHistoricalInfoResponse,
    ) -> Result<Self> {
        Ok(QueryHistoricalInfoResponse {
            hist: proto.hist.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<QueryHistoricalInfoResponse>
    for proto::cosmos::staking::v1beta1::QueryHistoricalInfoResponse
{
    fn from(query_historical_info_response: QueryHistoricalInfoResponse) -> Self {
        proto::cosmos::staking::v1beta1::QueryHistoricalInfoResponse {
            hist: query_historical_info_response.hist.map(Into::into),
        }
    }
}
