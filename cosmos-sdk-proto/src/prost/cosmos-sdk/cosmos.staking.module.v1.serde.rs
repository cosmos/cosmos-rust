// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hooks_order.is_empty() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.bech32_prefix_validator.is_empty() {
            len += 1;
        }
        if !self.bech32_prefix_consensus.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.staking.module.v1.Module", len)?;
        if !self.hooks_order.is_empty() {
            struct_ser.serialize_field("hooksOrder", &self.hooks_order)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.bech32_prefix_validator.is_empty() {
            struct_ser.serialize_field("bech32PrefixValidator", &self.bech32_prefix_validator)?;
        }
        if !self.bech32_prefix_consensus.is_empty() {
            struct_ser.serialize_field("bech32PrefixConsensus", &self.bech32_prefix_consensus)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hooks_order",
            "hooksOrder",
            "authority",
            "bech32_prefix_validator",
            "bech32PrefixValidator",
            "bech32_prefix_consensus",
            "bech32PrefixConsensus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HooksOrder,
            Authority,
            Bech32PrefixValidator,
            Bech32PrefixConsensus,
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
                            "hooksOrder" | "hooks_order" => Ok(GeneratedField::HooksOrder),
                            "authority" => Ok(GeneratedField::Authority),
                            "bech32PrefixValidator" | "bech32_prefix_validator" => {
                                Ok(GeneratedField::Bech32PrefixValidator)
                            }
                            "bech32PrefixConsensus" | "bech32_prefix_consensus" => {
                                Ok(GeneratedField::Bech32PrefixConsensus)
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
            type Value = Module;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.module.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Module, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hooks_order__ = None;
                let mut authority__ = None;
                let mut bech32_prefix_validator__ = None;
                let mut bech32_prefix_consensus__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HooksOrder => {
                            if hooks_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hooksOrder"));
                            }
                            hooks_order__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bech32PrefixValidator => {
                            if bech32_prefix_validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bech32PrefixValidator",
                                ));
                            }
                            bech32_prefix_validator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bech32PrefixConsensus => {
                            if bech32_prefix_consensus__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bech32PrefixConsensus",
                                ));
                            }
                            bech32_prefix_consensus__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Module {
                    hooks_order: hooks_order__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    bech32_prefix_validator: bech32_prefix_validator__.unwrap_or_default(),
                    bech32_prefix_consensus: bech32_prefix_consensus__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.staking.module.v1.Module", FIELDS, GeneratedVisitor)
    }
}
