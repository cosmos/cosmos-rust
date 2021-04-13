//! Signer info.

use super::{AuthInfo, Fee, ModeInfo, SequenceNumber, SignMode};
use crate::{crypto::PublicKey, proto, Error, Result};
use std::convert::{TryFrom, TryInto};

/// [`SignerInfo`] describes the public key and signing mode of a single top-level signer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerInfo {
    /// [`PublicKey`] of the signer.
    ///
    /// It is optional for accounts that already exist in state. If unset, the verifier can use the
    /// required signer address for this position and lookup the public key.
    pub public_key: Option<PublicKey>,

    /// Signing mode.
    ///
    /// Nested structure to support nested multisig [`PublicKey`]s.
    pub mode_info: ModeInfo,

    /// Sequence of the account, which describes the number of committed transactions signed by a
    /// given address.
    ///
    /// It is used to prevent replay attacks.
    pub sequence: SequenceNumber,
}

impl SignerInfo {
    /// Create [`SignerInfo`] for a single direct signer.
    pub fn single_direct(public_key: Option<PublicKey>, sequence: SequenceNumber) -> SignerInfo {
        SignerInfo {
            public_key,
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
    }

    /// Get the [`AuthInfo`] for this signer with the given fee.
    ///
    /// This is primarily useful for cases involving a single signer.
    pub fn auth_info(self, fee: Fee) -> AuthInfo {
        AuthInfo {
            signer_infos: vec![self],
            fee,
        }
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::SignerInfo> for SignerInfo {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmos::tx::v1beta1::SignerInfo) -> Result<SignerInfo> {
        Ok(SignerInfo {
            public_key: proto.public_key.map(TryFrom::try_from).transpose()?,
            mode_info: proto
                .mode_info
                .ok_or(Error::MissingField { name: "mode_info" })?
                .try_into()?,
            sequence: proto.sequence,
        })
    }
}

impl From<SignerInfo> for proto::cosmos::tx::v1beta1::SignerInfo {
    fn from(signer_info: SignerInfo) -> proto::cosmos::tx::v1beta1::SignerInfo {
        proto::cosmos::tx::v1beta1::SignerInfo {
            public_key: signer_info.public_key.map(Into::into),
            mode_info: Some(signer_info.mode_info.into()),
            sequence: signer_info.sequence,
        }
    }
}
