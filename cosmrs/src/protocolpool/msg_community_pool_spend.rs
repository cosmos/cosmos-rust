use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;

use crate::{proto, tx::Msg, ErrorReport, Result};

/// MsgCommunityPoolSpend represents a message to set a withdraw address for staking rewards.
#[derive(Clone, Debug, PartialEq)]
pub struct MsgCommunityPoolSpend {
    pub authority: String,
    pub recipient: String,
    pub amount: Vec<Coin>,
}

impl Msg for MsgCommunityPoolSpend {
    type Proto = proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend;
}

impl TryFrom<proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend> for MsgCommunityPoolSpend {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend,
    ) -> Result<MsgCommunityPoolSpend> {
        MsgCommunityPoolSpend::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend> for MsgCommunityPoolSpend {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend,
    ) -> Result<MsgCommunityPoolSpend> {
        Ok(MsgCommunityPoolSpend {
            authority: proto.authority.parse()?,
            recipient: proto.recipient.parse()?,
            amount: proto.amount.clone(),
        })
    }
}

impl From<MsgCommunityPoolSpend> for proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend {
    fn from(coin: MsgCommunityPoolSpend) -> proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend {
        proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend::from(&coin)
    }
}

impl From<&MsgCommunityPoolSpend> for proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend {
    fn from(msg: &MsgCommunityPoolSpend) -> proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend {
        proto::cosmos::protocolpool::v1::MsgCommunityPoolSpend {
            authority: msg.authority.to_string(),
            recipient: msg.recipient.to_string(),
            amount: msg.amount.clone(),
        }
    }
}
