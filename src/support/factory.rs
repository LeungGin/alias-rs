use super::{linux::LinuxAlias, macos::MacosAlias, windows::WindowsAlias};
use crate::core::{
    alias::Alias,
    alias_import::{AliasImport, AliasImporter},
    error::AliasError,
};
use env::consts::OS;
use std::{collections::HashMap, env, rc::Rc};

pub fn get_alias(
    setting_path: &Option<String>,
    runtime_variables: &HashMap<String, String>,
) -> Result<Option<Box<dyn Alias>>, AliasError> {
    Ok(if OS == "macos" {
        Some(Box::new(MacosAlias::new(setting_path, runtime_variables)?))
    } else if OS == "linux" {
        Some(Box::new(LinuxAlias::new(setting_path, runtime_variables)?))
    } else if OS == "windows" {
        Some(Box::new(WindowsAlias::new(
            setting_path,
            runtime_variables,
        )?))
    } else {
        None
    })
}

pub fn get_alias_importer(
    setting_path: &Option<String>,
    runtime_variables: &HashMap<String, String>,
) -> Result<Option<Box<dyn AliasImport>>, AliasError> {
    Ok(
        if let Some(alias) = get_alias(setting_path, runtime_variables)? {
            Some(Box::new(AliasImporter::new(Rc::new(alias))?))
        } else {
            None
        },
    )
}
