use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};

/// MsgSend represents a message to send coins from one account to another.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSend {
    /// Sender's address.
    pub from_address: AccountId,

    /// Recipient's address.
    pub to_address: AccountId,

    /// Amount to send
    pub amount: Vec<Coin>,
}

impl Msg for MsgSend {
    type Proto = proto::cosmos::bank::v1beta1::MsgSend;
}

impl TryFrom<proto::cosmos::bank::v1beta1::MsgSend> for MsgSend {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::bank::v1beta1::MsgSend) -> Result<MsgSend> {
        MsgSend::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::bank::v1beta1::MsgSend> for MsgSend {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::bank::v1beta1::MsgSend) -> Result<MsgSend> {
        Ok(MsgSend {
            from_address: proto.from_address.parse()?,
            to_address: proto.to_address.parse()?,
            amount: proto
                .amount
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgSend> for proto::cosmos::bank::v1beta1::MsgSend {
    fn from(coin: MsgSend) -> proto::cosmos::bank::v1beta1::MsgSend {
        proto::cosmos::bank::v1beta1::MsgSend::from(&coin)
    }
}

impl From<&MsgSend> for proto::cosmos::bank::v1beta1::MsgSend {
    fn from(msg: &MsgSend) -> proto::cosmos::bank::v1beta1::MsgSend {
        proto::cosmos::bank::v1beta1::MsgSend {
            from_address: msg.from_address.to_string(),
            to_address: msg.to_address.to_string(),
            amount: msg.amount.iter().map(Into::into).collect(),
        }
    }
}
