use crate::{proto, tx::Msg, AccountId, Coin, Error, ErrorReport, Result};

/// MsgUndelegate represents a message to undelegate coins from a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUndelegate {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// Validator's address.
    pub validator_address: AccountId,

    /// Amount to UnDelegate
    pub amount: Coin,
}

impl Msg for MsgUndelegate {
    type Proto = proto::cosmos::staking::v1beta1::MsgUndelegate;
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgUndelegate> for MsgUndelegate {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::staking::v1beta1::MsgUndelegate) -> Result<MsgUndelegate> {
        MsgUndelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgUndelegate> for MsgUndelegate {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::staking::v1beta1::MsgUndelegate) -> Result<MsgUndelegate> {
        let amount = proto
            .amount
            .as_ref()
            .ok_or(Error::MissingField { name: "amount" })?;

        Ok(MsgUndelegate {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
            amount: Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            },
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
        let amount = proto::cosmos::base::v1beta1::Coin {
            denom: msg.amount.denom.to_string(),
            amount: msg.amount.amount.to_string(),
        };

        proto::cosmos::staking::v1beta1::MsgUndelegate {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
            amount: Some(amount),
        }
    }
}
