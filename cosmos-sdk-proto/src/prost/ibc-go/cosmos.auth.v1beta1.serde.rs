// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for BaseAccount {
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
        if self.pub_key.is_some() {
            len += 1;
        }
        if self.account_number != 0 {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.auth.v1beta1.BaseAccount", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pub_key.as_ref() {
            struct_ser.serialize_field("pubKey", v)?;
        }
        if self.account_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "accountNumber",
                ToString::to_string(&self.account_number).as_str(),
            )?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BaseAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pub_key",
            "pubKey",
            "account_number",
            "accountNumber",
            "sequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            PubKey,
            AccountNumber,
            Sequence,
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
                            "address" => Ok(GeneratedField::Address),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "accountNumber" | "account_number" => Ok(GeneratedField::AccountNumber),
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
            type Value = BaseAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.auth.v1beta1.BaseAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BaseAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut pub_key__ = None;
                let mut account_number__ = None;
                let mut sequence__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = map_.next_value()?;
                        }
                        GeneratedField::AccountNumber => {
                            if account_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountNumber"));
                            }
                            account_number__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
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
                Ok(BaseAccount {
                    address: address__.unwrap_or_default(),
                    pub_key: pub_key__,
                    account_number: account_number__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.auth.v1beta1.BaseAccount", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ModuleAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_account.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.auth.v1beta1.ModuleAccount", len)?;
        if let Some(v) = self.base_account.as_ref() {
            struct_ser.serialize_field("baseAccount", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ModuleAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["base_account", "baseAccount", "name", "permissions"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseAccount,
            Name,
            Permissions,
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
                            "baseAccount" | "base_account" => Ok(GeneratedField::BaseAccount),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = ModuleAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.auth.v1beta1.ModuleAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModuleAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_account__ = None;
                let mut name__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseAccount => {
                            if base_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAccount"));
                            }
                            base_account__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModuleAccount {
                    base_account: base_account__,
                    name: name__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.auth.v1beta1.ModuleAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ModuleCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module_name.is_empty() {
            len += 1;
        }
        if !self.derivation_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.auth.v1beta1.ModuleCredential", len)?;
        if !self.module_name.is_empty() {
            struct_ser.serialize_field("moduleName", &self.module_name)?;
        }
        if !self.derivation_keys.is_empty() {
            struct_ser.serialize_field(
                "derivationKeys",
                &self
                    .derivation_keys
                    .iter()
                    .map(pbjson::private::base64::encode)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ModuleCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module_name",
            "moduleName",
            "derivation_keys",
            "derivationKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModuleName,
            DerivationKeys,
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
                            "moduleName" | "module_name" => Ok(GeneratedField::ModuleName),
                            "derivationKeys" | "derivation_keys" => {
                                Ok(GeneratedField::DerivationKeys)
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
            type Value = ModuleCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.auth.v1beta1.ModuleCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModuleCredential, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut module_name__ = None;
                let mut derivation_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ModuleName => {
                            if module_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleName"));
                            }
                            module_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DerivationKeys => {
                            if derivation_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("derivationKeys"));
                            }
                            derivation_keys__ = Some(
                                map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(ModuleCredential {
                    module_name: module_name__.unwrap_or_default(),
                    derivation_keys: derivation_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.auth.v1beta1.ModuleCredential",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_memo_characters != 0 {
            len += 1;
        }
        if self.tx_sig_limit != 0 {
            len += 1;
        }
        if self.tx_size_cost_per_byte != 0 {
            len += 1;
        }
        if self.sig_verify_cost_ed25519 != 0 {
            len += 1;
        }
        if self.sig_verify_cost_secp256k1 != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.auth.v1beta1.Params", len)?;
        if self.max_memo_characters != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maxMemoCharacters",
                ToString::to_string(&self.max_memo_characters).as_str(),
            )?;
        }
        if self.tx_sig_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "txSigLimit",
                ToString::to_string(&self.tx_sig_limit).as_str(),
            )?;
        }
        if self.tx_size_cost_per_byte != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "txSizeCostPerByte",
                ToString::to_string(&self.tx_size_cost_per_byte).as_str(),
            )?;
        }
        if self.sig_verify_cost_ed25519 != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "sigVerifyCostEd25519",
                ToString::to_string(&self.sig_verify_cost_ed25519).as_str(),
            )?;
        }
        if self.sig_verify_cost_secp256k1 != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "sigVerifyCostSecp256k1",
                ToString::to_string(&self.sig_verify_cost_secp256k1).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_memo_characters",
            "maxMemoCharacters",
            "tx_sig_limit",
            "txSigLimit",
            "tx_size_cost_per_byte",
            "txSizeCostPerByte",
            "sig_verify_cost_ed25519",
            "sigVerifyCostEd25519",
            "sig_verify_cost_secp256k1",
            "sigVerifyCostSecp256k1",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxMemoCharacters,
            TxSigLimit,
            TxSizeCostPerByte,
            SigVerifyCostEd25519,
            SigVerifyCostSecp256k1,
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
                            "maxMemoCharacters" | "max_memo_characters" => {
                                Ok(GeneratedField::MaxMemoCharacters)
                            }
                            "txSigLimit" | "tx_sig_limit" => Ok(GeneratedField::TxSigLimit),
                            "txSizeCostPerByte" | "tx_size_cost_per_byte" => {
                                Ok(GeneratedField::TxSizeCostPerByte)
                            }
                            "sigVerifyCostEd25519" | "sig_verify_cost_ed25519" => {
                                Ok(GeneratedField::SigVerifyCostEd25519)
                            }
                            "sigVerifyCostSecp256k1" | "sig_verify_cost_secp256k1" => {
                                Ok(GeneratedField::SigVerifyCostSecp256k1)
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
                formatter.write_str("struct cosmos.auth.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut max_memo_characters__ = None;
                let mut tx_sig_limit__ = None;
                let mut tx_size_cost_per_byte__ = None;
                let mut sig_verify_cost_ed25519__ = None;
                let mut sig_verify_cost_secp256k1__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxMemoCharacters => {
                            if max_memo_characters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMemoCharacters"));
                            }
                            max_memo_characters__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxSigLimit => {
                            if tx_sig_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txSigLimit"));
                            }
                            tx_sig_limit__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxSizeCostPerByte => {
                            if tx_size_cost_per_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txSizeCostPerByte"));
                            }
                            tx_size_cost_per_byte__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SigVerifyCostEd25519 => {
                            if sig_verify_cost_ed25519__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sigVerifyCostEd25519",
                                ));
                            }
                            sig_verify_cost_ed25519__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SigVerifyCostSecp256k1 => {
                            if sig_verify_cost_secp256k1__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sigVerifyCostSecp256k1",
                                ));
                            }
                            sig_verify_cost_secp256k1__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Params {
                    max_memo_characters: max_memo_characters__.unwrap_or_default(),
                    tx_sig_limit: tx_sig_limit__.unwrap_or_default(),
                    tx_size_cost_per_byte: tx_size_cost_per_byte__.unwrap_or_default(),
                    sig_verify_cost_ed25519: sig_verify_cost_ed25519__.unwrap_or_default(),
                    sig_verify_cost_secp256k1: sig_verify_cost_secp256k1__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmos.auth.v1beta1.Params", FIELDS, GeneratedVisitor)
    }
}
