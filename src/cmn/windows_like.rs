use crate::core::error::{AliasError, ErrorKind};
use encoding_rs::GBK;
use std::{
    io::Write,
    process::{Command, ExitStatus},
};

use super::files;

pub fn get_local_app_home() -> String {
    std::env::var("LocalAppData").map_or(String::default(), |val| val)
}

pub fn create_ansi_file(path: &String, content: &String) -> Result<(), AliasError> {
    let encoded_str = GBK.encode(&content).0;
    files::create_with_all_dir(path)
        .and_then(|mut f| f.write_all(&encoded_str))
        .map_err(|e| AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!(" create ansi file fail :: {}", e),
        })?;
    Ok(())
}

pub struct ExecuteCmdResult {
    pub status: ExitStatus,
    pub stdout: String,
}

impl ExecuteCmdResult {
    pub fn get_stdout_vec(&self) -> Option<Vec<String>> {
        let result: Vec<String> = self
            .stdout
            .split("\r\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

pub fn execute_cmd(cmd: &String) -> Result<ExecuteCmdResult, AliasError> {
    let output = match Command::new("cmd").args(["/C", cmd]).output() {
        Ok(out) => out,
        Err(e) => {
            return Err(AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("execute cmd fail :: {} :: {}", cmd, e),
            })
        }
    };
    let stdout = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };
    let (decoded_str, _, _) = GBK.decode(&stdout);
    Ok(ExecuteCmdResult {
        status: output.status,
        stdout: decoded_str.to_string(),
    })
}

pub fn execute_cmd_in_powershell(cmd: &String) -> Result<ExecuteCmdResult, AliasError> {
    // comand execute in cmd.exe can not use '$args'
    // (for example, PowerShell -ExecutionPolicy Bypass -Command xxx &args),
    // only supported in script
    let cmd = format!(
        "PowerShell -ExecutionPolicy Bypass -Command {}",
        convert_to_bat_str_arg(cmd.to_string())
    );
    execute_cmd(&cmd)
}

pub fn convert_to_bat_str_arg(string_arg: String) -> String {
    string_arg.replace("|", "^|").replace("$", "^$")
}

pub fn user_env_var_exist(var_name: &String) -> Result<bool, AliasError> {
    let ps_cmd = format!("(Get-ItemProperty -Path 'HKCU:\\Environment').{}", var_name);
    let result = execute_cmd_in_powershell(&ps_cmd)?;
    if result.status.success() {
        Ok(!result.stdout.is_empty())
    } else {
        Err(AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!(
                "verify user environment variable exist fail :: {} :: {}",
                var_name, result.stdout
            ),
        })
    }
}

pub fn get_user_env_var(var_name: &String) -> Result<Option<String>, AliasError> {
    if !user_env_var_exist(var_name)? {
        return Ok(None);
    }
    let ps_cmd = format!(
        "(reg query 'HKEY_CURRENT_USER\\Environment' /v '{}')[2].ToString()",
        var_name
    );
    let result = execute_cmd_in_powershell(&ps_cmd)?;
    if result.status.success() {
        if let Some(stdout_vec) = result.get_stdout_vec() {
            Ok(Some(
                stdout_vec
                    .get(0)
                    .unwrap()
                    .splitn(4, "    ")
                    .last()
                    .unwrap()
                    .to_owned(),
            ))
        } else {
            Ok(None)
        }
    } else {
        Err(AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!(
                "get user environment variable fail :: {} :: {}",
                var_name, result.stdout
            ),
        })
    }
}

pub fn set_user_env_var(var_name: String, var_value: String) -> Result<(), AliasError> {
    let ps_cmd = format!(
        "[System.Environment]::SetEnvironmentVariable('{}', '{}', 'USER')",
        var_name, var_value
    );
    let result = execute_cmd_in_powershell(&ps_cmd)?;
    if !result.status.success() {
        return Err(AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!(
                "set user environment variable fail :: {}={} :: {}",
                var_name, var_value, result.stdout
            ),
        });
    }
    Ok(())
}
