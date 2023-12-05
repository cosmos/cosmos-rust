use crate::{proto, AccountId, ErrorReport, Result};

/// QuerySigningInfoRequest is the request type for the Query/SigningInfo RPC
/// method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySigningInfoRequest {
    /// cons_address is the address to query signing info of
    pub cons_address: AccountId,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::QuerySigningInfoRequest>
    for QuerySigningInfoRequest
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QuerySigningInfoRequest,
    ) -> Result<Self> {
        Ok(QuerySigningInfoRequest {
            cons_address: proto.cons_address.parse()?,
        })
    }
}

impl From<QuerySigningInfoRequest> for proto::cosmos::slashing::v1beta1::QuerySigningInfoRequest {
    fn from(signing_info_response: QuerySigningInfoRequest) -> Self {
        proto::cosmos::slashing::v1beta1::QuerySigningInfoRequest {
            cons_address: signing_info_response.cons_address.to_string(),
        }
    }
}
