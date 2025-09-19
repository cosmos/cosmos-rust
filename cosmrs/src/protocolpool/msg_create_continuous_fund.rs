use cosmos_sdk_proto::Timestamp;

use crate::{proto, tx::Msg, ErrorReport, Result};

/// MsgCreateContinuousFund represents a message to withdraw a delegator's reward from a validator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsgCreateContinuousFund {
    pub authority: String,
    pub recipient: String,
    pub percentage: String,
    pub expiry: Option<Timestamp>,
}

impl Msg for MsgCreateContinuousFund {
    type Proto = proto::cosmos::protocolpool::v1::MsgCreateContinuousFund;
}

impl TryFrom<proto::cosmos::protocolpool::v1::MsgCreateContinuousFund> for MsgCreateContinuousFund {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::protocolpool::v1::MsgCreateContinuousFund,
    ) -> Result<MsgCreateContinuousFund> {
        MsgCreateContinuousFund::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::protocolpool::v1::MsgCreateContinuousFund>
    for MsgCreateContinuousFund
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::protocolpool::v1::MsgCreateContinuousFund,
    ) -> Result<MsgCreateContinuousFund> {
        Ok(MsgCreateContinuousFund {
            authority: proto.authority.parse()?,
            recipient: proto.recipient.parse()?,
            percentage: proto.percentage.parse()?,
            expiry: proto.expiry,
        })
    }
}

impl From<MsgCreateContinuousFund> for proto::cosmos::protocolpool::v1::MsgCreateContinuousFund {
    fn from(
        coin: MsgCreateContinuousFund,
    ) -> proto::cosmos::protocolpool::v1::MsgCreateContinuousFund {
        proto::cosmos::protocolpool::v1::MsgCreateContinuousFund::from(&coin)
    }
}

impl From<&MsgCreateContinuousFund> for proto::cosmos::protocolpool::v1::MsgCreateContinuousFund {
    fn from(
        msg: &MsgCreateContinuousFund,
    ) -> proto::cosmos::protocolpool::v1::MsgCreateContinuousFund {
        proto::cosmos::protocolpool::v1::MsgCreateContinuousFund {
            authority: msg.authority.to_string(),
            recipient: msg.recipient.to_string(),
            percentage: msg.percentage.to_string(),
            expiry: msg.expiry,
        }
    }
}
