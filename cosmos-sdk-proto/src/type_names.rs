//! Type name registry: used to compute type URLs.

// TODO(tarcieri): generate these automatically using `prost-build`
// See: https://github.com/tokio-rs/prost/issues/926

use crate::{cosmos, ibc, traits::Name};

macro_rules! impl_name {
    ($type:ty, $package:expr, $name:expr) => {
        impl Name for $type {
            const NAME: &'static str = $name;
            const PACKAGE: &'static str = $package;

            fn full_name() -> String {
                full_name::<Self>()
            }
        }
    };
}

impl_name!(
    ibc::core::client::v1::ClientUpdateProposal,
    "ibc.core.client.v1",
    "ClientUpdateProposal"
);
impl_name!(
    ibc::core::client::v1::MsgUpdateClient,
    "ibc.core.client.v1",
    "MsgUpdateClient"
);

impl_name!(
    ibc::core::channel::v1::MsgChannelOpenConfirm,
    "ibc.core.channel.v1",
    "MsgChannelOpenConfirm"
);
impl_name!(
    ibc::core::channel::v1::MsgAcknowledgement,
    "ibc.core.channel.v1",
    "MsgAcknowledgement"
);
impl_name!(
    ibc::core::channel::v1::MsgChannelOpenAck,
    "ibc.core.channel.v1",
    "MsgChannelOpenAck"
);
impl_name!(
    ibc::core::channel::v1::MsgTimeout,
    "ibc.core.channel.v1",
    "MsgTimeout"
);
impl_name!(
    ibc::core::channel::v1::MsgRecvPacket,
    "ibc.core.channel.v1",
    "MsgRecvPacket"
);

impl_name!(
    ibc::applications::transfer::v1::MsgTransfer,
    "ibc.applications.transfer.v1",
    "MsgTransfer"
);

impl_name!(
    cosmos::upgrade::v1beta1::SoftwareUpgradeProposal,
    "cosmos.upgrade.v1beta1",
    "SoftwareUpgradeProposal"
);

impl_name!(
    cosmos::params::v1beta1::ParameterChangeProposal,
    "cosmos.params.v1beta1",
    "ParameterChangeProposal"
);

impl_name!(
    cosmos::gov::v1beta1::TextProposal,
    "cosmos.gov.v1beta1",
    "TextProposal"
);
impl_name!(
    cosmos::gov::v1beta1::MsgSubmitProposal,
    "cosmos.gov.v1beta1",
    "MsgSubmitProposal"
);
impl_name!(
    cosmos::gov::v1beta1::MsgDeposit,
    "cosmos.gov.v1beta1",
    "MsgDeposit"
);
impl_name!(
    cosmos::gov::v1beta1::MsgVote,
    "cosmos.gov.v1beta1",
    "MsgVote"
);

impl_name!(
    cosmos::crypto::secp256k1::PubKey,
    "cosmos.crypto.secp256k1",
    "PubKey"
);

impl_name!(
    cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward,
    "cosmos.distribution.v1beta1",
    "MsgWithdrawDelegatorReward"
);
impl_name!(
    cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission,
    "cosmos.distribution.v1beta1",
    "MsgWithdrawValidatorCommission"
);
impl_name!(
    cosmos::distribution::v1beta1::MsgFundCommunityPool,
    "cosmos.distribution.v1beta1",
    "MsgFundCommunityPool"
);
impl_name!(
    cosmos::distribution::v1beta1::CommunityPoolSpendProposal,
    "cosmos.distribution.v1beta1",
    "CommunityPoolSpendProposal"
);
impl_name!(
    cosmos::distribution::v1beta1::MsgSetWithdrawAddress,
    "cosmos.distribution.v1beta1",
    "MsgSetWithdrawAddress"
);

impl_name!(
    cosmos::vesting::v1beta1::PeriodicVestingAccount,
    "cosmos.vesting.v1beta1",
    "PeriodicVestingAccount"
);

impl_name!(
    cosmos::bank::v1beta1::MsgSend,
    "cosmos.bank.v1beta1",
    "MsgSend"
);
impl_name!(
    cosmos::bank::v1beta1::MsgMultiSend,
    "cosmos.bank.v1beta1",
    "MsgMultiSend"
);

