//! Distribution module support
//!
//! <https://docs.cosmos.network/master/modules/distribution/>

use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};

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

/// MsgFundCommunityPool represents a message to send coins from depositor to the community pool.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgFundCommunityPool {
    /// Depositor's address.
    pub depositor: AccountId,

    /// Amount to deposit.
    pub amount: Vec<Coin>,
}

impl Msg for MsgFundCommunityPool {
    type Proto = proto::cosmos::distribution::v1beta1::MsgFundCommunityPool;
}

impl TryFrom<proto::cosmos::distribution::v1beta1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::distribution::v1beta1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        MsgFundCommunityPool::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::distribution::v1beta1::MsgFundCommunityPool> for MsgFundCommunityPool {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::distribution::v1beta1::MsgFundCommunityPool,
    ) -> Result<MsgFundCommunityPool> {
        let mut amounts = vec![];
        for amount in proto.amount.iter() {
            amounts.push(Coin {
                denom: amount.denom.parse()?,
                amount: amount.amount.parse()?,
            })
        }
        Ok(MsgFundCommunityPool {
            depositor: proto.depositor.parse()?,
            amount: amounts,
        })
    }
}

impl From<MsgFundCommunityPool> for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    fn from(
        coin: MsgFundCommunityPool,
    ) -> proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
        proto::cosmos::distribution::v1beta1::MsgFundCommunityPool::from(&coin)
    }
}

impl From<&MsgFundCommunityPool> for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    fn from(
        msg: &MsgFundCommunityPool,
    ) -> proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
        let mut amounts = vec![];
        for amount in msg.amount.iter() {
            amounts.push(proto::cosmos::base::v1beta1::Coin {
                denom: amount.denom.to_string(),
                amount: amount.amount.to_string(),
            })
        }
        proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
            depositor: "".to_string(),
            amount: amounts,
        }
    }
}
