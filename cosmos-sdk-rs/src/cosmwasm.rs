//! CosmWasm messages
//!
//! - Tutorial: <https://docs.cosmwasm.com/>
//! - Protocol Docs: <https://github.com/CosmWasm/wasmd/blob/master/docs/proto/proto.md>

pub use crate::proto::cosmwasm::wasm::v1beta1::AccessType;
use crate::{
    proto,
    tx::{Msg, MsgType},
    AccountId, Coin, Error, Result,
};
use std::convert::{TryFrom, TryInto};

/// AccessConfig access control type.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AccessConfig {
    /// Access type granted.
    pub permission: AccessType,

    /// Account address with the associated permission.
    pub address: AccountId,
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::AccessConfig> for AccessConfig {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmwasm::wasm::v1beta1::AccessConfig) -> Result<AccessConfig> {
        AccessConfig::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1beta1::AccessConfig> for AccessConfig {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmwasm::wasm::v1beta1::AccessConfig) -> Result<AccessConfig> {
        Ok(AccessConfig {
            permission: AccessType::from_i32(proto.permission).ok_or(Error::InvalidEnumValue {
                name: "permission",
                found_value: proto.permission,
            })?,
            address: proto.address.parse()?,
        })
    }
}

impl From<AccessConfig> for proto::cosmwasm::wasm::v1beta1::AccessConfig {
    fn from(config: AccessConfig) -> proto::cosmwasm::wasm::v1beta1::AccessConfig {
        proto::cosmwasm::wasm::v1beta1::AccessConfig::from(&config)
    }
}

