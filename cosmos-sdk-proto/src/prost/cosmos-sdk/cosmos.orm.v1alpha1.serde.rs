// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for ModuleSchemaDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema_file.is_empty() {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.orm.v1alpha1.ModuleSchemaDescriptor", len)?;
        if !self.schema_file.is_empty() {
            struct_ser.serialize_field("schemaFile", &self.schema_file)?;
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
impl<'de> serde::Deserialize<'de> for ModuleSchemaDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["schema_file", "schemaFile", "prefix"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SchemaFile,
            Prefix,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schemaFile" | "schema_file" => Ok(GeneratedField::SchemaFile),
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
            type Value = ModuleSchemaDescriptor;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.orm.v1alpha1.ModuleSchemaDescriptor")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<ModuleSchemaDescriptor, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut schema_file__ = None;
                let mut prefix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SchemaFile => {
                            if schema_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaFile"));
                            }
                            schema_file__ = Some(map_.next_value()?);
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
                Ok(ModuleSchemaDescriptor {
                    schema_file: schema_file__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.orm.v1alpha1.ModuleSchemaDescriptor",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for module_schema_descriptor::FileEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.proto_file_name.is_empty() {
            len += 1;
        }
        if self.storage_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.orm.v1alpha1.ModuleSchemaDescriptor.FileEntry", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.proto_file_name.is_empty() {
            struct_ser.serialize_field("protoFileName", &self.proto_file_name)?;
        }
        if self.storage_type != 0 {
            let v = StorageType::try_from(self.storage_type).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.storage_type))
            })?;
            struct_ser.serialize_field("storageType", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for module_schema_descriptor::FileEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "proto_file_name",
            "protoFileName",
            "storage_type",
            "storageType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProtoFileName,
            StorageType,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "protoFileName" | "proto_file_name" => {
                                Ok(GeneratedField::ProtoFileName)
                            }
                            "storageType" | "storage_type" => Ok(GeneratedField::StorageType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = module_schema_descriptor::FileEntry;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.orm.v1alpha1.ModuleSchemaDescriptor.FileEntry")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<module_schema_descriptor::FileEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut proto_file_name__ = None;
                let mut storage_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ProtoFileName => {
                            if proto_file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoFileName"));
                            }
                            proto_file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageType => {
                            if storage_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageType"));
                            }
                            storage_type__ = Some(map_.next_value::<StorageType>()? as i32);
                        }
                    }
                }
                Ok(module_schema_descriptor::FileEntry {
                    id: id__.unwrap_or_default(),
                    proto_file_name: proto_file_name__.unwrap_or_default(),
                    storage_type: storage_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.orm.v1alpha1.ModuleSchemaDescriptor.FileEntry",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StorageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DefaultUnspecified => "STORAGE_TYPE_DEFAULT_UNSPECIFIED",
            Self::Memory => "STORAGE_TYPE_MEMORY",
            Self::Transient => "STORAGE_TYPE_TRANSIENT",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StorageType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STORAGE_TYPE_DEFAULT_UNSPECIFIED",
            "STORAGE_TYPE_MEMORY",
            "STORAGE_TYPE_TRANSIENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageType;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
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

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
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

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STORAGE_TYPE_DEFAULT_UNSPECIFIED" => Ok(StorageType::DefaultUnspecified),
                    "STORAGE_TYPE_MEMORY" => Ok(StorageType::Memory),
                    "STORAGE_TYPE_TRANSIENT" => Ok(StorageType::Transient),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
