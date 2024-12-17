use env::consts::OS;
use std::env;

use crate::core::{
    alias::Alias,
    alias_manage::{AliasManage, AliasManager},
    error::AliasError,
};

use super::{linux::LinuxAlias, macos::MacosAlias, windows::WindowsAlias};

pub fn support_target_os() -> bool {
    OS == "macos" || OS == "linux" || OS == "windows"
}

pub fn get_alias() -> Result<Option<Box<dyn Alias>>, AliasError> {
    Ok(if OS == "macos" {
        Some(Box::new(MacosAlias::new()?))
    } else if OS == "linux" {
        Some(Box::new(LinuxAlias::new()?))
    } else if OS == "windows" {
        Some(Box::new(WindowsAlias::new()?))
    } else {
        None
    })
}

pub fn get_alias_manage() -> Result<Option<Box<dyn AliasManage>>, AliasError> {
    Ok(if OS == "macos" {
        Some(Box::new(AliasManager::new(Box::new(MacosAlias::new()?))))
    } else if OS == "linux" {
        Some(Box::new(AliasManager::new(Box::new(LinuxAlias::new()?))))
    } else if OS == "windows" {
        Some(Box::new(AliasManager::new(Box::new(WindowsAlias::new()?))))
    } else {
        None
    })
}
