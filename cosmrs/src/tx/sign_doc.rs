//! Signing document.

use super::{AccountNumber, AuthInfo, Body, Raw};
use crate::{
    crypto::secp256k1,
    proto::{self, traits::MessageExt},
    Result,
};
use tendermint::chain;

/// [`SignDoc`] is the type used for generating sign bytes for `SIGN_MODE_DIRECT`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignDoc {
    /// `body_bytes` is protobuf serialization of a transaction [`Body`] that matches the
    /// representation in a [`Raw`] transaction.
    pub body_bytes: Vec<u8>,

    /// `auth_info_bytes` is a protobuf serialization of an [`AuthInfo`] that matches the
    /// representation in a [`Raw`].
    pub auth_info_bytes: Vec<u8>,

    /// `chain_id` is the unique identifier of the chain this transaction targets.
    ///
    /// It prevents signed transactions from being used on another chain by an
    /// attacker.
    pub chain_id: String,

    /// `account_number` is the account number of the account in state
    pub account_number: AccountNumber,
}

impl SignDoc {
    /// Create a new [`SignDoc`] from a given transaction [`Body`] and [`AuthInfo`].
    pub fn new(
        body: &Body,
        auth_info: &AuthInfo,
        chain_id: &chain::Id,
        account_number: u64,
    ) -> Result<Self> {
        Ok(Self {
            // TODO(tarcieri): optimize away `Clone` calls with reference conversions
            body_bytes: body.clone().into_bytes()?,
            auth_info_bytes: auth_info.clone().into_bytes()?,
            chain_id: chain_id.to_string(),
            account_number,
        })
    }

    /// Convert to a Protocol Buffers representation.
    pub fn into_proto(self) -> proto::cosmos::tx::v1beta1::SignDoc {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.into_proto().to_bytes()?)
    }

    /// Sign this [`SignDoc`], producing a [`Raw`] transaction.
    pub fn sign(self, signing_key: &secp256k1::SigningKey) -> Result<Raw> {
        // TODO(tarcieri): optimize away `Clone` calls with reference conversions
        let sign_doc_bytes = self.clone().into_bytes()?;
        let signature = signing_key.sign(&sign_doc_bytes)?;

        Ok(proto::cosmos::tx::v1beta1::TxRaw {
            body_bytes: self.body_bytes,
            auth_info_bytes: self.auth_info_bytes,
            signatures: vec![signature.to_vec()],
        }
        .into())
    }
}

impl From<proto::cosmos::tx::v1beta1::SignDoc> for SignDoc {
    fn from(proto: proto::cosmos::tx::v1beta1::SignDoc) -> SignDoc {
        SignDoc {
            body_bytes: proto.body_bytes,
            auth_info_bytes: proto.auth_info_bytes,
            chain_id: proto.chain_id,
            account_number: proto.account_number,
        }
    }
}

impl From<SignDoc> for proto::cosmos::tx::v1beta1::SignDoc {
    fn from(sign_doc: SignDoc) -> proto::cosmos::tx::v1beta1::SignDoc {
        proto::cosmos::tx::v1beta1::SignDoc {
            body_bytes: sign_doc.body_bytes,
            auth_info_bytes: sign_doc.auth_info_bytes,
            chain_id: sign_doc.chain_id,
            account_number: sign_doc.account_number,
        }
    }
}
