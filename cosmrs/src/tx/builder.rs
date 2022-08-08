//! Transaction builder.

use super::Body;
use crate::Any;
use tendermint::block;

/// Transaction [`Body`] builder which simplifies incrementally assembling and
/// signing a transaction.
#[derive(Clone, Debug, Default)]
pub struct BodyBuilder {
    /// Transaction body in-progress.
    body: Body,
}

impl BodyBuilder {
    /// Create a new transaction builder in the default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a message to the transaction.
    pub fn msg(&mut self, msg: impl Into<Any>) -> &mut Self {
        self.body.messages.push(msg.into());
        self
    }

    /// Add multiple messages to the transaction.
    pub fn msgs(&mut self, msgs: impl IntoIterator<Item = Any>) -> &mut Self {
        self.body.messages.extend(msgs);
        self
    }

    /// Set the transaction memo.
    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.memo = memo.into();
        self
    }

    /// Set the timeout height.
    pub fn timeout_height(&mut self, height: impl Into<block::Height>) -> &mut Self {
        self.body.timeout_height = height.into();
        self
    }

    /// Add an extension option.
    pub fn extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.extension_options.push(option.into());
        self
    }

    /// Add a non-critical extension option.
    pub fn non_critical_extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.extension_options.push(option.into());
        self
    }

    /// Return the finished [`Body`].
    pub fn finish(&self) -> Body {
        self.into()
    }
}

impl From<BodyBuilder> for Body {
    fn from(builder: BodyBuilder) -> Body {
        builder.body
    }
}

impl From<&BodyBuilder> for Body {
    fn from(builder: &BodyBuilder) -> Body {
        builder.body.clone()
    }
}
