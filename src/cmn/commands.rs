use super::windows_like;
use crate::core::error::{AliasError, ErrorKind};
use encoding_rs::UTF_8;
use std::{
    env::consts::OS,
    process::{Command, ExitStatus},
};

pub struct ExecuteResult {
    pub status: ExitStatus,
    pub stdout: String,
}

impl ExecuteResult {
    pub fn _get_stdout_vec(&self) -> Option<Vec<String>> {
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

pub fn execute(cmd: &String) -> Result<ExecuteResult, AliasError> {
    if OS == "macos" || OS == "linux" {
        execute_unix(cmd)
    } else if OS == "windows" {
        let win_result = windows_like::execute_cmd(cmd)?;
        Ok(ExecuteResult {
            status: win_result.status,
            stdout: win_result.stdout,
        })
    } else {
        Err(AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("unsupported os :: {}", OS),
        })
    }
}

fn execute_unix(cmd: &String) -> Result<ExecuteResult, AliasError> {
    let output = match Command::new("sh").arg("-c").arg(cmd).output() {
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
    let (decoded_str, _, _) = UTF_8.decode(&stdout);
    Ok(ExecuteResult {
        status: output.status,
        stdout: decoded_str.to_string(),
    })
}
