use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgUnjail defines the Msg/Unjail request type
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUnjail {
    /// Address of the validator to unjail.
    pub validator_addr: AccountId,
}

impl Msg for MsgUnjail {
    type Proto = proto::cosmos::slashing::v1beta1::MsgUnjail;
}

impl TryFrom<proto::cosmos::slashing::v1beta1::MsgUnjail> for MsgUnjail {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjail) -> Result<Self> {
        Ok(MsgUnjail {
            validator_addr: proto.validator_addr.parse()?,
        })
    }
}

impl From<MsgUnjail> for cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjail {
    fn from(msg_unjail: MsgUnjail) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjail {
            validator_addr: msg_unjail.validator_addr.to_string(),
        }
    }
}

/// MsgUnjailResponse defines the Msg/Unjail response type
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUnjailResponse {}

impl Msg for MsgUnjailResponse {
    type Proto = proto::cosmos::slashing::v1beta1::MsgUnjailResponse;
}

impl TryFrom<proto::cosmos::slashing::v1beta1::MsgUnjailResponse> for MsgUnjailResponse {
    type Error = ErrorReport;

    fn try_from(
        _proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjailResponse,
    ) -> Result<Self> {
        Ok(MsgUnjailResponse {})
    }
}

impl From<MsgUnjailResponse> for cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjailResponse {
    fn from(_: MsgUnjailResponse) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::MsgUnjailResponse {}
    }
}
