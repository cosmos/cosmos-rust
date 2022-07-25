//! Registry of type URLs associated with various protobuf types defined in
//! this crate.

// TODO(tarcieri): leverage first-class support for type URLs in prost?
// See: https://github.com/tokio-rs/prost/issues/299

use crate::{cosmos, traits::TypeUrl};

#[cfg(feature = "cosmwasm")]
use crate::cosmwasm;

impl TypeUrl for cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl TypeUrl for cosmos::bank::v1beta1::MsgMultiSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgFundCommunityPool {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
}

impl TypeUrl for cosmos::feegrant::v1beta1::MsgGrantAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgGrantAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgRevokeAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::BasicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.BasicAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::PeriodicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.PeriodicAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.AllowedMsgAllowance";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

impl TypeUrl for cosmos::base::abci::v1beta1::MsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.MsgData";
}

impl TypeUrl for cosmos::base::abci::v1beta1::TxMsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.TxMsgData";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgStoreCode {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCode";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgInstantiateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContract";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContract";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgMigrateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContract";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgUpdateAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdmin";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgClearAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdmin";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgStoreCodeResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCodeResponse";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgInstantiateContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgExecuteContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgMigrateContractResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContractResponse";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgUpdateAdminResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdminResponse";
}

#[cfg(feature = "cosmwasm")]
impl TypeUrl for cosmwasm::wasm::v1::MsgClearAdminResponse {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdminResponse";
}
