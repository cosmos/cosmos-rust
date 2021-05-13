//! Staking module support
//!
//! <https://docs.cosmos.network/master/modules/staking/>

use crate::{
    proto,
    tx::{Msg, MsgType},
    AccountId, Coin, Result,
};
use std::convert::{TryFrom, TryInto};

/// MsgDelegate represents a message to delegate coins to a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgDelegate {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// Validator's address.
    pub validator_address: AccountId,

    /// Amount to send
    pub amount: Option<Coin>,
}

impl MsgType for MsgDelegate {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmos::staking::v1beta1::MsgDelegate::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmos::staking::v1beta1::MsgDelegate::from(self).to_msg()
    }
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgDelegate> for MsgDelegate {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmos::staking::v1beta1::MsgDelegate) -> Result<MsgDelegate> {
        MsgDelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgDelegate> for MsgDelegate {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmos::staking::v1beta1::MsgDelegate) -> Result<MsgDelegate> {
        Ok(MsgDelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
            amount: None, // proto.amount.try_into()?,
        })
    }
}

impl From<MsgDelegate> for proto::cosmos::staking::v1beta1::MsgDelegate {
    fn from(coin: MsgDelegate) -> proto::cosmos::staking::v1beta1::MsgDelegate {
        proto::cosmos::staking::v1beta1::MsgDelegate::from(&coin)
    }
}

impl From<&MsgDelegate> for proto::cosmos::staking::v1beta1::MsgDelegate {
    fn from(msg: &MsgDelegate) -> proto::cosmos::staking::v1beta1::MsgDelegate {
        proto::cosmos::staking::v1beta1::MsgDelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: None, // msg.amount.into(),
        }
    }
}

// impl MsgProto for proto::cosmos::staking::v1beta1::MsgDelegate {
//     const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
// }