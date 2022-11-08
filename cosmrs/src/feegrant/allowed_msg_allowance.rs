use crate::{proto, tx::Msg, Any, ErrorReport, Result};

/// AllowedMsgAllowance creates allowance only for specified message types.
#[derive(Clone, Debug, PartialEq)]
pub struct AllowedMsgAllowance {
    /// allowance can be any of basic and filtered fee allowance.
    pub allowance: Option<Any>,

    /// allowed_messages are the messages for which the grantee has the access.
    pub allowed_messages: Vec<String>,
}

impl Msg for AllowedMsgAllowance {
    type Proto = proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance;
}

impl TryFrom<proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance> for AllowedMsgAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance,
    ) -> Result<AllowedMsgAllowance> {
        AllowedMsgAllowance::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance> for AllowedMsgAllowance {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance,
    ) -> Result<AllowedMsgAllowance> {
        Ok(AllowedMsgAllowance {
            allowance: proto.allowance.clone(),
            allowed_messages: proto
                .allowed_messages
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<AllowedMsgAllowance> for proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    fn from(
        allowance: AllowedMsgAllowance,
    ) -> proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
        proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance::from(&allowance)
    }
}

impl From<&AllowedMsgAllowance> for proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    fn from(
        allowance: &AllowedMsgAllowance,
    ) -> proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
        proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
            allowance: allowance.allowance.clone().map(Into::into),
            allowed_messages: allowance.allowed_messages.iter().map(Into::into).collect(),
        }
    }
}
