use crate::proto;

/// QueryHistoricalInfoRequest is request type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryHistoricalInfoRequest {
    /// height defines at which height to query the historical info.
    pub height: i64,
}

impl From<proto::cosmos::staking::v1beta1::QueryHistoricalInfoRequest>
    for QueryHistoricalInfoRequest
{
    fn from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::QueryHistoricalInfoRequest) -> Self {
        QueryHistoricalInfoRequest {
            height: proto.height,
        }
    }
}

impl From<QueryHistoricalInfoRequest>
    for proto::cosmos::staking::v1beta1::QueryHistoricalInfoRequest
{
    fn from(query_historical_info_request: QueryHistoricalInfoRequest) -> Self {
        proto::cosmos::staking::v1beta1::QueryHistoricalInfoRequest {
            height: query_historical_info_request.height,
        }
    }
}
