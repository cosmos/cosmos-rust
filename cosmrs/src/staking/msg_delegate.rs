use crate::{proto, tx::Msg, AccountId, Coin, Error, ErrorReport, Result};

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

impl Msg for MsgDelegate {
    type Proto = proto::cosmos::staking::v1beta1::MsgDelegate;
}

impl TryFrom<proto::cosmos::staking::v1beta1::MsgDelegate> for MsgDelegate {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::staking::v1beta1::MsgDelegate) -> Result<MsgDelegate> {
        MsgDelegate::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::staking::v1beta1::MsgDelegate> for MsgDelegate {
    type Error = ErrorReport;

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
