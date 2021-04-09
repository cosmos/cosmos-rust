//! Protocol Buffer-encoded transaction builder

// Includes code originally from ibc-rs:
// <https://github.com/informalsystems/ibc-rs>
// Copyright © 2020 Informal Systems Inc.
// Licensed under the Apache 2.0 license

use crate::{
    prost_ext::MessageExt,
    tx::{self, Fee},
    SigningKey,
};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{mode_info, AuthInfo, ModeInfo, SignDoc, SignerInfo};
use eyre::Result;
use tendermint::chain;

/// Protocol Buffer-encoded transaction builder
pub struct Builder {
    /// Chain ID
    chain_id: chain::Id,

    /// Account number to include in transactions
    account_number: u64,
}

impl Builder {
    /// Create a new transaction builder
    pub fn new(chain_id: chain::Id, account_number: u64) -> Self {
        Self {
            chain_id,
            account_number,
        }
    }

    /// Borrow this transaction builder's chain ID
    pub fn chain_id(&self) -> &chain::Id {
        &self.chain_id
    }

    /// Get this transaction builder's account number
    pub fn account_number(&self) -> u64 {
        self.account_number
    }

    /// Build and sign a transaction containing the given messages
    pub fn sign_tx(
        &self,
        body: tx::Body,
        signing_key: &SigningKey,
        sequence: u64,
        fee: Fee,
    ) -> Result<tx::Raw> {
        let public_key = signing_key.public_key();
        let single = mode_info::Single { mode: 1 };

        let mode = Some(ModeInfo {
            sum: Some(mode_info::Sum::Single(single)),
        });

        let signer_info = SignerInfo {
            public_key: Some(public_key.to_any()?),
            mode_info: mode,
            sequence,
        };

        let auth_info = AuthInfo {
            signer_infos: vec![signer_info],
            fee: Some(fee.into()),
        };

        let body_bytes = body.into_bytes()?;
        let auth_info_bytes = auth_info.to_bytes()?;

        let sign_doc = SignDoc {
            body_bytes: body_bytes.clone(),
            auth_info_bytes: auth_info_bytes.clone(),
            chain_id: self.chain_id.to_string(),
            account_number: self.account_number,
        };

        let sign_doc_bytes = sign_doc.to_bytes()?;
        let signed = signing_key.sign(&sign_doc_bytes)?;

        Ok(cosmos_sdk_proto::cosmos::tx::v1beta1::TxRaw {
            body_bytes,
            auth_info_bytes,
            signatures: vec![signed.as_ref().to_vec()],
        }
        .into())
    }
}
