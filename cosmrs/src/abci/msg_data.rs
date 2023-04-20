use super::Data;
use crate::{
    proto::{self, traits::Message},
    tx::Msg,
    ErrorReport, Result,
};
use eyre::eyre;

/// MsgData defines the data returned in a Result object during message execution.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgData {
    /// Incoming message type that emitted this result data, for example `"/cosmos.bank.v1beta1.MsgSend"`.
    pub msg_type: String,

    /// Binary data emitted by this message.
    // Do note that usually the data has to be decoded into the corresponding protobuf `Response` type.
    // For example, if the data was emitted as a result of a `MsgSend`, i.e. `msg.msg_type == "/cosmos.bank.v1beta1.MsgSend"`,
    // then you should decode it into `"/cosmos.bank.v1beta1.MsgSendResponse"
    pub data: Vec<u8>,
}

impl MsgData {
    /// Attempts to decode the `data` field of this result into the specified `Msg` type.
    pub fn try_decode_as<M: Msg>(&self) -> Result<M> {
        M::Proto::decode(&*self.data)?.try_into()
    }
}

impl Msg for MsgData {
    type Proto = proto::cosmos::base::abci::v1beta1::MsgData;
}

impl TryFrom<proto::cosmos::base::abci::v1beta1::MsgData> for MsgData {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::base::abci::v1beta1::MsgData) -> Result<MsgData> {
        Ok(MsgData {
            msg_type: proto.msg_type,
            data: proto.data,
        })
    }
}

impl From<MsgData> for proto::cosmos::base::abci::v1beta1::MsgData {
    fn from(msg_data: MsgData) -> Self {
        proto::cosmos::base::abci::v1beta1::MsgData {
            msg_type: msg_data.msg_type,
            data: msg_data.data,
        }
    }
}

/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object for each message.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TxMsgData {
    /// Data emitted by the messages in a particular transaction.
    // Note: this field will be deprecated and not populated as of cosmos-sdk 0.46.
    // It will be superseded by `msg_responses` field of type Vec<Any>
    pub data: Vec<MsgData>,
}

impl TryFrom<Data> for TxMsgData {
    type Error = ErrorReport;

    fn try_from(data: Data) -> Result<TxMsgData> {
        proto::cosmos::base::abci::v1beta1::TxMsgData::decode(data.as_ref())?.try_into()
    }
}

impl Msg for TxMsgData {
    type Proto = proto::cosmos::base::abci::v1beta1::TxMsgData;
}

impl TryFrom<proto::cosmos::base::abci::v1beta1::TxMsgData> for TxMsgData {
    type Error = ErrorReport;

    #[allow(deprecated)]
    fn try_from(proto: proto::cosmos::base::abci::v1beta1::TxMsgData) -> Result<TxMsgData> {
        // TODO(tarcieri): parse `msg_responses`
        if !proto.msg_responses.is_empty() {
            return Err(eyre!("TxMsgData::msg_responses unsupported"));
        }

        Ok(TxMsgData {
            data: proto
                .data
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<TxMsgData> for proto::cosmos::base::abci::v1beta1::TxMsgData {
    #[allow(deprecated)]
    fn from(tx_msg_data: TxMsgData) -> Self {
        proto::cosmos::base::abci::v1beta1::TxMsgData {
            data: tx_msg_data.data.into_iter().map(Into::into).collect(),
            msg_responses: vec![], // TODO(tarcieri): serialize responses
        }
    }
}
