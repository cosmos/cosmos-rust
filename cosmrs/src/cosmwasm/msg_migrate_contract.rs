use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};
use std::convert::TryFrom;

/// MsgMigrateContract runs a code upgrade/ downgrade for a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMigrateContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,

    /// CodeID references the new WASM code
    pub code_id: u64,

    /// Msg json encoded message to be passed to the contract on migration
    pub msg: Vec<u8>,
}

impl Msg for MsgMigrateContract {
    type Proto = proto::cosmwasm::wasm::v1::MsgMigrateContract;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgMigrateContract> for MsgMigrateContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgMigrateContract,
    ) -> Result<MsgMigrateContract> {
        Ok(MsgMigrateContract {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
            code_id: proto.code_id,
            msg: proto.msg,
        })
    }
}

impl From<MsgMigrateContract> for proto::cosmwasm::wasm::v1::MsgMigrateContract {
    fn from(msg: MsgMigrateContract) -> proto::cosmwasm::wasm::v1::MsgMigrateContract {
        proto::cosmwasm::wasm::v1::MsgMigrateContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            code_id: msg.code_id,
            msg: msg.msg,
        }
    }
}

/// MsgMigrateContractResponse returns contract migration result data.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMigrateContractResponse {
    /// Data contains same raw bytes returned as data from the wasm contract.
    /// (May be empty)
    pub data: Vec<u8>,
}

impl Msg for MsgMigrateContractResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgMigrateContractResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgMigrateContractResponse> for MsgMigrateContractResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgMigrateContractResponse,
    ) -> Result<MsgMigrateContractResponse> {
        Ok(MsgMigrateContractResponse { data: proto.data })
    }
}

impl From<MsgMigrateContractResponse> for proto::cosmwasm::wasm::v1::MsgMigrateContractResponse {
    fn from(
        msg: MsgMigrateContractResponse,
    ) -> proto::cosmwasm::wasm::v1::MsgMigrateContractResponse {
        proto::cosmwasm::wasm::v1::MsgMigrateContractResponse { data: msg.data }
    }
}
