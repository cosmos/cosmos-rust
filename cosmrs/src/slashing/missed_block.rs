use crate::proto;

/// MissedBlock contains height and missed status as boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissedBlock {
    /// index is the height at which the block was missed.
    pub index: i64,

    /// missed is the missed status.
    pub missed: bool,
}

impl From<proto::cosmos::slashing::v1beta1::MissedBlock> for MissedBlock {
    fn from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::MissedBlock) -> MissedBlock {
        MissedBlock {
            index: proto.index,
            missed: proto.missed,
        }
    }
}

impl From<MissedBlock> for cosmos_sdk_proto::cosmos::slashing::v1beta1::MissedBlock {
    fn from(missed_block: MissedBlock) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::MissedBlock {
            index: missed_block.index,
            missed: missed_block.missed,
        }
    }
}
