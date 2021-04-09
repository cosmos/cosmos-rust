//! Transaction bodies.

use super::Msg;
use crate::{prost_ext::MessageExt, Result};
use cosmos_sdk_proto::cosmos;
use prost_types::Any;
use std::convert::{TryFrom, TryInto};
use tendermint::block;

/// [`Body`] of a transaction that all signers sign over.
///
/// This type is known as `TxBody` in the Golang cosmos-sdk.
#[derive(Clone, Debug)]
pub struct Body {
    /// `messages` is a list of messages to be executed. The required signers of
    /// those messages define the number and order of elements in `AuthInfo`'s
    /// signer_infos and Tx's signatures. Each required signer address is added to
    /// the list only the first time it occurs.
    ///
    /// By convention, the first required signer (usually from the first message)
    /// is referred to as the primary signer and pays the fee for the whole
    /// transaction.
    pub messages: Vec<Msg>,

    /// `memo` is any arbitrary memo to be added to the transaction.
    pub memo: String,

    /// `timeout` is the block height after which this transaction will not
    /// be processed by the chain
    pub timeout_height: block::Height,

    /// `extension_options` are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, the transaction will be rejected
    pub extension_options: Vec<Any>,

    /// `extension_options` are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, they will be ignored
    pub non_critical_extension_options: Vec<Any>,
}

impl Body {
    /// Create a new [`Body`] from the given messages, memo, and timeout height.
    pub fn new<I>(
        messages: I,
        memo: impl Into<String>,
        timeout_height: impl Into<block::Height>,
    ) -> Self
    where
        I: IntoIterator<Item = Msg>,
    {
        Body {
            messages: messages.into_iter().map(Into::into).collect(),
            memo: memo.into(),
            timeout_height: timeout_height.into(),
            extension_options: Default::default(),
            non_critical_extension_options: Default::default(),
        }
    }

    /// Convert the body to a Protocol Buffers representation.
    pub fn into_proto(self) -> cosmos::tx::v1beta1::TxBody {
        self.into()
    }

    /// Serialize this type as an encoded Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        self.into_proto().to_bytes()
    }
}

impl From<Body> for cosmos::tx::v1beta1::TxBody {
    fn from(body: Body) -> cosmos::tx::v1beta1::TxBody {
        cosmos::tx::v1beta1::TxBody {
            messages: body.messages.into_iter().map(Into::into).collect(),
            memo: body.memo,
            timeout_height: body.timeout_height.into(),
            extension_options: body.extension_options,
            non_critical_extension_options: body.non_critical_extension_options,
        }
    }
}

impl TryFrom<cosmos::tx::v1beta1::TxBody> for Body {
    type Error = eyre::Report;

    fn try_from(proto: cosmos::tx::v1beta1::TxBody) -> Result<Body> {
        Ok(Body {
            messages: proto.messages.into_iter().map(Into::into).collect(),
            memo: proto.memo,
            timeout_height: proto
                .timeout_height
                .try_into()
                .map_err(|_| tendermint::error::Kind::Parse)?,
            extension_options: proto.extension_options,
            non_critical_extension_options: proto.non_critical_extension_options,
        })
    }
}
