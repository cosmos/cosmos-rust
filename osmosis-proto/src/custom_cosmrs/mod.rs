//! Transaction messages

use prost_types::Any;
use cosmrs::Error;
use cosmrs::ErrorReport;
use eyre::Result;
use cosmrs::prost_ext::MessageExt;

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
    type Proto: MsgProto;

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
        self.into().to_any()
    }
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: Default + MessageExt + Sized {
    /// Type URL value
    const TYPE_URL: &'static str;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self> {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value)?)
        } else {
            Err(Error::MsgType {
                expected: Self::TYPE_URL,
                found: any.type_url.clone(),
            }
                .into())
        }
    }

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> Result<Any> {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }
}

#[cfg(feature = "osmosis")]
impl MsgProto for super::osmosis::gamm::v1beta1::Pool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Pool";
}
