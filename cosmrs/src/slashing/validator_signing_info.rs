use crate::{proto, AccountId, ErrorReport, Result};
use cosmos_sdk_proto::Timestamp;
use tendermint::time::Time;

/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatorSigningInfo {
    /// Address of the validator
    pub address: AccountId,

    /// Height at which validator was first a candidate OR was unjailed
    pub start_height: i64,

    /// Index which is incremented each time the validator was a bonded
    /// in a block and may have signed a precommit or not. This in conjunction with the
    /// `SignedBlocksWindow` param determines the index in the `MissedBlocksBitArray`.
    pub index_offset: i64,

    /// Timestamp until which the validator is jailed due to liveness downtime.
    pub jailed_until: Option<Time>,

    /// Whether or not a validator has been tombstoned (killed out of validator set). It is set
    /// once the validator commits an equivocation or for any other configured misbehiavor.
    pub tombstoned: bool,

    /// A counter kept to avoid unnecessary array reads.
    /// Note that `Sum(MissedBlocksBitArray)` always equals `MissedBlocksCounter`.
    pub missed_blocks_counter: i64,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::ValidatorSigningInfo> for ValidatorSigningInfo {
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorSigningInfo,
    ) -> Result<ValidatorSigningInfo> {
        Ok(ValidatorSigningInfo {
            address: proto.address.parse()?,
            start_height: proto.start_height,
            index_offset: proto.index_offset,
            jailed_until: proto
                .jailed_until
                // annoyingly, tendermint uses a different type for its protobuf `Timestamp` than the one
                // in cosmos proto files. However, internally they have exactly the same layout
                .map(|jailed_until| {
                    cosmos_sdk_proto::tendermint::google::protobuf::Timestamp {
                        seconds: jailed_until.seconds,
                        nanos: jailed_until.nanos,
                    }
                    .try_into()
                })
                .transpose()?,
            tombstoned: proto.tombstoned,
            missed_blocks_counter: proto.missed_blocks_counter,
        })
    }
}

impl From<ValidatorSigningInfo> for proto::cosmos::slashing::v1beta1::ValidatorSigningInfo {
    fn from(signing_info: ValidatorSigningInfo) -> Self {
        proto::cosmos::slashing::v1beta1::ValidatorSigningInfo {
            address: signing_info.address.to_string(),
            start_height: signing_info.start_height,
            index_offset: signing_info.index_offset,
            jailed_until: signing_info
                .jailed_until
                .map(cosmos_sdk_proto::tendermint::google::protobuf::Timestamp::from)
                .map(|t| Timestamp {
                    seconds: t.seconds,
                    nanos: t.nanos,
                }),
            tombstoned: signing_info.tombstoned,
            missed_blocks_counter: signing_info.missed_blocks_counter,
        }
    }
}
