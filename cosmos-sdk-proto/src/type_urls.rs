//! Registry of type URLs associated with various protobuf types defined in
//! this crate.

// TODO(tarcieri): leverage first-class support for type URLs in prost?
// See: https://github.com/tokio-rs/prost/issues/299

use crate::{cosmos, ibc, traits::TypeUrl};

impl TypeUrl for ibc::core::client::v1::ClientUpdateProposal {
    const TYPE_URL: &'static str = "/ibc.core.client.v1.ClientUpdateProposal";
}

impl TypeUrl for cosmos::upgrade::v1beta1::SoftwareUpgradeProposal {
    const TYPE_URL: &'static str = "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal";
}

impl TypeUrl for cosmos::params::v1beta1::ParameterChangeProposal {
    const TYPE_URL: &'static str = "/cosmos.params.v1beta1.ParameterChangeProposal";
}

impl TypeUrl for cosmos::distribution::v1beta1::CommunityPoolSpendProposal {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal";
}
impl TypeUrl for cosmos::gov::v1beta1::TextProposal {
    const TYPE_URL: &'static str = "/cosmos.gov.v1beta1.TextProposal";
}

impl TypeUrl for cosmos::crypto::secp256k1::PubKey {
    const TYPE_URL: &'static str = "/cosmos.crypto.secp256k1.PubKey";
}

impl TypeUrl for cosmos::vesting::v1beta1::PeriodicVestingAccount {
    const TYPE_URL: &'static str = "/cosmos.vesting.v1beta1.PeriodicVestingAccount";
}

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

impl TypeUrl for cosmos::auth::v1beta1::BaseAccount {
    const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.BaseAccount";
}

impl TypeUrl for cosmos::auth::v1beta1::ModuleAccount {
    const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.ModuleAccount";
}

impl TypeUrl for cosmos::tx::v1beta1::Tx {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.Tx";
}

impl TypeUrl for cosmos::tx::v1beta1::AuthInfo {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.AuthInfo";
}

impl TypeUrl for cosmos::tx::v1beta1::Fee {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.Fee";
}

impl TypeUrl for cosmos::tx::v1beta1::TxBody {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.TxBody";
}

impl TypeUrl for cosmos::tx::v1beta1::SignerInfo {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.SingerInfo";
}

impl TypeUrl for cosmos::tx::v1beta1::ModeInfo {
    const TYPE_URL: &'static str = "/cosmos.tx.v1beta1.ModeInfo";
}

impl TypeUrl for cosmos::tx::signing::v1beta1::SignatureDescriptors {
    const TYPE_URL: &'static str = "/cosmos.tx.signing.v1beta1.SignatureDescriptors";
}

impl TypeUrl for cosmos::tx::signing::v1beta1::SignatureDescriptor {
    const TYPE_URL: &'static str = "/cosmos.tx.signing.v1beta1.SignatureDescriptor";
}

impl TypeUrl for cosmos::tx::signing::v1beta1::signature_descriptor::Data {
    const TYPE_URL: &'static str = "/cosmos.tx.signing.v1beta1.signature_descriptor.Data";
}

impl TypeUrl for cosmos::tx::signing::v1beta1::signature_descriptor::data::Single {
    const TYPE_URL: &'static str = "/cosmos.tx.signing.v1beta1.signature_descriptor.data.Single";
}

impl TypeUrl for cosmos::tx::signing::v1beta1::signature_descriptor::data::Multi {
    const TYPE_URL: &'static str = "/cosmos.tx.signing.v1beta1.signature_descriptor.data.Multi";
}

impl TypeUrl for ibc::applications::transfer::v1::MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";
}

#[cfg(feature = "cosmwasm")]
mod wasm {
    use crate::{cosmwasm, traits::TypeUrl};

    impl TypeUrl for cosmwasm::wasm::v1::MigrateContractProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MigrateContractProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::UpdateInstantiateConfigProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.UpdateInstantiateConfigProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::SudoContractProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.SudoContractProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::ExecuteContractProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.ExecuteContractProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::UpdateAdminProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.UpdateAdminProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::ClearAdminProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.ClearAdminProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::PinCodesProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.PinCodesProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::UnpinCodesProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.UnpinCodesProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::InstantiateContractProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.InstantiateContractProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::StoreCodeProposal {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.StoreCodeProposal";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgStoreCode {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCode";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgInstantiateContract {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContract";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgExecuteContract {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContract";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgMigrateContract {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContract";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgUpdateAdmin {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdmin";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgClearAdmin {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdmin";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgStoreCodeResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgStoreCodeResponse";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgInstantiateContractResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContractResponse";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgExecuteContractResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContractResponse";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgMigrateContractResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgMigrateContractResponse";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgUpdateAdminResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgUpdateAdminResponse";
    }

    impl TypeUrl for cosmwasm::wasm::v1::MsgClearAdminResponse {
        const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgClearAdminResponse";
    }
}
