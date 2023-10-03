//! Transaction messages

use crate::{
    proto::traits::{MessageExt, Name},
    Any, ErrorReport, Result,
};

/// Message types.
///
/// Types which impl this trait map one-to-one with a corresponding Protocol
/// Buffers type, but can assert additional invariants and/or additional
/// functionality beyond the raw proto, as well as providing a more idiomatic
/// Rust type to work with.
pub trait Msg:
    Clone + Sized + TryFrom<Self::Proto, Error = ErrorReport> + Into<Self::Proto>
{
    /// Protocol Buffers type
    type Proto: Default + MessageExt + Name + Sized;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self> {
        Self::Proto::from_any(any)?.try_into()
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any> {
        self.clone().into_any()
    }

    /// Convert this message proto into [`Any`].
    fn into_any(self) -> Result<Any> {
        Ok(self.into().to_any()?)
    }
}
