use crate::{
    cmn::{files, windows_like},
    core::{
        alias::Alias,
        alias_setting::{self, AliasSetting},
        error::{AliasError, ErrorKind},
    },
};
use std::collections::HashMap;

const DEFAULT_HOME: &str = "alias-rs";
const DEFAULT_SCRIPT_HOME_NAME: &str = "script";
const DEFAULT_SCRIPT_HOME_ENV_NAME: &str = "ALIAS_SCRIPT_HOME";
const DEFAULT_SETTING_NAME: &str = "alias-setting.toml";

pub fn get_default_home() -> String {
    windows_like::get_local_app_home() + "\\" + DEFAULT_HOME
}

pub fn get_default_script_home() -> String {
    get_default_home() + "\\" + DEFAULT_SCRIPT_HOME_NAME
}

pub fn get_default_setting_path() -> String {
    get_default_home() + "\\" + DEFAULT_SETTING_NAME
}

pub struct WindowsAlias {
    pub setting: AliasSetting,
}

impl WindowsAlias {
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
        if setting.script.home_env_name.is_none() {
            setting.script.home_env_name = Some(DEFAULT_SCRIPT_HOME_ENV_NAME.to_owned())
        }
        Ok(Self { setting })
    }

    fn build_alias_script_path(&self, alias: &String) -> String {
        format!(
            "{}\\{}.bat",
            self.setting.script.home.as_ref().unwrap(),
            alias
        )
    }
}

impl Alias for WindowsAlias {
    fn init(&self) -> Result<(), AliasError> {
        // set 'script home' env
        let home_name = self.setting.script.home_env_name.as_ref().unwrap();
        let home_value = self.setting.script.home.as_ref().unwrap();
        let old_home_value = windows_like::get_user_env_var(&home_name)?;
        if old_home_value.is_none() || &old_home_value.unwrap() != home_value {
            windows_like::set_user_env_var(home_name.clone(), home_value.clone())?;
        }
        // set 'Path' env
        let path_name = "Path".to_owned();
        let home_var_placeholder = format!("%{}%", home_name);
        match windows_like::get_user_env_var(&path_name)? {
            Some(old_var_value) => {
                if !old_var_value.contains(&home_var_placeholder) {
                    windows_like::set_user_env_var(
                        path_name,
                        format!("{};{}", old_var_value, home_var_placeholder),
                    )?;
                }
            }
            None => {
                windows_like::set_user_env_var(path_name, home_var_placeholder)?;
            }
        }
        Ok(())
    }

    fn setting(&self) -> AliasSetting {
        self.setting.clone()
    }

    fn set(&self, alias: String, command: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        let bat_script = format!(
            "PowerShell -ExecutionPolicy Bypass -Command {} ^$args",
            windows_like::convert_to_bat_str_arg(command)
        );
        windows_like::create_ansi_file(&alias_script_path, &bat_script)?;
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
