//! Signer info.

use super::{AuthInfo, Fee, ModeInfo, SequenceNumber, SignMode};
use crate::{
    crypto::{LegacyAminoMultisig, PublicKey},
    proto, Any, Error, ErrorReport, Result,
};
use eyre::WrapErr;

/// [`SignerInfo`] describes the public key and signing mode of a single top-level signer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerInfo {
    /// [`PublicKey`] of the signer.
    ///
    /// It is optional for accounts that already exist in state. If unset, the verifier can use the
    /// required signer address for this position and lookup the public key.
    pub public_key: Option<SignerPublicKey>,

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
            public_key: public_key.map(Into::into),
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
    type Error = ErrorReport;

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

/// Signer's public key.
#[derive(Clone, Debug, PartialEq)]
pub enum SignerPublicKey {
    /// Single singer.
    Single(PublicKey),

    /// Legacy Amino multisig.
    LegacyAminoMultisig(LegacyAminoMultisig),

    /// Other key types beyond the ones provided above.
    Any(Any),
}

impl SignerPublicKey {
    /// Get the type URL for this [`SignerPublicKey`].
    pub fn type_url(&self) -> &str {
        match self {
            Self::Single(pk) => pk.type_url(),
            Self::LegacyAminoMultisig(_) => LegacyAminoMultisig::TYPE_URL,
            Self::Any(any) => &any.type_url,
        }
    }

    /// Get the [`PublicKey`] for a single signer, if applicable.
    pub fn single(&self) -> Option<&PublicKey> {
        match self {
            Self::Single(pk) => Some(pk),
            _ => None,
        }
    }

    /// Get the [`LegacyAminoMultisig`] key info, if applicable.
    pub fn legacy_amino_multisig(&self) -> Option<&LegacyAminoMultisig> {
        match self {
            Self::LegacyAminoMultisig(amino_multisig) => Some(amino_multisig),
            _ => None,
        }
    }
}

impl Eq for SignerPublicKey {}

impl From<PublicKey> for SignerPublicKey {
    fn from(pk: PublicKey) -> SignerPublicKey {
        Self::Single(pk)
    }
}

impl From<LegacyAminoMultisig> for SignerPublicKey {
    fn from(pk: LegacyAminoMultisig) -> SignerPublicKey {
        Self::LegacyAminoMultisig(pk)
    }
}

impl From<SignerPublicKey> for Any {
    fn from(public_key: SignerPublicKey) -> Any {
        match public_key {
            SignerPublicKey::Single(pk) => pk.into(),
            SignerPublicKey::LegacyAminoMultisig(pk) => pk.into(),
            SignerPublicKey::Any(any) => any,
        }
    }
}

impl TryFrom<Any> for SignerPublicKey {
    type Error = ErrorReport;

    fn try_from(any: Any) -> Result<Self> {
        match any.type_url.as_str() {
            PublicKey::ED25519_TYPE_URL | PublicKey::SECP256K1_TYPE_URL => {
                PublicKey::try_from(any).map(Into::into)
            }
            LegacyAminoMultisig::TYPE_URL => LegacyAminoMultisig::try_from(any).map(Into::into),
            _ => Ok(Self::Any(any)),
        }
    }
}

impl TryFrom<SignerPublicKey> for PublicKey {
    type Error = ErrorReport;

    fn try_from(public_key: SignerPublicKey) -> Result<PublicKey> {
        match public_key {
            SignerPublicKey::Single(pk) => Ok(pk),
            _ => Err(Error::Crypto).wrap_err_with(|| {
                format!(
                    "expected `SignerPublicKey::Single`, got: {}",
                    public_key.type_url()
                )
            }),
        }
    }
}
