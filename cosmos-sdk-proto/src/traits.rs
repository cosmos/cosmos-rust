//! Support traits for Cosmos SDK protobufs.

pub use prost::Message;

use crate::Any;
use prost::{DecodeError, EncodeError};
use std::str::FromStr;

/// Associate a type URL with a given proto.
pub trait TypeUrl: Message {
    /// Type URL value
    const TYPE_URL: &'static str;
}

/// Extension trait for [`Message`].
pub trait MessageExt: Message {
    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self, DecodeError>
    where
        Self: Default + Sized + TypeUrl,
    {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value)?)
        } else {
            let mut err = DecodeError::new(format!(
                "expected type URL: \"{}\" (got: \"{}\")",
                Self::TYPE_URL,
                &any.type_url
            ));
            err.push("unexpected type URL", "type_url");
            Err(err)
        }
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any, EncodeError>
    where
        Self: TypeUrl,
    {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
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
