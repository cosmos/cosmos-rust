use crate::staking::Validator;
use crate::{proto, ErrorReport, Result};
use tendermint::block::Header;

/// HistoricalInfo contains header and validator information for a given block.
/// It is stored as part of staking module's state, which persists the `n` most
/// recent HistoricalInfo
/// (`n` is set by the staking module's `historical_entries` parameter).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoricalInfo {
    /// Header of the block
    pub header: Option<Header>,

    /// The validator set at the block
    pub valset: Vec<Validator>,
}

impl TryFrom<proto::cosmos::staking::v1beta1::HistoricalInfo> for HistoricalInfo {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::staking::v1beta1::HistoricalInfo) -> Result<Self> {
        Ok(HistoricalInfo {
            header: proto.header.map(TryInto::try_into).transpose()?,
            valset: proto
                .valset
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
        })
    }
}

impl From<HistoricalInfo> for proto::cosmos::staking::v1beta1::HistoricalInfo {
    fn from(historical_info: HistoricalInfo) -> Self {
        proto::cosmos::staking::v1beta1::HistoricalInfo {
            header: historical_info.header.map(Into::into),
            valset: historical_info.valset.into_iter().map(Into::into).collect(),
        }
    }
}
