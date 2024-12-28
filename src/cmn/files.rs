use super::commands;

use crate::core::error::{AliasError, AliasErrorCode};

use std::{
    env::consts::OS,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

fn get_home_dir() -> Result<String, AliasError> {
    if OS == "macos" || OS == "linux" || OS == "windows" {
        let result = commands::execute(&"echo ~".to_owned())?;
        if !result.status.success() {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("get home dir fail :: {}", result.stdout),
            });
        }
        Ok(result.stdout)
    } else {
        Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("unsupported os :: {}", OS),
        })
    }
}

fn replace_home_dir_char(path: String) -> Result<String, AliasError> {
    if path.starts_with("~") {
        let home_dir = get_home_dir()?;
        if path.len() == 1 {
            Ok(home_dir)
        } else {
            Ok(home_dir + &path[1..path.len()])
        }
    } else {
        Ok(path.to_owned())
    }
}

fn to_path_buf(path: &String) -> Result<PathBuf, AliasError> {
    let path = replace_home_dir_char(path.to_owned())?;
    Ok(PathBuf::from_str(&path).unwrap())
}

pub fn create_if_absent(path: &String) -> Result<File, AliasError> {
    let path_buf = to_path_buf(path)?;
    if path_buf.exists() {
        return File::open(&path_buf).map_err(|e| AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("open file fail :: {} :: {}", path_buf.display(), e),
        });
    }
    create(path)
}

pub fn create(path: &String) -> Result<File, AliasError> {
    let path_buf = to_path_buf(path)?;
    if let Some(parent) = path_buf.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("create dir fail :: {} :: {}", parent.display(), e),
            });
        }
    }
    return File::create(&path_buf).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("create file fail :: {} :: {}", path_buf.display(), e),
    });
}

pub fn remove(path: &String) -> Result<(), AliasError> {
    let path_buf = to_path_buf(path)?;
    fs::remove_file(path_buf.display().to_string()).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("remove file fail :: {} :: {}", path_buf.display(), e),
    })
}

pub fn overwrite(
    file: &mut File,
    path_string: &String,
    content: &String,
) -> Result<(), AliasError> {
    overwrite_with_bytes(file, path_string, content.as_bytes())
}

pub fn overwrite_with_bytes(
    file: &mut File,
    path_string: &String,
    content: &[u8],
) -> Result<(), AliasError> {
    if let Err(e) = file.write_all(content) {
        return Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("rewrite file fail :: {} :: {}", path_string, e),
        });
    }
    Ok(())
}
