use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgUpdateAdmin sets a new admin for a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUpdateAdmin {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// NewAdmin address to be set
    pub new_admin: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,
}

impl Msg for MsgUpdateAdmin {
    type Proto = proto::cosmwasm::wasm::v1::MsgUpdateAdmin;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        MsgUpdateAdmin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmwasm::wasm::v1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        Ok(MsgUpdateAdmin {
            sender: proto.sender.parse()?,
            new_admin: proto.new_admin.parse()?,
            contract: proto.contract.parse()?,
        })
    }
}

impl From<MsgUpdateAdmin> for proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
    fn from(msg: MsgUpdateAdmin) -> proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
        proto::cosmwasm::wasm::v1::MsgUpdateAdmin::from(&msg)
    }
}

impl From<&MsgUpdateAdmin> for proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
    fn from(msg: &MsgUpdateAdmin) -> proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
        proto::cosmwasm::wasm::v1::MsgUpdateAdmin {
            sender: msg.sender.to_string(),
            new_admin: msg.new_admin.to_string(),
            contract: msg.contract.to_string(),
        }
    }
}

/// MsgUpdateAdminResponse returns empty data
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUpdateAdminResponse {}

impl Msg for MsgUpdateAdminResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse> for MsgUpdateAdminResponse {
    type Error = ErrorReport;

    fn try_from(
        _proto: proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse,
    ) -> Result<MsgUpdateAdminResponse> {
        Ok(MsgUpdateAdminResponse {})
    }
}

impl From<MsgUpdateAdminResponse> for proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse {
    fn from(_msg: MsgUpdateAdminResponse) -> proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse {
        proto::cosmwasm::wasm::v1::MsgUpdateAdminResponse {}
    }
}
