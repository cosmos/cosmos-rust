//! Type name registry: used to compute type URLs.

// TODO(tarcieri): generate these automatically using `prost-build`
// See: https://github.com/tokio-rs/prost/issues/926

use crate::{cosmos, ibc, traits::Name};

impl Name for ibc::core::client::v1::ClientUpdateProposal {
    const NAME: &'static str = "ClientUpdateProposal";
    const PACKAGE: &'static str = "ibc.core.client.v1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::upgrade::v1beta1::SoftwareUpgradeProposal {
    const NAME: &'static str = "SoftwareUpgradeProposal";
    const PACKAGE: &'static str = "cosmos.upgrade.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::params::v1beta1::ParameterChangeProposal {
    const NAME: &'static str = "ParameterChangeProposal";
    const PACKAGE: &'static str = "cosmos.params.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::distribution::v1beta1::CommunityPoolSpendProposal {
    const NAME: &'static str = "CommunityPoolSpendProposal";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::gov::v1beta1::TextProposal {
    const NAME: &'static str = "TextProposal";
    const PACKAGE: &'static str = "cosmos.gov.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::crypto::secp256k1::PubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "cosmos.crypto.secp256k1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::vesting::v1beta1::PeriodicVestingAccount {
    const NAME: &'static str = "PeriodicVestingAccount";
    const PACKAGE: &'static str = "cosmos.vesting.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::bank::v1beta1::MsgSend {
    const NAME: &'static str = "MsgSend";
    const PACKAGE: &'static str = "cosmos.bank.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::bank::v1beta1::MsgMultiSend {
    const NAME: &'static str = "MsgMultiSend";
    const PACKAGE: &'static str = "cosmos.bank.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const NAME: &'static str = "MsgSetWithdrawAddress";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const NAME: &'static str = "MsgWithdrawDelegatorReward";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
    const NAME: &'static str = "MsgWithdrawValidatorCommission";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::distribution::v1beta1::MsgFundCommunityPool {
    const NAME: &'static str = "MsgFundCommunityPool";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::feegrant::v1beta1::MsgGrantAllowance {
    const NAME: &'static str = "MsgGrantAllowance";
    const PACKAGE: &'static str = "cosmos.feegrant.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    const NAME: &'static str = "MsgRevokeAllowance";
    const PACKAGE: &'static str = "cosmos.feegrant.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::feegrant::v1beta1::BasicAllowance {
    const NAME: &'static str = "BasicAllowance";
    const PACKAGE: &'static str = "cosmos.feegrant.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::feegrant::v1beta1::PeriodicAllowance {
    const NAME: &'static str = "PeriodicAllowance";
    const PACKAGE: &'static str = "cosmos.feegrant.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    const NAME: &'static str = "AllowedMsgAllowance";
    const PACKAGE: &'static str = "cosmos.feegrant.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::staking::v1beta1::MsgDelegate {
    const NAME: &'static str = "MsgDelegate";
    const PACKAGE: &'static str = "cosmos.staking.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::staking::v1beta1::MsgUndelegate {
    const NAME: &'static str = "MsgUndelegate";
    const PACKAGE: &'static str = "cosmos.staking.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::staking::v1beta1::MsgBeginRedelegate {
    const NAME: &'static str = "MsgBeginRedelegate";
    const PACKAGE: &'static str = "cosmos.staking.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::base::abci::v1beta1::MsgData {
    const NAME: &'static str = "MsgData";
    const PACKAGE: &'static str = "cosmos.base.v1beta1.abci";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::base::abci::v1beta1::TxMsgData {
    const NAME: &'static str = "TxMsgData";
    const PACKAGE: &'static str = "cosmos.base.v1beta1.abci";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::auth::v1beta1::BaseAccount {
    const NAME: &'static str = "BaseAccount";
    const PACKAGE: &'static str = "cosmos.auth.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::auth::v1beta1::ModuleAccount {
    const NAME: &'static str = "ModuleAccount";
    const PACKAGE: &'static str = "cosmos.auth.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::Tx {
    const NAME: &'static str = "Tx";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::AuthInfo {
    const NAME: &'static str = "AuthInfo";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::Fee {
    const NAME: &'static str = "Fee";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::TxBody {
    const NAME: &'static str = "TxBody";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::SignerInfo {
    const NAME: &'static str = "SingerInfo";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for cosmos::tx::v1beta1::ModeInfo {
    const NAME: &'static str = "ModeInfo";
    const PACKAGE: &'static str = "cosmos.tx.v1beta1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

impl Name for ibc::applications::transfer::v1::MsgTransfer {
    const NAME: &'static str = "MsgTransfer";
    const PACKAGE: &'static str = "ibc.applications.transfer.v1";

    fn full_name() -> String {
        full_name::<Self>()
    }
}

#[cfg(feature = "cosmwasm")]
mod wasm {
    use super::full_name;
    use crate::{cosmwasm, traits::Name};

    const PACKAGE: &str = "cosmwasm.wasm.v1";

    impl Name for cosmwasm::wasm::v1::MigrateContractProposal {
        const NAME: &'static str = "MigrateContractProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::UpdateInstantiateConfigProposal {
        const NAME: &'static str = "UpdateInstantiateConfigProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::SudoContractProposal {
        const NAME: &'static str = "SudoContractProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::ExecuteContractProposal {
        const NAME: &'static str = "ExecuteContractProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::UpdateAdminProposal {
        const NAME: &'static str = "UpdateAdminProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::ClearAdminProposal {
        const NAME: &'static str = "ClearAdminProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::PinCodesProposal {
        const NAME: &'static str = "PinCodesProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::UnpinCodesProposal {
        const NAME: &'static str = "UnpinCodesProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::InstantiateContractProposal {
        const NAME: &'static str = "InstantiateContractProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::StoreCodeProposal {
        const NAME: &'static str = "StoreCodeProposal";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgStoreCode {
        const NAME: &'static str = "MsgStoreCode";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgInstantiateContract {
        const NAME: &'static str = "MsgInstantiateContract";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgExecuteContract {
        const NAME: &'static str = "MsgExecuteContract";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgMigrateContract {
        const NAME: &'static str = "MsgMigrateContract";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgUpdateAdmin {
        const NAME: &'static str = "MsgUpdateAdmin";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgClearAdmin {
        const NAME: &'static str = "MsgClearAdmin";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgStoreCodeResponse {
        const NAME: &'static str = "MsgStoreCodeResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgInstantiateContractResponse {
        const NAME: &'static str = "MsgInstantiateContractResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgExecuteContractResponse {
        const NAME: &'static str = "MsgExecuteContractResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgMigrateContractResponse {
        const NAME: &'static str = "MsgMigrateContractResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgUpdateAdminResponse {
        const NAME: &'static str = "MsgUpdateAdminResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }

    impl Name for cosmwasm::wasm::v1::MsgClearAdminResponse {
        const NAME: &'static str = "MsgClearAdminResponse";
        const PACKAGE: &'static str = PACKAGE;

        fn full_name() -> String {
            full_name::<Self>()
        }
    }
}

// TODO(tarcieri): remove this when tokio-rs/prost#923 is released (v0.12.1?)
fn full_name<T: Name>() -> String {
    format!("{}.{}", T::PACKAGE, T::NAME)
}
