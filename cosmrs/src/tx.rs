//! Cosmos SDK transaction support.
//!
//! ## About
//!
//! The [Cosmos SDK](https://v1.cosmos.network/sdk) defines a standard
//! transaction format used by blockchain applications built on the SDK.
//!
//! Transactions are comprised of metadata held in contexts and [`Msg`]s
//! that trigger state changes within a module through the module's [`Msg`] service.
//!
//! When users want to interact with an application and make state changes
//! (e.g. sending coins), they create transactions. Each of a transaction's [`Msg`]s
//! must be signed using the private key associated with the appropriate account(s),
//! before the transaction is broadcasted to the network.
//!
//! A transaction must then be included in a block, validated, and approved by the
//! network through the consensus process.
//!
//! ## Usage
//!
//! The following example illustrates how to build, sign, and parse
//! a Cosmos SDK transaction:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! use cosmrs::{
//!     bank::MsgSend,
//!     crypto::secp256k1,
//!     tx::{self, Fee, Msg, SignDoc, SignerInfo, Tx},
//!     AccountId, Coin
//! };
//!
//! // Generate sender private key.
//! // In real world usage, this account would need to be funded before use.
//! let sender_private_key = secp256k1::SigningKey::random();
//! let sender_public_key = sender_private_key.public_key();
//! let sender_account_id = sender_public_key.account_id("cosmos")?;
//!
//! // Parse recipient address from Bech32.
//! let recipient_account_id = "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z"
//!     .parse::<AccountId>()?;
//!
//! ///////////////////////////
//! // Building transactions //
//! ///////////////////////////
//!
//! // We'll be doing a simple send transaction.
//! // First we'll create a "Coin" amount to be sent, in this case 1 million uatoms.
//! let amount = Coin {
//!     amount: 1_000_000u128,
//!     denom: "uatom".parse()?,
//! };
//!
//! // Next we'll create a send message (from the "bank" module) for the coin
//! // amount we created above.
//! let msg_send = MsgSend {
//!     from_address: sender_account_id.clone(),
//!     to_address: recipient_account_id,
//!     amount: vec![amount.clone()],
//! };
//!
//! // Transaction metadata: chain, account, sequence, gas, fee, timeout, and memo.
//! let chain_id = "cosmoshub-4".parse()?;
//! let account_number = 1;
//! let sequence_number = 0;
//! let gas = 100_000u64;
//! let timeout_height = 9001u16;
//! let memo = "example memo";
//!
//! // Create transaction body from the MsgSend, memo, and timeout height.
//! let tx_body = tx::Body::new(vec![msg_send.to_any()?], memo, timeout_height);
//!
//! // Create signer info from public key and sequence number.
//! // This uses a standard "direct" signature from a single signer.
//! let signer_info = SignerInfo::single_direct(Some(sender_public_key), sequence_number);
//!
//! // Compute auth info from signer info by associating a fee.
//! let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, gas));
//!
//! //////////////////////////
//! // Signing transactions //
//! //////////////////////////
//!
//! // The "sign doc" contains a message to be signed.
//! let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
//!
//! // Sign the "sign doc" with the sender's private key, producing a signed raw transaction.
//! let tx_signed = sign_doc.sign(&sender_private_key)?;
//!
//! // Serialize the raw transaction as bytes (i.e. `Vec<u8>`).
//! let tx_bytes = tx_signed.to_bytes()?;
//!
//! //////////////////////////
//! // Parsing transactions //
//! //////////////////////////
//!
//! // Parse the serialized bytes from above into a `cosmrs::Tx`
//! let tx_parsed = Tx::from_bytes(&tx_bytes)?;
//! assert_eq!(tx_parsed.body, tx_body);
//! assert_eq!(tx_parsed.auth_info, auth_info);
//! # Ok(())
//! # }
//! ```

pub mod mode_info;

mod auth_info;
mod body;
mod builder;
mod fee;
mod msg;
mod raw;
mod sign_doc;
mod signer_info;

pub use self::{
    auth_info::AuthInfo,
    body::Body,
    builder::BodyBuilder,
    fee::Fee,
    mode_info::ModeInfo,
    msg::Msg,
    raw::Raw,
    sign_doc::SignDoc,
    signer_info::{SignerInfo, SignerPublicKey},
};
pub use crate::{
    proto::{cosmos::tx::signing::v1beta1::SignMode, traits::MessageExt},
    ErrorReport,
};

use crate::{
    proto::{self, traits::Message},
    Error, Gas, Result,
};

#[cfg(feature = "rpc")]
use {crate::rpc, tendermint::Hash};

/// Account number.
pub type AccountNumber = u64;

/// Sequence number.
pub type SequenceNumber = u64;

/// Serialized signature.
pub type SignatureBytes = Vec<u8>;

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
    ///
    /// Signatures are provided as raw bytes so as to support current and future signature types.
    /// [`AuthInfo`] should be introspected to determine the signature algorithm used.
    pub signatures: Vec<SignatureBytes>,
}

impl Tx {
    /// Parse a [`Tx`] from serialized bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Tx> {
        Tx::try_from(bytes)
    }

    /// Use RPC to find a transaction by its hash.
    #[cfg(feature = "rpc")]
    pub async fn find_by_hash<C>(rpc_client: &C, tx_hash: Hash) -> Result<Tx>
    where
        C: rpc::Client + Send + Sync,
    {
        let response = rpc_client.tx(tx_hash, false).await?;
        Tx::from_bytes(&response.tx)
    }
}

impl TryFrom<&[u8]> for Tx {
    type Error = ErrorReport;

    fn try_from(bytes: &[u8]) -> Result<Tx> {
        proto::cosmos::tx::v1beta1::Tx::decode(bytes)?.try_into()
    }
}

impl TryFrom<proto::cosmos::tx::v1beta1::Tx> for Tx {
    type Error = ErrorReport;

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
            signatures: proto.signatures,
        })
    }
}

impl From<Tx> for proto::cosmos::tx::v1beta1::Tx {
    fn from(tx: Tx) -> proto::cosmos::tx::v1beta1::Tx {
        proto::cosmos::tx::v1beta1::Tx {
            body: Some(tx.body.into()),
            auth_info: Some(tx.auth_info.into()),
            signatures: tx.signatures,
        }
    }
}
