//! Transactions.

pub mod mode_info;

mod auth_info;
mod body;
mod fee;
mod msg;
mod raw;
mod sign_doc;
mod signer_info;

pub use self::{
    auth_info::AuthInfo,
    body::Body,
    fee::Fee,
    mode_info::ModeInfo,
    msg::{Msg, MsgProto, MsgType},
    raw::Raw,
    sign_doc::SignDoc,
    signer_info::SignerInfo,
};
pub use crate::proto::cosmos::tx::signing::v1beta1::SignMode;
pub use tendermint::abci::Gas;

use crate::{crypto::secp256k1, proto, Error, Result};
use prost::Message;
use std::convert::{TryFrom, TryInto};

/// Account number.
pub type AccountNumber = u64;

/// Sequence number.
pub type SequenceNumber = u64;

/// [`Tx`] is the standard type used for broadcasting transactions.
#[derive(Clone, Debug)]
pub struct Tx {
    /// Processable content of the transaction
    pub body: Body,

    /// Authorization related content of the transaction, specifically signers, signer modes
    /// and [`Fee`].
    pub auth_info: AuthInfo,

    /// List of signatures that matches the length and order of [`AuthInfo`]’s `signer_info`s to
    /// allow connecting signature meta information like public key and signing mode by position.
    pub signatures: Vec<secp256k1::Signature>,
}

impl TryFrom<&[u8]> for Tx {
    type Error = eyre::Report;

    fn try_from(bytes: &[u8]) -> Result<Tx> {
        proto::cosmos::tx::v1beta1::Tx::decode(bytes)?.try_into()
    }
}
impl TryFrom<proto::cosmos::tx::v1beta1::Tx> for Tx {
    type Error = eyre::Report;

    fn try_from(proto: proto::cosmos::tx::v1beta1::Tx) -> Result<Tx> {
        Ok(Tx {
            body: proto
                .body
                .ok_or(Error::MissingField { name: "body" })?
                .try_into()?,
            auth_info: proto
                .auth_info
                .ok_or(Error::MissingField { name: "auth_info" })?
                .try_into()?,
            signatures: proto
                .signatures
                .iter()
                .map(|sig| secp256k1::Signature::try_from(sig.as_slice()))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Tx> for proto::cosmos::tx::v1beta1::Tx {
    fn from(tx: Tx) -> proto::cosmos::tx::v1beta1::Tx {
        proto::cosmos::tx::v1beta1::Tx {
            body: Some(tx.body.into()),
            auth_info: Some(tx.auth_info.into()),
            signatures: tx
                .signatures
                .iter()
                .map(|sig| sig.as_ref().to_vec())
                .collect(),
        }
    }
}
