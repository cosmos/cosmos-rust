use crate::{
    proto::{self, traits::MessageExt},
    ErrorReport, Result,
};

/// Data represents signature data
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    /// sum is the oneof that specifies whether this represents single or multi-signature data
    pub sum: Option<data::Sum>,
}

impl Data {
    /// Convert the body to a Protocol Buffers representation.
    pub fn into_proto(self) -> proto::cosmos::tx::signing::v1beta1::signature_descriptor::Data {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.into_proto().to_bytes()?)
    }
}

impl TryFrom<proto::cosmos::tx::signing::v1beta1::signature_descriptor::Data> for Data {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::tx::signing::v1beta1::signature_descriptor::Data,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            sum: proto.sum.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<Data> for proto::cosmos::tx::signing::v1beta1::signature_descriptor::Data {
    fn from(value: Data) -> Self {
        Self {
            sum: value.sum.map(Into::into),
        }
    }
}

/// Nested message and enum types in `Data`.
pub mod data {
    use crate::crypto::CompactBitArray;
    use crate::{
        proto::{self, traits::MessageExt},
        ErrorReport, Result,
    };
    use proto::cosmos::tx::signing::v1beta1::SignMode;

    /// Single is the signature data for a single signer
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Single {
        /// mode is the signing mode of the single signer
        pub mode: SignMode,
        /// signature is the raw signature bytes
        pub signature: Vec<u8>,
    }

    impl Single {
        /// Convert the body to a Protocol Buffers representation.
        pub fn into_proto(
            self,
        ) -> proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Single {
            self.into()
        }

        /// Encode this type using Protocol Buffers.
        pub fn into_bytes(self) -> Result<Vec<u8>> {
            Ok(self.into_proto().to_bytes()?)
        }
    }

    impl TryFrom<proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Single> for Single {
        type Error = ErrorReport;
        fn try_from(
            proto: proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Single,
        ) -> Result<Self, Self::Error> {
            let mode = match proto.mode {
                0 => SignMode::Unspecified,
                1 => SignMode::Direct,
                2 => SignMode::Textual,
                3 => SignMode::DirectAux,
                127 => SignMode::LegacyAminoJson,
                191 => SignMode::Eip191,
                n => return Err(eyre::eyre!("unknow sign mode : {}", n)),
            };
            Ok(Self {
                mode,
                signature: proto.signature,
            })
        }
    }

    impl From<Single> for proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Single {
        fn from(value: Single) -> Self {
            let mode = match value.mode {
                SignMode::Unspecified => 0,
                SignMode::Direct => 1,
                SignMode::Textual => 2,
                SignMode::DirectAux => 3,
                SignMode::LegacyAminoJson => 127,
                SignMode::Eip191 => 191,
            };
            Self {
                mode,
                signature: value.signature,
            }
        }
    }

    /// Multi is the signature data for a multisig public key
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Multi {
        /// bitarray specifies which keys within the multisig are signing
        pub bitarray: Option<CompactBitArray>,
        /// signatures is the signatures of the multi-signature
        pub signatures: Vec<super::Data>,
    }
    impl Multi {
        /// Convert the body to a Protocol Buffers representation.
        pub fn into_proto(
            self,
        ) -> proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Multi {
            self.into()
        }

        /// Encode this type using Protocol Buffers.
        pub fn into_bytes(self) -> Result<Vec<u8>> {
            Ok(self.into_proto().to_bytes()?)
        }
    }

    impl TryFrom<proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Multi> for Multi {
        type Error = ErrorReport;

        fn try_from(
            proto: proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Multi,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                bitarray: proto.bitarray.map(TryInto::try_into).transpose()?,
                signatures: proto
                    .signatures
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
    }

    impl From<Multi> for proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Multi {
        fn from(value: Multi) -> Self {
            Self {
                bitarray: value.bitarray.map(Into::into),
                signatures: value.signatures.into_iter().map(Into::into).collect(),
            }
        }
    }
    /// sum is the oneof that specifies whether this represents single or multi-signature data

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Sum {
        /// single represents a single signer
        Single(Single),
        /// multi represents a multisig signer
        Multi(Multi),
    }

    impl TryFrom<proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Sum> for Sum {
        type Error = eyre::ErrReport;
        fn try_from(
            proto: proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Sum,
        ) -> Result<Self, Self::Error> {
            match proto {
                proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Sum::Single(
                    single,
                ) => Ok(Sum::Single(single.try_into()?)),
                proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Sum::Multi(
                    multi,
                ) => Ok(Sum::Multi(multi.try_into()?)),
            }
        }
    }

    impl From<Sum> for proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::Sum {
        fn from(value: Sum) -> Self {
            match value {
                Sum::Single(v) => Self::Single(v.into()),
                Sum::Multi(v) => Self::Multi(v.into()),
            }
        }
    }
}