impl_name!(
    cosmos::feegrant::v1beta1::MsgGrantAllowance,
    "cosmos.feegrant.v1beta1",
    "MsgGrantAllowance"
);
impl_name!(
    cosmos::feegrant::v1beta1::MsgRevokeAllowance,
    "cosmos.feegrant.v1beta1",
    "MsgRevokeAllowance"
);
impl_name!(
    cosmos::feegrant::v1beta1::BasicAllowance,
    "cosmos.feegrant.v1beta1",
    "BasicAllowance"
);
impl_name!(
    cosmos::feegrant::v1beta1::PeriodicAllowance,
    "cosmos.feegrant.v1beta1",
    "PeriodicAllowance"
);
impl_name!(
    cosmos::feegrant::v1beta1::AllowedMsgAllowance,
    "cosmos.feegrant.v1beta1",
    "AllowedMsgAllowance"
);

impl_name!(
    cosmos::slashing::v1beta1::GenesisState,
    "cosmos.slashing.v1beta1",
    "MissedBlocks"
);
impl_name!(
    cosmos::slashing::v1beta1::MissedBlock,
    "cosmos.slashing.v1beta1",
    "MissedBlock"
);
impl_name!(
    cosmos::slashing::v1beta1::MsgUnjail,
    "cosmos.slashing.v1beta1",
    "MsgUnjail"
);
impl_name!(
    cosmos::slashing::v1beta1::MsgUnjailResponse,
    "cosmos.slashing.v1beta1",
    "MsgUnjailResponse"
);
impl_name!(
    cosmos::slashing::v1beta1::Params,
    "cosmos.slashing.v1beta1",
    "Params"
);
impl_name!(
    cosmos::slashing::v1beta1::QueryParamsRequest,
    "cosmos.slashing.v1beta1",
    "QueryParamsRequest"
);
impl_name!(
    cosmos::slashing::v1beta1::QueryParamsResponse,
    "cosmos.slashing.v1beta1",
    "QueryParamsResponse"
);
impl_name!(
    cosmos::slashing::v1beta1::QuerySigningInfoRequest,
    "cosmos.slashing.v1beta1",
    "QuerySigningInfoRequest"
);
impl_name!(
    cosmos::slashing::v1beta1::QuerySigningInfoResponse,
    "cosmos.slashing.v1beta1",
    "QuerySigningInfoResponse"
);
impl_name!(
    cosmos::slashing::v1beta1::QuerySigningInfosRequest,
    "cosmos.slashing.v1beta1",
    "QuerySigningInfosRequest"
);
impl_name!(
    cosmos::slashing::v1beta1::QuerySigningInfosResponse,
    "cosmos.slashing.v1beta1",
    "QuerySigningInfosResponse"
);
impl_name!(
    cosmos::slashing::v1beta1::SigningInfo,
    "cosmos.slashing.v1beta1",
    "SigningInfo"
);
impl_name!(
    cosmos::slashing::v1beta1::ValidatorMissedBlocks,
    "cosmos.slashing.v1beta1",
    "ValidatorMissedBlocks"
);
impl_name!(
    cosmos::slashing::v1beta1::ValidatorSigningInfo,
    "cosmos.slashing.v1beta1",
    "ValidatorSigningInfo"
);

impl_name!(
    cosmos::staking::v1beta1::MsgEditValidatorResponse,
    "cosmos.staking.v1beta1",
    "MsgEditValidatorResponse"
);
impl_name!(
    cosmos::staking::v1beta1::MsgCreateValidator,
    "cosmos.staking.v1beta1",
    "MsgCreateValidator"
);
impl_name!(
    cosmos::staking::v1beta1::MsgEditValidator,
    "cosmos.staking.v1beta1",
    "MsgEditValidator"
);
impl_name!(
    cosmos::staking::v1beta1::MsgDelegate,
    "cosmos.staking.v1beta1",
    "MsgDelegate"
);
impl_name!(
    cosmos::staking::v1beta1::MsgUndelegate,
    "cosmos.staking.v1beta1",
    "MsgUndelegate"
);
impl_name!(
    cosmos::staking::v1beta1::MsgBeginRedelegate,
    "cosmos.staking.v1beta1",
    "MsgBeginRedelegate"
);
impl_name!(
    cosmos::staking::v1beta1::MsgCreateValidatorResponse,
    "cosmos.staking.v1beta1",
    "MsgCreateValidatorResponse"
);
impl_name!(
    cosmos::staking::v1beta1::MsgDelegateResponse,
    "cosmos.staking.v1beta1",
    "MsgDelegateResponse"
);

impl_name!(
    cosmos::base::abci::v1beta1::MsgData,
    "cosmos.base.v1beta1.abci",
    "MsgData"
);
impl_name!(
    cosmos::base::abci::v1beta1::TxMsgData,
    "cosmos.base.v1beta1.abci",
    "TxMsgData"
);

