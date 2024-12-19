use crate::{
    cmn::{files, windows_like},
    core::{
        alias::{Alias, AliasBase, SetType},
        alias_setting::{self, AliasGroupSetting, AliasSetting, TomlSetting},
        error::AliasError,
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

const _DEFAULT_ROOT: &str = "%LocalAppData%/.rb-alias";
const DEFAULT_SETTING_FILE_PATH: &str = "%LocalAppData%/.rb-alias/alias-setting.toml";
const DEFAULT_SCRIPT_ROOT: &str = "%LocalAppData%/.rb-alias/script";

pub struct WindowsAlias {
    alias_base: AliasBase,
}

impl WindowsAlias {
    pub fn new(setting_path: Option<PathBuf>) -> Result<Self, AliasError> {
        let setting_path =
            setting_path.unwrap_or(PathBuf::from_str(DEFAULT_SETTING_FILE_PATH).unwrap());
        Ok(Self {
            alias_base: AliasBase::new(setting_path)?,
        })
    }

    fn get_script_root_env_var_name(&self) -> String {
        let global_setting = &self.alias_base.setting.global;
        if global_setting.script_root_env_var_name.is_some() {
            global_setting
                .script_root_env_var_name
                .as_ref()
                .unwrap()
                .to_string()
        } else {
            alias_setting::DEFAULT_SCRIPT_ROOT_ENV_VAR_NAME.to_owned()
        }
    }

    fn get_script_root_path(&self) -> String {
        let global_setting = &self.alias_base.setting.global;
        if global_setting.script_root.is_some() {
            global_setting.script_root.as_ref().unwrap().to_string()
        } else {
            DEFAULT_SCRIPT_ROOT.to_owned()
        }
    }

    /// 因为Windows的系统限制，PowerShell更便于实现逻辑，
    /// 但ps1脚本默认禁用，所以使用bat脚本执行PowerShell.exe
    /// 的方式绕过系统限制执行ps1命令
    fn commit_alias_script(&self) -> Result<(), AliasError> {
        let script_root_path = self.get_script_root_path();
        for (alias, set_cache) in &self.alias_base.set_buffer {
            let script_path = Path::new(&script_root_path).join(alias.to_owned() + ".bat");
            files::remove(&script_path)?;
            if set_cache.set_type == SetType::Set {
                let script = format!(
                    "PowerShell -ExecutionPolicy Bypass -Command {} ^$args",
                    windows_like::convert_to_bat_str_arg(
                        set_cache.setting.as_ref().unwrap().cmd.clone()
                    )
                );
                windows_like::create_ansi_file(&script_path, &script)?;
            }
        }
        Ok(())
    }

    fn set_script_to_env_var(&self) -> Result<(), AliasError> {
        // set 'script root' env var
        let var_name = self.get_script_root_env_var_name();
        let var_value = self.get_script_root_path();
        let old_var_value = windows_like::get_user_env_var(&var_name)?;
        if old_var_value.is_none() || old_var_value.unwrap() != var_value {
            windows_like::set_user_env_var(var_name.clone(), var_value)?;
        }
        // set 'path' env var
        let script_root_placeholder = format!("%{}%", var_name);
        match windows_like::get_user_env_var(&"Path".to_owned())? {
            Some(old_var_value) => {
                if !old_var_value.contains(&script_root_placeholder) {
                    windows_like::set_user_env_var(
                        "Path".to_owned(),
                        format!("{};{}", old_var_value, script_root_placeholder),
                    )?;
                }
            }
            None => {
                windows_like::set_user_env_var("Path".to_owned(), script_root_placeholder)?;
            }
        }
        Ok(())
    }
}

impl Alias for WindowsAlias {
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

    /*
    PSshell: 直接查询注册表（满足需求）
    (reg query "HKCU\Environment" /v "Path")[2] -split "    " | Select-Object -Last 1

    PSshell: 设置用户级环境变量（满足需求）
    [System. Environment]::SetEnvironmentVariable("新环境变量名", "旧变量值" + ";" + "新变量值", "USER")

    PSshell: 通过cmd执行ps1脚本绕过系统限制
    PowerShell -ExecutionPolicy Bypass -File ./xxx.ps1
    */
    fn commit(&mut self) -> Result<(), AliasError> {
        self.alias_base.commit()?; // commit setting
        self.commit_alias_script()?;
        self.set_script_to_env_var()?;
        self.alias_base.clear_cache();
        Ok(())
    }

    fn overwrite_setting(&mut self, setting: TomlSetting) -> Result<(), AliasError> {
        self.alias_base.overwrite_setting(&setting)
    }
}
