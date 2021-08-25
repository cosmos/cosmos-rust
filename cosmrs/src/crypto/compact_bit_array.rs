//! Compact bit array.

use crate::proto;

/// [`CompactBitArray`] is an implementation of a space efficient bit array.
///
/// This is used to ensure that the encoded data takes up a minimal amount of
/// space after proto encoding.
#[derive(Clone, Debug, PartialEq)]
pub struct CompactBitArray {
    // TODO(tarcieri): better internal representation for this, e.g. `bitvec`
    inner: proto::cosmos::crypto::multisig::v1beta1::CompactBitArray,
}

impl CompactBitArray {
    /// Create a new [`CompactBitArray`] from a given number of extra
    /// bits stored and a byte slice containing the bits.
    pub fn new(extra_bits_stored: u32, elems: impl Into<Vec<u8>>) -> CompactBitArray {
        let inner = proto::cosmos::crypto::multisig::v1beta1::CompactBitArray {
            extra_bits_stored,
            elems: elems.into(),
        };

        CompactBitArray { inner }
    }
}

impl Eq for CompactBitArray {}

impl From<proto::cosmos::crypto::multisig::v1beta1::CompactBitArray> for CompactBitArray {
    fn from(proto: proto::cosmos::crypto::multisig::v1beta1::CompactBitArray) -> CompactBitArray {
        CompactBitArray { inner: proto }
    }
}

impl From<CompactBitArray> for proto::cosmos::crypto::multisig::v1beta1::CompactBitArray {
    fn from(
        bitarray: CompactBitArray,
    ) -> proto::cosmos::crypto::multisig::v1beta1::CompactBitArray {
        bitarray.inner
    }
}
