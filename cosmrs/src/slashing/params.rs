use crate::{proto, ErrorReport, Result};
use std::time::Duration;

/// Params represents the parameters used for by the slashing module.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Params {
    /// Signed blocks window
    pub signed_blocks_window: i64,

    /// Minimum signed per window
    pub min_signed_per_window: Vec<u8>,

    /// Downtime jail duration
    pub downtime_jail_duration: Option<Duration>,

    /// Slash fraction for double sign
    pub slash_fraction_double_sign: Vec<u8>,

    /// Slash fraction for downtime
    pub slash_fraction_downtime: Vec<u8>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::Params> for Params {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::Params) -> Result<Params> {
        Ok(Params {
            signed_blocks_window: proto.signed_blocks_window,
            min_signed_per_window: proto.min_signed_per_window,
            downtime_jail_duration: proto
                .downtime_jail_duration
                .map(TryFrom::try_from)
                .transpose()?,
            slash_fraction_double_sign: proto.slash_fraction_double_sign,
            slash_fraction_downtime: proto.slash_fraction_downtime,
        })
    }
}

impl From<Params> for proto::cosmos::slashing::v1beta1::Params {
    fn from(params: Params) -> Self {
        proto::cosmos::slashing::v1beta1::Params {
            signed_blocks_window: params.signed_blocks_window,
            min_signed_per_window: params.min_signed_per_window,
            downtime_jail_duration: params
                .downtime_jail_duration
                .map(TryInto::try_into)
                .transpose()
                .expect("invalid downtime jail duration"), // same fallible serialisation concern as with feegrant::PeriodicAllowance
            slash_fraction_double_sign: params.slash_fraction_double_sign,
            slash_fraction_downtime: params.slash_fraction_downtime,
        }
    }
}
