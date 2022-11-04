use crate::{proto, tx::Msg, AccountId, Coin, Error, ErrorReport, Result};

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
    pub amount: Coin,
}

impl Msg for MsgBeginRedelegate {
    type Proto = proto::cosmos::staking::v1beta1::MsgBeginRedelegate;
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgBeginRedelegate> for MsgBeginRedelegate {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::staking::v1beta1::MsgBeginRedelegate,
    ) -> Result<MsgBeginRedelegate> {
        MsgBeginRedelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgBeginRedelegate> for MsgBeginRedelegate {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::staking::v1beta1::MsgBeginRedelegate,
    ) -> Result<MsgBeginRedelegate> {
        let amount = proto
            .amount
            .as_ref()
            .ok_or(Error::MissingField { name: "amount" })?;

        Ok(MsgBeginRedelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_src_address: proto.validator_src_address.parse()?,
            validator_dst_address: proto.validator_dst_address.parse()?,
            amount: Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            },
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
        let amount = proto::cosmos::base::v1beta1::Coin {
            denom: msg.amount.denom.to_string(),
            amount: msg.amount.amount.to_string(),
        };

        proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_src_address: msg.validator_src_address.to_string(),
            validator_dst_address: msg.validator_dst_address.to_string(),
            amount: Some(amount),
        }
    }
}