impl From<&AccessConfig> for proto::cosmwasm::wasm::v1beta1::AccessConfig {
    fn from(config: &AccessConfig) -> proto::cosmwasm::wasm::v1beta1::AccessConfig {
        proto::cosmwasm::wasm::v1beta1::AccessConfig {
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

    /// Source is a valid absolute HTTPS URI to the contract's source code,
    /// optional
    pub source: Option<String>,

    /// Builder is a valid docker image name with tag, optional
    pub builder: Option<String>,

    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    pub instantiate_permission: Option<AccessConfig>,
}

impl MsgType for MsgStoreCode {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgStoreCode::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgStoreCode::from(self.clone()).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgStoreCode> for MsgStoreCode {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmwasm::wasm::v1beta1::MsgStoreCode) -> Result<MsgStoreCode> {
        let source = if proto.source.is_empty() {
            None
        } else {
            Some(proto.source)
        };

        let builder = if proto.builder.is_empty() {
            None
        } else {
            Some(proto.builder)
        };

        Ok(MsgStoreCode {
            sender: proto.sender.parse()?,
            wasm_byte_code: proto.wasm_byte_code,
            source,
            builder,
            instantiate_permission: proto
                .instantiate_permission
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<MsgStoreCode> for proto::cosmwasm::wasm::v1beta1::MsgStoreCode {
    fn from(msg: MsgStoreCode) -> proto::cosmwasm::wasm::v1beta1::MsgStoreCode {
        proto::cosmwasm::wasm::v1beta1::MsgStoreCode {
            sender: msg.sender.to_string(),
            wasm_byte_code: msg.wasm_byte_code,
            source: msg.source.unwrap_or_default(),
            builder: msg.builder.unwrap_or_default(),
            instantiate_permission: msg.instantiate_permission.map(Into::into),
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

    /// InitMsg json encoded message to be passed to the contract on instantiation
    pub init_msg: Vec<u8>,

    /// Funds coins that are transferred to the contract on instantiation
    pub funds: Vec<Coin>,
}

impl MsgType for MsgInstantiateContract {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract::from_msg(msg)
            .and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract::from(self.clone()).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract> for MsgInstantiateContract {
    type Error = eyre::Report;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract,
    ) -> Result<MsgInstantiateContract> {
        let label = if proto.label.is_empty() {
            None
        } else {
            Some(proto.label)
        };
        let admin = if proto.admin.is_empty() {
            None
        } else {
            Some(proto.admin.parse())
        };
        Ok(MsgInstantiateContract {
            sender: proto.sender.parse()?,
            admin: admin.transpose()?,
            code_id: proto.code_id,
            label,
            init_msg: proto.init_msg,
            funds: proto
                .funds
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgInstantiateContract> for proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract {
    fn from(msg: MsgInstantiateContract) -> proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract {
        proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract {
            sender: msg.sender.to_string(),
            admin: msg.admin.map(|admin| admin.to_string()).unwrap_or_default(),
            code_id: msg.code_id,
            label: msg.label.unwrap_or_default(),
            init_msg: msg.init_msg,
            funds: msg.funds.into_iter().map(Into::into).collect(),
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

impl MsgType for MsgExecuteContract {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgExecuteContract::from_msg(msg)
            .and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgExecuteContract::from(self.clone()).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgExecuteContract> for MsgExecuteContract {
    type Error = eyre::Report;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1beta1::MsgExecuteContract,
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

impl From<MsgExecuteContract> for proto::cosmwasm::wasm::v1beta1::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> proto::cosmwasm::wasm::v1beta1::MsgExecuteContract {
        proto::cosmwasm::wasm::v1beta1::MsgExecuteContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            msg: msg.msg,
            funds: msg.funds.iter().map(Into::into).collect(),
        }
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

    /// MigrateMsg json encoded message to be passed to the contract on migration
    pub migrate_msg: Vec<u8>,
}

impl MsgType for MsgMigrateContract {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgMigrateContract::from_msg(msg)
            .and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgMigrateContract::from(self.clone()).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgMigrateContract> for MsgMigrateContract {
    type Error = eyre::Report;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1beta1::MsgMigrateContract,
    ) -> Result<MsgMigrateContract> {
        Ok(MsgMigrateContract {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
            code_id: proto.code_id,
            migrate_msg: proto.migrate_msg,
        })
    }
}

impl From<MsgMigrateContract> for proto::cosmwasm::wasm::v1beta1::MsgMigrateContract {
    fn from(msg: MsgMigrateContract) -> proto::cosmwasm::wasm::v1beta1::MsgMigrateContract {
        proto::cosmwasm::wasm::v1beta1::MsgMigrateContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            code_id: msg.code_id,
            migrate_msg: msg.migrate_msg,
        }
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

impl MsgType for MsgUpdateAdmin {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin::from(self).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        MsgUpdateAdmin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        Ok(MsgUpdateAdmin {
            sender: proto.sender.parse()?,
            new_admin: proto.new_admin.parse()?,
            contract: proto.contract.parse()?,
        })
    }
}

impl From<MsgUpdateAdmin> for proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
    fn from(msg: MsgUpdateAdmin) -> proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
        proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin::from(&msg)
    }
}

impl From<&MsgUpdateAdmin> for proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
    fn from(msg: &MsgUpdateAdmin) -> proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
        proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
            sender: msg.sender.to_string(),
            new_admin: msg.new_admin.to_string(),
            contract: msg.contract.to_string(),
        }
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

impl MsgType for MsgClearAdmin {
    fn from_msg(msg: &Msg) -> Result<Self> {
        proto::cosmwasm::wasm::v1beta1::MsgClearAdmin::from_msg(msg).and_then(TryInto::try_into)
    }

    fn to_msg(&self) -> Result<Msg> {
        proto::cosmwasm::wasm::v1beta1::MsgClearAdmin::from(self).to_msg()
    }
}

impl TryFrom<proto::cosmwasm::wasm::v1beta1::MsgClearAdmin> for MsgClearAdmin {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmwasm::wasm::v1beta1::MsgClearAdmin) -> Result<MsgClearAdmin> {
        MsgClearAdmin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmwasm::wasm::v1beta1::MsgClearAdmin> for MsgClearAdmin {
    type Error = eyre::Report;

    fn try_from(proto: &proto::cosmwasm::wasm::v1beta1::MsgClearAdmin) -> Result<MsgClearAdmin> {
        Ok(MsgClearAdmin {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
        })
    }
}

impl From<MsgClearAdmin> for proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
    fn from(msg: MsgClearAdmin) -> proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
        proto::cosmwasm::wasm::v1beta1::MsgClearAdmin::from(&msg)
    }
}

impl From<&MsgClearAdmin> for proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
    fn from(msg: &MsgClearAdmin) -> proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
        proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
        }
    }
}
