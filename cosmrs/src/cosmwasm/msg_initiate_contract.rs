use crate::{
    proto::{self, traits::ParseOptional},
    tx::Msg,
    AccountId, Coin, ErrorReport, Result,
};

/// MsgInstantiateContract create a new smart contract instance for the given
/// code id.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgInstantiateContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Admin is an optional address that can execute migrations
    pub admin: Option<AccountId>,

    /// CodeID is the reference to the stored WASM code
    pub code_id: u64,

    /// Label is optional metadata to be stored with a contract instance.
    pub label: Option<String>,

    /// Msg json encoded message to be passed to the contract on instantiation
    pub msg: Vec<u8>,

    /// Funds coins that are transferred to the contract on instantiation
    pub funds: Vec<Coin>,
}

impl Msg for MsgInstantiateContract {
    type Proto = proto::cosmwasm::wasm::v1::MsgInstantiateContract;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgInstantiateContract> for MsgInstantiateContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgInstantiateContract,
    ) -> Result<MsgInstantiateContract> {
        Ok(MsgInstantiateContract {
            sender: proto.sender.parse()?,
            admin: proto.admin.parse_optional()?,
            code_id: proto.code_id,
            label: proto.label.parse_optional()?,
            msg: proto.msg,
            funds: proto
                .funds
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgInstantiateContract> for proto::cosmwasm::wasm::v1::MsgInstantiateContract {
    fn from(msg: MsgInstantiateContract) -> proto::cosmwasm::wasm::v1::MsgInstantiateContract {
        proto::cosmwasm::wasm::v1::MsgInstantiateContract {
            sender: msg.sender.to_string(),
            admin: msg.admin.map(|admin| admin.to_string()).unwrap_or_default(),
            code_id: msg.code_id,
            label: msg.label.unwrap_or_default(),
            msg: msg.msg,
            funds: msg.funds.into_iter().map(Into::into).collect(),
        }
    }
}

/// MsgInstantiateContractResponse return instantiation result data
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    pub address: AccountId,

    /// Data contains base64-encoded bytes to returned from the contract
    pub data: Vec<u8>,
}

impl Msg for MsgInstantiateContractResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse>
    for MsgInstantiateContractResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse,
    ) -> Result<MsgInstantiateContractResponse> {
        Ok(MsgInstantiateContractResponse {
            address: proto.address.parse()?,
            data: proto.data,
        })
    }
}

impl From<MsgInstantiateContractResponse>
    for proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse
{
    fn from(
        msg: MsgInstantiateContractResponse,
    ) -> proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse {
        proto::cosmwasm::wasm::v1::MsgInstantiateContractResponse {
            address: msg.address.to_string(),
            data: msg.data,
        }
    }
}
