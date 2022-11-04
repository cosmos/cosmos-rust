use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgSetWithdrawAddress represents a message to set a withdraw address for staking rewards.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSetWithdrawAddress {
    /// Delegator's address.
    pub delegator_address: AccountId,

    /// withdraw address.
    pub withdraw_address: AccountId,
}

impl Msg for MsgSetWithdrawAddress {
    type Proto = proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress;
}

impl TryFrom<proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress>
    for MsgSetWithdrawAddress
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress,
    ) -> Result<MsgSetWithdrawAddress> {
        MsgSetWithdrawAddress::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress>
    for MsgSetWithdrawAddress
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress,
    ) -> Result<MsgSetWithdrawAddress> {
        Ok(MsgSetWithdrawAddress {
            delegator_address: proto.delegator_address.parse()?,
            withdraw_address: proto.withdraw_address.parse()?,
        })
    }
}

impl From<MsgSetWithdrawAddress> for proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    fn from(
        coin: MsgSetWithdrawAddress,
    ) -> proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
        proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress::from(&coin)
    }
}

impl From<&MsgSetWithdrawAddress> for proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    fn from(
        msg: &MsgSetWithdrawAddress,
    ) -> proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
        proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
            delegator_address: msg.delegator_address.to_string(),
            withdraw_address: msg.withdraw_address.to_string(),
        }
    }
}
