//! CosmWasm messages
//!
//! - Tutorial: <https://docs.cosmwasm.com/>
//! - Protocol Docs: <https://github.com/CosmWasm/wasmd/blob/master/docs/proto/proto.md>

pub use crate::proto::cosmwasm::wasm::v1::AccessType;
use crate::{
    prost_ext::ParseOptional, proto, tx::Msg, AccountId, Coin, Error, ErrorReport, Result,
};

/// AccessConfig access control type.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AccessConfig {
    /// Access type granted.
    pub permission: AccessType,

    /// Account address with the associated permission.
    pub address: AccountId,
}

impl TryFrom<proto::cosmwasm::wasm::v1::AccessConfig> for AccessConfig {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::AccessConfig) -> Result<AccessConfig> {
        AccessConfig::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1::AccessConfig> for AccessConfig {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmwasm::wasm::v1::AccessConfig) -> Result<AccessConfig> {
        Ok(AccessConfig {
            permission: AccessType::from_i32(proto.permission).ok_or(Error::InvalidEnumValue {
                name: "permission",
                found_value: proto.permission,
            })?,
            address: proto.address.parse()?,
        })
    }
}

impl From<AccessConfig> for proto::cosmwasm::wasm::v1::AccessConfig {
    fn from(config: AccessConfig) -> proto::cosmwasm::wasm::v1::AccessConfig {
        proto::cosmwasm::wasm::v1::AccessConfig::from(&config)
    }
}

impl From<&AccessConfig> for proto::cosmwasm::wasm::v1::AccessConfig {
    fn from(config: &AccessConfig) -> proto::cosmwasm::wasm::v1::AccessConfig {
        proto::cosmwasm::wasm::v1::AccessConfig {
            permission: config.permission as i32,
            address: config.address.to_string(),
        }
    }
}

/// MsgStoreCode submit Wasm code to the system
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCode {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// WASMByteCode can be raw or gzip compressed
    pub wasm_byte_code: Vec<u8>,

    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    pub instantiate_permission: Option<AccessConfig>,
}

impl Msg for MsgStoreCode {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCode;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCode> for MsgStoreCode {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::MsgStoreCode) -> Result<MsgStoreCode> {
        Ok(MsgStoreCode {
            sender: proto.sender.parse()?,
            wasm_byte_code: proto.wasm_byte_code,
            instantiate_permission: proto
                .instantiate_permission
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<MsgStoreCode> for proto::cosmwasm::wasm::v1::MsgStoreCode {
    fn from(msg: MsgStoreCode) -> proto::cosmwasm::wasm::v1::MsgStoreCode {
        proto::cosmwasm::wasm::v1::MsgStoreCode {
            sender: msg.sender.to_string(),
            wasm_byte_code: msg.wasm_byte_code,
            instantiate_permission: msg.instantiate_permission.map(Into::into),
        }
    }
}

/// MsgStoreCodeResponse returns store result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    pub code_id: u64,
}

impl Msg for MsgStoreCodeResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCodeResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCodeResponse> for MsgStoreCodeResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgStoreCodeResponse,
    ) -> Result<MsgStoreCodeResponse> {
        Ok(MsgStoreCodeResponse {
            code_id: proto.code_id,
        })
    }
}

impl From<MsgStoreCodeResponse> for proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
    fn from(msg: MsgStoreCodeResponse) -> proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
        proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
            code_id: msg.code_id,
        }
    }
}

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
