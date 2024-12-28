use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_SCRIPT_ROOT_ENV_VAR_NAME: &str = "RB_ALIAS_SCRIPT_ROOT";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TomlSetting {
    pub global: GlobalSetting,
    pub alias: HashMap<String, AliasGroupSetting>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalSetting {
    pub script_root: Option<String>,
    pub script_root_env_var_name: Option<String>, // TODO 通过定义脚本路径环境变量，实现无副作用的全局环境变量设置
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AliasGroupSetting {
    pub mapping: HashMap<String, AliasSetting>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AliasSetting {
    pub cmd: String,
}

impl AliasGroupSetting {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
}
