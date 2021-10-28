//! Cryptographic functionality

pub mod secp256k1;

mod compact_bit_array;
mod legacy_amino;
mod public_key;

pub use self::{
    compact_bit_array::CompactBitArray, legacy_amino::LegacyAminoMultisig, public_key::PublicKey,
};
