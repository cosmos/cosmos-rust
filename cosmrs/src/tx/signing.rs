pub mod signature_descriptor;
use crate::crypto::PublicKey;
use crate::{
    proto::{self, traits::MessageExt},
    ErrorReport, Result,
};

/// SignatureDescriptors wraps multiple SignatureDescriptor's.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureDescriptors {
    /// signatures are the signature descriptors
    pub signatures: Vec<SignatureDescriptor>,
}

impl SignatureDescriptors {
    /// Convert the body to a Protocol Buffers representation.
    pub fn into_proto(self) -> proto::cosmos::tx::signing::v1beta1::SignatureDescriptors {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.into_proto().to_bytes()?)
    }
}

impl TryFrom<cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptors>
    for SignatureDescriptors
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptors,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            signatures: proto
                .signatures
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<SignatureDescriptors>
    for cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptors
{
    fn from(value: SignatureDescriptors) -> Self {
        Self {
            signatures: value.signatures.into_iter().map(Into::into).collect(),
        }
    }
}

/// SignatureDescriptor is a convenience type which represents the full data for
/// a signature including the public key of the signer, signing modes and the
/// signature itself. It is primarily used for coordinating signatures between
/// clients.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureDescriptor {
    /// public_key is the public key of the signer
    pub public_key: Option<PublicKey>,
    pub data: Option<signature_descriptor::Data>,
    /// sequence is the sequence of the account, which describes the
    /// number of committed transactions signed by a given address. It is used to prevent
    /// replay attacks.
    pub sequence: u64,
}

impl SignatureDescriptor {
    /// Convert the body to a Protocol Buffers representation.
    pub fn into_proto(self) -> proto::cosmos::tx::signing::v1beta1::SignatureDescriptor {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.into_proto().to_bytes()?)
    }
}

impl TryFrom<cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptor>
    for SignatureDescriptor
{
    type Error = ErrorReport;

    fn try_from(
        proto: cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptor,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            public_key: proto.public_key.map(TryFrom::try_from).transpose()?,
            data: proto.data.map(TryInto::try_into).transpose()?,
            sequence: proto.sequence,
        })
    }
}

impl From<SignatureDescriptor>
    for cosmos_sdk_proto::cosmos::tx::signing::v1beta1::SignatureDescriptor
{
    fn from(value: SignatureDescriptor) -> Self {
        Self {
            public_key: value
                .public_key
                .map(|key| key.to_any().expect("never failed")),
            data: value.data.map(Into::into),
            sequence: value.sequence,
        }
    }
}
