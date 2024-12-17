use super::unix_like_base::UnixLikeAlias;
use crate::core::{
    alias::Alias,
    alias_setting::{AliasGroupSetting, AliasSetting, TomlSetting},
    error::AliasError,
};
use std::collections::HashMap;

pub struct MacosAlias {
    unix_like_base: UnixLikeAlias,
}

impl MacosAlias {
    pub fn new() -> Result<Self, AliasError> {
        Ok(Self {
            unix_like_base: UnixLikeAlias::new()?,
        })
    }
}

impl Alias for MacosAlias {
    fn get(&self, group: &String, alias: &String) -> Result<Option<AliasSetting>, AliasError> {
        self.unix_like_base.get(group, alias)
    }

    fn get_group(&self, group: &String) -> Result<Option<AliasGroupSetting>, AliasError> {
        self.unix_like_base.get_group(group)
    }

    fn get_all(&self) -> Result<HashMap<String, AliasGroupSetting>, AliasError> {
        self.unix_like_base.get_all()
    }

    fn set(
        &mut self,
        group: String,
        alias: String,
        setting: AliasSetting,
    ) -> Result<(), AliasError> {
        self.unix_like_base.set(group, alias, setting)
    }

    fn remove(&mut self, group: &String, alias: &String) -> Result<(), AliasError> {
        self.unix_like_base.remove(group, alias)
    }

    fn remove_group(&mut self, group: &String) -> Result<(), AliasError> {
        self.unix_like_base.remove_group(group)
    }

    fn clear(&mut self) -> Result<(), AliasError> {
        self.unix_like_base.clear()
    }

    fn commit(&mut self) -> Result<(), AliasError> {
        self.unix_like_base.commit()
    }

    fn overwrite_setting(&mut self, setting: TomlSetting) -> Result<(), AliasError> {
        self.unix_like_base.overwrite_setting(setting)
    }
}
