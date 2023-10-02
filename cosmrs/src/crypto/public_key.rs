//! Public keys

use crate::{
    proto::{
        self,
        traits::{Message, MessageExt},
    },
    AccountId, Any, Error, ErrorReport, Result,
};
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use subtle_encoding::base64;

/// Public keys
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "PublicKeyJson", into = "PublicKeyJson")]
pub struct PublicKey(tendermint::PublicKey);

impl PublicKey {
    /// Protobuf [`Any`] type URL for Ed25519 public keys
    pub const ED25519_TYPE_URL: &'static str = "/cosmos.crypto.ed25519.PubKey";

    /// Protobuf [`Any`] type URL for secp256k1 public keys
    pub const SECP256K1_TYPE_URL: &'static str = "/cosmos.crypto.secp256k1.PubKey";

    /// Parse public key from Cosmos JSON format.
    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str::<PublicKey>(s)?)
    }

    /// Serialize public key as Cosmos JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("JSON serialization error")
    }

    /// Get the [`AccountId`] for this [`PublicKey`] (if applicable).
    pub fn account_id(&self, prefix: &str) -> Result<AccountId> {
        match &self.0 {
            tendermint::PublicKey::Secp256k1(encoded_point) => {
                let id = tendermint::account::Id::from(*encoded_point);
                AccountId::new(prefix, id.as_bytes())
            }
            _ => Err(Error::Crypto.into()),
        }
    }

    /// Get the type URL for this [`PublicKey`].
    pub fn type_url(&self) -> &'static str {
        match &self.0 {
            tendermint::PublicKey::Ed25519(_) => Self::ED25519_TYPE_URL,
            tendermint::PublicKey::Secp256k1(_) => Self::SECP256K1_TYPE_URL,
            // `tendermint::PublicKey` is `non_exhaustive`
            _ => unreachable!("unknown pubic key type"),
        }
    }

    /// Convert this [`PublicKey`] to a Protobuf [`Any`] type.
    pub fn to_any(&self) -> Result<Any> {
        let value = match self.0 {
            tendermint::PublicKey::Ed25519(_) => proto::cosmos::crypto::ed25519::PubKey {
                key: self.to_bytes(),
            }
            .to_bytes()?,
            tendermint::PublicKey::Secp256k1(_) => proto::cosmos::crypto::secp256k1::PubKey {
                key: self.to_bytes(),
            }
            .to_bytes()?,
            _ => return Err(Error::Crypto.into()),
        };

        Ok(Any {
            type_url: self.type_url().to_owned(),
            value,
        })
    }

    /// Serialize this [`PublicKey`] as a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<k256::ecdsa::VerifyingKey> for PublicKey {
    fn from(vk: k256::ecdsa::VerifyingKey) -> PublicKey {
        PublicKey(vk.into())
    }
}

impl From<&k256::ecdsa::VerifyingKey> for PublicKey {
    fn from(vk: &k256::ecdsa::VerifyingKey) -> PublicKey {
        PublicKey::from(*vk)
    }
}

impl TryFrom<Any> for PublicKey {
    type Error = ErrorReport;

    fn try_from(any: Any) -> Result<PublicKey> {
        PublicKey::try_from(&any)
    }
}

impl TryFrom<&Any> for PublicKey {
    type Error = ErrorReport;

    fn try_from(any: &Any) -> Result<PublicKey> {
        match any.type_url.as_str() {
            Self::ED25519_TYPE_URL => {
                proto::cosmos::crypto::ed25519::PubKey::decode(&*any.value)?.try_into()
            }
            Self::SECP256K1_TYPE_URL => {
                proto::cosmos::crypto::secp256k1::PubKey::decode(&*any.value)?.try_into()
            }
            other => Err(Error::Crypto)
                .wrap_err_with(|| format!("invalid type URL for public key: {}", other)),
        }
    }
}

impl TryFrom<proto::cosmos::crypto::ed25519::PubKey> for PublicKey {
    type Error = ErrorReport;

