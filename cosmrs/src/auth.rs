//! Authentication module: AuthN-related functionality.

mod base_account;
mod module_account;

pub use self::{base_account::BaseAccount, module_account::ModuleAccount};
