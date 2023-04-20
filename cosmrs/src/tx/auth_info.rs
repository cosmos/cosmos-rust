//! Auth info.

use super::{Fee, SignerInfo};
use crate::{
    proto::{self, traits::MessageExt},
    Error, ErrorReport, Result,
};

/// [`AuthInfo`] describes the fee and signer modes that are used to sign a transaction.
// TODO(tarcieri): support for the `tip` field
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthInfo {
    /// Defines the signing modes for the required signers.
    ///
    /// The number and order of elements must match the required signers from transaction
    /// [`Body`][`super::Body`]’s messages. The first element is the primary signer and the one
    /// which pays the [`Fee`].
    pub signer_infos: Vec<SignerInfo>,

    /// [`Fee`] and gas limit for the transaction.
    ///
    /// The first signer is the primary signer and the one which pays the fee.
    /// The fee can be calculated based on the cost of evaluating the body and doing signature
    /// verification of the signers. This can be estimated via simulation.
    pub fee: Fee,
}

impl AuthInfo {
    /// Convert to a Protocol Buffers representation.
    pub fn into_proto(self) -> proto::cosmos::tx::v1beta1::AuthInfo {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.into_proto().to_bytes()?)
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::AuthInfo> for AuthInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::tx::v1beta1::AuthInfo) -> Result<AuthInfo> {
        // TODO(tarcieri): parse tip
        Ok(AuthInfo {
            signer_infos: proto
                .signer_infos
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            fee: proto
                .fee
                .ok_or(Error::MissingField { name: "fee" })?
                .try_into()?,
        })
    }
}

impl From<AuthInfo> for proto::cosmos::tx::v1beta1::AuthInfo {
    fn from(auth_info: AuthInfo) -> proto::cosmos::tx::v1beta1::AuthInfo {
        proto::cosmos::tx::v1beta1::AuthInfo {
            signer_infos: auth_info.signer_infos.into_iter().map(Into::into).collect(),
            fee: Some(auth_info.fee.into()),
            tip: None,
        }
    }
}
