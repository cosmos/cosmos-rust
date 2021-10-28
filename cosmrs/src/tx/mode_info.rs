//! Mode info.

use super::SignMode;
use crate::{crypto::CompactBitArray, proto, Error, ErrorReport, Result};

/// [`ModeInfo`] describes the signing mode of a single or nested multisig signer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeInfo {
    /// Single represents a single signer.
    Single(Single),

    /// Multi represents a nested multisig signer.
    Multi(Multi),
}

impl ModeInfo {
    /// Create [`ModeInfo`] for a single signer using the given mode.
    pub fn single(sign_mode: SignMode) -> ModeInfo {
        ModeInfo::Single(sign_mode.into())
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::ModeInfo> for ModeInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::tx::v1beta1::ModeInfo) -> Result<ModeInfo> {
        match proto.sum {
            Some(proto::cosmos::tx::v1beta1::mode_info::Sum::Single(single)) => {
                Ok(ModeInfo::Single(single.into()))
            }
            Some(proto::cosmos::tx::v1beta1::mode_info::Sum::Multi(multi)) => {
                Ok(ModeInfo::Multi(multi.try_into()?))
            }
            None => Err(Error::MissingField { name: "mode_info" }.into()),
        }
    }
}

impl From<ModeInfo> for proto::cosmos::tx::v1beta1::ModeInfo {
    fn from(mode_info: ModeInfo) -> proto::cosmos::tx::v1beta1::ModeInfo {
        proto::cosmos::tx::v1beta1::ModeInfo {
            sum: Some(match mode_info {
                ModeInfo::Single(single) => {
                    proto::cosmos::tx::v1beta1::mode_info::Sum::Single(single.into())
                }
                ModeInfo::Multi(multi) => {
                    proto::cosmos::tx::v1beta1::mode_info::Sum::Multi(multi.into())
                }
            }),
        }
    }
}

impl From<Single> for ModeInfo {
    fn from(single: Single) -> ModeInfo {
        ModeInfo::Single(single)
    }
}

impl From<Multi> for ModeInfo {
    fn from(multi: Multi) -> ModeInfo {
        ModeInfo::Multi(multi)
    }
}

/// [`Single`] is the mode info for a single signer.
///
/// It is structured as a message to allow for additional fields such as locale for
/// `SIGN_MODE_TEXTUAL` in the future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Single {
    /// Signing mode of the single signer.
    pub mode: SignMode,
}

impl From<proto::cosmos::tx::v1beta1::mode_info::Single> for Single {
    fn from(proto: proto::cosmos::tx::v1beta1::mode_info::Single) -> Single {
        Single { mode: proto.mode() }
    }
}

impl From<Single> for proto::cosmos::tx::v1beta1::mode_info::Single {
    fn from(single: Single) -> proto::cosmos::tx::v1beta1::mode_info::Single {
        proto::cosmos::tx::v1beta1::mode_info::Single {
            mode: single.mode as _,
        }
    }
}

impl From<SignMode> for Single {
    fn from(mode: SignMode) -> Single {
        Single { mode }
    }
}

impl From<Single> for SignMode {
    fn from(single: Single) -> SignMode {
        single.mode
    }
}

/// [`Multi`] is the mode info for a multisig public key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multi {
    /// Specifies which keys within the multisig are signing.
    pub bitarray: CompactBitArray,

    /// Corresponding modes of the signers of the multisig which could include nested
    /// multisig public keys.
    pub mode_infos: Vec<ModeInfo>,
}

impl TryFrom<proto::cosmos::tx::v1beta1::mode_info::Multi> for Multi {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::tx::v1beta1::mode_info::Multi) -> Result<Multi> {
        Ok(Multi {
            bitarray: proto
                .bitarray
                .map(Into::into)
                .ok_or(Error::MissingField { name: "bitarray" })?,
            mode_infos: proto
                .mode_infos
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Multi> for proto::cosmos::tx::v1beta1::mode_info::Multi {
    fn from(multi: Multi) -> proto::cosmos::tx::v1beta1::mode_info::Multi {
        proto::cosmos::tx::v1beta1::mode_info::Multi {
            bitarray: Some(multi.bitarray.into()),
            mode_infos: multi.mode_infos.into_iter().map(Into::into).collect(),
        }
    }
}
