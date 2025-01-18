use crate::{
    cmn::{files, windows_like},
    core::{
        alias::Alias,
        alias_setting::{AliasSetting, AliasSettingLoader},
        error::AliasError,
    },
};
use std::collections::HashMap;

const _DEFAULT_HOME: &str = "%LocalAppData%/.alias-rs";
const DEFAULT_SCRIPT_HOME: &str = "%LocalAppData%/.alias-rs/script";
const DEFAULT_SCRIPT_HOME_ENV_NAME: &str = "ALIAS_SCRIPT_HOME";
const DEFAULT_SETTING_PATH: &str = "%LocalAppData%/.alias-rs/alias-setting.toml";

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
            .map_or(DEFAULT_SETTING_PATH.to_owned(), |f| f.to_owned());
        let mut setting_loader = AliasSettingLoader::new(&setting_path, &runtime_variables)?;
        if setting_loader.setting.script.home.is_none() {
            setting_loader.setting.script.home = Some(DEFAULT_SCRIPT_HOME.to_owned());
        }
        if setting_loader.setting.script.home_env_name.is_none() {
            setting_loader.setting.script.home_env_name =
                Some(DEFAULT_SCRIPT_HOME_ENV_NAME.to_owned())
        }
        Ok(Self {
            setting: setting_loader.setting,
        })
    }

    fn build_alias_script_path(&self, alias: &String) -> String {
        format!(
            "{}/{}.bat",
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
        files::remove(&alias_script_path)?;
        let bat_script = format!(
            "PowerShell -ExecutionPolicy Bypass -Command {} ^$args",
            windows_like::convert_to_bat_str_arg(command)
        );
        windows_like::create_ansi_file(&alias_script_path, &bat_script)?;
        Ok(())
    }

    fn remove(&self, alias: String) -> Result<(), AliasError> {
        let alias_script_path = self.build_alias_script_path(&alias);
        files::remove(&alias_script_path)?;
        Ok(())
    }

    fn list(&self) -> Result<Option<Vec<String>>, AliasError> {
        files::list_dir(&self.setting.script.home.as_ref().unwrap())
    }
}
