use crate::{proto, ErrorReport, Gas, Result};
use serde::{Deserialize, Serialize};

/// [`GasInfo`] defines constraints for how much gas to use to execute a
/// transaction.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    pub gas_wanted: Gas,

    /// GasUsed is the amount of gas actually consumed.
    pub gas_used: Gas,
}

impl TryFrom<proto::cosmos::base::abci::v1beta1::GasInfo> for GasInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::base::abci::v1beta1::GasInfo) -> Result<GasInfo> {
        Ok(GasInfo {
            gas_wanted: proto.gas_wanted,
            gas_used: proto.gas_used,
        })
    }
}

impl From<GasInfo> for proto::cosmos::base::abci::v1beta1::GasInfo {
    fn from(info: GasInfo) -> Self {
        proto::cosmos::base::abci::v1beta1::GasInfo {
            gas_wanted: info.gas_wanted,
            gas_used: info.gas_wanted,
        }
    }
}
