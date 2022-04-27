//! Prost extension traits

use crate::Result;
use std::str::FromStr;

/// Extension trait for prost messages.
// TODO(tarcieri): decide if this trait should really be sealed or if it should be public
pub trait MessageExt: prost::Message {
    /// Serialize this protobuf message as a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
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
