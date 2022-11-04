use super::MultiSendIo;
use crate::{proto, tx::Msg, ErrorReport, Result};

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
