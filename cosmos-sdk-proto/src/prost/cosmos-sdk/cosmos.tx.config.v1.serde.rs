impl serde::Serialize for Config {
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
        let mut struct_ser = serializer.serialize_struct("cosmos.tx.config.v1.Config", len)?;
        if true {
            struct_ser.serialize_field("skipAnteHandler", &self.skip_ante_handler)?;
        }
        if true {
            struct_ser.serialize_field("skipPostHandler", &self.skip_post_handler)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "skip_ante_handler",
            "skipAnteHandler",
            "skip_post_handler",
            "skipPostHandler",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SkipAnteHandler,
            SkipPostHandler,
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
                            "skipAnteHandler" | "skip_ante_handler" => Ok(GeneratedField::SkipAnteHandler),
                            "skipPostHandler" | "skip_post_handler" => Ok(GeneratedField::SkipPostHandler),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.tx.config.v1.Config")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut skip_ante_handler__ = None;
                let mut skip_post_handler__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SkipAnteHandler => {
                            if skip_ante_handler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipAnteHandler"));
                            }
                            skip_ante_handler__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipPostHandler => {
                            if skip_post_handler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipPostHandler"));
                            }
                            skip_post_handler__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Config {
                    skip_ante_handler: skip_ante_handler__.unwrap_or_default(),
                    skip_post_handler: skip_post_handler__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.tx.config.v1.Config", FIELDS, GeneratedVisitor)
    }
}
