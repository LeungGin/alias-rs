use crate::{
    cmn::files,
    core::{
        alias::Alias,
        error::{AliasError, AliasErrorCode},
    },
};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use super::alias_setting::TomlSetting;

pub trait AliasManage {
    fn rebuild(&mut self) -> Result<(), AliasError>;
    fn export(&self, export_path: &String) -> Result<(), AliasError>;
    fn import(&mut self, import_path: &String) -> Result<(), AliasError>;
}

pub struct AliasManager {
    alias_base: Box<dyn Alias>,
}

impl AliasManager {
    pub fn new(alias_base: Box<dyn Alias>) -> Self {
        Self { alias_base }
    }
}

impl AliasManage for AliasManager {
    fn rebuild(&mut self) -> Result<(), AliasError> {
        let groups = self.alias_base.get_all()?;
        self.alias_base.clear()?;
        for (group_name, group) in groups {
            for (alias, alias_setting) in group.mapping {
                self.alias_base
                    .set(group_name.clone(), alias, alias_setting)?;
            }
        }
        self.alias_base.commit()?;
        Ok(())
    }

    fn export(&self, export_path: &String) -> Result<(), AliasError> {
        // alias setting serialize
        let aliases = self.alias_base.get_all()?;
        let content = match toml::to_string(&aliases) {
            Ok(str) => str,
            Err(e) => {
                return Err(AliasError {
                    err: AliasErrorCode::Unkonw,
                    msg: format!("serialize alias setting error :: {}", e),
                })
            }
        };
        // write export file
        if !fs::exists(export_path).unwrap_or(false) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("export path is exists :: {}", export_path),
            });
        }
        let mut export_file = files::create_if_absent(export_path)?;
        files::overwrite(&mut export_file, export_path, &content)?;
        Ok(())
    }

    fn import(&mut self, import_path: &String) -> Result<(), AliasError> {
        // deserialize import
        let import_path = Path::new(import_path);
        if !import_path.exists() {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("import file is not exists :: {}", import_path.display()),
            });
        }
        let mut import_file = match File::open(import_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(AliasError {
                    err: AliasErrorCode::Unkonw,
                    msg: format!(
                        "import file open error :: {} :: {}",
                        import_path.display(),
                        e
                    ),
                })
            }
        };
        let mut import_content = String::new();
        if let Err(e) = import_file.read_to_string(&mut import_content) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!(
                    "deserialize import file error :: {} :: {}",
                    import_path.display(),
                    e
                ),
            });
        }
        // parse import aliases
        let toml_setting: TomlSetting = match toml::from_str(&import_content) {
            Ok(map) => map,
            Err(e) => {
                return Err(AliasError {
                    err: AliasErrorCode::Unkonw,
                    msg: format!(
                        "deserialize toml from import file error :: {} :: {}",
                        import_path.display(),
                        e
                    ),
                })
            }
        };
        // set
        self.alias_base.overwrite_setting(toml_setting)?;
        self.rebuild()?;
        Ok(())
    }
}
