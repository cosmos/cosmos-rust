use crate::{proto, tx::Msg, ErrorReport, Result};

/// MsgCancelContinuousFund represents a message to withdraw a delegator's reward from a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgCancelContinuousFund {
    pub authority: String,
    pub recipient: String,
}

impl Msg for MsgCancelContinuousFund {
    type Proto = proto::cosmos::protocolpool::v1::MsgCancelContinuousFund;
}

impl TryFrom<proto::cosmos::protocolpool::v1::MsgCancelContinuousFund> for MsgCancelContinuousFund {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::protocolpool::v1::MsgCancelContinuousFund,
    ) -> Result<MsgCancelContinuousFund> {
        MsgCancelContinuousFund::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::protocolpool::v1::MsgCancelContinuousFund>
    for MsgCancelContinuousFund
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::protocolpool::v1::MsgCancelContinuousFund,
    ) -> Result<MsgCancelContinuousFund> {
        Ok(MsgCancelContinuousFund {
            authority: proto.authority.parse()?,
            recipient: proto.recipient.parse()?,
        })
    }
}

impl From<MsgCancelContinuousFund> for proto::cosmos::protocolpool::v1::MsgCancelContinuousFund {
    fn from(
        coin: MsgCancelContinuousFund,
    ) -> proto::cosmos::protocolpool::v1::MsgCancelContinuousFund {
        proto::cosmos::protocolpool::v1::MsgCancelContinuousFund::from(&coin)
    }
}

impl From<&MsgCancelContinuousFund> for proto::cosmos::protocolpool::v1::MsgCancelContinuousFund {
    fn from(
        msg: &MsgCancelContinuousFund,
    ) -> proto::cosmos::protocolpool::v1::MsgCancelContinuousFund {
        proto::cosmos::protocolpool::v1::MsgCancelContinuousFund {
            authority: msg.authority.to_string(),
            recipient: msg.recipient.to_string(),
        }
    }
}
