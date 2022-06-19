//! Bank module support
//!
//! <https://docs.cosmos.network/master/modules/bank/>

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

/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMultiSend {
    /// Sender account/amount pairs.
    pub inputs: Vec<MultiSendIo>,

    /// Recipient account/amount pairs.
    pub outputs: Vec<MultiSendIo>,
}

impl Msg for MsgMultiSend {
    type Proto = proto::cosmos::bank::v1beta1::MsgMultiSend;
}

impl TryFrom<proto::cosmos::bank::v1beta1::MsgMultiSend> for MsgMultiSend {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::bank::v1beta1::MsgMultiSend) -> Result<MsgMultiSend> {
        MsgMultiSend::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::bank::v1beta1::MsgMultiSend> for MsgMultiSend {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::bank::v1beta1::MsgMultiSend) -> Result<MsgMultiSend> {
        Ok(MsgMultiSend {
            inputs: proto
                .inputs
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            outputs: proto
                .outputs
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgMultiSend> for proto::cosmos::bank::v1beta1::MsgMultiSend {
    fn from(coin: MsgMultiSend) -> proto::cosmos::bank::v1beta1::MsgMultiSend {
        proto::cosmos::bank::v1beta1::MsgMultiSend::from(&coin)
    }
}

impl From<&MsgMultiSend> for proto::cosmos::bank::v1beta1::MsgMultiSend {
    fn from(msg: &MsgMultiSend) -> proto::cosmos::bank::v1beta1::MsgMultiSend {
        proto::cosmos::bank::v1beta1::MsgMultiSend {
            inputs: msg.inputs.iter().map(Into::into).collect(),
            outputs: msg.outputs.iter().map(Into::into).collect(),
        }
    }
}

/// Represents a MultiSend Input or Output
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MultiSendIo {
    /// The address that `coins` will be sent to/from
    pub address: AccountId,

    /// The coins to send to/from `address`
    pub coins: Vec<Coin>,
}

impl TryFrom<proto::cosmos::bank::v1beta1::Input> for MultiSendIo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::bank::v1beta1::Input) -> Result<MultiSendIo> {
        MultiSendIo::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::bank::v1beta1::Input> for MultiSendIo {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::bank::v1beta1::Input) -> Result<MultiSendIo> {
        Ok(MultiSendIo {
            address: proto.address.parse()?,
            coins: proto
                .coins
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<proto::cosmos::bank::v1beta1::Output> for MultiSendIo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::bank::v1beta1::Output) -> Result<MultiSendIo> {
        MultiSendIo::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::bank::v1beta1::Output> for MultiSendIo {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::bank::v1beta1::Output) -> Result<MultiSendIo> {
        Ok(MultiSendIo {
            address: proto.address.parse()?,
            coins: proto
                .coins
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MultiSendIo> for proto::cosmos::bank::v1beta1::Output {
    fn from(output: MultiSendIo) -> proto::cosmos::bank::v1beta1::Output {
        proto::cosmos::bank::v1beta1::Output::from(&output)
    }
}

impl From<&MultiSendIo> for proto::cosmos::bank::v1beta1::Output {
    fn from(output: &MultiSendIo) -> proto::cosmos::bank::v1beta1::Output {
        proto::cosmos::bank::v1beta1::Output {
            address: output.address.to_string(),
            coins: output.coins.iter().map(Into::into).collect(),
        }
    }
}

impl From<MultiSendIo> for proto::cosmos::bank::v1beta1::Input {
    fn from(input: MultiSendIo) -> proto::cosmos::bank::v1beta1::Input {
        proto::cosmos::bank::v1beta1::Input::from(&input)
    }
}

impl From<&MultiSendIo> for proto::cosmos::bank::v1beta1::Input {
    fn from(input: &MultiSendIo) -> proto::cosmos::bank::v1beta1::Input {
        proto::cosmos::bank::v1beta1::Input {
            address: input.address.to_string(),
            coins: input.coins.iter().map(Into::into).collect(),
        }
    }
}
