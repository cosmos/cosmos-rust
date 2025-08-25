// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for MsgLoadTest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.caller.is_empty() {
            len += 1;
        }
        if !self.ops.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.benchmark.v1.MsgLoadTest", len)?;
        if !self.caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "caller",
                pbjson::private::base64::encode(&self.caller).as_str(),
            )?;
        }
        if !self.ops.is_empty() {
            struct_ser.serialize_field("ops", &self.ops)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgLoadTest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["caller", "ops"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Caller,
            Ops,
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
                            "caller" => Ok(GeneratedField::Caller),
                            "ops" => Ok(GeneratedField::Ops),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLoadTest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.benchmark.v1.MsgLoadTest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgLoadTest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut caller__ = None;
                let mut ops__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Caller => {
                            if caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caller"));
                            }
                            caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Ops => {
                            if ops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ops"));
                            }
                            ops__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgLoadTest {
                    caller: caller__.unwrap_or_default(),
                    ops: ops__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.benchmark.v1.MsgLoadTest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgLoadTestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_time != 0 {
            len += 1;
        }
        if self.total_errors != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.benchmark.v1.MsgLoadTestResponse", len)?;
        if self.total_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "totalTime",
                alloc::string::ToString::to_string(&self.total_time).as_str(),
            )?;
        }
        if self.total_errors != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "totalErrors",
                alloc::string::ToString::to_string(&self.total_errors).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgLoadTestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["total_time", "totalTime", "total_errors", "totalErrors"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalTime,
            TotalErrors,
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
                            "totalTime" | "total_time" => Ok(GeneratedField::TotalTime),
                            "totalErrors" | "total_errors" => Ok(GeneratedField::TotalErrors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLoadTestResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.benchmark.v1.MsgLoadTestResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgLoadTestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_time__ = None;
                let mut total_errors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalTime => {
                            if total_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalTime"));
                            }
                            total_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalErrors => {
                            if total_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalErrors"));
                            }
                            total_errors__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgLoadTestResponse {
                    total_time: total_time__.unwrap_or_default(),
                    total_errors: total_errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.benchmark.v1.MsgLoadTestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Op {
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
        if !self.actor.is_empty() {
            len += 1;
        }
        if self.key_length != 0 {
            len += 1;
        }
        if self.value_length != 0 {
            len += 1;
        }
        if self.iterations != 0 {
            len += 1;
        }
        if self.delete {
            len += 1;
        }
        if self.exists {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.benchmark.v1.Op", len)?;
        if self.seed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "seed",
                alloc::string::ToString::to_string(&self.seed).as_str(),
            )?;
        }
        if !self.actor.is_empty() {
            struct_ser.serialize_field("actor", &self.actor)?;
        }
        if self.key_length != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "keyLength",
                alloc::string::ToString::to_string(&self.key_length).as_str(),
            )?;
        }
        if self.value_length != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "valueLength",
                alloc::string::ToString::to_string(&self.value_length).as_str(),
            )?;
        }
        if self.iterations != 0 {
            struct_ser.serialize_field("iterations", &self.iterations)?;
        }
        if self.delete {
            struct_ser.serialize_field("delete", &self.delete)?;
        }
        if self.exists {
            struct_ser.serialize_field("exists", &self.exists)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Op {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seed",
            "actor",
            "key_length",
            "keyLength",
            "value_length",
            "valueLength",
            "iterations",
            "delete",
            "exists",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seed,
            Actor,
            KeyLength,
            ValueLength,
            Iterations,
            Delete,
            Exists,
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
                            "actor" => Ok(GeneratedField::Actor),
                            "keyLength" | "key_length" => Ok(GeneratedField::KeyLength),
                            "valueLength" | "value_length" => Ok(GeneratedField::ValueLength),
                            "iterations" => Ok(GeneratedField::Iterations),
                            "delete" => Ok(GeneratedField::Delete),
                            "exists" => Ok(GeneratedField::Exists),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Op;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.benchmark.v1.Op")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Op, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut seed__ = None;
                let mut actor__ = None;
                let mut key_length__ = None;
                let mut value_length__ = None;
                let mut iterations__ = None;
                let mut delete__ = None;
                let mut exists__ = None;
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
                        GeneratedField::Actor => {
                            if actor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            actor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyLength => {
                            if key_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyLength"));
                            }
                            key_length__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValueLength => {
                            if value_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueLength"));
                            }
                            value_length__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Iterations => {
                            if iterations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterations"));
                            }
                            iterations__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Delete => {
                            if delete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            delete__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exists => {
                            if exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            exists__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Op {
                    seed: seed__.unwrap_or_default(),
                    actor: actor__.unwrap_or_default(),
                    key_length: key_length__.unwrap_or_default(),
                    value_length: value_length__.unwrap_or_default(),
                    iterations: iterations__.unwrap_or_default(),
                    delete: delete__.unwrap_or_default(),
                    exists: exists__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.benchmark.v1.Op", FIELDS, GeneratedVisitor)
    }
}
