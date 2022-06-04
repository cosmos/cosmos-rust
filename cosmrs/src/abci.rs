//! Abci-related functionality

use crate::tx::Msg;
use crate::{proto, ErrorReport, Result};
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint::abci::Gas;

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

impl TryFrom<tendermint::abci::Data> for TxMsgData {
    type Error = ErrorReport;

    fn try_from(data: tendermint::abci::Data) -> Result<TxMsgData> {
        let proto = proto::cosmos::base::abci::v1beta1::TxMsgData::decode(data.value().as_ref())?;
        proto.try_into()
    }
}

impl Msg for TxMsgData {
    type Proto = proto::cosmos::base::abci::v1beta1::TxMsgData;
}

impl TryFrom<proto::cosmos::base::abci::v1beta1::TxMsgData> for TxMsgData {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::base::abci::v1beta1::TxMsgData) -> Result<TxMsgData> {
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
    fn from(tx_msg_data: TxMsgData) -> Self {
        proto::cosmos::base::abci::v1beta1::TxMsgData {
            data: tx_msg_data.data.into_iter().map(Into::into).collect(),
        }
    }
}

/// GasInfo defines tx execution gas context.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    pub gas_wanted: Gas,

    /// GasUsed is the amount of gas actually consumed.
    pub gas_used: Gas,
}

impl TryFrom<proto::cosmos::base::abci::v1beta1::GasInfo> for GasInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::base::abci::v1beta1::GasInfo) -> Result<GasInfo> {
        Ok(GasInfo {
            gas_wanted: Gas::from(proto.gas_wanted),
            gas_used: Gas::from(proto.gas_used),
        })
    }
}

impl From<GasInfo> for proto::cosmos::base::abci::v1beta1::GasInfo {
    fn from(info: GasInfo) -> Self {
        proto::cosmos::base::abci::v1beta1::GasInfo {
            gas_wanted: info.gas_wanted.value(),
            gas_used: info.gas_wanted.value(),
        }
    }
}
