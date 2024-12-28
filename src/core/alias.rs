use regex::Regex;

use super::{
    alias_setting::{AliasGroupSetting, AliasSetting, TomlSetting},
    error::{AliasError, AliasErrorCode},
};
use crate::cmn::files;
use std::{collections::HashMap, io::Read};

pub trait Alias {
    fn get(&self, group: &String, alias: &String) -> Result<Option<AliasSetting>, AliasError>;
    fn get_group(&self, group: &String) -> Result<Option<AliasGroupSetting>, AliasError>;
    fn get_all(&self) -> Result<HashMap<String, AliasGroupSetting>, AliasError>;
    fn set(
        &mut self,
        group: String,
        alias: String,
        setting: AliasSetting,
    ) -> Result<(), AliasError>;
    fn remove(&mut self, group: &String, alias: &String) -> Result<(), AliasError>;
    fn remove_group(&mut self, group: &String) -> Result<(), AliasError>;
    fn clear(&mut self) -> Result<(), AliasError>;
    fn commit(&mut self) -> Result<(), AliasError>;
    fn overwrite_setting(&mut self, setting: TomlSetting) -> Result<(), AliasError>;
}

// TODO 在初始化时便可确认各种路径，保存到结构体属性中，避免每次查询获取
// TODO 保持new的简单，不应该返回Result，应该通过【预热】函数装载相关可预知配置和参数
pub struct AliasBase {
    setting_path: String,
    pub setting: TomlSetting,
    pub set_buffer: HashMap<String, AliasSetCache>, // key=alias
}

pub struct AliasSetCache {
    pub set_type: SetType,
    pub group: String,
    pub setting: Option<AliasSetting>,
}

#[derive(PartialEq)]
pub enum SetType {
    Set,
    Remove,
}

impl AliasBase {
    pub fn new(
        setting_path: &String,
        runtime_variables: &HashMap<String, String>,
    ) -> Result<Self, AliasError> {
        let setting = load_setting(setting_path, runtime_variables)?;
        Ok(Self {
            setting_path: setting_path.to_owned(),
            setting,
            set_buffer: HashMap::new(),
        })
    }

    pub fn get_alias_group(&self, group: &String) -> Result<Option<AliasGroupSetting>, AliasError> {
        match self.setting.alias.get(group) {
            Some(obj) => Ok(Some(obj.clone())),
            None => Ok(None),
        }
    }

    pub fn overwrite_setting(&mut self, setting: &TomlSetting) -> Result<(), AliasError> {
        let mut setting_file = files::create_if_absent(&self.setting_path)?;
        let content = match toml::to_string(setting) {
            Ok(str) => str,
            Err(e) => {
                return Err(AliasError {
                    err: AliasErrorCode::Unkonw,
                    msg: format!("serialize alias setting fail :: {}", e),
                })
            }
        };
        files::overwrite(&mut setting_file, &self.setting_path, &content)?;
        Ok(())
    }

    pub fn clear_cache(&mut self) {
        self.set_buffer.clear();
    }
}

impl Alias for AliasBase {
    fn get(&self, group: &String, alias: &String) -> Result<Option<AliasSetting>, AliasError> {
        if let Some(alias_group) = self.get_alias_group(group)? {
            return Ok(alias_group.mapping.get(alias).cloned());
        }
        Ok(None)
    }

    fn get_group(&self, group: &String) -> Result<Option<AliasGroupSetting>, AliasError> {
        Ok(self.get_alias_group(group)?)
    }

    fn get_all(&self) -> Result<HashMap<String, AliasGroupSetting>, AliasError> {
        Ok(self.setting.alias.clone())
    }

    fn set(
        &mut self,
        group: String,
        alias: String,
        setting: AliasSetting,
    ) -> Result<(), AliasError> {
        self.set_buffer.insert(
            alias.clone(),
            AliasSetCache {
                set_type: SetType::Set,
                group,
                setting: Some(setting),
            },
        );
        Ok(())
    }

    fn remove(&mut self, group: &String, alias: &String) -> Result<(), AliasError> {
        self.set_buffer.insert(
            alias.clone(),
            AliasSetCache {
                set_type: SetType::Remove,
                group: group.clone(),
                setting: None,
            },
        );
        Ok(())
    }

    fn remove_group(&mut self, group: &String) -> Result<(), AliasError> {
        if let Some(group_setting) = self.get_group(group)? {
            for alias in group_setting.mapping.keys() {
                self.set_buffer.insert(
                    alias.clone(),
                    AliasSetCache {
                        set_type: SetType::Remove,
                        group: group.clone(),
                        setting: None,
                    },
                );
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), AliasError> {
        self.setting.alias.clear();
        self.overwrite_setting(&self.setting.clone())?;
        // TODO 逻辑存在bug，无法删除已生成的脚本等
        Ok(())
    }

    fn commit(&mut self) -> Result<(), AliasError> {
        for (alias, set_cache) in &self.set_buffer {
            match set_cache.set_type {
                SetType::Set => {
                    if !self.setting.alias.contains_key(&set_cache.group) {
                        self.setting
                            .alias
                            .insert(set_cache.group.clone(), AliasGroupSetting::new());
                    }
                    self.setting
                        .alias
                        .get_mut(&set_cache.group)
                        .unwrap()
                        .mapping
                        .insert(alias.clone(), set_cache.setting.clone().unwrap());
                }
                SetType::Remove => {
                    if let Some(group) = self.setting.alias.get_mut(&set_cache.group) {
                        group.mapping.remove(alias);
                    }
                }
            }
        }
        self.overwrite_setting(&self.setting.clone())?;
        Ok(())
    }

    fn overwrite_setting(&mut self, setting: TomlSetting) -> Result<(), AliasError> {
        self.overwrite_setting(&setting)
    }
}

pub fn load_setting(
    setting_path: &String,
    runtime_variables: &HashMap<String, String>,
) -> Result<TomlSetting, AliasError> {
    // get setting content string
    let mut setting_file = files::create_if_absent(setting_path)?;
    let mut content = String::new();
    if let Err(e) = setting_file.read_to_string(&mut content) {
        return Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("read alias setting file fail :: {}", e),
        });
    }
    // replace placeholder with runtime variables
    for (key, val) in runtime_variables {
        let regex = Regex::new(&format!("\\{{\\{{{}\\}}\\}}", key)).unwrap();
        content = regex.replace_all(&content, val).to_string();
    }
    // deserialize setting
    return Ok(match toml::from_str(&content) {
        Ok(toml_setting) => toml_setting,
        Err(e) => {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("deserialize alias setting fail :: {}", e),
            })
        }
    });
}
