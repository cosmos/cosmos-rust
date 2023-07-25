//! CosmWasm messages
//!
//! - Tutorial: <https://docs.cosmwasm.com/>
//! - Protocol Docs: <https://github.com/CosmWasm/wasmd/blob/master/docs/proto/proto.md>

mod absolute_tx_position;
mod access_config;
mod code_info_response;
mod contract_code_history_entry;
mod contract_info;
mod msg_clear_admin;
mod msg_execute_contract;
mod msg_initiate_contract;
mod msg_migrate_contract;
mod msg_store_code;
mod msg_update_admin;
mod query_code_response;

pub use self::{
    absolute_tx_position::AbsoluteTxPosition,
    access_config::AccessConfig,
    code_info_response::CodeInfoResponse,
    contract_code_history_entry::ContractCodeHistoryEntry,
    contract_info::ContractInfo,
    msg_clear_admin::{MsgClearAdmin, MsgClearAdminResponse},
    msg_execute_contract::{MsgExecuteContract, MsgExecuteContractResponse},
    msg_initiate_contract::{MsgInstantiateContract, MsgInstantiateContractResponse},
    msg_migrate_contract::{MsgMigrateContract, MsgMigrateContractResponse},
    msg_store_code::{MsgStoreCode, MsgStoreCodeResponse},
    msg_update_admin::{MsgUpdateAdmin, MsgUpdateAdminResponse},
    query_code_response::QueryCodeResponse,
};
pub use crate::proto::cosmwasm::wasm::v1::AccessType;

/// The ID of a particular contract code assigned by the chain.
pub type ContractCodeId = u64;
