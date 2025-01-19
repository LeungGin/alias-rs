use super::error::{AliasError, ErrorKind};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AliasSetting {
    pub script: Script,
    pub aliases: HashMap<String, String>,
}

impl Default for AliasSetting {
    fn default() -> Self {
        Self {
            script: Script::default(),
            aliases: HashMap::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Script {
    pub home: Option<String>,
    pub home_env_name: Option<String>,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            home: None,
            home_env_name: None,
        }
    }
}

pub fn load(
    setting_path: &String,
    runtime_variables: &HashMap<String, String>,
) -> Result<AliasSetting, AliasError> {
    // get setting content
    let path = Path::new(&setting_path);
    let mut content = if path.exists() {
        fs::read_to_string(path).map_err(|e| AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("read setting fail :: {}", e),
        })?
    } else {
        // not exist
        let content = toml::to_string_pretty(&AliasSetting::default()).map_err(|e| AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("serialize default setting fail :: {}", e),
        })?;
        File::create_new(setting_path)
            .and_then(|mut f| f.write_all(content.as_bytes()))
            .map_err(|e| AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("create default setting fail :: {}", e),
            })?;
        content
    };
    // replace placeholder with runtime variables
    for (name, val) in runtime_variables {
        let regex = Regex::new(&format!("\\{{\\{{{}\\}}\\}}", name)).unwrap();
        content = regex.replace_all(&content, val).to_string();
    }
    // deserialize setting
    Ok(toml::from_str(&content).map_err(|e| AliasError {
        kind: ErrorKind::Unkonw,
        msg: format!("deserialize setting fail :: {}", e),
    })?)
}
