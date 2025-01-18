use super::unix_like_base::UnixLikeAlias;
use crate::core::{alias::Alias, alias_setting::AliasSetting, error::AliasError};
use std::collections::HashMap;

pub struct LinuxAlias {
    unix_like_base: UnixLikeAlias,
}

impl LinuxAlias {
    pub fn new(
        setting_path: &Option<String>,
        runtime_variables: &HashMap<String, String>,
    ) -> Result<Self, AliasError> {
        Ok(Self {
            unix_like_base: UnixLikeAlias::new(setting_path, runtime_variables)?,
        })
    }
}

impl Alias for LinuxAlias {
    fn init(&self) -> Result<(), AliasError> {
        self.unix_like_base.init()
    }

    fn setting(&self) -> AliasSetting {
        self.unix_like_base.setting()
    }

    fn set(&self, alias: String, command: String) -> Result<(), AliasError> {
        self.unix_like_base.set(alias, command)
    }

    fn remove(&self, alias: String) -> Result<(), AliasError> {
        self.unix_like_base.remove(alias)
    }

    fn list(&self) -> Result<Option<Vec<String>>, AliasError> {
        self.unix_like_base.list()
    }
}
