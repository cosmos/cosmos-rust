use crate::{proto, AccountId, Coin, ErrorReport, Result};

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
