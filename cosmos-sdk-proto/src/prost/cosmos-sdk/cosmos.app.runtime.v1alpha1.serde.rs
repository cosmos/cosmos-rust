// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_name.is_empty() {
            len += 1;
        }
        if !self.begin_blockers.is_empty() {
            len += 1;
        }
        if !self.end_blockers.is_empty() {
            len += 1;
        }
        if !self.init_genesis.is_empty() {
            len += 1;
        }
        if !self.export_genesis.is_empty() {
            len += 1;
        }
        if !self.override_store_keys.is_empty() {
            len += 1;
        }
        if !self.order_migrations.is_empty() {
            len += 1;
        }
        if !self.precommiters.is_empty() {
            len += 1;
        }
        if !self.prepare_check_staters.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.app.runtime.v1alpha1.Module", len)?;
        if !self.app_name.is_empty() {
            struct_ser.serialize_field("appName", &self.app_name)?;
        }
        if !self.begin_blockers.is_empty() {
            struct_ser.serialize_field("beginBlockers", &self.begin_blockers)?;
        }
        if !self.end_blockers.is_empty() {
            struct_ser.serialize_field("endBlockers", &self.end_blockers)?;
        }
        if !self.init_genesis.is_empty() {
            struct_ser.serialize_field("initGenesis", &self.init_genesis)?;
        }
        if !self.export_genesis.is_empty() {
            struct_ser.serialize_field("exportGenesis", &self.export_genesis)?;
        }
        if !self.override_store_keys.is_empty() {
            struct_ser.serialize_field("overrideStoreKeys", &self.override_store_keys)?;
        }
        if !self.order_migrations.is_empty() {
            struct_ser.serialize_field("orderMigrations", &self.order_migrations)?;
        }
        if !self.precommiters.is_empty() {
            struct_ser.serialize_field("precommiters", &self.precommiters)?;
        }
        if !self.prepare_check_staters.is_empty() {
            struct_ser.serialize_field("prepareCheckStaters", &self.prepare_check_staters)?;
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
        const FIELDS: &[&str] = &[
            "app_name",
            "appName",
            "begin_blockers",
            "beginBlockers",
            "end_blockers",
            "endBlockers",
            "init_genesis",
            "initGenesis",
            "export_genesis",
            "exportGenesis",
            "override_store_keys",
            "overrideStoreKeys",
            "order_migrations",
            "orderMigrations",
            "precommiters",
            "prepare_check_staters",
            "prepareCheckStaters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppName,
            BeginBlockers,
            EndBlockers,
            InitGenesis,
            ExportGenesis,
            OverrideStoreKeys,
            OrderMigrations,
            Precommiters,
            PrepareCheckStaters,
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
                            "appName" | "app_name" => Ok(GeneratedField::AppName),
                            "beginBlockers" | "begin_blockers" => Ok(GeneratedField::BeginBlockers),
                            "endBlockers" | "end_blockers" => Ok(GeneratedField::EndBlockers),
                            "initGenesis" | "init_genesis" => Ok(GeneratedField::InitGenesis),
                            "exportGenesis" | "export_genesis" => Ok(GeneratedField::ExportGenesis),
                            "overrideStoreKeys" | "override_store_keys" => {
                                Ok(GeneratedField::OverrideStoreKeys)
                            }
                            "orderMigrations" | "order_migrations" => {
                                Ok(GeneratedField::OrderMigrations)
                            }
                            "precommiters" => Ok(GeneratedField::Precommiters),
                            "prepareCheckStaters" | "prepare_check_staters" => {
                                Ok(GeneratedField::PrepareCheckStaters)
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

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.app.runtime.v1alpha1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Module, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut app_name__ = None;
                let mut begin_blockers__ = None;
                let mut end_blockers__ = None;
                let mut init_genesis__ = None;
                let mut export_genesis__ = None;
                let mut override_store_keys__ = None;
                let mut order_migrations__ = None;
                let mut precommiters__ = None;
                let mut prepare_check_staters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appName"));
                            }
                            app_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BeginBlockers => {
                            if begin_blockers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beginBlockers"));
                            }
                            begin_blockers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndBlockers => {
                            if end_blockers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endBlockers"));
                            }
                            end_blockers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitGenesis => {
                            if init_genesis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initGenesis"));
                            }
                            init_genesis__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExportGenesis => {
                            if export_genesis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exportGenesis"));
                            }
                            export_genesis__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverrideStoreKeys => {
                            if override_store_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideStoreKeys"));
                            }
                            override_store_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderMigrations => {
                            if order_migrations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderMigrations"));
                            }
                            order_migrations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Precommiters => {
                            if precommiters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("precommiters"));
                            }
                            precommiters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrepareCheckStaters => {
                            if prepare_check_staters__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "prepareCheckStaters",
                                ));
                            }
                            prepare_check_staters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Module {
                    app_name: app_name__.unwrap_or_default(),
                    begin_blockers: begin_blockers__.unwrap_or_default(),
                    end_blockers: end_blockers__.unwrap_or_default(),
                    init_genesis: init_genesis__.unwrap_or_default(),
                    export_genesis: export_genesis__.unwrap_or_default(),
                    override_store_keys: override_store_keys__.unwrap_or_default(),
                    order_migrations: order_migrations__.unwrap_or_default(),
                    precommiters: precommiters__.unwrap_or_default(),
                    prepare_check_staters: prepare_check_staters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.app.runtime.v1alpha1.Module",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StoreKeyConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module_name.is_empty() {
            len += 1;
        }
        if !self.kv_store_key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.app.runtime.v1alpha1.StoreKeyConfig", len)?;
        if !self.module_name.is_empty() {
            struct_ser.serialize_field("moduleName", &self.module_name)?;
        }
        if !self.kv_store_key.is_empty() {
            struct_ser.serialize_field("kvStoreKey", &self.kv_store_key)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StoreKeyConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["module_name", "moduleName", "kv_store_key", "kvStoreKey"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModuleName,
            KvStoreKey,
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
                            "moduleName" | "module_name" => Ok(GeneratedField::ModuleName),
                            "kvStoreKey" | "kv_store_key" => Ok(GeneratedField::KvStoreKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoreKeyConfig;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.app.runtime.v1alpha1.StoreKeyConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<StoreKeyConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut module_name__ = None;
                let mut kv_store_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ModuleName => {
                            if module_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleName"));
                            }
                            module_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KvStoreKey => {
                            if kv_store_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kvStoreKey"));
                            }
                            kv_store_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StoreKeyConfig {
                    module_name: module_name__.unwrap_or_default(),
                    kv_store_key: kv_store_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.app.runtime.v1alpha1.StoreKeyConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
