use crate::{
    cmn::{files, unix_like},
    core::{
        alias::Alias,
        alias_setting::{self, AliasSetting},
        error::{AliasError, ErrorKind},
    },
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    process::Command,
};

const DEFAULT_HOME: &str = ".alias-rs";
const DEFAULT_SCRIPT_HOME_NAME: &str = "script";
const DEFAULT_SETTING_NAME: &str = "alias-setting.toml";

pub fn get_default_home() -> String {
    unix_like::get_home() + "/" + DEFAULT_HOME
}

pub fn get_default_script_home() -> String {
    get_default_home() + "/" + DEFAULT_SCRIPT_HOME_NAME
}

pub fn get_default_setting_path() -> String {
    get_default_home() + "/" + DEFAULT_SETTING_NAME
}

pub struct UnixLikeAlias {
    pub setting: AliasSetting,
}

impl UnixLikeAlias {
    pub fn new(
        setting_path: &Option<String>,
        runtime_variables: &HashMap<String, String>,
    ) -> Result<Self, AliasError> {
        let setting_path = setting_path
            .as_ref()
            .map_or(get_default_setting_path(), |f| f.to_owned());
        let mut setting = alias_setting::load(&setting_path, &runtime_variables)?;
        if setting.script.home.is_none() {
            setting.script.home = Some(get_default_script_home());
        }
        Ok(Self { setting })
    }

    fn build_alias_script_path(&self, alias: &String) -> String {
        format!(
            "{}/{}.sh",
            self.setting.script.home.as_ref().unwrap(),
            alias
        )
    }
}

impl Alias for UnixLikeAlias {
    fn init(&self) -> Result<(), AliasError> {
        // read profile
        let (profile_path, mut profile) = unix_like::get_shell_profile()?;
        let mut profile_content = String::new();
        if let Err(e) = profile.read_to_string(&mut profile_content) {
            return Err(AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("read shell profile fail :: {}", e),
            });
        }
        // set script home
        let source_script_home_cmd = format!(
            "# alias-rs :: start\nexport PATH=$PATH:{}\n# alias-rs :: end",
            &self.setting.script.home.as_ref().unwrap()
        );
        if profile_content.contains(&source_script_home_cmd) {
            return Ok(());
        }
        profile_content.push_str("\n\n");
        profile_content.push_str(&source_script_home_cmd);
        // overwrite profile
        File::create(profile_path.clone())
            .and_then(|mut f| f.write_all(profile_content.as_bytes()))
            .map_err(|e| AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("overwrite profile fail :: {}", e),
            })?;
        // source profile
        match Command::new("source").arg(profile_path).status() {
            Ok(exit_status) => match exit_status.success() {
                true => Ok(()),
                false => Err(AliasError {
                    kind: ErrorKind::Unkonw,
                    msg: format!(
                        "source shell profile fail :: exit_code={}",
                        exit_status.code().unwrap_or(-999)
                    ),
                }),
            },
            Err(e) => Err(AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("source shell profile fail :: {}", e),
            }),
        }
    }

    fn setting(&self) -> AliasSetting {
        self.setting.clone()
    }

    fn set(&self, alias: String, command: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        files::create_with_all_dir(&alias_script_path)
            .and_then(|mut f| f.write_all(command.as_bytes()))
            .map_err(|e| AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("create alias script fail :: {}", e),
            })?;
        Ok(())
    }

    fn remove(&self, alias: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        files::remove_if_present(&alias_script_path).map_err(|e| AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("remove alias script fail :: {}", e),
        })?;
        Ok(())
    }

    fn list(&self) -> Result<Option<Vec<String>>, AliasError> {
        files::list_dir(&self.setting.script.home.as_ref().unwrap()).map_err(|e| AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("list alias script fail :: {}", e),
        })
    }
}
