use crate::slashing::validator_signing_info::ValidatorSigningInfo;
use crate::{proto, AccountId, ErrorReport, Result};

/// SigningInfo stores validator signing info of corresponding address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SigningInfo {
    /// address is the validator address.
    pub address: AccountId,

    /// validator_signing_info represents the signing info of this validator.
    pub validator_signing_info: Option<ValidatorSigningInfo>,
}

impl TryFrom<proto::cosmos::slashing::v1beta1::SigningInfo> for SigningInfo {
    type Error = ErrorReport;

    fn try_from(proto: cosmos_sdk_proto::cosmos::slashing::v1beta1::SigningInfo) -> Result<Self> {
        Ok(SigningInfo {
            address: proto.address.parse()?,
            validator_signing_info: proto
                .validator_signing_info
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

impl From<SigningInfo> for cosmos_sdk_proto::cosmos::slashing::v1beta1::SigningInfo {
    fn from(signing_info: SigningInfo) -> Self {
        cosmos_sdk_proto::cosmos::slashing::v1beta1::SigningInfo {
            address: signing_info.address.to_string(),
            validator_signing_info: signing_info.validator_signing_info.map(Into::into),
        }
    }
}
