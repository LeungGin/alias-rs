use crate::{
    cmn::{files, unix_like},
    core::{
        alias::Alias,
        alias_setting::{AliasSetting, AliasSettingLoader},
        error::{AliasError, AliasErrorCode},
    },
};
use std::{collections::HashMap, io::Read, process::Command};

const _DEFAULT_HOME: &str = "~/.alias-rs";
const DEFAULT_SCRIPT_HOME: &str = "~/.alias-rs/script";
const DEFAULT_SETTING_PATH: &str = "~/.alias-rs/alias-setting.toml";

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
            .map_or(DEFAULT_SETTING_PATH.to_owned(), |f| f.to_owned());
        let mut setting_loader = AliasSettingLoader::new(&setting_path, &runtime_variables)?;
        if setting_loader.setting.script.home.is_none() {
            setting_loader.setting.script.home = Some(DEFAULT_SCRIPT_HOME.to_owned());
        }
        Ok(Self {
            setting: setting_loader.setting,
        })
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
                err: AliasErrorCode::Unkonw,
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
        files::overwrite(&mut profile, &profile_path, &profile_content)?;
        // source profile
        match Command::new("source").arg(profile_path).status() {
            Ok(exit_status) => match exit_status.success() {
                true => Ok(()),
                false => Err(AliasError {
                    err: AliasErrorCode::Unkonw,
                    msg: format!(
                        "source shell profile fail :: exit_code={}",
                        exit_status.code().unwrap_or(-999)
                    ),
                }),
            },
            Err(e) => Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("source shell profile fail :: {}", e),
            }),
        }
    }

    fn setting(&self) -> AliasSetting {
        self.setting.clone()
    }

    fn set(&self, alias: String, command: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        let mut alias_script = files::create_if_absent(&alias_script_path)?;
        files::overwrite(&mut alias_script, &alias_script_path, &command)
    }

    fn remove(&self, alias: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        files::remove(&alias_script_path)
    }

    fn list(&self) -> Result<Option<Vec<String>>, AliasError> {
        files::list_dir(&self.setting.script.home.as_ref().unwrap())
    }
}
