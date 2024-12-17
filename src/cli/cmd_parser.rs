use std::env;

use clap::Parser;

use crate::{
    core::{
        alias_setting::AliasSetting,
        error::{AliasError, AliasErrorCode},
    },
    support::factory::{get_alias, get_alias_manage, support_target_os},
};

use super::cmd::{Cli, Command::*};

pub fn parse() -> Result<(), AliasError> {
    if !support_target_os() {
        eprintln!("unsupport os :: {}", env::consts::OS);
        return Err(AliasError::new(AliasErrorCode::Unkonw, String::from("")));
    }
    match Cli::parse().command {
        Set {
            alias,
            command,
            group,
        } => {
            let group_name = match group {
                Some(group) => group,
                None => "default".to_owned(),
            };
            let mut alias_impl = get_alias()?.unwrap();
            alias_impl.set(group_name, alias, AliasSetting { cmd: command })?;
            alias_impl.commit()?;
        }
        Remove { alias } => {
            let mut alias_impl = get_alias()?.unwrap();
            alias_impl.remove(&"default".to_owned(), &alias)?;
            alias_impl.commit()?;
        }
        List {} => {
            let list = get_alias()?.unwrap().get_all()?;
            let mut total = 0;
            for (group, group_setting) in &list {
                total += group_setting.mapping.len();
                println!("group: {}", group);
                for (alias, alias_setting) in &group_setting.mapping {
                    println!("    {} -> {}", alias, alias_setting.cmd);
                }
            }
            println!("total {}", total);
        }
        Clear {} => {
            get_alias()?.unwrap().clear()?;
            get_alias_manage()?.unwrap().rebuild()?; // TODO clear实现存在bug，临时解决方案
        }
        Export { export_path } => {
            get_alias_manage()?.unwrap().export(&export_path)?;
            println!("export see: {}", export_path);
        }
        Import { import_path } => {
            get_alias_manage()?.unwrap().import(&import_path)?;
        }
        Rebuild {} => {
            get_alias_manage()?.unwrap().rebuild()?;
        }
    }
    println!("done");
    return Ok(());
}
