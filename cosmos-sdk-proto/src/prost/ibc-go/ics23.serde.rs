// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for BatchEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.BatchEntry", len)?;
        if let Some(v) = self.proof.as_ref() {
            match v {
                batch_entry::Proof::Exist(v) => {
                    struct_ser.serialize_field("exist", v)?;
                }
                batch_entry::Proof::Nonexist(v) => {
                    struct_ser.serialize_field("nonexist", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exist", "nonexist"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exist,
            Nonexist,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exist" => Ok(GeneratedField::Exist),
                            "nonexist" => Ok(GeneratedField::Nonexist),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.BatchEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(batch_entry::Proof::Exist);
                        }
                        GeneratedField::Nonexist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonexist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(batch_entry::Proof::Nonexist);
                        }
                    }
                }
                Ok(BatchEntry { proof: proof__ })
            }
        }
        deserializer.deserialize_struct("ics23.BatchEntry", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BatchProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.BatchProof", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["entries"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.BatchProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchProof {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.BatchProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CommitmentProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.CommitmentProof", len)?;
        if let Some(v) = self.proof.as_ref() {
            match v {
                commitment_proof::Proof::Exist(v) => {
                    struct_ser.serialize_field("exist", v)?;
                }
                commitment_proof::Proof::Nonexist(v) => {
                    struct_ser.serialize_field("nonexist", v)?;
                }
                commitment_proof::Proof::Batch(v) => {
                    struct_ser.serialize_field("batch", v)?;
                }
                commitment_proof::Proof::Compressed(v) => {
                    struct_ser.serialize_field("compressed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommitmentProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exist", "nonexist", "batch", "compressed"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exist,
            Nonexist,
            Batch,
            Compressed,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exist" => Ok(GeneratedField::Exist),
                            "nonexist" => Ok(GeneratedField::Nonexist),
                            "batch" => Ok(GeneratedField::Batch),
                            "compressed" => Ok(GeneratedField::Compressed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitmentProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.CommitmentProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommitmentProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(commitment_proof::Proof::Exist);
                        }
                        GeneratedField::Nonexist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonexist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(commitment_proof::Proof::Nonexist);
                        }
                        GeneratedField::Batch => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(commitment_proof::Proof::Batch);
                        }
                        GeneratedField::Compressed => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressed"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(commitment_proof::Proof::Compressed);
                        }
                    }
                }
                Ok(CommitmentProof { proof: proof__ })
            }
        }
        deserializer.deserialize_struct("ics23.CommitmentProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CompressedBatchEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.CompressedBatchEntry", len)?;
        if let Some(v) = self.proof.as_ref() {
            match v {
                compressed_batch_entry::Proof::Exist(v) => {
                    struct_ser.serialize_field("exist", v)?;
                }
                compressed_batch_entry::Proof::Nonexist(v) => {
                    struct_ser.serialize_field("nonexist", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CompressedBatchEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exist", "nonexist"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exist,
            Nonexist,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exist" => Ok(GeneratedField::Exist),
                            "nonexist" => Ok(GeneratedField::Nonexist),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompressedBatchEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.CompressedBatchEntry")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CompressedBatchEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(compressed_batch_entry::Proof::Exist);
                        }
                        GeneratedField::Nonexist => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonexist"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(compressed_batch_entry::Proof::Nonexist);
                        }
                    }
                }
                Ok(CompressedBatchEntry { proof: proof__ })
            }
        }
        deserializer.deserialize_struct("ics23.CompressedBatchEntry", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CompressedBatchProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if !self.lookup_inners.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.CompressedBatchProof", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if !self.lookup_inners.is_empty() {
            struct_ser.serialize_field("lookupInners", &self.lookup_inners)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CompressedBatchProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["entries", "lookup_inners", "lookupInners"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            LookupInners,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entries" => Ok(GeneratedField::Entries),
                            "lookupInners" | "lookup_inners" => Ok(GeneratedField::LookupInners),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompressedBatchProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.CompressedBatchProof")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CompressedBatchProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut lookup_inners__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LookupInners => {
                            if lookup_inners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookupInners"));
                            }
                            lookup_inners__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CompressedBatchProof {
                    entries: entries__.unwrap_or_default(),
                    lookup_inners: lookup_inners__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.CompressedBatchProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CompressedExistenceProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.leaf.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.CompressedExistenceProof", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "value",
                pbjson::private::base64::encode(&self.value).as_str(),
            )?;
        }
        if let Some(v) = self.leaf.as_ref() {
            struct_ser.serialize_field("leaf", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CompressedExistenceProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value", "leaf", "path"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Leaf,
            Path,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "leaf" => Ok(GeneratedField::Leaf),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompressedExistenceProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.CompressedExistenceProof")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CompressedExistenceProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                let mut leaf__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Leaf => {
                            if leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            leaf__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(CompressedExistenceProof {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    leaf: leaf__,
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.CompressedExistenceProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CompressedNonExistenceProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.left.is_some() {
            len += 1;
        }
        if self.right.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ics23.CompressedNonExistenceProof", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if let Some(v) = self.left.as_ref() {
            struct_ser.serialize_field("left", v)?;
        }
        if let Some(v) = self.right.as_ref() {
            struct_ser.serialize_field("right", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CompressedNonExistenceProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "left", "right"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Left,
            Right,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "left" => Ok(GeneratedField::Left),
                            "right" => Ok(GeneratedField::Right),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompressedNonExistenceProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.CompressedNonExistenceProof")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CompressedNonExistenceProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut left__ = None;
                let mut right__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Left => {
                            if left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            left__ = map_.next_value()?;
                        }
                        GeneratedField::Right => {
                            if right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            right__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CompressedNonExistenceProof {
                    key: key__.unwrap_or_default(),
                    left: left__,
                    right: right__,
                })
            }
        }
        deserializer.deserialize_struct(
            "ics23.CompressedNonExistenceProof",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ExistenceProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.leaf.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.ExistenceProof", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "value",
                pbjson::private::base64::encode(&self.value).as_str(),
            )?;
        }
        if let Some(v) = self.leaf.as_ref() {
            struct_ser.serialize_field("leaf", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ExistenceProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value", "leaf", "path"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Leaf,
            Path,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "leaf" => Ok(GeneratedField::Leaf),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExistenceProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.ExistenceProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExistenceProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                let mut leaf__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Leaf => {
                            if leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            leaf__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExistenceProof {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    leaf: leaf__,
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.ExistenceProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for HashOp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoHash => "NO_HASH",
            Self::Sha256 => "SHA256",
            Self::Sha512 => "SHA512",
            Self::Keccak => "KECCAK",
            Self::Ripemd160 => "RIPEMD160",
            Self::Bitcoin => "BITCOIN",
            Self::Sha512256 => "SHA512_256",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HashOp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_HASH",
            "SHA256",
            "SHA512",
            "KECCAK",
            "RIPEMD160",
            "BITCOIN",
            "SHA512_256",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HashOp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_HASH" => Ok(HashOp::NoHash),
                    "SHA256" => Ok(HashOp::Sha256),
                    "SHA512" => Ok(HashOp::Sha512),
                    "KECCAK" => Ok(HashOp::Keccak),
                    "RIPEMD160" => Ok(HashOp::Ripemd160),
                    "BITCOIN" => Ok(HashOp::Bitcoin),
                    "SHA512_256" => Ok(HashOp::Sha512256),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InnerOp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hash != 0 {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        if !self.suffix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.InnerOp", len)?;
        if self.hash != 0 {
            let v = HashOp::try_from(self.hash)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hash)))?;
            struct_ser.serialize_field("hash", &v)?;
        }
        if !self.prefix.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "prefix",
                pbjson::private::base64::encode(&self.prefix).as_str(),
            )?;
        }
        if !self.suffix.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "suffix",
                pbjson::private::base64::encode(&self.suffix).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InnerOp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["hash", "prefix", "suffix"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Prefix,
            Suffix,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InnerOp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.InnerOp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InnerOp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut prefix__ = None;
                let mut suffix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value::<HashOp>()? as i32);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(InnerOp {
                    hash: hash__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                    suffix: suffix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.InnerOp", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InnerSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.child_order.is_empty() {
            len += 1;
        }
        if self.child_size != 0 {
            len += 1;
        }
        if self.min_prefix_length != 0 {
            len += 1;
        }
        if self.max_prefix_length != 0 {
            len += 1;
        }
        if !self.empty_child.is_empty() {
            len += 1;
        }
        if self.hash != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.InnerSpec", len)?;
        if !self.child_order.is_empty() {
            struct_ser.serialize_field("childOrder", &self.child_order)?;
        }
        if self.child_size != 0 {
            struct_ser.serialize_field("childSize", &self.child_size)?;
        }
        if self.min_prefix_length != 0 {
            struct_ser.serialize_field("minPrefixLength", &self.min_prefix_length)?;
        }
        if self.max_prefix_length != 0 {
            struct_ser.serialize_field("maxPrefixLength", &self.max_prefix_length)?;
        }
        if !self.empty_child.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "emptyChild",
                pbjson::private::base64::encode(&self.empty_child).as_str(),
            )?;
        }
        if self.hash != 0 {
            let v = HashOp::try_from(self.hash)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hash)))?;
            struct_ser.serialize_field("hash", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InnerSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "child_order",
            "childOrder",
            "child_size",
            "childSize",
            "min_prefix_length",
            "minPrefixLength",
            "max_prefix_length",
            "maxPrefixLength",
            "empty_child",
            "emptyChild",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChildOrder,
            ChildSize,
            MinPrefixLength,
            MaxPrefixLength,
            EmptyChild,
            Hash,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "childOrder" | "child_order" => Ok(GeneratedField::ChildOrder),
                            "childSize" | "child_size" => Ok(GeneratedField::ChildSize),
                            "minPrefixLength" | "min_prefix_length" => {
                                Ok(GeneratedField::MinPrefixLength)
                            }
                            "maxPrefixLength" | "max_prefix_length" => {
                                Ok(GeneratedField::MaxPrefixLength)
                            }
                            "emptyChild" | "empty_child" => Ok(GeneratedField::EmptyChild),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InnerSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.InnerSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InnerSpec, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut child_order__ = None;
                let mut child_size__ = None;
                let mut min_prefix_length__ = None;
                let mut max_prefix_length__ = None;
                let mut empty_child__ = None;
                let mut hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChildOrder => {
                            if child_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childOrder"));
                            }
                            child_order__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::ChildSize => {
                            if child_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childSize"));
                            }
                            child_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinPrefixLength => {
                            if min_prefix_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPrefixLength"));
                            }
                            min_prefix_length__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxPrefixLength => {
                            if max_prefix_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPrefixLength"));
                            }
                            max_prefix_length__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmptyChild => {
                            if empty_child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emptyChild"));
                            }
                            empty_child__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value::<HashOp>()? as i32);
                        }
                    }
                }
                Ok(InnerSpec {
                    child_order: child_order__.unwrap_or_default(),
                    child_size: child_size__.unwrap_or_default(),
                    min_prefix_length: min_prefix_length__.unwrap_or_default(),
                    max_prefix_length: max_prefix_length__.unwrap_or_default(),
                    empty_child: empty_child__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.InnerSpec", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LeafOp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hash != 0 {
            len += 1;
        }
        if self.prehash_key != 0 {
            len += 1;
        }
        if self.prehash_value != 0 {
            len += 1;
        }
        if self.length != 0 {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.LeafOp", len)?;
        if self.hash != 0 {
            let v = HashOp::try_from(self.hash)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hash)))?;
            struct_ser.serialize_field("hash", &v)?;
        }
        if self.prehash_key != 0 {
            let v = HashOp::try_from(self.prehash_key).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.prehash_key))
            })?;
            struct_ser.serialize_field("prehashKey", &v)?;
        }
        if self.prehash_value != 0 {
            let v = HashOp::try_from(self.prehash_value).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.prehash_value))
            })?;
            struct_ser.serialize_field("prehashValue", &v)?;
        }
        if self.length != 0 {
            let v = LengthOp::try_from(self.length).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.length))
            })?;
            struct_ser.serialize_field("length", &v)?;
        }
        if !self.prefix.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "prefix",
                pbjson::private::base64::encode(&self.prefix).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LeafOp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "prehash_key",
            "prehashKey",
            "prehash_value",
            "prehashValue",
            "length",
            "prefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            PrehashKey,
            PrehashValue,
            Length,
            Prefix,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "prehashKey" | "prehash_key" => Ok(GeneratedField::PrehashKey),
                            "prehashValue" | "prehash_value" => Ok(GeneratedField::PrehashValue),
                            "length" => Ok(GeneratedField::Length),
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LeafOp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.LeafOp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LeafOp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut prehash_key__ = None;
                let mut prehash_value__ = None;
                let mut length__ = None;
                let mut prefix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value::<HashOp>()? as i32);
                        }
                        GeneratedField::PrehashKey => {
                            if prehash_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prehashKey"));
                            }
                            prehash_key__ = Some(map_.next_value::<HashOp>()? as i32);
                        }
                        GeneratedField::PrehashValue => {
                            if prehash_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prehashValue"));
                            }
                            prehash_value__ = Some(map_.next_value::<HashOp>()? as i32);
                        }
                        GeneratedField::Length => {
                            if length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            length__ = Some(map_.next_value::<LengthOp>()? as i32);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(LeafOp {
                    hash: hash__.unwrap_or_default(),
                    prehash_key: prehash_key__.unwrap_or_default(),
                    prehash_value: prehash_value__.unwrap_or_default(),
                    length: length__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.LeafOp", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LengthOp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoPrefix => "NO_PREFIX",
            Self::VarProto => "VAR_PROTO",
            Self::VarRlp => "VAR_RLP",
            Self::Fixed32Big => "FIXED32_BIG",
            Self::Fixed32Little => "FIXED32_LITTLE",
            Self::Fixed64Big => "FIXED64_BIG",
            Self::Fixed64Little => "FIXED64_LITTLE",
            Self::Require32Bytes => "REQUIRE_32_BYTES",
            Self::Require64Bytes => "REQUIRE_64_BYTES",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LengthOp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_PREFIX",
            "VAR_PROTO",
            "VAR_RLP",
            "FIXED32_BIG",
            "FIXED32_LITTLE",
            "FIXED64_BIG",
            "FIXED64_LITTLE",
            "REQUIRE_32_BYTES",
            "REQUIRE_64_BYTES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LengthOp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_PREFIX" => Ok(LengthOp::NoPrefix),
                    "VAR_PROTO" => Ok(LengthOp::VarProto),
                    "VAR_RLP" => Ok(LengthOp::VarRlp),
                    "FIXED32_BIG" => Ok(LengthOp::Fixed32Big),
                    "FIXED32_LITTLE" => Ok(LengthOp::Fixed32Little),
                    "FIXED64_BIG" => Ok(LengthOp::Fixed64Big),
                    "FIXED64_LITTLE" => Ok(LengthOp::Fixed64Little),
                    "REQUIRE_32_BYTES" => Ok(LengthOp::Require32Bytes),
                    "REQUIRE_64_BYTES" => Ok(LengthOp::Require64Bytes),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for NonExistenceProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.left.is_some() {
            len += 1;
        }
        if self.right.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.NonExistenceProof", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if let Some(v) = self.left.as_ref() {
            struct_ser.serialize_field("left", v)?;
        }
        if let Some(v) = self.right.as_ref() {
            struct_ser.serialize_field("right", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NonExistenceProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "left", "right"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Left,
            Right,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "left" => Ok(GeneratedField::Left),
                            "right" => Ok(GeneratedField::Right),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NonExistenceProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.NonExistenceProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NonExistenceProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut left__ = None;
                let mut right__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Left => {
                            if left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            left__ = map_.next_value()?;
                        }
                        GeneratedField::Right => {
                            if right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            right__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NonExistenceProof {
                    key: key__.unwrap_or_default(),
                    left: left__,
                    right: right__,
                })
            }
        }
        deserializer.deserialize_struct("ics23.NonExistenceProof", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProofSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.leaf_spec.is_some() {
            len += 1;
        }
        if self.inner_spec.is_some() {
            len += 1;
        }
        if self.max_depth != 0 {
            len += 1;
        }
        if self.min_depth != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ics23.ProofSpec", len)?;
        if let Some(v) = self.leaf_spec.as_ref() {
            struct_ser.serialize_field("leafSpec", v)?;
        }
        if let Some(v) = self.inner_spec.as_ref() {
            struct_ser.serialize_field("innerSpec", v)?;
        }
        if self.max_depth != 0 {
            struct_ser.serialize_field("maxDepth", &self.max_depth)?;
        }
        if self.min_depth != 0 {
            struct_ser.serialize_field("minDepth", &self.min_depth)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProofSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "leaf_spec",
            "leafSpec",
            "inner_spec",
            "innerSpec",
            "max_depth",
            "maxDepth",
            "min_depth",
            "minDepth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LeafSpec,
            InnerSpec,
            MaxDepth,
            MinDepth,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "leafSpec" | "leaf_spec" => Ok(GeneratedField::LeafSpec),
                            "innerSpec" | "inner_spec" => Ok(GeneratedField::InnerSpec),
                            "maxDepth" | "max_depth" => Ok(GeneratedField::MaxDepth),
                            "minDepth" | "min_depth" => Ok(GeneratedField::MinDepth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProofSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ics23.ProofSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProofSpec, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut leaf_spec__ = None;
                let mut inner_spec__ = None;
                let mut max_depth__ = None;
                let mut min_depth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LeafSpec => {
                            if leaf_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leafSpec"));
                            }
                            leaf_spec__ = map_.next_value()?;
                        }
                        GeneratedField::InnerSpec => {
                            if inner_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("innerSpec"));
                            }
                            inner_spec__ = map_.next_value()?;
                        }
                        GeneratedField::MaxDepth => {
                            if max_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDepth"));
                            }
                            max_depth__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinDepth => {
                            if min_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDepth"));
                            }
                            min_depth__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ProofSpec {
                    leaf_spec: leaf_spec__,
                    inner_spec: inner_spec__,
                    max_depth: max_depth__.unwrap_or_default(),
                    min_depth: min_depth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ics23.ProofSpec", FIELDS, GeneratedVisitor)
    }
}
