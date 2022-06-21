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

impl MsgProto for proto::cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl MsgProto for proto::cosmos::bank::v1beta1::MsgMultiSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
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

impl MsgProto for proto::cosmos::feegrant::v1beta1::MsgGrantAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgGrantAllowance";
}

impl MsgProto for proto::cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgRevokeAllowance";
}

impl MsgProto for proto::cosmos::feegrant::v1beta1::BasicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.BasicAllowance";
}

impl MsgProto for proto::cosmos::feegrant::v1beta1::PeriodicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.PeriodicAllowance";
}

impl MsgProto for proto::cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.AllowedMsgAllowance";
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

impl MsgProto for proto::cosmos::base::abci::v1beta1::MsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.MsgData";
}

impl MsgProto for proto::cosmos::base::abci::v1beta1::TxMsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.TxMsgData";
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

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCodeResponse";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgExecuteContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgMigrateContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdminResponse";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1::MsgClearAdminResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdminResponse";
}
