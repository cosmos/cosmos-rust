// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for ConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmos.base.node.v1beta1.ConfigRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.node.v1beta1.ConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ConfigRequest {})
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.node.v1beta1.ConfigRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.minimum_gas_price.is_empty() {
            len += 1;
        }
        if !self.pruning_keep_recent.is_empty() {
            len += 1;
        }
        if !self.pruning_interval.is_empty() {
            len += 1;
        }
        if self.halt_height != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.base.node.v1beta1.ConfigResponse", len)?;
        if !self.minimum_gas_price.is_empty() {
            struct_ser.serialize_field("minimumGasPrice", &self.minimum_gas_price)?;
        }
        if !self.pruning_keep_recent.is_empty() {
            struct_ser.serialize_field("pruningKeepRecent", &self.pruning_keep_recent)?;
        }
        if !self.pruning_interval.is_empty() {
            struct_ser.serialize_field("pruningInterval", &self.pruning_interval)?;
        }
        if self.halt_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "haltHeight",
                ToString::to_string(&self.halt_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_gas_price",
            "minimumGasPrice",
            "pruning_keep_recent",
            "pruningKeepRecent",
            "pruning_interval",
            "pruningInterval",
            "halt_height",
            "haltHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumGasPrice,
            PruningKeepRecent,
            PruningInterval,
            HaltHeight,
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
                            "minimumGasPrice" | "minimum_gas_price" => {
                                Ok(GeneratedField::MinimumGasPrice)
                            }
                            "pruningKeepRecent" | "pruning_keep_recent" => {
                                Ok(GeneratedField::PruningKeepRecent)
                            }
                            "pruningInterval" | "pruning_interval" => {
                                Ok(GeneratedField::PruningInterval)
                            }
                            "haltHeight" | "halt_height" => Ok(GeneratedField::HaltHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.node.v1beta1.ConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut minimum_gas_price__ = None;
                let mut pruning_keep_recent__ = None;
                let mut pruning_interval__ = None;
                let mut halt_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinimumGasPrice => {
                            if minimum_gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumGasPrice"));
                            }
                            minimum_gas_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PruningKeepRecent => {
                            if pruning_keep_recent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pruningKeepRecent"));
                            }
                            pruning_keep_recent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PruningInterval => {
                            if pruning_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pruningInterval"));
                            }
                            pruning_interval__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HaltHeight => {
                            if halt_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("haltHeight"));
                            }
                            halt_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ConfigResponse {
                    minimum_gas_price: minimum_gas_price__.unwrap_or_default(),
                    pruning_keep_recent: pruning_keep_recent__.unwrap_or_default(),
                    pruning_interval: pruning_interval__.unwrap_or_default(),
                    halt_height: halt_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.node.v1beta1.ConfigResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmos.base.node.v1beta1.StatusRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.node.v1beta1.StatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatusRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StatusRequest {})
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.node.v1beta1.StatusRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.earliest_store_height != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        if !self.validator_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.base.node.v1beta1.StatusResponse", len)?;
        if self.earliest_store_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "earliestStoreHeight",
                ToString::to_string(&self.earliest_store_height).as_str(),
            )?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.app_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "appHash",
                pbjson::private::base64::encode(&self.app_hash).as_str(),
            )?;
        }
        if !self.validator_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "validatorHash",
                pbjson::private::base64::encode(&self.validator_hash).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "earliest_store_height",
            "earliestStoreHeight",
            "height",
            "timestamp",
            "app_hash",
            "appHash",
            "validator_hash",
            "validatorHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EarliestStoreHeight,
            Height,
            Timestamp,
            AppHash,
            ValidatorHash,
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
                            "earliestStoreHeight" | "earliest_store_height" => {
                                Ok(GeneratedField::EarliestStoreHeight)
                            }
                            "height" => Ok(GeneratedField::Height),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "appHash" | "app_hash" => Ok(GeneratedField::AppHash),
                            "validatorHash" | "validator_hash" => Ok(GeneratedField::ValidatorHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.node.v1beta1.StatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatusResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut earliest_store_height__ = None;
                let mut height__ = None;
                let mut timestamp__ = None;
                let mut app_hash__ = None;
                let mut validator_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EarliestStoreHeight => {
                            if earliest_store_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "earliestStoreHeight",
                                ));
                            }
                            earliest_store_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::AppHash => {
                            if app_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appHash"));
                            }
                            app_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValidatorHash => {
                            if validator_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorHash"));
                            }
                            validator_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(StatusResponse {
                    earliest_store_height: earliest_store_height__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    timestamp: timestamp__,
                    app_hash: app_hash__.unwrap_or_default(),
                    validator_hash: validator_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.node.v1beta1.StatusResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
