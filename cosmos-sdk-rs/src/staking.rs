//! Staking module support
//!
//! <https://docs.cosmos.network/master/modules/staking/>

use crate::{
    proto,
    tx::{Msg, MsgType},
    AccountId, Coin, Error, Result,
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
    pub amount: Coin,
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
        let amount = proto
            .amount
            .as_ref()
            .ok_or(Error::MissingField { name: "amount" })?;

        Ok(MsgDelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
            amount: Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            },
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
        let amount = proto::cosmos::base::v1beta1::Coin {
            denom: msg.amount.denom.to_string(),
            amount: msg.amount.amount.to_string(),
        };

        proto::cosmos::staking::v1beta1::MsgDelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: Some(amount),
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
        let proto_amount = msg
            .amount
            .as_ref()
            .map(|amount| proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            });
        proto::cosmos::staking::v1beta1::MsgUndelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: proto_amount,
        }
    }
}

/// MsgBeginRedelegate represents a message to redelegate coins from one validator to another.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgBeginRedelegate {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// Source validator's address.
    pub validator_src_address: AccountId,

    /// Destination validator's address.
    pub validator_dst_address: AccountId,

    /// Amount to UnDelegate
    pub amount: Option<Coin>,
}

impl MsgType for MsgBeginRedelegate {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmos::staking::v1beta1::MsgBeginRedelegate::from_msg(msg)
            .and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmos::staking::v1beta1::MsgBeginRedelegate::from(self).to_msg()
    }
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgBeginRedelegate> for MsgBeginRedelegate {
    type Error = eyre::Report;

    fn try_from(
        proto: proto::cosmos::staking::v1beta1::MsgBeginRedelegate,
    ) -> Result<MsgBeginRedelegate> {
        MsgBeginRedelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgBeginRedelegate> for MsgBeginRedelegate {
    type Error = eyre::Report;

    fn try_from(
        proto: &proto::cosmos::staking::v1beta1::MsgBeginRedelegate,
    ) -> Result<MsgBeginRedelegate> {
        let amount = if let Some(amount) = &proto.amount {
            Some(Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            })
        } else {
            None
        };
        Ok(MsgBeginRedelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_src_address: proto.validator_src_address.parse()?,
            validator_dst_address: proto.validator_dst_address.parse()?,
            amount,
        })
    }
}

impl From<MsgBeginRedelegate> for proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
    fn from(coin: MsgBeginRedelegate) -> proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
        proto::cosmos::staking::v1beta1::MsgBeginRedelegate::from(&coin)
    }
}

impl From<&MsgBeginRedelegate> for proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
    fn from(msg: &MsgBeginRedelegate) -> proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
        let proto_amount = msg
            .amount
            .as_ref()
            .map(|amount| proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            });
        proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_src_address: msg.validator_src_address.to_string(),
            validator_dst_address: msg.validator_dst_address.to_string(),
            amount: proto_amount,
        }
    }
}
