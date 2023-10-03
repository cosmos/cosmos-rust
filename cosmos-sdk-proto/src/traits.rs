//! Support traits for Cosmos SDK protobufs.

pub use prost::{Message, Name};

use crate::Any;
use prost::{DecodeError, EncodeError};
use std::str::FromStr;

/// Extension trait for [`Message`].
pub trait MessageExt: Message {
    /// Parse this message proto from [`Any`].
    #[deprecated(since = "0.20.0", note = "use Any::to_msg instead")]
    fn from_any(any: &Any) -> Result<Self, DecodeError>
    where
        Self: Default + Name + Sized,
    {
        any.to_msg()
    }

    /// Serialize this message proto as [`Any`].
    #[deprecated(since = "0.20.0", note = "use Any::from_msg instead")]
    fn to_any(&self) -> Result<Any, EncodeError>
    where
        Self: Name + Sized,
    {
        Any::from_msg(self)
    }

    /// Serialize this protobuf message as a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}

impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = Vec::new();
        Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}

/// Extension traits for optionally parsing non-empty strings.
///
/// This is a common pattern in Cosmos SDK protobufs.
pub trait ParseOptional: AsRef<str> {
    /// Parse optional field.
    fn parse_optional<T: FromStr>(&self) -> Result<Option<T>, T::Err> {
        if self.as_ref().is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.as_ref().parse()?))
        }
    }
}

impl ParseOptional for str {}
impl ParseOptional for String {}
