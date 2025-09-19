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
        if !self.bech32_prefix.is_empty() {
            len += 1;
        }
        if !self.module_account_permissions.is_empty() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.enable_unordered_transactions {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.auth.module.v1.Module", len)?;
        if !self.bech32_prefix.is_empty() {
            struct_ser.serialize_field("bech32Prefix", &self.bech32_prefix)?;
        }
        if !self.module_account_permissions.is_empty() {
            struct_ser
                .serialize_field("moduleAccountPermissions", &self.module_account_permissions)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if self.enable_unordered_transactions {
            struct_ser.serialize_field(
                "enableUnorderedTransactions",
                &self.enable_unordered_transactions,
            )?;
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
            "bech32_prefix",
            "bech32Prefix",
            "module_account_permissions",
            "moduleAccountPermissions",
            "authority",
            "enable_unordered_transactions",
            "enableUnorderedTransactions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bech32Prefix,
            ModuleAccountPermissions,
            Authority,
            EnableUnorderedTransactions,
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
                            "bech32Prefix" | "bech32_prefix" => Ok(GeneratedField::Bech32Prefix),
                            "moduleAccountPermissions" | "module_account_permissions" => {
                                Ok(GeneratedField::ModuleAccountPermissions)
                            }
                            "authority" => Ok(GeneratedField::Authority),
                            "enableUnorderedTransactions" | "enable_unordered_transactions" => {
                                Ok(GeneratedField::EnableUnorderedTransactions)
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
                formatter.write_str("struct cosmos.auth.module.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Module, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bech32_prefix__ = None;
                let mut module_account_permissions__ = None;
                let mut authority__ = None;
                let mut enable_unordered_transactions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bech32Prefix => {
                            if bech32_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bech32Prefix"));
                            }
                            bech32_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModuleAccountPermissions => {
                            if module_account_permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "moduleAccountPermissions",
                                ));
                            }
                            module_account_permissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableUnorderedTransactions => {
                            if enable_unordered_transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "enableUnorderedTransactions",
                                ));
                            }
                            enable_unordered_transactions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Module {
                    bech32_prefix: bech32_prefix__.unwrap_or_default(),
                    module_account_permissions: module_account_permissions__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    enable_unordered_transactions: enable_unordered_transactions__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.auth.module.v1.Module", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ModuleAccountPermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.auth.module.v1.ModuleAccountPermission", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ModuleAccountPermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["account", "permissions"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            Permissions,
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
                            "account" => Ok(GeneratedField::Account),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModuleAccountPermission;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.auth.module.v1.ModuleAccountPermission")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<ModuleAccountPermission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModuleAccountPermission {
                    account: account__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.auth.module.v1.ModuleAccountPermission",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
