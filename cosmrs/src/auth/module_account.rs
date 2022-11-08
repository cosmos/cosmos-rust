use super::BaseAccount;
use crate::{proto, ErrorReport, Result};

/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleAccount {
    /// [`BaseAccount`] specification of this module account.
    pub base_account: Option<BaseAccount>,

    /// Name of the module.
    pub name: String,

    /// Permissions associated with this module account.
    pub permissions: Vec<String>,
}

impl TryFrom<proto::cosmos::auth::v1beta1::ModuleAccount> for ModuleAccount {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::auth::v1beta1::ModuleAccount) -> Result<ModuleAccount> {
        Ok(ModuleAccount {
            base_account: proto.base_account.map(TryFrom::try_from).transpose()?,
            name: proto.name,
            permissions: proto.permissions,
        })
    }
}

impl From<ModuleAccount> for proto::cosmos::auth::v1beta1::ModuleAccount {
    fn from(account: ModuleAccount) -> proto::cosmos::auth::v1beta1::ModuleAccount {
        proto::cosmos::auth::v1beta1::ModuleAccount {
            base_account: account.base_account.map(Into::into),
            name: account.name,
            permissions: account.permissions,
        }
    }
}
