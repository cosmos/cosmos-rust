//! Bank module support
//!
//! <https://docs.cosmos.network/master/modules/bank/>

use crate::{
    tx::{Msg, MsgType},
    AccountId, Coin, Result,
};
use cosmos_sdk_proto::cosmos;
use std::convert::{TryFrom, TryInto};

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

impl MsgType for MsgSend {
    fn from_msg(msg: &Msg) -> Result<Self> {
        cosmos::bank::v1beta1::MsgSend::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        cosmos::bank::v1beta1::MsgSend::from(self).to_msg()
    }
}

impl TryFrom<cosmos::bank::v1beta1::MsgSend> for MsgSend {
    type Error = eyre::Report;

    fn try_from(proto: cosmos::bank::v1beta1::MsgSend) -> Result<MsgSend> {
        MsgSend::try_from(&proto)
    }
}

impl TryFrom<&cosmos::bank::v1beta1::MsgSend> for MsgSend {
    type Error = eyre::Report;

    fn try_from(proto: &cosmos::bank::v1beta1::MsgSend) -> Result<MsgSend> {
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

impl From<MsgSend> for cosmos::bank::v1beta1::MsgSend {
    fn from(coin: MsgSend) -> cosmos::bank::v1beta1::MsgSend {
        cosmos::bank::v1beta1::MsgSend::from(&coin)
    }
}

impl From<&MsgSend> for cosmos::bank::v1beta1::MsgSend {
    fn from(msg: &MsgSend) -> cosmos::bank::v1beta1::MsgSend {
        cosmos::bank::v1beta1::MsgSend {
            from_address: msg.from_address.to_string(),
            to_address: msg.to_address.to_string(),
            amount: msg.amount.iter().map(Into::into).collect(),
        }
    }
}
