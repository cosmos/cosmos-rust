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
        let amount = if let Some(amount) = &proto.amount {
            Some(Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            })
        } else {
            None
        };
        Ok(MsgDelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
            amount,
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
        let proto_amount = if let Some(amount) = &msg.amount {
            Some(proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            })
        } else {
            None
        };
        proto::cosmos::staking::v1beta1::MsgDelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: proto_amount,
        }
    }
}

/// MsgUndelegate represents a message to undelegate coins from a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUndelegate {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// Validator's address.
    pub validator_address: AccountId,

    /// Amount to UnDelegate
    pub amount: Option<Coin>,
}

impl MsgType for MsgUndelegate {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmos::staking::v1beta1::MsgUndelegate::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmos::staking::v1beta1::MsgUndelegate::from(self).to_msg()
    }
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgUndelegate> for MsgUndelegate {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmos::staking::v1beta1::MsgUndelegate) -> Result<MsgUndelegate> {
        MsgUndelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgUndelegate> for MsgUndelegate {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmos::staking::v1beta1::MsgUndelegate) -> Result<MsgUndelegate> {
        let amount = if let Some(amount) = &proto.amount {
            Some(Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            })
        } else {
            None
        };
        Ok(MsgUndelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
            amount,
        })
    }
}

impl From<MsgUndelegate> for proto::cosmos::staking::v1beta1::MsgUndelegate {
    fn from(coin: MsgUndelegate) -> proto::cosmos::staking::v1beta1::MsgUndelegate {
        proto::cosmos::staking::v1beta1::MsgUndelegate::from(&coin)
    }
}

impl From<&MsgUndelegate> for proto::cosmos::staking::v1beta1::MsgUndelegate {
    fn from(msg: &MsgUndelegate) -> proto::cosmos::staking::v1beta1::MsgUndelegate {
        let proto_amount = if let Some(amount) = &msg.amount {
            Some(proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            })
        } else {
            None
        };
        proto::cosmos::staking::v1beta1::MsgUndelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: proto_amount,
        }
    }
}