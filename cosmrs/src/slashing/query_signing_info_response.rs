use crate::slashing::ValidatorSigningInfo;
use crate::{proto, ErrorReport, Result};

/// QuerySigningInfoResponse is the response type for the Query/SigningInfo RPC
/// method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySigningInfoResponse {
    /// val_signing_info is the signing info of requested val cons address
    pub val_signing_info: Option<ValidatorSigningInfo>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::QuerySigningInfoResponse>
    for QuerySigningInfoResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::QuerySigningInfoResponse,
    ) -> Result<Self> {
        Ok(QuerySigningInfoResponse {
            val_signing_info: proto.val_signing_info.map(TryFrom::try_from).transpose()?,
        })
    }
}

impl From<QuerySigningInfoResponse> for proto::cosmos::slashing::v1beta1::QuerySigningInfoResponse {
    fn from(signing_info_response: QuerySigningInfoResponse) -> Self {
        proto::cosmos::slashing::v1beta1::QuerySigningInfoResponse {
            val_signing_info: signing_info_response.val_signing_info.map(Into::into),
        }
    }
}
