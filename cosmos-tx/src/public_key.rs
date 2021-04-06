//! Public keys

// TODO(tarcieri): upstream this to `tendermint-rs`?

use crate::{prost_ext::MessageExt, AccountId, Error, Result};
use cosmos_sdk_proto::cosmos;
use ecdsa::elliptic_curve::sec1::ToEncodedPoint;
use eyre::WrapErr;
use prost_types::Any;
use std::convert::{TryFrom, TryInto};

/// Protobuf [`Any`] type URL for Ed25519 public keys
const ED25519_TYPE_URL: &str = "/cosmos.crypto.ed25519.PubKey";

/// Protobuf [`Any`] type URL for secp256k1 public keys
const SECP256K1_TYPE_URL: &str = "/cosmos.crypto.secp256k1.PubKey";

/// Public keys
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PublicKey(tendermint::PublicKey);

impl PublicKey {
    /// Get the [`AccountId`] for this [`PublicKey`] (if applicable).
    // TODO(tarcieri): upstream our `AccountId` type to tendermint-rs?
    pub fn account_id(&self, prefix: &str) -> Result<AccountId> {
        match &self.0 {
            tendermint::PublicKey::Secp256k1(encoded_point) => {
                let id = tendermint::account::Id::from(*encoded_point);
                AccountId::new(prefix, id.as_bytes().try_into()?)
            }
            _ => Err(Error::Crypto.into()),
        }
    }

    /// Convert this [`PublicKey`] to a Protobuf [`Any`] type.
    pub fn to_any(&self) -> Result<Any> {
        match self.0 {
            tendermint::PublicKey::Ed25519(_) => {
                let proto = cosmos::crypto::secp256k1::PubKey {
                    key: self.to_bytes(),
                };

                Ok(Any {
                    type_url: ED25519_TYPE_URL.to_owned(),
                    value: proto.to_bytes()?,
                })
            }
            tendermint::PublicKey::Secp256k1(_) => {
                let proto = cosmos::crypto::secp256k1::PubKey {
                    key: self.to_bytes(),
                };

                Ok(Any {
                    type_url: SECP256K1_TYPE_URL.to_owned(),
                    value: proto.to_bytes()?,
                })
            }
            _ => Err(Error::Crypto.into()),
        }
    }

    /// Serialize this [`PublicKey`] as a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl From<k256::ecdsa::VerifyingKey> for PublicKey {
    fn from(vk: k256::ecdsa::VerifyingKey) -> PublicKey {
        PublicKey::from(&vk)
    }
}

impl From<&k256::ecdsa::VerifyingKey> for PublicKey {
    fn from(vk: &k256::ecdsa::VerifyingKey) -> PublicKey {
        PublicKey(vk.to_encoded_point(true).into())
    }
}

impl TryFrom<&Any> for PublicKey {
    type Error = eyre::Report;

    fn try_from(any: &Any) -> Result<PublicKey> {
        match any.type_url.as_str() {
            SECP256K1_TYPE_URL => tendermint::PublicKey::from_raw_secp256k1(&any.value)
                .map(Into::into)
                .ok_or_else(|| Error::Crypto.into()),
            other => Err(Error::Crypto)
                .wrap_err_with(|| format!("invalid type URL for public key: {}", other)),
        }
    }
}

impl From<tendermint::PublicKey> for PublicKey {
    fn from(pk: tendermint::PublicKey) -> PublicKey {
        PublicKey(pk)
    }
}

impl From<PublicKey> for tendermint::PublicKey {
    fn from(pk: PublicKey) -> tendermint::PublicKey {
        pk.0
    }
}
