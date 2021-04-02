//! Transaction signing key

use crate::{PublicKey, Signature};
use core::convert::TryFrom;
use ecdsa::elliptic_curve::sec1::ToEncodedPoint;
use eyre::Result;
use rand_core::{CryptoRng, RngCore};

/// Transaction signing key.
pub struct SigningKey {
    inner: Box<dyn Secp256k1Signer>,
}

impl SigningKey {
    /// Generate a random signing key.
    pub fn random(rng: impl CryptoRng + RngCore) -> Self {
        Self {
            inner: Box::new(k256::ecdsa::SigningKey::random(rng)),
        }
    }

    /// Initialize from a raw scalar value (big endian).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let signing_key = k256::ecdsa::SigningKey::from_bytes(bytes)?;
        Ok(Self {
            inner: Box::new(signing_key),
        })
    }

    /// Sign the given message, returning a signature.
    pub fn sign(&self, msg: &[u8]) -> Result<Signature> {
        Ok(self.inner.try_sign(msg)?)
    }

    /// Get the Tendermint public key for this [`SigningKey`]
    pub fn public_key(&self) -> PublicKey {
        self.inner.public_key()
    }
}

impl From<Box<dyn Secp256k1Signer>> for SigningKey {
    fn from(signer: Box<dyn Secp256k1Signer>) -> Self {
        Self { inner: signer }
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = eyre::Report;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(bytes)
    }
}

/// ECDSA/secp256k1 signer
pub trait Secp256k1Signer: ecdsa::signature::Signer<Signature> {
    /// Get the Tendermint public key for this signer
    fn public_key(&self) -> PublicKey;
}

impl<T> Secp256k1Signer for T
where
    T: ecdsa::signature::Signer<Signature>,
    k256::ecdsa::VerifyingKey: for<'a> From<&'a T>,
{
    fn public_key(&self) -> PublicKey {
        k256::ecdsa::VerifyingKey::from(self)
            .to_encoded_point(true)
            .into()
    }
}
