// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for BlockMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response_commit.is_some() {
            len += 1;
        }
        if self.request_finalize_block.is_some() {
            len += 1;
        }
        if self.response_finalize_block.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.store.v1beta1.BlockMetadata", len)?;
        if let Some(v) = self.response_commit.as_ref() {
            struct_ser.serialize_field("responseCommit", v)?;
        }
        if let Some(v) = self.request_finalize_block.as_ref() {
            struct_ser.serialize_field("requestFinalizeBlock", v)?;
        }
        if let Some(v) = self.response_finalize_block.as_ref() {
            struct_ser.serialize_field("responseFinalizeBlock", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BlockMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response_commit",
            "responseCommit",
            "request_finalize_block",
            "requestFinalizeBlock",
            "response_finalize_block",
            "responseFinalizeBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponseCommit,
            RequestFinalizeBlock,
            ResponseFinalizeBlock,
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
                            "responseCommit" | "response_commit" => {
                                Ok(GeneratedField::ResponseCommit)
                            }
                            "requestFinalizeBlock" | "request_finalize_block" => {
                                Ok(GeneratedField::RequestFinalizeBlock)
                            }
                            "responseFinalizeBlock" | "response_finalize_block" => {
                                Ok(GeneratedField::ResponseFinalizeBlock)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.store.v1beta1.BlockMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockMetadata, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut response_commit__ = None;
                let mut request_finalize_block__ = None;
                let mut response_finalize_block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResponseCommit => {
                            if response_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseCommit"));
                            }
                            response_commit__ = map_.next_value()?;
                        }
                        GeneratedField::RequestFinalizeBlock => {
                            if request_finalize_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requestFinalizeBlock",
                                ));
                            }
                            request_finalize_block__ = map_.next_value()?;
                        }
                        GeneratedField::ResponseFinalizeBlock => {
                            if response_finalize_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "responseFinalizeBlock",
                                ));
                            }
                            response_finalize_block__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BlockMetadata {
                    response_commit: response_commit__,
                    request_finalize_block: request_finalize_block__,
                    response_finalize_block: response_finalize_block__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.store.v1beta1.BlockMetadata",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CommitId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.store.v1beta1.CommitID", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommitId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["version", "hash"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
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
            type Value = CommitId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.store.v1beta1.CommitID")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommitId, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(CommitId {
                    version: version__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.store.v1beta1.CommitID", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CommitInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.store_infos.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.store.v1beta1.CommitInfo", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if !self.store_infos.is_empty() {
            struct_ser.serialize_field("storeInfos", &self.store_infos)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommitInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["version", "store_infos", "storeInfos", "timestamp"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            StoreInfos,
            Timestamp,
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
                            "version" => Ok(GeneratedField::Version),
                            "storeInfos" | "store_infos" => Ok(GeneratedField::StoreInfos),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.store.v1beta1.CommitInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommitInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut store_infos__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::StoreInfos => {
                            if store_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storeInfos"));
                            }
                            store_infos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CommitInfo {
                    version: version__.unwrap_or_default(),
                    store_infos: store_infos__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("cosmos.store.v1beta1.CommitInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StoreInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.commit_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.store.v1beta1.StoreInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.commit_id.as_ref() {
            struct_ser.serialize_field("commitId", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StoreInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "commit_id", "commitId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CommitId,
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
                            "name" => Ok(GeneratedField::Name),
                            "commitId" | "commit_id" => Ok(GeneratedField::CommitId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.store.v1beta1.StoreInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoreInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut commit_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CommitId => {
                            if commit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitId"));
                            }
                            commit_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StoreInfo {
                    name: name__.unwrap_or_default(),
                    commit_id: commit_id__,
                })
            }
        }
        deserializer.deserialize_struct("cosmos.store.v1beta1.StoreInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StoreKvPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_key.is_empty() {
            len += 1;
        }
        if self.delete {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.store.v1beta1.StoreKVPair", len)?;
        if !self.store_key.is_empty() {
            struct_ser.serialize_field("storeKey", &self.store_key)?;
        }
        if self.delete {
            struct_ser.serialize_field("delete", &self.delete)?;
        }
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
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StoreKvPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["store_key", "storeKey", "delete", "key", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreKey,
            Delete,
            Key,
            Value,
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
                            "storeKey" | "store_key" => Ok(GeneratedField::StoreKey),
                            "delete" => Ok(GeneratedField::Delete),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreKvPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.store.v1beta1.StoreKVPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoreKvPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut store_key__ = None;
                let mut delete__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreKey => {
                            if store_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storeKey"));
                            }
                            store_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Delete => {
                            if delete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            delete__ = Some(map_.next_value()?);
                        }
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
                    }
                }
                Ok(StoreKvPair {
                    store_key: store_key__.unwrap_or_default(),
                    delete: delete__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.store.v1beta1.StoreKVPair",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
