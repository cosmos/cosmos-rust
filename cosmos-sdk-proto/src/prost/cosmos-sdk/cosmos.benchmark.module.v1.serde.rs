// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for GeneratorParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seed != 0 {
            len += 1;
        }
        if self.bucket_count != 0 {
            len += 1;
        }
        if self.key_mean != 0 {
            len += 1;
        }
        if self.key_std_dev != 0 {
            len += 1;
        }
        if self.value_mean != 0 {
            len += 1;
        }
        if self.value_std_dev != 0 {
            len += 1;
        }
        if self.genesis_count != 0 {
            len += 1;
        }
        if self.insert_weight != 0. {
            len += 1;
        }
        if self.update_weight != 0. {
            len += 1;
        }
        if self.get_weight != 0. {
            len += 1;
        }
        if self.delete_weight != 0. {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.benchmark.module.v1.GeneratorParams", len)?;
        if self.seed != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "seed",
                alloc::string::ToString::to_string(&self.seed).as_str(),
            )?;
        }
        if self.bucket_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bucketCount",
                alloc::string::ToString::to_string(&self.bucket_count).as_str(),
            )?;
        }
        if self.key_mean != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "keyMean",
                alloc::string::ToString::to_string(&self.key_mean).as_str(),
            )?;
        }
        if self.key_std_dev != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "keyStdDev",
                alloc::string::ToString::to_string(&self.key_std_dev).as_str(),
            )?;
        }
        if self.value_mean != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valueMean",
                alloc::string::ToString::to_string(&self.value_mean).as_str(),
            )?;
        }
        if self.value_std_dev != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valueStdDev",
                alloc::string::ToString::to_string(&self.value_std_dev).as_str(),
            )?;
        }
        if self.genesis_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "genesisCount",
                alloc::string::ToString::to_string(&self.genesis_count).as_str(),
            )?;
        }
        if self.insert_weight != 0. {
            struct_ser.serialize_field("insertWeight", &self.insert_weight)?;
        }
        if self.update_weight != 0. {
            struct_ser.serialize_field("updateWeight", &self.update_weight)?;
        }
        if self.get_weight != 0. {
            struct_ser.serialize_field("getWeight", &self.get_weight)?;
        }
        if self.delete_weight != 0. {
            struct_ser.serialize_field("deleteWeight", &self.delete_weight)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GeneratorParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seed",
            "bucket_count",
            "bucketCount",
            "key_mean",
            "keyMean",
            "key_std_dev",
            "keyStdDev",
            "value_mean",
            "valueMean",
            "value_std_dev",
            "valueStdDev",
            "genesis_count",
            "genesisCount",
            "insert_weight",
            "insertWeight",
            "update_weight",
            "updateWeight",
            "get_weight",
            "getWeight",
            "delete_weight",
            "deleteWeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seed,
            BucketCount,
            KeyMean,
            KeyStdDev,
            ValueMean,
            ValueStdDev,
            GenesisCount,
            InsertWeight,
            UpdateWeight,
            GetWeight,
            DeleteWeight,
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
                            "seed" => Ok(GeneratedField::Seed),
                            "bucketCount" | "bucket_count" => Ok(GeneratedField::BucketCount),
                            "keyMean" | "key_mean" => Ok(GeneratedField::KeyMean),
                            "keyStdDev" | "key_std_dev" => Ok(GeneratedField::KeyStdDev),
                            "valueMean" | "value_mean" => Ok(GeneratedField::ValueMean),
                            "valueStdDev" | "value_std_dev" => Ok(GeneratedField::ValueStdDev),
                            "genesisCount" | "genesis_count" => Ok(GeneratedField::GenesisCount),
                            "insertWeight" | "insert_weight" => Ok(GeneratedField::InsertWeight),
                            "updateWeight" | "update_weight" => Ok(GeneratedField::UpdateWeight),
                            "getWeight" | "get_weight" => Ok(GeneratedField::GetWeight),
                            "deleteWeight" | "delete_weight" => Ok(GeneratedField::DeleteWeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeneratorParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.benchmark.module.v1.GeneratorParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GeneratorParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut seed__ = None;
                let mut bucket_count__ = None;
                let mut key_mean__ = None;
                let mut key_std_dev__ = None;
                let mut value_mean__ = None;
                let mut value_std_dev__ = None;
                let mut genesis_count__ = None;
                let mut insert_weight__ = None;
                let mut update_weight__ = None;
                let mut get_weight__ = None;
                let mut delete_weight__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seed => {
                            if seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seed"));
                            }
                            seed__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BucketCount => {
                            if bucket_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketCount"));
                            }
                            bucket_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::KeyMean => {
                            if key_mean__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMean"));
                            }
                            key_mean__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::KeyStdDev => {
                            if key_std_dev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyStdDev"));
                            }
                            key_std_dev__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValueMean => {
                            if value_mean__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMean"));
                            }
                            value_mean__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValueStdDev => {
                            if value_std_dev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueStdDev"));
                            }
                            value_std_dev__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::GenesisCount => {
                            if genesis_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisCount"));
                            }
                            genesis_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InsertWeight => {
                            if insert_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertWeight"));
                            }
                            insert_weight__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UpdateWeight => {
                            if update_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateWeight"));
                            }
                            update_weight__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::GetWeight => {
                            if get_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getWeight"));
                            }
                            get_weight__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DeleteWeight => {
                            if delete_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteWeight"));
                            }
                            delete_weight__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(GeneratorParams {
                    seed: seed__.unwrap_or_default(),
                    bucket_count: bucket_count__.unwrap_or_default(),
                    key_mean: key_mean__.unwrap_or_default(),
                    key_std_dev: key_std_dev__.unwrap_or_default(),
                    value_mean: value_mean__.unwrap_or_default(),
                    value_std_dev: value_std_dev__.unwrap_or_default(),
                    genesis_count: genesis_count__.unwrap_or_default(),
                    insert_weight: insert_weight__.unwrap_or_default(),
                    update_weight: update_weight__.unwrap_or_default(),
                    get_weight: get_weight__.unwrap_or_default(),
                    delete_weight: delete_weight__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.benchmark.module.v1.GeneratorParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.genesis_params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.benchmark.module.v1.Module", len)?;
        if let Some(v) = self.genesis_params.as_ref() {
            struct_ser.serialize_field("genesisParams", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["genesis_params", "genesisParams"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GenesisParams,
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
                            "genesisParams" | "genesis_params" => Ok(GeneratedField::GenesisParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Module;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.benchmark.module.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Module, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut genesis_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GenesisParams => {
                            if genesis_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesisParams"));
                            }
                            genesis_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Module {
                    genesis_params: genesis_params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.benchmark.module.v1.Module",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
