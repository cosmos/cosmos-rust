use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};
use std::convert::TryFrom;

/// MsgExecuteContract submits the given message data to a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,

    /// Msg json encoded message to be passed to the contract
    pub msg: Vec<u8>,

    /// Funds coins that are transferred to the contract on execution
    pub funds: Vec<Coin>,
}

impl Msg for MsgExecuteContract {
    type Proto = proto::cosmwasm::wasm::v1::MsgExecuteContract;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgExecuteContract> for MsgExecuteContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgExecuteContract,
    ) -> Result<MsgExecuteContract> {
        Ok(MsgExecuteContract {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
            msg: proto.msg.into_iter().map(Into::into).collect(),
            funds: proto
                .funds
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgExecuteContract> for proto::cosmwasm::wasm::v1::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> proto::cosmwasm::wasm::v1::MsgExecuteContract {
        proto::cosmwasm::wasm::v1::MsgExecuteContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            msg: msg.msg,
            funds: msg.funds.iter().map(Into::into).collect(),
        }
    }
}

/// MsgExecuteContractResponse returns execution result data.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgExecuteContractResponse {
    /// Data contains base64-encoded bytes to returned from the contract
    pub data: Vec<u8>,
}

impl Msg for MsgExecuteContractResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgExecuteContractResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgExecuteContractResponse> for MsgExecuteContractResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgExecuteContractResponse,
    ) -> Result<MsgExecuteContractResponse> {
        Ok(MsgExecuteContractResponse { data: proto.data })
    }
}

impl From<MsgExecuteContractResponse> for proto::cosmwasm::wasm::v1::MsgExecuteContractResponse {
    fn from(
        msg: MsgExecuteContractResponse,
    ) -> proto::cosmwasm::wasm::v1::MsgExecuteContractResponse {
        proto::cosmwasm::wasm::v1::MsgExecuteContractResponse { data: msg.data }
    }
}
