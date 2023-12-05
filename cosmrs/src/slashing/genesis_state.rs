use crate::slashing::{Params, SigningInfo, ValidatorMissedBlocks};
use crate::{proto, ErrorReport, Result};

/// GenesisState defines the slashing module's genesis state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenesisState {
    /// params defines all the paramaters of related to deposit.
    pub params: Option<Params>,

    /// signing_infos represents a map between validator addresses and their
    /// signing infos.
    pub signing_infos: Vec<SigningInfo>,

    /// missed_blocks represents a map between validator addresses and their
    /// missed blocks.
    pub missed_blocks: Vec<ValidatorMissedBlocks>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::GenesisState> for GenesisState {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::GenesisState) -> Result<Self> {
        Ok(GenesisState {
            params: proto.params.map(TryInto::try_into).transpose()?,
            signing_infos: proto
                .signing_infos
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            missed_blocks: proto.missed_blocks.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<GenesisState> for cosmos_sdk_proto::cosmos::slashing::v1beta1::GenesisState {
    fn from(genesis_state: GenesisState) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::GenesisState {
            params: genesis_state.params.map(Into::into),
            signing_infos: genesis_state
                .signing_infos
                .into_iter()
                .map(Into::into)
                .collect(),
            missed_blocks: genesis_state
                .missed_blocks
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
