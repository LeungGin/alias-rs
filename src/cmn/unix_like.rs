use crate::core::error::{AliasError, ErrorKind};
use std::{env, fs::File, path::PathBuf};

pub fn get_home() -> String {
    std::env::var("HOME").map_or(String::default(), |val| val)
}

pub enum Shell {
    Supported(String, String),
    Unsupported(String),
}

pub fn get_shell_type() -> Result<Shell, AliasError> {
    match env::var("SHELL") {
        Ok(shell) => Ok(match shell.as_str() {
            "/bin/zsh" => Shell::Supported(shell, format!("{}/.zshrc", get_home())),
            "/bin/bash" => Shell::Supported(shell, format!("{}/.bashrc", get_home())),
            "/bin/ksh" => Shell::Supported(shell, format!("{}/.kshrc", get_home())),
            "/bin/csh" => Shell::Supported(shell, format!("{}/.cshrc", get_home())),
            "/bin/dash" => Shell::Supported(shell, format!("{}/.bashrc", get_home())),
            "/bin/tcsh" => Shell::Supported(shell, format!("{}/.tcshrc", get_home())),
            "/bin/sh" => todo!(), // TODO /bin/sh 视连接的实际shell类型而定，通过【ps -p $$】结果中的CMD字段判断，如字段值为【-zsh】，负号无意义，表示/bin/zsh
            _ => Shell::Unsupported(shell),
        }),
        Err(e) => Err(AliasError {
            kind: ErrorKind::Unkonw,
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
                        kind: ErrorKind::Unkonw,
                        msg: format!(
                            "open shell profile file fail :: {} :: {}",
                            shell_profile_path, e
                        ),
                    }),
                };
            }
            return Err(AliasError {
                kind: ErrorKind::Unkonw,
                msg: format!("shell profile not exists :: {}", shell_profile_path),
            });
        }
        Shell::Unsupported(shell_name) => Err(AliasError {
            kind: ErrorKind::Unkonw,
            msg: format!("unsupported shell type :: {}", shell_name),
        }),
    }
}
