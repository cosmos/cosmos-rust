use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgWithdrawDelegatorReward represents a message to withdraw a delegator's reward from a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgWithdrawDelegatorReward {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// Validator's address.
    pub validator_address: AccountId,
}

impl Msg for MsgWithdrawDelegatorReward {
    type Proto = proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward;
}

impl TryFrom<proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward>
    for MsgWithdrawDelegatorReward
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward,
    ) -> Result<MsgWithdrawDelegatorReward> {
        MsgWithdrawDelegatorReward::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward>
    for MsgWithdrawDelegatorReward
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward,
    ) -> Result<MsgWithdrawDelegatorReward> {
        Ok(MsgWithdrawDelegatorReward {
            delegator_address: proto.delegator_address.parse()?,
            validator_address: proto.validator_address.parse()?,
        })
    }
}

impl From<MsgWithdrawDelegatorReward>
    for proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward
{
    fn from(
        coin: MsgWithdrawDelegatorReward,
    ) -> proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
        proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward::from(&coin)
    }
}

impl From<&MsgWithdrawDelegatorReward>
    for proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward
{
    fn from(
        msg: &MsgWithdrawDelegatorReward,
    ) -> proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
        proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
            delegator_address: msg.delegator_address.to_string(),
            validator_address: msg.validator_address.to_string(),
        }
    }
}
