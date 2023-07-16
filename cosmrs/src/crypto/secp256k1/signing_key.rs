//! Transaction signing key

use crate::{
    crypto::{secp256k1::Signature, PublicKey},
    ErrorReport, Result,
};
use ecdsa::signature::{Keypair, Signer};
use k256::ecdsa::VerifyingKey;
use rand_core::OsRng;

/// ECDSA/secp256k1 signing key (i.e. private key)
///
/// This is a wrapper type which supports any pluggable ECDSA/secp256k1 signer
/// implementation which impls the [`EcdsaSigner`] trait.
///
/// By default it uses [`k256::ecdsa::SigningKey`] as the signer implementation,
/// however it can be instantiated from any compatible signer (e.g. HSM, KMS,
/// etc) by using [`SigningKey::new`].
///
/// Supported alternative signer implementations:
/// - [`yubihsm::ecdsa::secp256k1::Signer`]: YubiHSM-backed ECDSA/secp256k1 signer
///
/// [`yubihsm::ecdsa::secp256k1::Signer`]: https://docs.rs/yubihsm/latest/yubihsm/ecdsa/secp256k1/type.Signer.html
pub struct SigningKey {
    inner: Box<dyn EcdsaSigner>,
}

impl SigningKey {
    /// Initialize from a provided signer object.
    ///
    /// Use [`SigningKey::from_slice`] to initialize from a raw private key.
    pub fn new(signer: Box<dyn EcdsaSigner>) -> Self {
        Self { inner: signer }
    }

    /// Initialize from a raw scalar value (big endian).
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let signing_key = k256::ecdsa::SigningKey::from_slice(bytes)?;
        Ok(Self::new(Box::new(signing_key)))
    }

    /// Generate a random signing key.
    pub fn random() -> Self {
        Self::new(Box::new(k256::ecdsa::SigningKey::random(&mut OsRng)))
    }

    /// Derive a signing key from a [`bip32::DerivationPath`].
    ///
    /// Note that [`bip32::DerivationPath`] impls [`std::str::FromStr`] and
    /// therefore you can use `parse()` to parse it from a string.
    #[cfg(feature = "bip32")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bip32")))]
    pub fn derive_from_path(
        seed: impl AsRef<[u8]>,
        path: &bip32::DerivationPath,
    ) -> bip32::Result<Self> {
        bip32::XPrv::derive_from_path(seed, path).map(Into::into)
    }

    /// Sign the given message, returning a signature.
    pub fn sign(&self, msg: &[u8]) -> Result<Signature> {
        Ok(self.inner.try_sign(msg)?)
    }

    /// Get the [`PublicKey`] for this [`SigningKey`].
    pub fn public_key(&self) -> PublicKey {
        self.inner.verifying_key().into()
    }
}

impl From<Box<dyn EcdsaSigner>> for SigningKey {
    fn from(signer: Box<dyn EcdsaSigner>) -> Self {
        Self::new(signer)
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = ErrorReport;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_slice(bytes)
    }
}

#[cfg(feature = "bip32")]
#[cfg_attr(docsrs, doc(cfg(feature = "bip32")))]
impl From<bip32::XPrv> for SigningKey {
    fn from(xprv: bip32::XPrv) -> SigningKey {
        SigningKey::from(&xprv)
    }
}

#[cfg(feature = "bip32")]
#[cfg_attr(docsrs, doc(cfg(feature = "bip32")))]
impl From<&bip32::XPrv> for SigningKey {
    fn from(xprv: &bip32::XPrv) -> SigningKey {
        Self {
            inner: Box::new(xprv.private_key().clone()),
        }
    }
}

/// ECDSA/secp256k1 signer trait.
///
/// This is a trait which enables plugging any backing signing implementation
/// which produces a compatible [`Signature`] and [`VerifyingKey`].
///
/// Note that this trait is bounded on [`ecdsa::signature::Signer`], which is
/// what is actually used to produce a signature for a given message.
pub trait EcdsaSigner:
    Signer<Signature> + Keypair<VerifyingKey = VerifyingKey> + Sync + Send
{
}

impl<T> EcdsaSigner for T where
    T: Signer<Signature> + Keypair<VerifyingKey = VerifyingKey> + Sync + Send
{
}
