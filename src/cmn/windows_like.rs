use crate::{
    cmn::files,
    core::error::{AliasError, AliasErrorCode},
};
use encoding_rs::GBK;
use std::{
    fs,
    process::{Command, ExitStatus},
};

pub fn create_ansi_file(path: &String, content: &String) -> Result<(), AliasError> {
    if !fs::exists(path).unwrap_or(false) {
        return Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("file exists :: {}", path),
        });
    }
    let mut file = files::create(path)?;
    let (encoded_str, _, _) = GBK.encode(&content);
    files::overwrite_with_bytes(&mut file, path, &encoded_str)?;
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
    let output = match Command::new("cmd").args(&["/C", cmd]).output() {
        Ok(out) => out,
        Err(e) => {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
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
    let cmd = format!(
        "PowerShell -ExecutionPolicy Bypass -Command {} ^$args",
        convert_to_bat_str_arg(cmd.to_string())
    );
    execute_cmd(&cmd)
}

pub fn convert_to_bat_str_arg(string_arg: String) -> String {
    string_arg.replace("|", "^|").replace("$", "^$")
}

pub fn get_user_env_var(var_name: &String) -> Result<Option<String>, AliasError> {
    let ps_cmd = format!(
        "(reg query \"HKCU\\Environment\" /v \"{}\")[2] -split \"    \" | Select-Object -Last 1",
        var_name
    );
    let result = execute_cmd_in_powershell(&ps_cmd)?;
    if result.status.success() {
        match result.get_stdout_vec() {
            Some(outputs) => Ok(Some(outputs.get(0).unwrap().to_string())),
            None => Ok(None),
        }
    } else {
        Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!(
                "get user environment variable fail :: {} :: {}",
                var_name, result.stdout
            ),
        })
    }
}

pub fn set_user_env_var(var_name: String, var_value: String) -> Result<(), AliasError> {
    let ps_cmd = format!(
        "[System.Environment]::SetEnvironmentVariable(\"{}\", \"{}\", \"USER\")",
        var_name, var_value
    );
    let result = execute_cmd_in_powershell(&ps_cmd)?;
    if !result.status.success() {
        return Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!(
                "set user environment variable fail :: {}={} :: {}",
                var_name, var_value, result.stdout
            ),
        });
    }
    Ok(())
}
