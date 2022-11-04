use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// WithdrawValidatorCommission represents a message to withdraw a validator's staking commission.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgWithdrawValidatorCommission {
    /// Validator's address.
    pub validator_address: AccountId,
}

impl Msg for MsgWithdrawValidatorCommission {
    type Proto = proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission;
}

impl TryFrom<proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission>
    for MsgWithdrawValidatorCommission
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission,
    ) -> Result<MsgWithdrawValidatorCommission> {
        MsgWithdrawValidatorCommission::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission>
    for MsgWithdrawValidatorCommission
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission,
    ) -> Result<MsgWithdrawValidatorCommission> {
        Ok(MsgWithdrawValidatorCommission {
            validator_address: proto.validator_address.parse()?,
        })
    }
}

impl From<MsgWithdrawValidatorCommission>
    for proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission
{
    fn from(
        coin: MsgWithdrawValidatorCommission,
    ) -> proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
        proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission::from(&coin)
    }
}

impl From<&MsgWithdrawValidatorCommission>
    for proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission
{
    fn from(
        msg: &MsgWithdrawValidatorCommission,
    ) -> proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
        proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
            validator_address: msg.validator_address.to_string(),
        }
    }
}