    fn try_from(public_key: proto::cosmos::crypto::ed25519::PubKey) -> Result<PublicKey> {
        tendermint::public_key::PublicKey::from_raw_ed25519(&public_key.key)
            .map(Into::into)
            .ok_or_else(|| Error::Crypto.into())
    }
}

impl TryFrom<proto::cosmos::crypto::secp256k1::PubKey> for PublicKey {
    type Error = ErrorReport;

    fn try_from(public_key: proto::cosmos::crypto::secp256k1::PubKey) -> Result<PublicKey> {
        tendermint::public_key::PublicKey::from_raw_secp256k1(&public_key.key)
            .map(Into::into)
            .ok_or_else(|| Error::Crypto.into())
    }
}

impl From<PublicKey> for Any {
    fn from(public_key: PublicKey) -> Any {
        // This is largely a workaround for `tendermint::PublicKey` being
        // marked `non_exhaustive`.
        public_key.to_any().expect("unsupported algorithm")
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

impl FromStr for PublicKey {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_json(s)
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        self.to_json()
    }
}

/// Serde encoding type for JSON public keys.
///
/// Uses Protobuf JSON encoding conventions.
#[derive(Deserialize, Serialize)]
struct PublicKeyJson {
    /// `@type` field e.g. `/cosmos.crypto.ed25519.PubKey`.
    #[serde(rename = "@type")]
    type_url: String,

    /// Key data: standard Base64 encoded with padding.
    key: String,
}

impl From<PublicKey> for PublicKeyJson {
    fn from(public_key: PublicKey) -> PublicKeyJson {
        PublicKeyJson::from(&public_key)
    }
}

impl From<&PublicKey> for PublicKeyJson {
    fn from(public_key: &PublicKey) -> PublicKeyJson {
        let type_url = public_key.type_url().to_owned();
        let key = String::from_utf8(base64::encode(public_key.to_bytes())).expect("UTF-8 error");
        PublicKeyJson { type_url, key }
    }
}

impl TryFrom<PublicKeyJson> for PublicKey {
    type Error = ErrorReport;

    fn try_from(json: PublicKeyJson) -> Result<PublicKey> {
        PublicKey::try_from(&json)
    }
}

impl TryFrom<&PublicKeyJson> for PublicKey {
    type Error = ErrorReport;

    fn try_from(json: &PublicKeyJson) -> Result<PublicKey> {
        let pk_bytes = base64::decode(&json.key)?;

        let tm_key = match json.type_url.as_str() {
            Self::ED25519_TYPE_URL => tendermint::PublicKey::from_raw_ed25519(&pk_bytes),
            Self::SECP256K1_TYPE_URL => tendermint::PublicKey::from_raw_secp256k1(&pk_bytes),
            other => {
                return Err(Error::Crypto)
                    .wrap_err_with(|| format!("invalid public key @type: {}", other))
            }
        };

        tm_key.map(Into::into).ok_or_else(|| Error::Crypto.into())
    }
}

#[cfg(test)]
mod tests {
    use super::PublicKey;

    const EXAMPLE_JSON: &str = "{\"@type\":\"/cosmos.crypto.ed25519.PubKey\",\"key\":\"sEEsVGkXvyewKLWMJbHVDRkBoerW0IIwmj1rHkabtHU=\"}";

    #[test]
    fn json_round_trip() {
        let example_key = EXAMPLE_JSON.parse::<PublicKey>().unwrap();

        // test try_from
        let tm_key: tendermint::public_key::PublicKey =
            example_key.try_into().expect("try_into failure");
        let example_key = PublicKey::try_from(tm_key).expect("try_from failure");

        assert_eq!(example_key.type_url(), "/cosmos.crypto.ed25519.PubKey");
        assert_eq!(
            example_key.to_bytes().as_slice(),
            &[
                176, 65, 44, 84, 105, 23, 191, 39, 176, 40, 181, 140, 37, 177, 213, 13, 25, 1, 161,
                234, 214, 208, 130, 48, 154, 61, 107, 30, 70, 155, 180, 117
            ]
        );
        assert_eq!(EXAMPLE_JSON, example_key.to_string());
    }
}