impl_name!(
    cosmos::auth::v1beta1::BaseAccount,
    "cosmos.auth.v1beta1",
    "BaseAccount"
);
impl_name!(
    cosmos::auth::v1beta1::ModuleAccount,
    "cosmos.auth.v1beta1",
    "ModuleAccount"
);

impl_name!(
    cosmos::authz::v1beta1::MsgExec,
    "cosmos.authz.v1beta1",
    "MsgExec"
);

impl_name!(cosmos::tx::v1beta1::Tx, "cosmos.tx.v1beta1", "Tx");
impl_name!(
    cosmos::tx::v1beta1::AuthInfo,
    "cosmos.tx.v1beta1",
    "AuthInfo"
);
impl_name!(cosmos::tx::v1beta1::Fee, "cosmos.tx.v1beta1", "Fee");
impl_name!(cosmos::tx::v1beta1::TxBody, "cosmos.tx.v1beta1", "TxBody");
impl_name!(
    cosmos::tx::v1beta1::SignerInfo,
    "cosmos.tx.v1beta1",
    "SingerInfo"
);
impl_name!(
    cosmos::tx::v1beta1::ModeInfo,
    "cosmos.tx.v1beta1",
    "ModeInfo"
);

#[cfg(feature = "cosmwasm")]
mod wasm {
    use super::full_name;
    use crate::{cosmwasm, traits::Name};

    const COSMWASM_PACKAGE: &str = "cosmwasm.wasm.v1";

    impl_name!(
        cosmwasm::wasm::v1::AccessConfigUpdate,
        COSMWASM_PACKAGE,
        "AccessConfigUpdate"
    );
    impl_name!(
        cosmwasm::wasm::v1::AccessConfig,
        COSMWASM_PACKAGE,
        "AccessConfig"
    );
    impl_name!(
        cosmwasm::wasm::v1::MigrateContractProposal,
        COSMWASM_PACKAGE,
        "MigrateContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UpdateInstantiateConfigProposal,
        COSMWASM_PACKAGE,
        "UpdateInstantiateConfigProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::SudoContractProposal,
        COSMWASM_PACKAGE,
        "SudoContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::ExecuteContractProposal,
        COSMWASM_PACKAGE,
        "ExecuteContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UpdateAdminProposal,
        COSMWASM_PACKAGE,
        "UpdateAdminProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::ClearAdminProposal,
        COSMWASM_PACKAGE,
        "ClearAdminProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::PinCodesProposal,
        COSMWASM_PACKAGE,
        "PinCodesProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UnpinCodesProposal,
        COSMWASM_PACKAGE,
        "UnpinCodesProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::InstantiateContractProposal,
        COSMWASM_PACKAGE,
        "InstantiateContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::StoreCodeProposal,
        COSMWASM_PACKAGE,
        "StoreCodeProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgStoreCode,
        COSMWASM_PACKAGE,
        "MsgStoreCode"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContract,
        COSMWASM_PACKAGE,
        "MsgInstantiateContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContract2,
        COSMWASM_PACKAGE,
        "MsgInstantiateContract2"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgExecuteContract,
        COSMWASM_PACKAGE,
        "MsgExecuteContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgMigrateContract,
        COSMWASM_PACKAGE,
        "MsgMigrateContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgUpdateAdmin,
        COSMWASM_PACKAGE,
        "MsgUpdateAdmin"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgClearAdmin,
        COSMWASM_PACKAGE,
        "MsgClearAdmin"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgStoreCodeResponse,
        COSMWASM_PACKAGE,
        "MsgStoreCodeResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContractResponse,
        COSMWASM_PACKAGE,
        "MsgInstantiateContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgExecuteContractResponse,
        COSMWASM_PACKAGE,
        "MsgExecuteContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgMigrateContractResponse,
        COSMWASM_PACKAGE,
        "MsgMigrateContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgUpdateAdminResponse,
        COSMWASM_PACKAGE,
        "MsgUpdateAdminResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgClearAdminResponse,
        COSMWASM_PACKAGE,
        "MsgClearAdminResponse"
    );
}

// TODO(tarcieri): remove this when tokio-rs/prost#923 is released (v0.12.1?)
fn full_name<T: Name>() -> String {
    format!("{}.{}", T::PACKAGE, T::NAME)
}
