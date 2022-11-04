use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgClearAdmin removes any admin stored for a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgClearAdmin {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,
}

impl Msg for MsgClearAdmin {
    type Proto = proto::cosmwasm::wasm::v1::MsgClearAdmin;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgClearAdmin> for MsgClearAdmin {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::MsgClearAdmin) -> Result<MsgClearAdmin> {
        MsgClearAdmin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1::MsgClearAdmin> for MsgClearAdmin {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmwasm::wasm::v1::MsgClearAdmin) -> Result<MsgClearAdmin> {
        Ok(MsgClearAdmin {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
        })
    }
}

impl From<MsgClearAdmin> for proto::cosmwasm::wasm::v1::MsgClearAdmin {
    fn from(msg: MsgClearAdmin) -> proto::cosmwasm::wasm::v1::MsgClearAdmin {
        proto::cosmwasm::wasm::v1::MsgClearAdmin::from(&msg)
    }
}

impl From<&MsgClearAdmin> for proto::cosmwasm::wasm::v1::MsgClearAdmin {
    fn from(msg: &MsgClearAdmin) -> proto::cosmwasm::wasm::v1::MsgClearAdmin {
        proto::cosmwasm::wasm::v1::MsgClearAdmin {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
        }
    }
}

/// MsgClearAdminResponse returns empty data
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgClearAdminResponse {}

impl Msg for MsgClearAdminResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgClearAdminResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgClearAdminResponse> for MsgClearAdminResponse {
    type Error = ErrorReport;

    fn try_from(
        _proto: proto::cosmwasm::wasm::v1::MsgClearAdminResponse,
    ) -> Result<MsgClearAdminResponse> {
        Ok(MsgClearAdminResponse {})
    }
}

impl From<MsgClearAdminResponse> for proto::cosmwasm::wasm::v1::MsgClearAdminResponse {
    fn from(_msg: MsgClearAdminResponse) -> proto::cosmwasm::wasm::v1::MsgClearAdminResponse {
        proto::cosmwasm::wasm::v1::MsgClearAdminResponse {}
    }
}
