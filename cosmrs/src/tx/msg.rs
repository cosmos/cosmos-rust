//! Transaction messages

use crate::{prost_ext::MessageExt, proto, Any, Error, ErrorReport, Result};

/// Message types.
///
/// Types which impl this trait map one-to-one with a corresponding Protocol
/// Buffers type, but can assert additional invariants and/or additional
/// functionality beyond the raw proto, as well as providing a more idiomatic
/// Rust type to work with.
pub trait Msg:
    Clone + Sized + TryFrom<Self::Proto, Error = ErrorReport> + Into<Self::Proto>
{
    /// Protocol Buffers type
    type Proto: MsgProto;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self> {
        Self::Proto::from_any(any)?.try_into()
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any> {
        self.clone().into_any()
    }

    /// Convert this message proto into [`Any`].
    fn into_any(self) -> Result<Any> {
        self.into().to_any()
    }
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: Default + MessageExt + Sized {
    /// Type URL value
    const TYPE_URL: &'static str;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self> {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value)?)
        } else {
            Err(Error::MsgType {
                expected: Self::TYPE_URL,
                found: any.type_url.clone(),
            }
            .into())
        }
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any> {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }
}

impl MsgProto for proto::cosmos::vesting::v1beta1::PeriodicVestingAccount {
    const TYPE_URL: &'static str = "/cosmos.vesting.v1beta1.PeriodicVestingAccount";
}

impl MsgProto for proto::cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgStoreCode {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCode";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgInstantiateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgMigrateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdmin";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgClearAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdmin";
}
