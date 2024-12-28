use crate::{
    cmn::{files, unix_like},
    core::{
        alias::{Alias, AliasBase, SetType},
        alias_setting::{AliasGroupSetting, AliasSetting, TomlSetting},
        error::{AliasError, AliasErrorCode},
    },
};

use std::{collections::HashMap, io::Read, process::Command};

const _DEFAULT_ROOT: &str = "~/.rb-alias";
const DEFAULT_SETTING_FILE_PATH: &str = "~/.rb-alias/alias-setting.toml";
const DEFAULT_SCRIPT_ROOT: &str = "~/.rb-alias/script";
const DEFAULT_DEFINE_SCRIPT_FILE_PATH: &str = "/define/alias-define.sh";

pub struct UnixLikeAlias {
    pub alias_base: AliasBase,
}

impl UnixLikeAlias {
    pub fn new(
        setting_path: Option<String>,
        runtime_variables: &HashMap<String, String>,
    ) -> Result<Self, AliasError> {
        let setting_path = setting_path.unwrap_or(DEFAULT_SETTING_FILE_PATH.to_owned());
        Ok(Self {
            alias_base: AliasBase::new(&setting_path, runtime_variables)?,
        })
    }

    fn get_script_root_path(&self) -> String {
        let global_setting = &self.alias_base.setting.global;
        if global_setting.script_root.is_some() {
            global_setting.script_root.as_ref().unwrap().to_string()
        } else {
            DEFAULT_SCRIPT_ROOT.to_owned()
        }
    }

    fn commit_alias_script(&self) -> Result<(), AliasError> {
        let script_root_path = self.get_script_root_path();
        for (alias, set_cache) in &self.alias_base.set_buffer {
            let script_path = format!("{}/{}.sh", script_root_path, alias);
            files::remove(&script_path)?;
            if set_cache.set_type == SetType::Set {
                self.create_alias_script(&script_path, &set_cache.setting.as_ref().unwrap())?;
            }
        }
        Ok(())
    }

    fn create_alias_script(
        &self,
        path: &String,
        alias_setting: &AliasSetting,
    ) -> Result<(), AliasError> {
        let mut script = files::create_if_absent(&path)?;
        files::overwrite(&mut script, path, &alias_setting.cmd)?;
        Ok(())
    }

    fn commit_define_script(&self) -> Result<(), AliasError> {
        let script_root_path = format!(
            "{}/{}",
            self.get_script_root_path(),
            DEFAULT_DEFINE_SCRIPT_FILE_PATH
        );
        self.overwrite_define_script(&script_root_path, &self.alias_base.get_all()?)
    }

    fn overwrite_define_script(
        &self,
        path: &String,
        alias_group_mapping: &HashMap<String, AliasGroupSetting>,
    ) -> Result<(), AliasError> {
        let mut content = String::new();
        for (_group, group_setting) in alias_group_mapping {
            for (alias, alias_setting) in &group_setting.mapping {
                content.push_str(&format!("alias {}=\"{}\"\n", alias, alias_setting.cmd));
            }
        }
        let mut define_script = files::create_if_absent(path)?;
        files::overwrite(&mut define_script, path, &content)?;
        Ok(())
    }

    fn write_define_script_path_into_shell_profile(&self) -> Result<(), AliasError> {
        // read profile
        let (profile_path, mut profile) = unix_like::get_shell_profile()?;
        let mut profile_content = String::new();
        if let Err(e) = profile.read_to_string(&mut profile_content) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("read shell profile fail :: {}", e),
            });
        }
        // set define
        let source_define_script_cmd = format!(
            "# rb-alias auto set :: start\nsource {}\n# rb-alias auto set :: end",
            self.get_script_root_path() + DEFAULT_DEFINE_SCRIPT_FILE_PATH
        );
        if profile_content.contains(&source_define_script_cmd) {
            return Ok(());
        }
        profile_content.push_str("\n\n");
        profile_content.push_str(&source_define_script_cmd);
        profile_content.push_str("\n\n");
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
}

impl Alias for UnixLikeAlias {
    fn get(&self, group: &String, alias: &String) -> Result<Option<AliasSetting>, AliasError> {
        self.alias_base.get(group, alias)
    }

    fn get_group(&self, group: &String) -> Result<Option<AliasGroupSetting>, AliasError> {
        self.alias_base.get_group(group)
    }

    fn get_all(&self) -> Result<HashMap<String, AliasGroupSetting>, AliasError> {
        self.alias_base.get_all()
    }

    fn set(
        &mut self,
        group: String,
        alias: String,
        setting: AliasSetting,
    ) -> Result<(), AliasError> {
        self.alias_base.set(group, alias, setting)
    }

    fn remove(&mut self, group: &String, alias: &String) -> Result<(), AliasError> {
        self.alias_base.remove(group, alias)
    }

    fn remove_group(&mut self, group: &String) -> Result<(), AliasError> {
        self.alias_base.remove_group(group)
    }

    fn clear(&mut self) -> Result<(), AliasError> {
        self.alias_base.clear()
    }

    fn commit(&mut self) -> Result<(), AliasError> {
        self.alias_base.commit()?; // commit setting
        self.commit_alias_script()?;
        self.commit_define_script()?;
        self.write_define_script_path_into_shell_profile()?;
        self.alias_base.clear_cache();
        Ok(())
    }

    fn overwrite_setting(&mut self, setting: TomlSetting) -> Result<(), AliasError> {
        self.alias_base.overwrite_setting(&setting)
    }
}
