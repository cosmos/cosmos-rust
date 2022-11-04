use crate::{proto, ErrorReport, Result};

/// AbsoluteTxPosition is a unique transaction position that allows for global
/// ordering of transactions.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    pub block_height: u64,
    /// TxIndex is a monotonic counter within the block (actual transaction index, or gas consumed)
    pub tx_index: u64,
}

impl TryFrom<proto::cosmwasm::wasm::v1::AbsoluteTxPosition> for AbsoluteTxPosition {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::AbsoluteTxPosition,
    ) -> Result<AbsoluteTxPosition> {
        Ok(AbsoluteTxPosition {
            block_height: proto.block_height,
            tx_index: proto.tx_index,
        })
    }
}

impl From<AbsoluteTxPosition> for proto::cosmwasm::wasm::v1::AbsoluteTxPosition {
    fn from(pos: AbsoluteTxPosition) -> Self {
        proto::cosmwasm::wasm::v1::AbsoluteTxPosition {
            block_height: pos.block_height,
            tx_index: pos.tx_index,
        }
    }
}
