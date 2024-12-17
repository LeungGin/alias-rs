use crate::core::error::{AliasError, AliasErrorCode};
use std::{env, fs::File, path::PathBuf};

pub enum Shell {
    Supported(String, String),
    Unsupported(String),
}

pub fn get_shell_type() -> Result<Shell, AliasError> {
    match env::var("SHELL") {
        Ok(shell) => Ok(match shell.as_str() {
            "/bin/zsh" => Shell::Supported(shell, String::from("~/.zshrc")),
            "/bin/bash" => Shell::Supported(shell, String::from("~/.bashrc")),
            "/bin/ksh" => Shell::Supported(shell, String::from("~/.kshrc")),
            "/bin/csh" => Shell::Supported(shell, String::from("~/.cshrc")),
            "/bin/dash" => Shell::Supported(shell, String::from("~/.bashrc")),
            "/bin/tcsh" => Shell::Supported(shell, String::from("~/.tcshrc")),
            "/bin/sh" => todo!(), // /bin/sh 视连接的实际shell类型而定，通过【ps -p $$】结果中的CMD字段判断，如字段值为【-zsh】，负号无意义，表示/bin/zsh
            _ => Shell::Unsupported(shell),
        }),
        Err(e) => Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("get shell type fail :: {}", e),
        }),
    }
}

pub fn get_shell_profile() -> Result<(String, File), AliasError> {
    match get_shell_type()? {
        Shell::Supported(_shell_name, shell_profile_path) => {
            let profile_path = PathBuf::from(&shell_profile_path);
            if profile_path.exists() {
                return match File::open(&profile_path) {
                    Ok(f) => Ok((shell_profile_path, f)),
                    Err(e) => Err(AliasError {
                        err: AliasErrorCode::Unkonw,
                        msg: format!(
                            "open shell profile file fail :: {} :: {}",
                            shell_profile_path, e
                        ),
                    }),
                };
            }
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("shell profile not exists :: {}", shell_profile_path),
            });
        }
        Shell::Unsupported(shell_name) => Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("unsupported shell type :: {}", shell_name),
        }),
    }
}
