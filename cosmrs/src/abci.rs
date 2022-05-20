//! Abci-related functionality

use crate::tx::Msg;
use crate::{proto, ErrorReport, Result};
use prost::Message;
use prost_types::Any;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgData {
    pub msg_type: String,
    pub data: Vec<u8>,
}

impl MsgData {
    // MsgData has the same structure as `Any` and is actually treated as such,
    // so abuse this fact to attempt to decode it into the corresponding proto type.
    pub fn try_decode<M: Msg>(&self) -> Result<M> {
        let as_any = Any {
            type_url: self.msg_type.clone(),
            value: self.data.clone(),
        };
        
        M::from_any(&as_any)
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

impl<'a> TryFrom<&'a [u8]> for TxMsgData {
    type Error = ErrorReport;

    fn try_from(value: &'a [u8]) -> Result<TxMsgData> {
        let proto = proto::cosmos::base::abci::v1beta1::TxMsgData::decode(value)?;
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
