use crate::proto;
use crate::slashing::MissedBlock;

/// ValidatorMissedBlocks contains array of missed blocks of corresponding
/// address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatorMissedBlocks {
    /// address is the validator address.
    pub address: String,

    /// missed_blocks is an array of missed blocks by the validator.
    pub missed_blocks: Vec<MissedBlock>,
}

impl From<proto::cosmos::slashing::v1beta1::ValidatorMissedBlocks> for ValidatorMissedBlocks {
    fn from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorMissedBlocks,
    ) -> ValidatorMissedBlocks {
        ValidatorMissedBlocks {
            address: proto.address,
            missed_blocks: proto.missed_blocks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ValidatorMissedBlocks>
    for cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorMissedBlocks
{
    fn from(missed_blocks: ValidatorMissedBlocks) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorMissedBlocks {
            address: missed_blocks.address,
            missed_blocks: missed_blocks
                .missed_blocks
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
