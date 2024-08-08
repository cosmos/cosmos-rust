use super::Data;
use crate::{
    proto::{self, traits::Message, Any},
    tx::Msg,
    ErrorReport, Result,
};
use eyre::eyre;
use serde::{Deserialize, Serialize};

/// MsgData defines the data returned in a Result object during message execution.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
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

/// The messages responses of the TxMsgData. Corresponds to `Any`
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MsgResponse {
    /// Response message type that emitted this result data, for example `"/cosmwasm.wasm.v1.MsgExecuteContractResponse"`.
    pub type_url: String,

    /// Binary data emitted by this response.
    pub value: Vec<u8>,
}

impl MsgResponse {
    /// Attempts to decode the `data` field of this result into the specified `Msg` type.
    pub fn try_decode_as<M: Msg>(&self) -> Result<M> {
        M::Proto::decode(&*self.value)?.try_into()
    }
}

impl From<Any> for MsgResponse {
    fn from(any: Any) -> Self {
        MsgResponse {
            type_url: any.type_url,
            value: any.value,
        }
    }
}

impl From<MsgResponse> for Any {
    fn from(msg_response: MsgResponse) -> Self {
        Any {
            type_url: msg_response.type_url,
            value: msg_response.value,
        }
    }
}

/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object for each message.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TxMsgData {
    /// Data emitted by the messages in a particular transaction.
    // Note: this field will be deprecated and not populated as of cosmos-sdk 0.46.
    // It will be superseded by `msg_responses` field of type Vec<Any>
    pub data: Vec<MsgData>,

    /// This field contains the Msg handler responses packed into Anys.
    // Note: this field is an empty vec for chains running cosmos-sdk < 0.46.
    pub msg_responses: Vec<MsgResponse>,
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
        // this case should be impossible as with the switch in cosmos-sdk 0.46 only one of those should contain any data
        if !proto.msg_responses.is_empty() && !proto.data.is_empty() {
            return Err(eyre!(
                "TxMsgData: both msg_responses and data fields are populated"
            ));
        }

        Ok(TxMsgData {
            data: proto
                .data
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            msg_responses: proto.msg_responses.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<TxMsgData> for proto::cosmos::base::abci::v1beta1::TxMsgData {
    #[allow(deprecated)]
    fn from(tx_msg_data: TxMsgData) -> Self {
        proto::cosmos::base::abci::v1beta1::TxMsgData {
            data: tx_msg_data.data.into_iter().map(Into::into).collect(),
            msg_responses: tx_msg_data
                .msg_responses
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
