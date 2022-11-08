use super::AccessType;
use crate::{proto, AccountId, Error, ErrorReport, Result};

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
