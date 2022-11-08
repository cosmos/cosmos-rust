//! Application/BlockChain Interface (ABCI)-related functionality.

mod gas_info;
mod msg_data;

pub use self::{
    gas_info::GasInfo,
    msg_data::{MsgData, TxMsgData},
};

/// Transaction data.
pub type Data = Vec<u8>;
