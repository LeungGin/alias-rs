use super::cmd::{Cli, Command::*};
use crate::{
    core::error::{AliasError, AliasErrorCode},
    support::factory::{get_alias, get_alias_importer},
};
use clap::Parser;
use std::{
    collections::HashMap,
    env::{self, consts::OS},
};

pub fn parse() -> Result<(), AliasError> {
    if !support_target_os() {
        eprintln!("unsupport os :: {}", env::consts::OS);
        return Err(AliasError::new(AliasErrorCode::Unkonw, String::from("")));
    }

    let cli = Cli::parse();
    let setting_path = cli.setting_path;
    let runtime_variables = runtime_variables_vec_to_map(cli.runtime_variables)?;

    match cli.command {
        Init {} => {
            let alias_impl = get_alias(&setting_path, &runtime_variables)?.unwrap();
            alias_impl.init()?;
        }
        Set { alias, command } => {
            let alias_impl = get_alias(&setting_path, &runtime_variables)?.unwrap();
            alias_impl.set(alias, command)?;
        }
        Remove { alias } => {
            let alias_impl = get_alias(&setting_path, &runtime_variables)?.unwrap();
            alias_impl.remove(alias)?;
        }
        List {} => {
            let alias_impl = get_alias(&setting_path, &runtime_variables)?.unwrap();
            let list = alias_impl.list()?;
            if let Some(list) = list {
                for alias in list {
                    println!("{}", alias);
                }
            }
        }
        Export { export_path: _ } => {
            todo!();
        }
        Import {} => {
            let alias_importer = get_alias_importer(&setting_path, &runtime_variables)?.unwrap();
            alias_importer.import()?;
        }
    }
    println!("done");
    return Ok(());
}

fn support_target_os() -> bool {
    OS == "macos" || OS == "linux" || OS == "windows"
}

fn runtime_variables_vec_to_map(
    kv_variables: Vec<String>,
) -> Result<HashMap<String, String>, AliasError> {
    let mut map: HashMap<String, String> = HashMap::new();
    for kv in kv_variables {
        let split: Vec<&str> = kv.split('=').collect();
        if split.len() != 2 {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!(
                    "runtime variables define should be like \"--define key=value\" :: {}",
                    kv
                ),
            });
        }
        map.insert(split[0].to_owned(), split[1].to_owned());
    }
    Ok(map)
}
