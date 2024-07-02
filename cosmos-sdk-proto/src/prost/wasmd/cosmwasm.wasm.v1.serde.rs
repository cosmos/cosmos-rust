// @generated
impl serde::Serialize for AbsoluteTxPosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_height != 0 {
            len += 1;
        }
        if self.tx_index != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AbsoluteTxPosition", len)?;
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if self.tx_index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txIndex", ToString::to_string(&self.tx_index).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbsoluteTxPosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["block_height", "blockHeight", "tx_index", "txIndex"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeight,
            TxIndex,
        }
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
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "txIndex" | "tx_index" => Ok(GeneratedField::TxIndex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbsoluteTxPosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AbsoluteTxPosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbsoluteTxPosition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block_height__ = None;
                let mut tx_index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxIndex => {
                            if tx_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txIndex"));
                            }
                            tx_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(AbsoluteTxPosition {
                    block_height: block_height__.unwrap_or_default(),
                    tx_index: tx_index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AbsoluteTxPosition",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AcceptedMessageKeysFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AcceptedMessageKeysFilter", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AcceptedMessageKeysFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["keys"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
        }
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
                            "keys" => Ok(GeneratedField::Keys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AcceptedMessageKeysFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AcceptedMessageKeysFilter")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AcceptedMessageKeysFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AcceptedMessageKeysFilter {
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AcceptedMessageKeysFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AcceptedMessagesFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AcceptedMessagesFilter", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field(
                "messages",
                &self
                    .messages
                    .iter()
                    .map(pbjson::private::base64::encode)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AcceptedMessagesFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["messages"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
        }
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
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AcceptedMessagesFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AcceptedMessagesFilter")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AcceptedMessagesFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(
                                map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(AcceptedMessagesFilter {
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AcceptedMessagesFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AccessConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permission != 0 {
            len += 1;
        }
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.AccessConfig", len)?;
        if self.permission != 0 {
            let v = AccessType::try_from(self.permission).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.permission))
            })?;
            struct_ser.serialize_field("permission", &v)?;
        }
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["permission", "addresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permission,
            Addresses,
        }
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
                            "permission" => Ok(GeneratedField::Permission),
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut permission__ = None;
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map_.next_value::<AccessType>()? as i32);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AccessConfig {
                    permission: permission__.unwrap_or_default(),
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.AccessConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AccessConfigUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AccessConfigUpdate", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessConfigUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "instantiate_permission",
            "instantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            InstantiatePermission,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
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
            type Value = AccessConfigUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessConfigUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessConfigUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AccessConfigUpdate {
                    code_id: code_id__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AccessConfigUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AccessType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            Self::Nobody => "ACCESS_TYPE_NOBODY",
            Self::Everybody => "ACCESS_TYPE_EVERYBODY",
            Self::AnyOfAddresses => "ACCESS_TYPE_ANY_OF_ADDRESSES",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AccessType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACCESS_TYPE_UNSPECIFIED",
            "ACCESS_TYPE_NOBODY",
            "ACCESS_TYPE_EVERYBODY",
            "ACCESS_TYPE_ANY_OF_ADDRESSES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessType;

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
                    "ACCESS_TYPE_UNSPECIFIED" => Ok(AccessType::Unspecified),
                    "ACCESS_TYPE_NOBODY" => Ok(AccessType::Nobody),
                    "ACCESS_TYPE_EVERYBODY" => Ok(AccessType::Everybody),
                    "ACCESS_TYPE_ANY_OF_ADDRESSES" => Ok(AccessType::AnyOfAddresses),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AccessTypeParam {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AccessTypeParam", len)?;
        if self.value != 0 {
            let v = AccessType::try_from(self.value).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.value))
            })?;
            struct_ser.serialize_field("value", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessTypeParam {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
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
            type Value = AccessTypeParam;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessTypeParam")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessTypeParam, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value::<AccessType>()? as i32);
                        }
                    }
                }
                Ok(AccessTypeParam {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AccessTypeParam",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AllowAllMessagesFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AllowAllMessagesFilter", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowAllMessagesFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = AllowAllMessagesFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AllowAllMessagesFilter")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AllowAllMessagesFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AllowAllMessagesFilter {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AllowAllMessagesFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ClearAdminProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ClearAdminProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClearAdminProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClearAdminProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ClearAdminProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClearAdminProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClearAdminProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ClearAdminProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if self.code_info.is_some() {
            len += 1;
        }
        if !self.code_bytes.is_empty() {
            len += 1;
        }
        if self.pinned {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Code", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.code_info.as_ref() {
            struct_ser.serialize_field("codeInfo", v)?;
        }
        if !self.code_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeBytes",
                pbjson::private::base64::encode(&self.code_bytes).as_str(),
            )?;
        }
        if self.pinned {
            struct_ser.serialize_field("pinned", &self.pinned)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "code_info",
            "codeInfo",
            "code_bytes",
            "codeBytes",
            "pinned",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            CodeInfo,
            CodeBytes,
            Pinned,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "codeInfo" | "code_info" => Ok(GeneratedField::CodeInfo),
                            "codeBytes" | "code_bytes" => Ok(GeneratedField::CodeBytes),
                            "pinned" => Ok(GeneratedField::Pinned),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Code;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Code")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Code, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut code_info__ = None;
                let mut code_bytes__ = None;
                let mut pinned__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CodeInfo => {
                            if code_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeInfo"));
                            }
                            code_info__ = map_.next_value()?;
                        }
                        GeneratedField::CodeBytes => {
                            if code_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeBytes"));
                            }
                            code_bytes__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pinned => {
                            if pinned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pinned"));
                            }
                            pinned__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Code {
                    code_id: code_id__.unwrap_or_default(),
                    code_info: code_info__,
                    code_bytes: code_bytes__.unwrap_or_default(),
                    pinned: pinned__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Code", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CodeGrant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code_hash.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.CodeGrant", len)?;
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CodeGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_hash",
            "codeHash",
            "instantiate_permission",
            "instantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeHash,
            InstantiatePermission,
        }
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
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
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
            type Value = CodeGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.CodeGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CodeGrant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_hash__ = None;
                let mut instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CodeGrant {
                    code_hash: code_hash__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.CodeGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CodeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code_hash.is_empty() {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if self.instantiate_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.CodeInfo", len)?;
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if let Some(v) = self.instantiate_config.as_ref() {
            struct_ser.serialize_field("instantiateConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CodeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_hash",
            "codeHash",
            "creator",
            "instantiate_config",
            "instantiateConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeHash,
            Creator,
            InstantiateConfig,
        }
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
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            "creator" => Ok(GeneratedField::Creator),
                            "instantiateConfig" | "instantiate_config" => {
                                Ok(GeneratedField::InstantiateConfig)
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
            type Value = CodeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.CodeInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CodeInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_hash__ = None;
                let mut creator__ = None;
                let mut instantiate_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstantiateConfig => {
                            if instantiate_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instantiateConfig"));
                            }
                            instantiate_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CodeInfo {
                    code_hash: code_hash__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    instantiate_config: instantiate_config__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.CodeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CodeInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if !self.data_hash.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.CodeInfoResponse", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if !self.data_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "dataHash",
                pbjson::private::base64::encode(&self.data_hash).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CodeInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "creator",
            "data_hash",
            "dataHash",
            "instantiate_permission",
            "instantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Creator,
            DataHash,
            InstantiatePermission,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "creator" => Ok(GeneratedField::Creator),
                            "dataHash" | "data_hash" => Ok(GeneratedField::DataHash),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
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
            type Value = CodeInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.CodeInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CodeInfoResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut creator__ = None;
                let mut data_hash__ = None;
                let mut instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataHash => {
                            if data_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataHash"));
                            }
                            data_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CodeInfoResponse {
                    code_id: code_id__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    data_hash: data_hash__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.CodeInfoResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for CombinedLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calls_remaining != 0 {
            len += 1;
        }
        if !self.amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.CombinedLimit", len)?;
        if self.calls_remaining != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "callsRemaining",
                ToString::to_string(&self.calls_remaining).as_str(),
            )?;
        }
        if !self.amounts.is_empty() {
            struct_ser.serialize_field("amounts", &self.amounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CombinedLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["calls_remaining", "callsRemaining", "amounts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CallsRemaining,
            Amounts,
        }
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
                            "callsRemaining" | "calls_remaining" => {
                                Ok(GeneratedField::CallsRemaining)
                            }
                            "amounts" => Ok(GeneratedField::Amounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CombinedLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.CombinedLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CombinedLimit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut calls_remaining__ = None;
                let mut amounts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CallsRemaining => {
                            if calls_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callsRemaining"));
                            }
                            calls_remaining__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Amounts => {
                            if amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amounts"));
                            }
                            amounts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CombinedLimit {
                    calls_remaining: calls_remaining__.unwrap_or_default(),
                    amounts: amounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.CombinedLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Contract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.contract_info.is_some() {
            len += 1;
        }
        if !self.contract_state.is_empty() {
            len += 1;
        }
        if !self.contract_code_history.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Contract", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.contract_info.as_ref() {
            struct_ser.serialize_field("contractInfo", v)?;
        }
        if !self.contract_state.is_empty() {
            struct_ser.serialize_field("contractState", &self.contract_state)?;
        }
        if !self.contract_code_history.is_empty() {
            struct_ser.serialize_field("contractCodeHistory", &self.contract_code_history)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Contract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "contract_info",
            "contractInfo",
            "contract_state",
            "contractState",
            "contract_code_history",
            "contractCodeHistory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            ContractInfo,
            ContractState,
            ContractCodeHistory,
        }
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
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "contractInfo" | "contract_info" => Ok(GeneratedField::ContractInfo),
                            "contractState" | "contract_state" => Ok(GeneratedField::ContractState),
                            "contractCodeHistory" | "contract_code_history" => {
                                Ok(GeneratedField::ContractCodeHistory)
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
            type Value = Contract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Contract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Contract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut contract_info__ = None;
                let mut contract_state__ = None;
                let mut contract_code_history__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractInfo => {
                            if contract_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractInfo"));
                            }
                            contract_info__ = map_.next_value()?;
                        }
                        GeneratedField::ContractState => {
                            if contract_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractState"));
                            }
                            contract_state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractCodeHistory => {
                            if contract_code_history__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractCodeHistory",
                                ));
                            }
                            contract_code_history__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Contract {
                    contract_address: contract_address__.unwrap_or_default(),
                    contract_info: contract_info__,
                    contract_state: contract_state__.unwrap_or_default(),
                    contract_code_history: contract_code_history__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Contract", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractCodeHistoryEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation != 0 {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if self.updated.is_some() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ContractCodeHistoryEntry", len)?;
        if self.operation != 0 {
            let v = ContractCodeHistoryOperationType::try_from(self.operation).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.operation))
            })?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.updated.as_ref() {
            struct_ser.serialize_field("updated", v)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractCodeHistoryEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["operation", "code_id", "codeId", "updated", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            CodeId,
            Updated,
            Msg,
        }
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
                            "operation" => Ok(GeneratedField::Operation),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "updated" => Ok(GeneratedField::Updated),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractCodeHistoryEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractCodeHistoryEntry")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractCodeHistoryEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut code_id__ = None;
                let mut updated__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ =
                                Some(map_.next_value::<ContractCodeHistoryOperationType>()? as i32);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Updated => {
                            if updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            updated__ = map_.next_value()?;
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ContractCodeHistoryEntry {
                    operation: operation__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    updated: updated__,
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ContractCodeHistoryEntry",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractCodeHistoryOperationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED",
            Self::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            Self::Migrate => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE",
            Self::Genesis => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ContractCodeHistoryOperationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractCodeHistoryOperationType;

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
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => {
                        Ok(ContractCodeHistoryOperationType::Unspecified)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => {
                        Ok(ContractCodeHistoryOperationType::Init)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => {
                        Ok(ContractCodeHistoryOperationType::Migrate)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => {
                        Ok(ContractCodeHistoryOperationType::Genesis)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContractExecutionAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grants.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ContractExecutionAuthorization", len)?;
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractExecutionAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["grants"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Grants,
        }
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
                            "grants" => Ok(GeneratedField::Grants),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractExecutionAuthorization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractExecutionAuthorization")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractExecutionAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut grants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContractExecutionAuthorization {
                    grants: grants__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ContractExecutionAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractGrant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.ContractGrant", len)?;
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract", "limit", "filter"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Contract,
            Limit,
            Filter,
        }
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
                            "contract" => Ok(GeneratedField::Contract),
                            "limit" => Ok(GeneratedField::Limit),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContractGrant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract__ = None;
                let mut limit__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = map_.next_value()?;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ContractGrant {
                    contract: contract__.unwrap_or_default(),
                    limit: limit__,
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.ContractGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if self.created.is_some() {
            len += 1;
        }
        if !self.ibc_port_id.is_empty() {
            len += 1;
        }
        if self.extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.ContractInfo", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if let Some(v) = self.created.as_ref() {
            struct_ser.serialize_field("created", v)?;
        }
        if !self.ibc_port_id.is_empty() {
            struct_ser.serialize_field("ibcPortId", &self.ibc_port_id)?;
        }
        if let Some(v) = self.extension.as_ref() {
            struct_ser.serialize_field("extension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "creator",
            "admin",
            "label",
            "created",
            "ibc_port_id",
            "ibcPortId",
            "extension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Creator,
            Admin,
            Label,
            Created,
            IbcPortId,
            Extension,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "creator" => Ok(GeneratedField::Creator),
                            "admin" => Ok(GeneratedField::Admin),
                            "label" => Ok(GeneratedField::Label),
                            "created" => Ok(GeneratedField::Created),
                            "ibcPortId" | "ibc_port_id" => Ok(GeneratedField::IbcPortId),
                            "extension" => Ok(GeneratedField::Extension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContractInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut creator__ = None;
                let mut admin__ = None;
                let mut label__ = None;
                let mut created__ = None;
                let mut ibc_port_id__ = None;
                let mut extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Created => {
                            if created__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            created__ = map_.next_value()?;
                        }
                        GeneratedField::IbcPortId => {
                            if ibc_port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcPortId"));
                            }
                            ibc_port_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ContractInfo {
                    code_id: code_id__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    created: created__,
                    ibc_port_id: ibc_port_id__.unwrap_or_default(),
                    extension: extension__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.ContractInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContractMigrationAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grants.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ContractMigrationAuthorization", len)?;
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractMigrationAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["grants"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Grants,
        }
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
                            "grants" => Ok(GeneratedField::Grants),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractMigrationAuthorization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractMigrationAuthorization")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractMigrationAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut grants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContractMigrationAuthorization {
                    grants: grants__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ContractMigrationAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ExecuteContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ExecuteContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "contract",
            "msg",
            "funds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Contract,
            Msg,
            Funds,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ExecuteContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ExecuteContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecuteContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ExecuteContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        if !self.codes.is_empty() {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        if !self.sequences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.codes.is_empty() {
            struct_ser.serialize_field("codes", &self.codes)?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        if !self.sequences.is_empty() {
            struct_ser.serialize_field("sequences", &self.sequences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params", "codes", "contracts", "sequences"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Codes,
            Contracts,
            Sequences,
        }
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
                            "params" => Ok(GeneratedField::Params),
                            "codes" => Ok(GeneratedField::Codes),
                            "contracts" => Ok(GeneratedField::Contracts),
                            "sequences" => Ok(GeneratedField::Sequences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut codes__ = None;
                let mut contracts__ = None;
                let mut sequences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Codes => {
                            if codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codes"));
                            }
                            codes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sequences => {
                            if sequences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequences"));
                            }
                            sequences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    codes: codes__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                    sequences: sequences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstantiateContract2Proposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.salt.is_empty() {
            len += 1;
        }
        if self.fix_msg {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.InstantiateContract2Proposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.salt.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("salt", pbjson::private::base64::encode(&self.salt).as_str())?;
        }
        if self.fix_msg {
            struct_ser.serialize_field("fixMsg", &self.fix_msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstantiateContract2Proposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "admin",
            "code_id",
            "codeId",
            "label",
            "msg",
            "funds",
            "salt",
            "fix_msg",
            "fixMsg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
            Salt,
            FixMsg,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "salt" => Ok(GeneratedField::Salt),
                            "fixMsg" | "fix_msg" => Ok(GeneratedField::FixMsg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstantiateContract2Proposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.InstantiateContract2Proposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<InstantiateContract2Proposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut salt__ = None;
                let mut fix_msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Salt => {
                            if salt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salt"));
                            }
                            salt__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FixMsg => {
                            if fix_msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixMsg"));
                            }
                            fix_msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InstantiateContract2Proposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    salt: salt__.unwrap_or_default(),
                    fix_msg: fix_msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.InstantiateContract2Proposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for InstantiateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.InstantiateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstantiateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "admin",
            "code_id",
            "codeId",
            "label",
            "msg",
            "funds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstantiateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.InstantiateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<InstantiateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InstantiateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.InstantiateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MaxCallsLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remaining != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MaxCallsLimit", len)?;
        if self.remaining != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("remaining", ToString::to_string(&self.remaining).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MaxCallsLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["remaining"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Remaining,
        }
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
                            "remaining" => Ok(GeneratedField::Remaining),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MaxCallsLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MaxCallsLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MaxCallsLimit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remaining__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Remaining => {
                            if remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remaining"));
                            }
                            remaining__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MaxCallsLimit {
                    remaining: remaining__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MaxCallsLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MaxFundsLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MaxFundsLimit", len)?;
        if !self.amounts.is_empty() {
            struct_ser.serialize_field("amounts", &self.amounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MaxFundsLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amounts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amounts,
        }
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
                            "amounts" => Ok(GeneratedField::Amounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MaxFundsLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MaxFundsLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MaxFundsLimit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amounts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amounts => {
                            if amounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amounts"));
                            }
                            amounts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MaxFundsLimit {
                    amounts: amounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MaxFundsLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MigrateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MigrateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MigrateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "contract",
            "code_id",
            "codeId",
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
            CodeId,
            Msg,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MigrateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MigrateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MigrateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                let mut code_id__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MigrateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MigrateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Model {
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
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Model", len)?;
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
impl<'de> serde::Deserialize<'de> for Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Model, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
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
                    }
                }
                Ok(Model {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Model", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddCodeUploadParamsAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddresses", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddCodeUploadParamsAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "addresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Addresses,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddCodeUploadParamsAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddresses")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddCodeUploadParamsAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddCodeUploadParamsAddresses {
                    authority: authority__.unwrap_or_default(),
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddresses",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddCodeUploadParamsAddressesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddressesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddCodeUploadParamsAddressesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgAddCodeUploadParamsAddressesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddressesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddCodeUploadParamsAddressesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddCodeUploadParamsAddressesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddressesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgClearAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgClearAdmin", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClearAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Contract,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClearAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgClearAdmin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgClearAdmin, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgClearAdmin {
                    sender: sender__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgClearAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgClearAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgClearAdminResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClearAdminResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgClearAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgClearAdminResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgClearAdminResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgClearAdminResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgClearAdminResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgExecuteContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgExecuteContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgExecuteContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract", "msg", "funds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Contract,
            Msg,
            Funds,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgExecuteContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgExecuteContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgExecuteContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgExecuteContract {
                    sender: sender__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgExecuteContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgExecuteContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgExecuteContractResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgExecuteContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgExecuteContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgExecuteContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgExecuteContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgExecuteContractResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgExecuteContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgIbcCloseChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgIBCCloseChannel", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgIbcCloseChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channel"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
        }
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgIbcCloseChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgIBCCloseChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgIbcCloseChannel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgIbcCloseChannel {
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgIBCCloseChannel",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgIbcSend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        if self.timeout_height != 0 {
            len += 1;
        }
        if self.timeout_timestamp != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgIBCSend", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if self.timeout_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "timeoutHeight",
                ToString::to_string(&self.timeout_height).as_str(),
            )?;
        }
        if self.timeout_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "timeoutTimestamp",
                ToString::to_string(&self.timeout_timestamp).as_str(),
            )?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgIbcSend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "timeout_height",
            "timeoutHeight",
            "timeout_timestamp",
            "timeoutTimestamp",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            TimeoutHeight,
            TimeoutTimestamp,
            Data,
        }
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
                            "channel" => Ok(GeneratedField::Channel),
                            "timeoutHeight" | "timeout_height" => Ok(GeneratedField::TimeoutHeight),
                            "timeoutTimestamp" | "timeout_timestamp" => {
                                Ok(GeneratedField::TimeoutTimestamp)
                            }
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgIbcSend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgIBCSend")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgIbcSend, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut timeout_height__ = None;
                let mut timeout_timestamp__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeoutHeight => {
                            if timeout_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutHeight"));
                            }
                            timeout_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TimeoutTimestamp => {
                            if timeout_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutTimestamp"));
                            }
                            timeout_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgIbcSend {
                    channel: channel__.unwrap_or_default(),
                    timeout_height: timeout_height__.unwrap_or_default(),
                    timeout_timestamp: timeout_timestamp__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgIBCSend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgIbcSendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sequence != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgIBCSendResponse", len)?;
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgIbcSendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sequence"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
        }
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgIbcSendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgIBCSendResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgIbcSendResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgIbcSendResponse {
                    sequence: sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgIBCSendResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgInstantiateContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgInstantiateContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgInstantiateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender", "admin", "code_id", "codeId", "label", "msg", "funds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgInstantiateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgInstantiateContract")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgInstantiateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgInstantiateContract {
                    sender: sender__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgInstantiateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgInstantiateContract2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.salt.is_empty() {
            len += 1;
        }
        if self.fix_msg {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgInstantiateContract2", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.salt.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("salt", pbjson::private::base64::encode(&self.salt).as_str())?;
        }
        if self.fix_msg {
            struct_ser.serialize_field("fixMsg", &self.fix_msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgInstantiateContract2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender", "admin", "code_id", "codeId", "label", "msg", "funds", "salt", "fix_msg",
            "fixMsg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
            Salt,
            FixMsg,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "salt" => Ok(GeneratedField::Salt),
                            "fixMsg" | "fix_msg" => Ok(GeneratedField::FixMsg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgInstantiateContract2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgInstantiateContract2")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgInstantiateContract2, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut salt__ = None;
                let mut fix_msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Salt => {
                            if salt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salt"));
                            }
                            salt__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FixMsg => {
                            if fix_msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixMsg"));
                            }
                            fix_msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgInstantiateContract2 {
                    sender: sender__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    salt: salt__.unwrap_or_default(),
                    fix_msg: fix_msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgInstantiateContract2",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgInstantiateContract2Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgInstantiateContract2Response", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgInstantiateContract2Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Data,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgInstantiateContract2Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgInstantiateContract2Response")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgInstantiateContract2Response, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgInstantiateContract2Response {
                    address: address__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgInstantiateContract2Response",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgInstantiateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgInstantiateContractResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgInstantiateContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Data,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgInstantiateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgInstantiateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgInstantiateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgInstantiateContractResponse {
                    address: address__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgInstantiateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgMigrateContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgMigrateContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMigrateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract", "code_id", "codeId", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Contract,
            CodeId,
            Msg,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "contract" => Ok(GeneratedField::Contract),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgMigrateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgMigrateContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgMigrateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract__ = None;
                let mut code_id__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgMigrateContract {
                    sender: sender__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgMigrateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgMigrateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgMigrateContractResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMigrateContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgMigrateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgMigrateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgMigrateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgMigrateContractResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgMigrateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPinCodes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgPinCodes", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPinCodes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            CodeIds,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgPinCodes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgPinCodes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPinCodes, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(MsgPinCodes {
                    authority: authority__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgPinCodes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgPinCodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgPinCodesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPinCodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgPinCodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgPinCodesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPinCodesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgPinCodesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgPinCodesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveCodeUploadParamsAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddresses", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveCodeUploadParamsAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "addresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Addresses,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveCodeUploadParamsAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddresses")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveCodeUploadParamsAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveCodeUploadParamsAddresses {
                    authority: authority__.unwrap_or_default(),
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddresses",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveCodeUploadParamsAddressesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddressesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveCodeUploadParamsAddressesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgRemoveCodeUploadParamsAddressesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddressesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveCodeUploadParamsAddressesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveCodeUploadParamsAddressesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddressesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgStoreAndInstantiateContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if self.unpin_code {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.builder.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgStoreAndInstantiateContract", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if self.unpin_code {
            struct_ser.serialize_field("unpinCode", &self.unpin_code)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.builder.is_empty() {
            struct_ser.serialize_field("builder", &self.builder)?;
        }
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreAndInstantiateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "unpin_code",
            "unpinCode",
            "admin",
            "label",
            "msg",
            "funds",
            "source",
            "builder",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            WasmByteCode,
            InstantiatePermission,
            UnpinCode,
            Admin,
            Label,
            Msg,
            Funds,
            Source,
            Builder,
            CodeHash,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "unpinCode" | "unpin_code" => Ok(GeneratedField::UnpinCode),
                            "admin" => Ok(GeneratedField::Admin),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "source" => Ok(GeneratedField::Source),
                            "builder" => Ok(GeneratedField::Builder),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgStoreAndInstantiateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgStoreAndInstantiateContract")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgStoreAndInstantiateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut unpin_code__ = None;
                let mut admin__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut source__ = None;
                let mut builder__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::UnpinCode => {
                            if unpin_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unpinCode"));
                            }
                            unpin_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Builder => {
                            if builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("builder"));
                            }
                            builder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgStoreAndInstantiateContract {
                    authority: authority__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    unpin_code: unpin_code__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    builder: builder__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgStoreAndInstantiateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgStoreAndInstantiateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmwasm.wasm.v1.MsgStoreAndInstantiateContractResponse",
            len,
        )?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreAndInstantiateContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Data,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgStoreAndInstantiateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmwasm.wasm.v1.MsgStoreAndInstantiateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgStoreAndInstantiateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgStoreAndInstantiateContractResponse {
                    address: address__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgStoreAndInstantiateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgStoreAndMigrateContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgStoreAndMigrateContract", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreAndMigrateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "contract",
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            WasmByteCode,
            InstantiatePermission,
            Contract,
            Msg,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgStoreAndMigrateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgStoreAndMigrateContract")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgStoreAndMigrateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgStoreAndMigrateContract {
                    authority: authority__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgStoreAndMigrateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgStoreAndMigrateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if !self.checksum.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmwasm.wasm.v1.MsgStoreAndMigrateContractResponse", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.checksum.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "checksum",
                pbjson::private::base64::encode(&self.checksum).as_str(),
            )?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreAndMigrateContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_id", "codeId", "checksum", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Checksum,
            Data,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "checksum" => Ok(GeneratedField::Checksum),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgStoreAndMigrateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgStoreAndMigrateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgStoreAndMigrateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut checksum__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Checksum => {
                            if checksum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checksum"));
                            }
                            checksum__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgStoreAndMigrateContractResponse {
                    code_id: code_id__.unwrap_or_default(),
                    checksum: checksum__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgStoreAndMigrateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgStoreCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgStoreCode", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            WasmByteCode,
            InstantiatePermission,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
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
            type Value = MsgStoreCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgStoreCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgStoreCode, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgStoreCode {
                    sender: sender__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgStoreCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgStoreCodeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if !self.checksum.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgStoreCodeResponse", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.checksum.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "checksum",
                pbjson::private::base64::encode(&self.checksum).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgStoreCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_id", "codeId", "checksum"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Checksum,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "checksum" => Ok(GeneratedField::Checksum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgStoreCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgStoreCodeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgStoreCodeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut checksum__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Checksum => {
                            if checksum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checksum"));
                            }
                            checksum__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgStoreCodeResponse {
                    code_id: code_id__.unwrap_or_default(),
                    checksum: checksum__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgStoreCodeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSudoContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgSudoContract", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSudoContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "contract", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Contract,
            Msg,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSudoContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgSudoContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSudoContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSudoContract {
                    authority: authority__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgSudoContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSudoContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgSudoContractResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSudoContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSudoContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgSudoContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSudoContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSudoContractResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgSudoContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpinCodes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgUnpinCodes", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpinCodes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            CodeIds,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnpinCodes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUnpinCodes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnpinCodes, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(MsgUnpinCodes {
                    authority: authority__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgUnpinCodes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUnpinCodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUnpinCodesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpinCodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgUnpinCodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUnpinCodesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnpinCodesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnpinCodesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUnpinCodesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.new_admin.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateAdmin", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.new_admin.is_empty() {
            struct_ser.serialize_field("newAdmin", &self.new_admin)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "new_admin", "newAdmin", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            NewAdmin,
            Contract,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "newAdmin" | "new_admin" => Ok(GeneratedField::NewAdmin),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateAdmin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateAdmin, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut new_admin__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateAdmin {
                    sender: sender__.unwrap_or_default(),
                    new_admin: new_admin__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.MsgUpdateAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateAdminResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateAdminResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgUpdateAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateAdminResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateAdminResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateAdminResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateAdminResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateContractLabel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.new_label.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateContractLabel", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.new_label.is_empty() {
            struct_ser.serialize_field("newLabel", &self.new_label)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateContractLabel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "new_label", "newLabel", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            NewLabel,
            Contract,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "newLabel" | "new_label" => Ok(GeneratedField::NewLabel),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateContractLabel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateContractLabel")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateContractLabel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut new_label__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewLabel => {
                            if new_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newLabel"));
                            }
                            new_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateContractLabel {
                    sender: sender__.unwrap_or_default(),
                    new_label: new_label__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateContractLabel",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateContractLabelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateContractLabelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateContractLabelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgUpdateContractLabelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateContractLabelResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateContractLabelResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateContractLabelResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateContractLabelResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateInstantiateConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if self.new_instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateInstantiateConfig", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.new_instantiate_permission.as_ref() {
            struct_ser.serialize_field("newInstantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateInstantiateConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "code_id",
            "codeId",
            "new_instantiate_permission",
            "newInstantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            CodeId,
            NewInstantiatePermission,
        }
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
                            "sender" => Ok(GeneratedField::Sender),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "newInstantiatePermission" | "new_instantiate_permission" => {
                                Ok(GeneratedField::NewInstantiatePermission)
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
            type Value = MsgUpdateInstantiateConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateInstantiateConfig")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateInstantiateConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut code_id__ = None;
                let mut new_instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewInstantiatePermission => {
                            if new_instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newInstantiatePermission",
                                ));
                            }
                            new_instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateInstantiateConfig {
                    sender: sender__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    new_instantiate_permission: new_instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateInstantiateConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateInstantiateConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmwasm.wasm.v1.MsgUpdateInstantiateConfigResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateInstantiateConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgUpdateInstantiateConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateInstantiateConfigResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateInstantiateConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateInstantiateConfigResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateInstantiateConfigResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
        }
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
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_upload_access.is_some() {
            len += 1;
        }
        if self.instantiate_default_permission != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Params", len)?;
        if let Some(v) = self.code_upload_access.as_ref() {
            struct_ser.serialize_field("codeUploadAccess", v)?;
        }
        if self.instantiate_default_permission != 0 {
            let v = AccessType::try_from(self.instantiate_default_permission).map_err(|_| {
                serde::ser::Error::custom(format!(
                    "Invalid variant {}",
                    self.instantiate_default_permission
                ))
            })?;
            struct_ser.serialize_field("instantiateDefaultPermission", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_upload_access",
            "codeUploadAccess",
            "instantiate_default_permission",
            "instantiateDefaultPermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeUploadAccess,
            InstantiateDefaultPermission,
        }
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
                            "codeUploadAccess" | "code_upload_access" => {
                                Ok(GeneratedField::CodeUploadAccess)
                            }
                            "instantiateDefaultPermission" | "instantiate_default_permission" => {
                                Ok(GeneratedField::InstantiateDefaultPermission)
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
            type Value = Params;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_upload_access__ = None;
                let mut instantiate_default_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeUploadAccess => {
                            if code_upload_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeUploadAccess"));
                            }
                            code_upload_access__ = map_.next_value()?;
                        }
                        GeneratedField::InstantiateDefaultPermission => {
                            if instantiate_default_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiateDefaultPermission",
                                ));
                            }
                            instantiate_default_permission__ =
                                Some(map_.next_value::<AccessType>()? as i32);
                        }
                    }
                }
                Ok(Params {
                    code_upload_access: code_upload_access__,
                    instantiate_default_permission: instantiate_default_permission__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PinCodesProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.PinCodesProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PinCodesProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeIds,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PinCodesProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.PinCodesProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinCodesProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(PinCodesProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.PinCodesProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllContractStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryAllContractStateRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllContractStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Pagination,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllContractStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryAllContractStateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllContractStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllContractStateRequest {
                    address: address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryAllContractStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllContractStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.models.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryAllContractStateResponse", len)?;
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllContractStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["models", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Models,
            Pagination,
        }
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
                            "models" => Ok(GeneratedField::Models),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllContractStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryAllContractStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllContractStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut models__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Models => {
                            if models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            models__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllContractStateResponse {
                    models: models__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryAllContractStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCodeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryCodeRequest", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_id", "codeId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryCodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodeRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryCodeRequest {
                    code_id: code_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryCodeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCodeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_info.is_some() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryCodeResponse", len)?;
        if let Some(v) = self.code_info.as_ref() {
            struct_ser.serialize_field("codeInfo", v)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_info", "codeInfo", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeInfo,
            Data,
        }
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
                            "codeInfo" | "code_info" => Ok(GeneratedField::CodeInfo),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryCodeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_info__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeInfo => {
                            if code_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeInfo"));
                            }
                            code_info__ = map_.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryCodeResponse {
                    code_info: code_info__,
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryCodeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCodesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryCodesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
        }
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryCodesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryCodesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryCodesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code_infos.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryCodesResponse", len)?;
        if !self.code_infos.is_empty() {
            struct_ser.serialize_field("codeInfos", &self.code_infos)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_infos", "codeInfos", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeInfos,
            Pagination,
        }
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
                            "codeInfos" | "code_infos" => Ok(GeneratedField::CodeInfos),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryCodesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_infos__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeInfos => {
                            if code_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeInfos"));
                            }
                            code_infos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryCodesResponse {
                    code_infos: code_infos__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryCodesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractHistoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractHistoryRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractHistoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Pagination,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractHistoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractHistoryRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractHistoryRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractHistoryRequest {
                    address: address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractHistoryRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractHistoryResponse {
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
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractHistoryResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractHistoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["entries", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            Pagination,
        }
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractHistoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractHistoryResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractHistoryResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractHistoryResponse {
                    entries: entries__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractHistoryResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractInfoRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractInfoRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractInfoRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryContractInfoRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractInfoRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.contract_info.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractInfoResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.contract_info.as_ref() {
            struct_ser.serialize_field("contractInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "contract_info", "contractInfo"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            ContractInfo,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "contractInfo" | "contract_info" => Ok(GeneratedField::ContractInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractInfoResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractInfoResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut contract_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractInfo => {
                            if contract_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractInfo"));
                            }
                            contract_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractInfoResponse {
                    address: address__.unwrap_or_default(),
                    contract_info: contract_info__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractInfoResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractsByCodeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractsByCodeRequest", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractsByCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_id", "codeId", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Pagination,
        }
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractsByCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractsByCodeRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractsByCodeRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractsByCodeRequest {
                    code_id: code_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractsByCodeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractsByCodeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contracts.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractsByCodeResponse", len)?;
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractsByCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contracts", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Contracts,
            Pagination,
        }
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
                            "contracts" => Ok(GeneratedField::Contracts),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractsByCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractsByCodeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractsByCodeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contracts__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractsByCodeResponse {
                    contracts: contracts__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractsByCodeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractsByCreatorRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.creator_address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractsByCreatorRequest", len)?;
        if !self.creator_address.is_empty() {
            struct_ser.serialize_field("creatorAddress", &self.creator_address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractsByCreatorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["creator_address", "creatorAddress", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatorAddress,
            Pagination,
        }
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
                            "creatorAddress" | "creator_address" => {
                                Ok(GeneratedField::CreatorAddress)
                            }
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractsByCreatorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractsByCreatorRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractsByCreatorRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut creator_address__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatorAddress => {
                            if creator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creatorAddress"));
                            }
                            creator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractsByCreatorRequest {
                    creator_address: creator_address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractsByCreatorRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractsByCreatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_addresses.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryContractsByCreatorResponse", len)?;
        if !self.contract_addresses.is_empty() {
            struct_ser.serialize_field("contractAddresses", &self.contract_addresses)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractsByCreatorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_addresses", "contractAddresses", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddresses,
            Pagination,
        }
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
                            "contractAddresses" | "contract_addresses" => {
                                Ok(GeneratedField::ContractAddresses)
                            }
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractsByCreatorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryContractsByCreatorResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractsByCreatorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_addresses__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddresses => {
                            if contract_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddresses"));
                            }
                            contract_addresses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractsByCreatorResponse {
                    contract_addresses: contract_addresses__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryContractsByCreatorResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
        }
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPinnedCodesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryPinnedCodesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPinnedCodesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
        }
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPinnedCodesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryPinnedCodesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPinnedCodesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPinnedCodesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryPinnedCodesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPinnedCodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code_ids.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryPinnedCodesResponse", len)?;
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPinnedCodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code_ids", "codeIds", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeIds,
            Pagination,
        }
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
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPinnedCodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryPinnedCodesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPinnedCodesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_ids__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPinnedCodesResponse {
                    code_ids: code_ids__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryPinnedCodesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRawContractStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.query_data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryRawContractStateRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.query_data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "queryData",
                pbjson::private::base64::encode(&self.query_data).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRawContractStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "query_data", "queryData"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            QueryData,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "queryData" | "query_data" => Ok(GeneratedField::QueryData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRawContractStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryRawContractStateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRawContractStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut query_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueryData => {
                            if query_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryData"));
                            }
                            query_data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRawContractStateRequest {
                    address: address__.unwrap_or_default(),
                    query_data: query_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryRawContractStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRawContractStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QueryRawContractStateResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRawContractStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRawContractStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QueryRawContractStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRawContractStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRawContractStateResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QueryRawContractStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QuerySmartContractStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.query_data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QuerySmartContractStateRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.query_data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "queryData",
                pbjson::private::base64::encode(&self.query_data).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySmartContractStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "query_data", "queryData"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            QueryData,
        }
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
                            "address" => Ok(GeneratedField::Address),
                            "queryData" | "query_data" => Ok(GeneratedField::QueryData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySmartContractStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QuerySmartContractStateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QuerySmartContractStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut query_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueryData => {
                            if query_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryData"));
                            }
                            query_data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QuerySmartContractStateRequest {
                    address: address__.unwrap_or_default(),
                    query_data: query_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QuerySmartContractStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QuerySmartContractStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.QuerySmartContractStateResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySmartContractStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySmartContractStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.QuerySmartContractStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QuerySmartContractStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QuerySmartContractStateResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.QuerySmartContractStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Sequence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id_key.is_empty() {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Sequence", len)?;
        if !self.id_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "idKey",
                pbjson::private::base64::encode(&self.id_key).as_str(),
            )?;
        }
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sequence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id_key", "idKey", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdKey,
            Value,
        }
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
                            "idKey" | "id_key" => Ok(GeneratedField::IdKey),
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
            type Value = Sequence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Sequence")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Sequence, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id_key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdKey => {
                            if id_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idKey"));
                            }
                            id_key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Sequence {
                    id_key: id_key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Sequence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StoreAndInstantiateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if self.unpin_code {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.builder.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmwasm.wasm.v1.StoreAndInstantiateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if self.unpin_code {
            struct_ser.serialize_field("unpinCode", &self.unpin_code)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.builder.is_empty() {
            struct_ser.serialize_field("builder", &self.builder)?;
        }
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoreAndInstantiateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "unpin_code",
            "unpinCode",
            "admin",
            "label",
            "msg",
            "funds",
            "source",
            "builder",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            WasmByteCode,
            InstantiatePermission,
            UnpinCode,
            Admin,
            Label,
            Msg,
            Funds,
            Source,
            Builder,
            CodeHash,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "unpinCode" | "unpin_code" => Ok(GeneratedField::UnpinCode),
                            "admin" => Ok(GeneratedField::Admin),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "source" => Ok(GeneratedField::Source),
                            "builder" => Ok(GeneratedField::Builder),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreAndInstantiateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.StoreAndInstantiateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StoreAndInstantiateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut unpin_code__ = None;
                let mut admin__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut source__ = None;
                let mut builder__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::UnpinCode => {
                            if unpin_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unpinCode"));
                            }
                            unpin_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Builder => {
                            if builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("builder"));
                            }
                            builder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(StoreAndInstantiateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    unpin_code: unpin_code__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    builder: builder__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.StoreAndInstantiateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StoreCodeAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grants.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.StoreCodeAuthorization", len)?;
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoreCodeAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["grants"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Grants,
        }
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
                            "grants" => Ok(GeneratedField::Grants),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreCodeAuthorization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.StoreCodeAuthorization")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StoreCodeAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut grants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StoreCodeAuthorization {
                    grants: grants__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.StoreCodeAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StoreCodeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if self.unpin_code {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.builder.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.StoreCodeProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if self.unpin_code {
            struct_ser.serialize_field("unpinCode", &self.unpin_code)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.builder.is_empty() {
            struct_ser.serialize_field("builder", &self.builder)?;
        }
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoreCodeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "unpin_code",
            "unpinCode",
            "source",
            "builder",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            WasmByteCode,
            InstantiatePermission,
            UnpinCode,
            Source,
            Builder,
            CodeHash,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "unpinCode" | "unpin_code" => Ok(GeneratedField::UnpinCode),
                            "source" => Ok(GeneratedField::Source),
                            "builder" => Ok(GeneratedField::Builder),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreCodeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.StoreCodeProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoreCodeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut unpin_code__ = None;
                let mut source__ = None;
                let mut builder__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::UnpinCode => {
                            if unpin_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unpinCode"));
                            }
                            unpin_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Builder => {
                            if builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("builder"));
                            }
                            builder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(StoreCodeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    unpin_code: unpin_code__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    builder: builder__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.StoreCodeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SudoContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.SudoContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SudoContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "contract", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
            Msg,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SudoContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.SudoContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SudoContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SudoContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.SudoContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UnpinCodesProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UnpinCodesProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnpinCodesProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeIds,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnpinCodesProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UnpinCodesProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinCodesProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(UnpinCodesProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UnpinCodesProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UpdateAdminProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.new_admin.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UpdateAdminProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.new_admin.is_empty() {
            struct_ser.serialize_field("newAdmin", &self.new_admin)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAdminProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "new_admin", "newAdmin", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NewAdmin,
            Contract,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "newAdmin" | "new_admin" => Ok(GeneratedField::NewAdmin),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAdminProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UpdateAdminProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdminProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut new_admin__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAdminProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    new_admin: new_admin__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UpdateAdminProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UpdateInstantiateConfigProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.access_config_updates.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UpdateInstantiateConfigProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.access_config_updates.is_empty() {
            struct_ser.serialize_field("accessConfigUpdates", &self.access_config_updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateInstantiateConfigProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "access_config_updates",
            "accessConfigUpdates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            AccessConfigUpdates,
        }
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "accessConfigUpdates" | "access_config_updates" => {
                                Ok(GeneratedField::AccessConfigUpdates)
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
            type Value = UpdateInstantiateConfigProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UpdateInstantiateConfigProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<UpdateInstantiateConfigProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut access_config_updates__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessConfigUpdates => {
                            if access_config_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "accessConfigUpdates",
                                ));
                            }
                            access_config_updates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateInstantiateConfigProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    access_config_updates: access_config_updates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UpdateInstantiateConfigProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
