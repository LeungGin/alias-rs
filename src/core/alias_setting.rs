use super::error::{AliasError, AliasErrorCode};
use crate::cmn::files;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AliasSetting {
    pub script: Script,
    pub aliases: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Script {
    pub home: Option<String>,
    pub home_env_name: Option<String>,
}

pub struct AliasSettingLoader {
    pub setting: AliasSetting,
}

impl AliasSettingLoader {
    pub fn new(
        setting_path: &String,
        runtime_variables: &HashMap<String, String>,
    ) -> Result<Self, AliasError> {
        Ok(Self {
            setting: load_setting(setting_path, runtime_variables)?,
        })
    }
}

fn load_setting(
    setting_path: &String,
    runtime_variables: &HashMap<String, String>,
) -> Result<AliasSetting, AliasError> {
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
        Ok(setting) => setting,
        Err(e) => {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("deserialize alias setting fail :: {}", e),
            })
        }
    });
}
