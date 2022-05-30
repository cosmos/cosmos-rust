//! Abci-related functionality

use crate::tx::Msg;
use crate::{proto, ErrorReport, Result};
use prost::Message;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgData {
    pub msg_type: String,
    pub data: Vec<u8>,
}

impl MsgData {
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

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TxMsgData {
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