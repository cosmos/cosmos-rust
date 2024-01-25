impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if true {
            len += 1;
        }
        if true {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.group.module.v1.Module", len)?;
        if let Some(v) = self.max_execution_period.as_ref() {
            struct_ser.serialize_field("maxExecutionPeriod", v)?;
        }
        if true {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxMetadataLen", ::alloc::string::ToString::to_string(&self.max_metadata_len).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_execution_period",
            "maxExecutionPeriod",
            "max_metadata_len",
            "maxMetadataLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxExecutionPeriod,
            MaxMetadataLen,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxExecutionPeriod" | "max_execution_period" => Ok(GeneratedField::MaxExecutionPeriod),
                            "maxMetadataLen" | "max_metadata_len" => Ok(GeneratedField::MaxMetadataLen),
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
                formatter.write_str("struct cosmos.group.module.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Module, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_execution_period__ = None;
                let mut max_metadata_len__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxExecutionPeriod => {
                            if max_execution_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxExecutionPeriod"));
                            }
                            max_execution_period__ = map_.next_value()?;
                        }
                        GeneratedField::MaxMetadataLen => {
                            if max_metadata_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMetadataLen"));
                            }
                            max_metadata_len__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Module {
                    max_execution_period: max_execution_period__,
                    max_metadata_len: max_metadata_len__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.group.module.v1.Module", FIELDS, GeneratedVisitor)
    }
}
