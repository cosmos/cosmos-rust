//! Support traits for Cosmos SDK protobufs.

pub use prost::{Message, Name};

use alloc::{string::String, vec::Vec};
use core::str::FromStr;
use prost::EncodeError;

/// Extension trait for [`Message`].
pub trait MessageExt: Message {
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
