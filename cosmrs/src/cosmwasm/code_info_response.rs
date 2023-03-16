use super::{AccessConfig, ContractCodeId};
use crate::{proto, AccountId, ErrorReport, Result};

/// CodeInfoResponse contains code meta data from CodeInfo
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CodeInfoResponse {
    /// CodeId of the stored contract code.
    pub code_id: ContractCodeId,

    /// Bech32 [`AccountId`] of the creator of this smart contract.
    pub creator: AccountId,

    /// sha256 hash of the code stored
    pub data_hash: Vec<u8>,

    /// Instantiate permission.
    pub instantiate_permission: Option<AccessConfig>,
}

impl TryFrom<proto::cosmwasm::wasm::v1::CodeInfoResponse> for CodeInfoResponse {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::CodeInfoResponse) -> Result<CodeInfoResponse> {
        Ok(CodeInfoResponse {
            code_id: proto.code_id,
            creator: proto.creator.parse()?,
            data_hash: proto.data_hash,
            instantiate_permission: proto
                .instantiate_permission
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

impl From<CodeInfoResponse> for proto::cosmwasm::wasm::v1::CodeInfoResponse {
    fn from(code_info: CodeInfoResponse) -> Self {
        proto::cosmwasm::wasm::v1::CodeInfoResponse {
            code_id: code_info.code_id,
            creator: code_info.creator.to_string(),
            data_hash: code_info.data_hash,
            instantiate_permission: code_info.instantiate_permission.map(Into::into),
        }
    }
}
